use std::{str::FromStr, convert::TryFrom, fmt::Display};
use crate::Error;

/// PNG Chunk Type
#[derive(Debug, Eq, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4]
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    
    fn is_valid(&self) -> bool {
        todo!();
    }

    fn is_critical(&self) -> bool {
        todo!();
    }

    fn is_public(&self) -> bool {
        todo!();
    }

    fn is_reserved_bit_valid(&self) -> bool {
        todo!();
    }

    fn is_safe_to_copy(&self) -> bool {
        todo!();
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self {
            bytes: value
        })
    }
}

impl FromStr for ChunkType {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() != 4 {
            return Err(Box::new(ChunkError::ByteLengthError(bytes.len())))
        }

        let sized: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        Ok(ChunkType::try_from(sized)?)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(std::fmt::Error)
        }
    }
}

/// Chunk type errors
#[derive(Debug)]
pub enum ChunkError {
    /// Chunk has incorrect number of bytes (4 expected)
    ByteLengthError(usize)
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::ByteLengthError(actual) => write!(f, "Expected 4 bytes but received {} when creating chunk type", actual)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }
}