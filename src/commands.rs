#![allow(unused)]
use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use crate::ParseError;
use std::fs;

/// Encodes a message into a PNG file and saves the result
pub fn encode(param: EncodeArgs) -> Result<bool> {
    println!("file_path {}", param.file_path.clone());
    let file_result = fs::read(param.file_path.clone());
    let file_bytes = file_result.unwrap();
    let mut png: Png = Png::try_from(file_bytes.as_slice()).unwrap();
    let chunk_type = param.chunk_type.as_bytes();
    let chunk_type: std::prelude::v1::Result<[u8; 4], std::array::TryFromSliceError> = chunk_type.try_into();
    let chunk_type = chunk_type.unwrap() as [u8;4];
    let chunk_type = ChunkType {contents: chunk_type};
    let message = param.message;
    let data_length = message.len() as u32;
    let message_bytes = message.as_bytes().to_vec();
    let crc = Chunk::get_crc(&chunk_type, &message_bytes);
    let chunk = Chunk { data_length, chunk_type, message_bytes, crc};
    png.append_chunk(chunk);
    let result = png.create_png(&param.output_file);
    if let Ok(x) = result {
        println!("Success filepath:{}", x);
    }
    Ok(true)
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(param: DecodeArgs) -> Result<bool> {
    let file_result = fs::read(param.file_path.clone());
    let file_bytes = file_result.unwrap();
    let png: Png = Png::try_from(file_bytes.as_slice()).unwrap();
    let some_chunk = png.chunk_by_type(&param.chunk_type);
    if let Some(chunk) = some_chunk {
        println!("{}", chunk.data_as_string()?);
    }
    Ok(true)
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(param: RemoveArgs) -> Result<bool> {
    let file_result = fs::read(param.file_path.clone());
    let file_bytes = file_result.unwrap();
    let mut png: Png = Png::try_from(file_bytes.as_slice()).unwrap();
    let result = png.remove_chunk(&param.chunk_type);
    if let Ok(x) = result {
        png.create_png(&param.file_path);
        return Ok(true);
    }
    Err(Box::new(ParseError {details: "error crc checksum".to_string()}))
}

/// Prints all of the chunks in a PNG file
pub fn print(param: PrintArgs) {
    let file_result = fs::read(param.file_path.clone());
    let file_bytes = file_result.unwrap();
    let mut png: Png = Png::try_from(file_bytes.as_slice()).unwrap();
    println!("{}", png.to_string());
}