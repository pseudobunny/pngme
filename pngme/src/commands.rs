use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Error, Result};
use std::fs::{metadata, File};
use std::io::{Read, Write};
use std::path::PathBuf;

fn load_file(filename: &PathBuf) -> Result<Vec<u8>> {
    let mut file = File::open(&filename)?;

    let metadata = metadata(&filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;

    Ok(buffer)
}

fn write_file(filename: PathBuf, bytes: &[u8]) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(bytes)?;

    Ok(())
}

fn load_png(filename: &PathBuf) -> Result<Png> {
    let file_contents = load_file(&filename)?;

    Png::try_from(&file_contents[..])
}

pub fn encode(
    filename: PathBuf,
    chunk_type: String,
    message: String,
    output: Option<PathBuf>,
) -> Result<()> {
    let mut png = load_png(&filename)?;
    let secret_chunk = Chunk::new(ChunkType::new(&chunk_type)?, message.as_bytes().to_vec());

    png.append_chunk(secret_chunk);

    match output {
        Some(path) => write_file(path, &png.as_bytes()),
        None => write_file(filename, &png.as_bytes()),
    }
}

pub fn decode(filename: PathBuf, chunk_type: String) -> Result<()> {
    let png = load_png(&filename)?;
    let secret_chunk = png.chunk_by_type(&chunk_type);

    match secret_chunk {
        Some(chunk) => {
            println!("{}", chunk.data_as_string()?);
            Ok(())
        }
        None => Err(Error::from(
            "Chunk type not found in png, no secret message to decode.",
        )),
    }
}

pub fn remove(filename: PathBuf, chunk_type: String) -> Result<()> {
    let mut png = load_png(&filename)?;
    let secret_chunk = png.chunk_by_type(&chunk_type);

    match secret_chunk {
        Some(_) => {
            png.remove_chunk(&chunk_type)?;
            write_file(filename, &png.as_bytes())?;

            Ok(())
        }
        None => Err(Error::from(
            "Chunk type not found in png, no secret message to remove.",
        )),
    }
}

pub fn print(filename: PathBuf) -> Result<()> {
    let png = load_png(&filename)?;

    println!("{}", png);

    Ok(())
}
