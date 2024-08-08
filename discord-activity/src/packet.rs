use serde_json::json;
use std::{
    error::Error,
    io::{Error as IoError, ErrorKind},
};
use uuid::Uuid;

pub enum Packet {
    HANDSHAKE(u64),
    FRAME(Vec<u8>),
    CLOSE,
    PING,
    PONG,
}

impl Packet {
    pub fn as_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend(&self.header());
        bytes.extend(&self.payload());

        bytes
    }

    pub fn header(&self) -> Vec<u8> {
        match self {
            Packet::HANDSHAKE(_) => {
                let mut bytes: Vec<u8> = vec![];

                bytes.extend(u32::to_le_bytes(0 as u32));
                bytes.extend(u32::to_le_bytes(self.payload_len()));

                bytes
            }
            Packet::FRAME(_p) => todo!(),
            Packet::CLOSE => todo!(),
            Packet::PING => todo!(),
            Packet::PONG => todo!(),
        }
    }

    pub fn payload(&self) -> Vec<u8> {
        match self {
            Packet::HANDSHAKE(client_id) => json!({
                "client_id": client_id.to_string(),
                "v": 1,
                "nonce": Uuid::new_v4().to_string()
            })
            .to_string()
            .as_bytes()
            .to_owned(),
            Packet::FRAME(p) => p.to_owned(),
            Packet::CLOSE => todo!(),
            Packet::PING => todo!(),
            Packet::PONG => todo!(),
        }
    }

    pub fn payload_len(&self) -> u32 {
        u32::try_from(self.payload().len()).unwrap()
    }

    pub fn parse(value: Vec<u8>) -> Result<(u32, usize, Vec<u8>), Box<dyn Error>> {
        if value.len() < 8 {
            return Err(Box::from(IoError::from(ErrorKind::InvalidInput)));
        }

        let header = u32::from_le_bytes(value[0..4].try_into()?);
        let len: usize = u32::from_le_bytes(value[4..8].try_into()?) as usize;
        let mut payload = value[8..].to_vec();

        if payload.len() > len {
            payload.resize(len, 0);
        } else if payload.len() < len {
            return Err(Box::from(IoError::from(ErrorKind::InvalidData)));
        }

        return Ok((header, len, payload));
    }
}

impl TryFrom<Vec<u8>> for Packet {
    type Error = IoError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let (header, _, payload) = Self::parse(value).unwrap();

        match header {
            0 => {
                todo!()
            }
            1 => {
                return Ok(Self::FRAME(payload));
            }
            2 => {
                return Ok(Self::CLOSE);
            }
            3 => {
                return Ok(Self::PING);
            }
            4 => {
                return Ok(Self::PONG);
            }
            _ => {
                return Err(IoError::from(ErrorKind::InvalidData));
            }
        };
    }
}
