use serde::{Serialize, Deserialize};
use alloc::string::String;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum CompressorError {
    Lz4(String),
    Zstd(String),
    InvalidBlockSize,
    IntegrityFailure,
    // Note: We remove Io and Serialization errors that depend on std::io or bincode's default std features
    // for the kernel-mode core.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MerkleNode {
    pub hash: [u8; 32],
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CompressionAlgorithm {
    None,
    Lz4,
    ZstdFast,
    ZstdDensity,
}

pub trait BlockCompressor {
    fn compress(&self, data: &[u8], algorithm: CompressionAlgorithm) -> Result<alloc::vec::Vec<u8>, CompressorError>;
    fn decompress(&self, compressed_data: &[u8], algorithm: CompressionAlgorithm) -> Result<alloc::vec::Vec<u8>, CompressorError>;
}
