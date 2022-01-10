use std::cmp::PartialEq;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct InvalidChunkTypeByteValue {
    value: u8,
}

impl std::error::Error for InvalidChunkTypeByteValue {}

impl fmt::Display for InvalidChunkTypeByteValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid byte value for a ChunkType: {}", self.value)
    }
}

#[derive(Debug, PartialEq)]
struct ChunkType {
    value: [char; 4],
}

impl ChunkType {
    fn new(value: [char; 4]) -> ChunkType {
        ChunkType { value }
    }

    #[allow(dead_code)]
    fn bytes(&self) -> [u8; 4] {
        let mut bytes = [0; 4];

        for value in self.value.iter().enumerate() {
            bytes[value.0] = *value.1 as u8;
        }
        bytes
    }

    #[allow(dead_code)]
    fn is_valid(&self) -> bool {
        todo!()
    }

    #[allow(dead_code)]
    fn is_critical(&self) -> bool {
        todo!()
    }

    #[allow(dead_code)]
    fn is_public(&self) -> bool {
        todo!()
    }

    #[allow(dead_code)]
    fn is_reserved_bit_valid(&self) -> bool {
        todo!()
    }

    #[allow(dead_code)]
    fn is_safe_to_copy(&self) -> bool {
        todo!()
    }
}

impl FromStr for ChunkType {
    // TODO raise custom error for invalid characters (e.g., non-alpha)
    type Err = InvalidChunkTypeByteValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed: [u8; 4] = [0, 0, 0, 0];

        for value in s.chars().enumerate() {
            parsed[value.0] = value.1 as u8;
        }
        Self::try_from(parsed)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = InvalidChunkTypeByteValue;

    fn try_from(values: [u8; 4]) -> Result<Self, Self::Error> {
        let mut parsed: [char; 4] = [' ', ' ', ' ', ' '];

        for value in values.iter().enumerate() {
            let int_value = *value.1 as i8;
            let is_valid = (65..=90).contains(&int_value) | (97..=122).contains(&int_value);
            let parsed_value = *value.1 as char;

            parsed[value.0] = if is_valid {
                parsed_value
            } else {
                return Err(InvalidChunkTypeByteValue { value: *value.1 });
            };
        }
        Ok(Self::new(parsed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_bytes_invalid() {
        let actual = ChunkType::try_from([50, 92, 123, 116]);
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), InvalidChunkTypeByteValue { value: 50 });
    }

    #[test]
    pub fn test_chunk_type_from_str_valid() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_from_str_invalid() {
        let expected = InvalidChunkTypeByteValue { value: 37 };
        let actual = ChunkType::from_str("%uSt").unwrap_err();
        assert_eq!(expected, actual);
    }

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
