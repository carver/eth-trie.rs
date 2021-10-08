use std::error::Error;
use std::fmt;

use rlp::DecoderError;

use crate::nibbles::Nibbles;

#[derive(Debug, PartialEq)]
pub enum TrieError {
    DB(String),
    Decoder(DecoderError),
    InvalidData,
    InvalidStateRoot,
    InvalidProof,
    MissingTrieNode {
        node_hash: Vec<u8>,
        traversed: Nibbles,
        root_hash: Option<Vec<u8>>,
        err_key: Option<Vec<u8>>,
    },
}

impl Error for TrieError {}

impl fmt::Display for TrieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TrieError::DB(ref err) => format!("trie error: {:?}", err),
            TrieError::Decoder(ref err) => format!("trie error: {:?}", err),
            TrieError::InvalidData => "trie error: invalid data".to_owned(),
            TrieError::InvalidStateRoot => "trie error: invalid state root".to_owned(),
            TrieError::InvalidProof => "trie error: invalid proof".to_owned(),
            TrieError::MissingTrieNode { .. } => "trie error: missing node".to_owned(),
        };
        write!(f, "{}", printable)
    }
}

impl From<DecoderError> for TrieError {
    fn from(error: DecoderError) -> Self {
        TrieError::Decoder(error)
    }
}

#[derive(Debug)]
pub enum MemDBError {}

impl Error for MemDBError {}

impl fmt::Display for MemDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error")
    }
}
