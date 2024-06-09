#![allow(unused_variables)]

use crate::{chunk_type, Error, Result};
use crate::chunk_type::ChunkType;
use std::io::Read;
use std::str::{self, Bytes, FromStr};
use crc32fast;
use std::fmt;

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

#[derive(Debug, Clone)]
pub struct Chunk{
    pub data_length: u32,
    pub chunk_type: ChunkType,
    pub message_bytes: Vec<u8>,
    pub crc: u32,
}

impl TryFrom<&Vec<u8>> for Chunk {
    type Error = Error;
    fn try_from(data: &Vec<u8>) -> Result<Self> {
        let data_length = &data[0..4];
        let [b1, b2, b3, b4]: [u8;4] = data_length.try_into().map(|x| x).ok().unwrap();
        let data_length = u32::from_be_bytes([b1, b2, b3, b4]);
        let chunk_type = ChunkType::from_str(str::from_utf8(&data[4..8]).unwrap()).unwrap();
        let message_bytes = &data[8..data.len()-4];
        let crc = &data[data.len()-4..];
        let [b1, b2, b3, b4]: [u8;4] = crc.try_into().map(|x| x).ok().unwrap();
        let crc = u32::from_be_bytes([b1, b2, b3, b4]);
        if crc != Chunk::get_crc(&chunk_type, &message_bytes.to_vec()) {
            return Err(Box::new(ParseError {details: "error crc checksum".to_string()}))
        }
        Ok(Chunk{
            data_length: data_length,
            chunk_type: chunk_type,
            message_bytes: message_bytes.to_vec(),
            crc: crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Chunk::get_crc(&chunk_type, &data);
        Chunk {
            data_length: data.len() as u32,
            chunk_type: chunk_type,
            message_bytes: data,
            crc: crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.data_length
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.message_bytes
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn get_crc(chunk_type: &ChunkType, data: &Vec<u8>) -> u32 {
        let var: Vec<u8> = chunk_type.bytes().as_ref().iter().chain(data.as_slice().iter()).copied().collect();
        let crc = crc32fast::hash(var.as_slice());
        crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.message_bytes.clone())?)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        let data_length_iter = self.data_length.to_be_bytes();
        let chunk_type_iter = self.chunk_type.bytes();
        let message_bytes_iter = &self.message_bytes;
        let crc_iter = self.crc.to_be_bytes();
        data_length_iter.iter()
            .chain(chunk_type_iter.iter())
            .chain(message_bytes_iter.iter())
            .chain(crc_iter.iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
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
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
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
        let chunk_type = "RuSt".as_bytes();
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
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
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

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
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
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
