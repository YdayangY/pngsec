use std::path::PathBuf;

pub enum PngSecArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
pub struct EncodeArgs {
    pub file_path: String,
    pub chunk_type: String,
    pub message: String,
    pub output_file: String,
}

pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: String,
}

pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: String,
}

pub struct PrintArgs {
    pub file_path: String,
}