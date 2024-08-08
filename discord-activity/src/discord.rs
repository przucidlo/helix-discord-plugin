use std::io::Error as IoError;
use std::str::FromStr;
use std::{env, path::PathBuf};

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
                    .invoke(Packet::HANDSHAKE(1003450375732482138))
                    .expect("Fail");

                if matches!(packet, Packet::FRAME(_)) {
                    println!("{:?}", String::from_utf8(packet.payload()));
                }

                return Ok(Self { socket });
            },
            Err(_) => panic!("Could not find TMPDIR variable"),
        };
    }
}
