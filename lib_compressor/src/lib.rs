#![no_std]
extern crate alloc;

pub mod common;
pub mod engine;
pub mod integrity;

pub use common::{CompressionAlgorithm, CompressorError, BlockCompressor};
pub use engine::HybridCompressor;
pub use integrity::MerkleTree;

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use alloc::vec;

    #[test]
    fn test_hybrid_compression_roundtrip() {
        let compressor = HybridCompressor::new();
        let data = b"This is some test data that should be compressed and then decompressed back to its original state.";
        
        let compressed = compressor.compress(data, CompressionAlgorithm::ZstdFast).unwrap();
        let decompressed = compressor.decompress(&compressed, CompressionAlgorithm::ZstdFast).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }
}
