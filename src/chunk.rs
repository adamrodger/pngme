use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};

/// Represents a single chunk in the PNG spec
pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    /// Create a new chunk
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self { chunk_type, data }
    }

    /// Length of the chunk
    fn length(&self) -> usize {
        self.data.len()
    }

    /// Chunk type
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Chunk data
    fn data(&self) -> &[u8] {
        &self.data
    }

    /// CRC of the entire chunk
    fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();
        crc::crc32::checksum_ieee(&bytes)
    }

    /// Chunk data as a string
    pub fn data_as_string(&self) -> Result<String> {
        let s = std::str::from_utf8(&self.data)?;
        Ok(s.to_string())
    }

    /// Entire chunk represented as bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        self.chunk_type
            .bytes()
            .iter()
            .chain(&self.data)
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return Err(Box::from(ChunkError::InputTooSmall));
        }

        // consume first 4 bytes as data length
        let (data_length, value) = value.split_at(4);
        let data_length = u32::from_be_bytes(data_length.try_into()?) as usize;

        // consume next 4 bytes as chunk type
        let (chunk_type_bytes, value) = value.split_at(4);

        let chunk_type_bytes: [u8; 4] = chunk_type_bytes.try_into()?;
        let chunk_type: ChunkType = ChunkType::try_from(chunk_type_bytes)?;

        if !chunk_type.is_valid() {
            return Err(Box::from(ChunkError::InvalidChunkType));
        }

        // final 4 bytes are CRC, everything before that is data
        if data_length != value.len() - 4 {
            return Err(Box::from(ChunkError::InvalidDataLength));
        }

        let (data, crc_bytes) = value.split_at(data_length);

        // validate CRC
        let new = Self {
            chunk_type,
            data: data.into(),
        };

        let actual_crc = new.crc();
        let expected_crc = u32::from_be_bytes(crc_bytes.try_into()?);

        if expected_crc != actual_crc {
            return Err(Box::from(ChunkError::InvalidCrc(expected_crc, actual_crc)));
        }

        Ok(new)
    }
}

#[derive(Debug)]
pub enum ChunkError {
    InputTooSmall,
    InvalidCrc(u32, u32),
    InvalidChunkType,
    InvalidDataLength,
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ChunkError::InputTooSmall => {
                write!(f, "At least 12 bytes must be supplied to construct a chunk")
            }
            ChunkError::InvalidCrc(expected, actual) => write!(
                f,
                "Invalid CRC when constructing chunk. Expected {} but found {}",
                expected, actual
            ),
            ChunkError::InvalidChunkType => write!(f, "Invalid chunk type"),
            ChunkError::InvalidDataLength => write!(f, "Invalid data length"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!"
            .bytes()
            .collect();
        Chunk::new(chunk_type, data)
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = b"RuSt";
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string: String =
            String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = b"RuSt";
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }
}
