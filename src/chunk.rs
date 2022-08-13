use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;
use std::io::ErrorKind::Other;
use std::io::{BufReader, Error, Read};
use std::string::FromUtf8Error;

use crate::chunk_type::ChunkType;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let chunk_type_bytes = chunk_type.bytes();

        let crc_bytes: Vec<u8> = chunk_type_bytes
            .iter()
            .chain(data.as_slice().iter())
            .copied()
            .collect();

        Chunk {
            length: data.len() as u32,
            chunk_type,
            crc: calculate_crc(crc_bytes.as_slice()),
            data,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        for b in self.length.to_be_bytes() {
            v.push(b);
        }

        for b in self.chunk_type.bytes() {
            v.push(b);
        }

        for b in &self.data {
            v.push(*b)
        }

        for b in self.crc.to_be_bytes() {
            v.push(b);
        }

        v
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);

        let mut length_bytes: [u8; 4] = [0; 4];
        reader.read_exact(&mut length_bytes)?;
        let length = u32::from_be_bytes(length_bytes);

        let mut chunk_type_bytes: [u8; 4] = [0; 4];
        reader.read_exact(&mut chunk_type_bytes)?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        let mut data: Vec<u8> = vec![0u8; length as usize];
        reader.read_exact(&mut data)?;

        if data.len() != length as usize {
            return Err(Error::new(
                Other,
                "length of data is not the same as the specified length",
            ));
        }

        let mut crc_bytes: [u8; 4] = [0; 4];
        reader.read_exact(&mut crc_bytes)?;
        let crc = u32::from_be_bytes(crc_bytes);

        let bytes_vector: Vec<u8> = chunk_type_bytes
            .iter()
            .chain(data.as_slice().iter())
            .copied()
            .collect();

        if crc != calculate_crc(bytes_vector.as_ref()) {
            println!("{crc} {}", calculate_crc(bytes_vector.as_ref()));
            return Err(Error::new(Other, "crc is incorrect"));
        }

        let chunk = Chunk {
            length,
            chunk_type,
            data,
            crc,
        };

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.data_as_string().unwrap())
    }
}

fn calculate_crc(bytes: &[u8]) -> u32 {
    let crc_algo: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    crc_algo.checksum(bytes)
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

        println!("LENGTH {:?}", message_bytes.len());

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
