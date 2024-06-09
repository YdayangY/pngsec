#![allow(unused)]
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use std::env;
use args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;


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

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();
    let command = args.next();
    match command {
        Some(x) => {
            match &x as &str {
                "encode" => {
                    let file_path = args.next().unwrap();
                    let chunk_type = args.next().unwrap();
                    let message = args.next().unwrap();
                    let output_file = args.next().unwrap();
                    let encode_args = EncodeArgs{file_path, chunk_type, message, output_file};
                    commands::encode(encode_args);
                },
                "decode" => {
                    let file_path = args.next().unwrap();
                    let chunk_type = args.next().unwrap();
                    let decode_args = DecodeArgs{file_path, chunk_type};
                    commands::decode(decode_args);
                },
                "remove"  => {
                    let file_path = args.next().unwrap();
                    let chunk_type = args.next().unwrap();
                    let remove_args = RemoveArgs{file_path, chunk_type};
                    commands::remove(remove_args);
                },
                "print" => {
                    let file_path = args.next().unwrap();
                    let print_args = PrintArgs{file_path};
                    commands::print(print_args);
                },
                _ => panic!("unresolved command, legal command is encode,decode,remove,print")
            }
        },
        _ => panic!("unresolved command, legal command is encode,decode,remove,print")
    };
    Ok(())
}