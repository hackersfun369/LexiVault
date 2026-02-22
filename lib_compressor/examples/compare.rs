use lexivault_lib::{HybridCompressor, BlockCompressor, CompressionAlgorithm};
use std::fs;

fn main() {
    let sample_path = "test_sample.java";
    let original_data = fs::read(sample_path).expect("Failed to read sample file");
    let original_size = original_data.len();

    let compressor = HybridCompressor::new();

    println!("--- LexiVault Comparative Benchmark ---");
    println!("File: {}", sample_path);
    println!("Original Size: {} bytes", original_size);

    // LexiVault Zstd
    let zstd_data = compressor.compress(&original_data, CompressionAlgorithm::ZstdFast).unwrap();
    println!("LexiVault (Zstd): {} bytes (Ratio: {:.2}x)", 
        zstd_data.len(), original_size as f64 / zstd_data.len() as f64);

    // LexiVault LZ4
    let lz4_data = compressor.compress(&original_data, CompressionAlgorithm::Lz4).unwrap();
    println!("LexiVault (LZ4): {} bytes (Ratio: {:.2}x)", 
        lz4_data.len(), original_size as f64 / lz4_data.len() as f64);
}
