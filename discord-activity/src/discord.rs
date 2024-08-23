use core::panic;
use std::io::Error as IoError;
use std::str::FromStr;
use std::thread;
use std::{env, path::PathBuf};

use crate::message::Message;
use crate::{packet::Packet, socket::Socket};

pub struct Discord {
    socket: Socket,
}

impl Discord {
    pub fn start() -> Result<Self, IoError> {
        let unix_path = env::var("TMPDIR");

        match unix_path {
            Ok(unix_path) => loop {
                let pathbuf = PathBuf::from_str(&format!("{unix_path}discord-ipc-0"))
                    .expect("Could not build socket path");

                let mut socket = Socket::connect(pathbuf).expect("Could not connect to socket");

                let packet = socket
                    .invoke(Packet::HANDSHAKE(1274291961792167997))
                    .expect("Fail");

                if matches!(packet, Packet::FRAME(_)) {
                    let message: Message = packet.payload().try_into().unwrap();

                    if !message.evt_matches("READY") {
                        panic!("Handshake failure")
                    }
                }

                let mut discord = Self { socket };

                discord.listen()?;

                return Ok(discord);
            },
            Err(_) => panic!("Could not find TMPDIR variable"),
        };
    }

    pub fn publish_file_activity(&mut self, file: &str) {
        self.socket
            .write(Message::file_activity(file).into())
            .unwrap();
    }

    fn listen(&mut self) -> Result<(), IoError> {
        let mut socket = self.socket.try_clone()?;

        let handle = thread::spawn(move || loop {
            let mut response = [0; 1024];

            if !socket.read(&mut response).is_ok() {
                break;
            }

            if response.len() == 0 {
                break;
            }

            let Ok(packet) = Packet::try_from(response.to_vec()) else {
                continue;
            };

            match packet {
                Packet::HANDSHAKE(_) => continue,
                Packet::FRAME(_) => {
                    continue;
                }
                Packet::CLOSE => break,
                Packet::PING(p) => match socket.write(Packet::PONG(p)) {
                    Ok(_) | Err(_) => continue,
                },
                Packet::PONG(_) => {
                    continue;
                }
            }
        });

        self.publish_file_activity("discord.rs");

        handle.join();

        Ok(())
    }
}
