use rand::prelude::*;
use std::{
    error::Error,
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

pub struct WebsocketClient {
    writer: Option<BufWriter<TcpStream>>,
}

struct Frame {
    payload: Vec<u8>,
}

impl Frame {
    fn new(payload: String) -> Self {
        Self {
            payload: payload.as_bytes().to_owned(),
        }
    }

    pub fn as_bytes(self) -> Vec<u8> {
        let mut bytes = vec![];
        let mut masking_key = [0u8; 4];

        rand::thread_rng().fill(&mut masking_key);

        let payload: Vec<u8> = self
            .payload
            .iter()
            .enumerate()
            .map(|(i, byte)| return byte ^ masking_key[i % 4])
            .collect();

        bytes.push(0x81);
        bytes.push((payload.len() | 0x80) as u8);
        bytes.extend_from_slice(&masking_key);
        bytes.extend_from_slice(&payload);

        bytes
    }
}

impl WebsocketClient {
    pub fn new() -> Self {
        Self { writer: None }
    }

    pub fn connect(&mut self, address: &str) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect(address);

        match stream {
            Ok(stream) => {
                let mut writer = BufWriter::new(stream.try_clone()?);
                let mut reader = BufReader::new(stream);

                match self.handshake(&mut writer, &mut reader) {
                    Ok(_) => {
                        println!("Handshake successful");

                        self.writer = Some(writer);
                    }
                    Err(_) => {
                        println!("Handshake failed");
                    }
                }
            }
            Err(_) => {
                println!("fail");
            }
        }

        return Ok(());
    }

    pub fn send(&mut self, payload: String) {
        match &mut self.writer {
            Some(writer) => {
                let frame = Frame::new(payload);

                writer.write(&frame.as_bytes());
                writer.flush();
            }
            None => panic!(),
        }
    }

    fn handshake(
        &self,
        writer: &mut BufWriter<TcpStream>,
        reader: &mut BufReader<TcpStream>,
    ) -> Result<(), Box<dyn Error>> {
        const SWITCH_PROTOCOLS_HEADER: &str = "HTTP/1.1 101 Switching Protocols\r\n";

        let mut response: Vec<String> = Vec::new();

        writer.write(self.handshake_message().as_bytes())?;
        writer.flush()?;

        // This is pretty naive approach, but it's fine for now
        loop {
            let mut line = String::new();

            reader.read_line(&mut line)?;

            response.push(line.clone());

            if line == "\r\n" {
                break;
            }
        }

        if let Some(line) = response.first() {
            if line == SWITCH_PROTOCOLS_HEADER {
                return Ok(());
            } else {
                panic!();
            }
        }

        panic!();
    }

    fn handshake_message(&self) -> String {
        vec![
            "GET / HTTP/1.1",
            "Host: example.com",
            "Connection: Upgrade",
            "Upgrade: websocket",
            "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==",
            "Sec-WebSocket-Version: 13",
            "\r\n",
        ]
        .join("\r\n")
    }
}
