use std::path::PathBuf;

#[derive(clap::Subcommand)]
pub enum Action {
    /// Encodes a hidden message in a PNG File
    Encode {
        /// Path to PNG file
        filename: PathBuf,
        /// Valid PNG spec Chunk Type for identifying the hidden message chunk
        chunk_type: String,
        /// Message to Encode
        message: String,
        /// File to output message to
        output: Option<PathBuf>
    },
    /// Decodes a hidden message in a PNG File
    Decode {
        /// Path to PNG file
        filename: PathBuf,
        /// Valid PNG spec Chunk Type for identifying the hidden message chunk
        chunk_type: String,
    },
    /// Removes a hidden message in a PNG File
    Remove {
        /// Path to PNG file
        filename: PathBuf,
        /// Valid PNG spec Chunk Type for identifying the hidden message chunk
        chunk_type: String,
    },
    /// Prints out the content of a PNG File
    Print {
        /// Path to PNG file
        filename: PathBuf,
    }
}