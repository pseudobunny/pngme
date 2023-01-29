use crate::chunk_type::ChunkType;
use crate::Error;
use crc::Crc;
use std::fmt;
use std::str::from_utf8;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

fn png_checksum(data: &[u8]) -> u32 {
    Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(data)
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // calculate length of chunk
        let length_buff: [u8; 4] = value[0..4].try_into()?;
        let length = u32::from_be_bytes(length_buff);

        // calculate type of chunk
        let chunk_type_buff: [u8; 4] = value[4..8].try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_buff)?;

        if !chunk_type.is_valid() {
            return Err(Error::from("Chunk type is not valid."));
        }

        // Read chunk data
        let end_of_data = 8 + length as usize;
        let data = value[8..end_of_data].iter().copied().collect::<Vec<u8>>();

        if data.len() != length as usize {
            return Err(Error::from("Data is not the length specified"));
        }

        // Read CRC and validate
        let crc_buff = value[end_of_data..end_of_data + 4].try_into()?;
        let crc = u32::from_be_bytes(crc_buff);

        if png_checksum(&value[4..end_of_data]) != crc {
            return Err(Error::from("Checksum validation failed."));
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data_as_string().map_err(|_| fmt::Error)?)
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let checksum_data = chunk_type
            .bytes()
            .into_iter()
            .chain(data.clone().into_iter())
            .collect::<Vec<u8>>();

        Chunk {
            length: data.len() as u32,
            chunk_type,
            data: data.clone(),
            crc: png_checksum(&checksum_data),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, Error> {
        let data_str = from_utf8(&self.data)?;
        Ok(data_str.to_string())
    }

    pub fn type_and_data_as_string(&self) -> Result<String, Error> {
        let type_and_data = [self.chunk_type.bytes().to_vec(), self.data.clone()].concat();
        let type_and_data_str = from_utf8(&type_and_data)?;
        Ok(type_and_data_str.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .into_iter()
            .chain(self.chunk_type.bytes())
            .chain(self.data.iter().copied())
            .chain(self.crc.to_be_bytes())
            .collect()
    }
}

#[cfg(test)]
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
