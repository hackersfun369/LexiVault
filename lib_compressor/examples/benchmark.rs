use lexivault_lib::{HybridCompressor, BlockCompressor, CompressionAlgorithm};
use std::time::Instant;

fn main() {
    let compressor = HybridCompressor::new();
    let data_size = 10 * 1024 * 1024; // 10MB
    let data = vec![0u8; data_size]; // All zeros compress very fast
    
    println!("Benchmarking Hybrid Engine (10MB payload)...");

    // Zstd Fast
    let start = Instant::now();
    let compressed = compressor.compress(&data, CompressionAlgorithm::ZstdFast).unwrap();
    let duration = start.elapsed();
    println!("Zstd Fast Compression: {} ms ({:.2} MB/s)", duration.as_millis(), data_size as f64 / 1024.0 / 1024.0 / duration.as_secs_f64());

    let start = Instant::now();
    let _decompressed = compressor.decompress(&compressed, CompressionAlgorithm::ZstdFast).unwrap();
    let duration = start.elapsed();
    println!("Zstd Decompression: {} ms ({:.2} MB/s)", duration.as_millis(), data_size as f64 / 1024.0 / 1024.0 / duration.as_secs_f64());

    // LZ4
    let start = Instant::now();
    let compressed_lz4 = compressor.compress(&data, CompressionAlgorithm::Lz4).unwrap();
    let duration = start.elapsed();
    println!("LZ4 Compression: {} ms ({:.2} MB/s)", duration.as_millis(), data_size as f64 / 1024.0 / 1024.0 / duration.as_secs_f64());

    let start = Instant::now();
    let _decompressed_lz4 = compressor.decompress(&compressed_lz4, CompressionAlgorithm::Lz4).unwrap();
    let duration = start.elapsed();
    println!("LZ4 Decompression: {} ms ({:.2} MB/s)", duration.as_millis(), data_size as f64 / 1024.0 / 1024.0 / duration.as_secs_f64());
}
