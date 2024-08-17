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
                let pathbuf = PathBuf::from_str(&format!("{}discord-ipc-0", unix_path))
                    .expect("Could not build socket path");

                let mut socket = Socket::connect(pathbuf).expect("Could not connect to socket");

                let packet = socket
                    .invoke(Packet::HANDSHAKE(1274291961792167997))
                    .expect("Fail");

                if matches!(packet, Packet::FRAME(_)) {
                    //TODO: Make sure handshake was successful
                }

                let mut discord = Self { socket };

                discord.listen()?;

                return Ok(discord);
            },
            Err(_) => panic!("Could not find TMPDIR variable"),
        };
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
                Packet::PING => {
                    println!("Received ping");
                }
                Packet::PONG => {
                    println!("Received pong");
                }
            }
        });

        self.socket.write(Message::idle_activity().into()).unwrap();

        handle.join();

        Ok(())
    }
}
