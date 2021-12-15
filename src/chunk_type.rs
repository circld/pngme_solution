use std::char::CharTryFromError;
use std::convert::TryFrom;
use std::num::NonZeroU8;
// use std::str::FromStr;
// use std::str::Utf8Error;

#[derive(Debug)]
struct ChunkType {
    value: [char; 4],
}

impl ChunkType {
    fn new(value: [char; 4]) -> ChunkType {
        ChunkType { value }
    }

    fn bytes(&self) -> [u8; 4] {
        let mut bytes = [0; 4];

        for value in self.value.iter().enumerate() {
            bytes[value.0] = *value.1 as u8;
        }
        bytes
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn is_critical(&self) -> bool {
        todo!()
    }

    fn is_public(&self) -> bool {
        todo!()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        todo!()
    }

    fn is_safe_to_copy(&self) -> bool {
        todo!()
    }
}

// impl FromStr for ChunkType {
//     type Err = Utf8Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         s.as_bytes()
//     }
// }

// need more practice with `Result` return types and error-handling
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = CharTryFromError;

    fn try_from(values: [u8; 4]) -> Result<Self, Self::Error> {
        let mut parsed: [char; 4] = [' ', ' ', ' ', ' '];

        for value in values.iter().enumerate() {
            if let Ok(parsed_value) = char::try_from(*value.1) {
                parsed[value.0] = parsed_value;
            }
        }
        Ok(Self::new(parsed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    //    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    //    #[test]
    //    pub fn test_chunk_type_from_str() {
    //        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
    //        let actual = ChunkType::from_str("RuSt").unwrap();
    //        assert_eq!(expected, actual);
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_critical() {
    //        let chunk = ChunkType::from_str("RuSt").unwrap();
    //        assert!(chunk.is_critical());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_not_critical() {
    //        let chunk = ChunkType::from_str("ruSt").unwrap();
    //        assert!(!chunk.is_critical());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_public() {
    //        let chunk = ChunkType::from_str("RUSt").unwrap();
    //        assert!(chunk.is_public());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_not_public() {
    //        let chunk = ChunkType::from_str("RuSt").unwrap();
    //        assert!(!chunk.is_public());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_reserved_bit_valid() {
    //        let chunk = ChunkType::from_str("RuSt").unwrap();
    //        assert!(chunk.is_reserved_bit_valid());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_reserved_bit_invalid() {
    //        let chunk = ChunkType::from_str("Rust").unwrap();
    //        assert!(!chunk.is_reserved_bit_valid());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_safe_to_copy() {
    //        let chunk = ChunkType::from_str("RuSt").unwrap();
    //        assert!(chunk.is_safe_to_copy());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_is_unsafe_to_copy() {
    //        let chunk = ChunkType::from_str("RuST").unwrap();
    //        assert!(!chunk.is_safe_to_copy());
    //    }
    //
    //    #[test]
    //    pub fn test_valid_chunk_is_valid() {
    //        let chunk = ChunkType::from_str("RuSt").unwrap();
    //        assert!(chunk.is_valid());
    //    }
    //
    //    #[test]
    //    pub fn test_invalid_chunk_is_valid() {
    //        let chunk = ChunkType::from_str("Rust").unwrap();
    //        assert!(!chunk.is_valid());
    //
    //        let chunk = ChunkType::from_str("Ru1t");
    //        assert!(chunk.is_err());
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_string() {
    //        let chunk = ChunkType::from_str("RuSt").unwrap();
    //        assert_eq!(&chunk.to_string(), "RuSt");
    //    }
    //
    //    #[test]
    //    pub fn test_chunk_type_trait_impls() {
    //        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //        let _chunk_string = format!("{}", chunk_type_1);
    //        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    //    }
}
