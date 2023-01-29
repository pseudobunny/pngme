use args::Action;
use clap::Parser;
use commands::{decode, encode, print, remove};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Args::parse();

    match args.action {
        Action::Encode {
            filename,
            chunk_type,
            message,
            output,
        } => commands::encode(filename, chunk_type, message, output),
        Action::Decode {
            filename,
            chunk_type,
        } => decode(filename, chunk_type),
        Action::Remove {
            filename,
            chunk_type,
        } => remove(filename, chunk_type),
        Action::Print { filename } => print(filename),
    }
}
