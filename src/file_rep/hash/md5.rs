use crate::file_rep::file_hasher::hash_file;
use crate::file_rep::hash_def::{HashValue};
use std::hash::{Hash, Hasher};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct HashMD5([u8; 16]);

impl HashValue for HashMD5 {
    fn new_hash_file(path: &PathBuf) -> io::Result<Self> {
        let result = hash_file::<md5::Md5>(path)?;
        Ok(Self(result.into())) //directly convert since compile time known size
    }

    fn new_from_string<S: AsRef<str>>(input: S) -> Option<Self> {
        let input = input.as_ref();

        if input.len() != 32 {
            return None;
        }

        let mut bytes = [0u8; 16];

        //pairs of hex chars to bytes
        for (i, chunk) in input.as_bytes().chunks(2).enumerate() {
            //two chars per byte
            if chunk.len() != 2 {
                return None;
            }

            //parse high and low nibble
            let high = match hex_char_to_int(chunk[0]) {
                Some(v) => v,
                None => return None,
            };

            let low = match hex_char_to_int(chunk[1]) {
                Some(v) => v,
                None => return None,
            };

            bytes[i] = (high << 4) | low;
        }

        Some(HashMD5(bytes))
    }

    fn equals(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    fn parse_hash_type_string<S: AsRef<str>>(input: S) -> bool {
        input.as_ref() == "md5"
    }

    fn signature_to_string() -> &'static str {
        "md5"
    }
}

impl PartialEq for HashMD5 {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for HashMD5 {}

impl Hash for HashMD5 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0)
    }
}

fn hex_char_to_int(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
