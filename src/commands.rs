use std::convert::TryFrom;
use std::fs;
use std::io::Error;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn run_encode(args: &EncodeArgs) -> Result<(), Error> {
    let file_bytes = fs::read(&args.file_path)?;

    let mut png = Png::try_from(file_bytes.as_slice())?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let new_chunk = Chunk::new(chunk_type, args.message.as_bytes().to_vec());

    png.append_chunk(new_chunk);

    if let Some(output_file) = &args.output_file {
        fs::write(output_file, png.as_bytes())?;
    }

    Ok(())
}

pub fn run_decode(args: &DecodeArgs) -> Result<(), Error> {
    let file_bytes = fs::read(&args.file_path)?;

    let png = Png::try_from(file_bytes.as_slice())?;

    let decoded_chunk = png.chunk_by_type(&args.chunk_type);

    match decoded_chunk {
        Some(chunk) => println!("{}", chunk),
        None => println!("That chunk doesn't exist"),
    }

    Ok(())
}

pub fn run_remove(args: &RemoveArgs) -> Result<(), Error> {
    let file_bytes = fs::read(&args.file_path)?;

    let mut png = Png::try_from(file_bytes.as_slice())?;

    let removed_chunk = png.remove_chunk(&args.chunk_type)?;

    println!("{}", removed_chunk);

    Ok(())
}

pub fn run_print(args: &PrintArgs) -> Result<(), Error> {
    let file_bytes = fs::read(&args.file_path)?;

    let png = Png::try_from(file_bytes.as_slice())?;

    println!("{}", png);

    Ok(())
}
