#![allow(unused_variables)]
#![allow(unused)]
use std::fmt;
use std::str::FromStr;
use std::str;


use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    pub contents: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        Result::Ok(ChunkType{contents: bytes})
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match str::from_utf8(&self.contents) {
            Ok(v) => v,
            _ => "",
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.details)
    }
}

impl std::error::Error for ParseError {}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        for byte in s.as_bytes() {
            if !byte.is_ascii_alphabetic() {
               return Err(Box::new(ParseError {details: "Input string must be alphabetic".to_string(),}));
            }
        }
        Ok(ChunkType{contents: s.as_bytes().try_into()?})
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.contents
    }

    pub fn is_critical(&self) -> bool {
        let png_bytes = &self.contents;
        let critical_info = &png_bytes[0];
        ((critical_info >> 5) & 1) == 0
    }

    pub fn is_public(&self) -> bool {
        let png_bytes = &self.contents;
        let critical_info = &png_bytes[1];
        ((critical_info >> 5) & 1) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let png_bytes: &[u8; 4] = &self.contents;
        let critical_info = &png_bytes[2];
        critical_info.is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let png_bytes = &self.contents;
        let critical_info = &png_bytes[3];
        ((critical_info >> 5) & 1) == 1
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_safe_to_copy() {
            return false;
        }
        let png_bytes = &self.contents;
        for byte in png_bytes {
            if !byte.is_ascii_alphabetic() {
                return false;
            }
        }
        true
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

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
