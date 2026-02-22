use crate::common::{CompressionAlgorithm, CompressorError, BlockCompressor};
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::ToString;

pub struct HybridCompressor;

impl HybridCompressor {
    pub fn new() -> Self {
        Self
    }
}

impl BlockCompressor for HybridCompressor {
    fn compress(&self, data: &[u8], algorithm: CompressionAlgorithm) -> Result<Vec<u8>, CompressorError> {
        match algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::Lz4 => {
                let mut compressed = vec![0u8; unsafe { lz4_sys::LZ4_compressBound(data.len() as i32) } as usize];
                let size = unsafe {
                    lz4_sys::LZ4_compress_default(
                        data.as_ptr() as *const i8,
                        compressed.as_mut_ptr() as *mut i8,
                        data.len() as i32,
                        compressed.len() as i32,
                    )
                };
                if size <= 0 {
                    return Err(CompressorError::Lz4("Compression failed".to_string()));
                }
                compressed.truncate(size as usize);
                Ok(compressed)
            }
            CompressionAlgorithm::ZstdFast | CompressionAlgorithm::ZstdDensity => {
                let level = if algorithm == CompressionAlgorithm::ZstdFast { 3 } else { 19 };
                zstd::encode_all(data, level)
                    .map_err(|e| CompressorError::Zstd(e.to_string()))
            }
        }
    }

    fn decompress(&self, compressed_data: &[u8], algorithm: CompressionAlgorithm) -> Result<Vec<u8>, CompressorError> {
        match algorithm {
            CompressionAlgorithm::None => Ok(compressed_data.to_vec()),
            CompressionAlgorithm::Lz4 => {
                // In a block-based system, we usually know the decompressed size (e.g. 64KB).
                // For the benchmark/generic use, we'll try to estimate or use a larger limit.
                let mut decompressed = vec![0u8; 10 * 1024 * 1024]; // 10MB for generic use
                let size = unsafe {
                    lz4_sys::LZ4_decompress_safe(
                        compressed_data.as_ptr() as *const i8,
                        decompressed.as_mut_ptr() as *mut i8,
                        compressed_data.len() as i32,
                        decompressed.len() as i32,
                    )
                };
                if size < 0 {
                    return Err(CompressorError::Lz4("Decompression failed".to_string()));
                }
                decompressed.truncate(size as usize);
                Ok(decompressed)
            }
            CompressionAlgorithm::ZstdFast | CompressionAlgorithm::ZstdDensity => {
                zstd::decode_all(compressed_data)
                    .map_err(|e| CompressorError::Zstd(e.to_string()))
            }
        }
    }
}
