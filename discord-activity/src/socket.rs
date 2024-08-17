use core::time;
use std::{
    io::{Error as IoError, ErrorKind, Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
    thread,
};

use crate::packet::Packet;

pub struct Socket {
    socket: UnixStream,
}

impl Socket {
    pub fn connect(path: PathBuf) -> Result<Self, IoError> {
        let socket = UnixStream::connect(path)?;
        socket.set_nonblocking(true)?;
        socket.set_read_timeout(Some(time::Duration::from_secs(30)))?;
        socket.set_write_timeout(Some(time::Duration::from_secs(30)))?;

        Ok(Self { socket })
    }

    pub fn invoke(&mut self, packet: Packet) -> Result<Packet, IoError> {
        let mut response = [0; 1024];

        self.socket.write_all(&packet.as_bytes())?;

        self.read(&mut response)?;

        Ok(Packet::try_from(response.to_vec())?)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, IoError> {
        loop {
            match self.socket.read(buffer) {
                Ok(size) => return Ok(size),
                Err(e) => match e.kind() {
                    ErrorKind::WouldBlock => {}
                    ErrorKind::Interrupted => {}
                    _ => return Err(e),
                },
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }

    pub fn write(&mut self, packet: Packet) -> Result<(), IoError> {
        self.socket.write_all(&packet.as_bytes())
    }

    pub fn try_clone(&self) -> Result<Self, IoError> {
        let socket = self.socket.try_clone()?;

        Ok(Self { socket })
    }
}
