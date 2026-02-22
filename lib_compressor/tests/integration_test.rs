extern crate alloc;
use lexivault_lib::{HybridCompressor, BlockCompressor, CompressionAlgorithm, MerkleTree};
use sha2::{Sha256, Digest};

#[test]
fn test_end_to_end_pipeline() {
    // 1. Data Preparation (Source Code example)
    let original_data = b"public class Test { public static void main(String[] args) { System.out.println(\"Hello World\"); } }";
    
    // Calculate original SHA-256
    let mut hasher = Sha256::new();
    hasher.update(original_data);
    let original_hash = hasher.finalize();

    // 2. Integration: Pre-processing (Simulated)
    // In a real run, the compressor_service would tokenise this.
    // For the test, we use the raw data but verify the chain.
    let blocks = vec![original_data.to_vec()]; // Single block for simplicity

    // 3. Integration: Driver Compression (Simulated)
    let compressor = HybridCompressor::new();
    let algorithm = CompressionAlgorithm::ZstdFast;
    let compressed_blocks: Vec<Vec<u8>> = blocks.iter()
        .map(|b| compressor.compress(b, algorithm).expect("Compression failed"))
        .collect();

    // 4. Integration: Integrity Generation
    let tree = MerkleTree::build(&blocks);
    let _root_hash = tree.root_hash;

    // --- ON-DISK STATE SIMULATED ---

    // 5. Integration: Driver Decompression (Simulated Read)
    let decompressed_blocks: Vec<Vec<u8>> = compressed_blocks.iter()
        .map(|cb| compressor.decompress(cb, algorithm).expect("Decompression failed"))
        .collect();

    // 6. Verification: Merkle Integrity Check
    for (i, block) in decompressed_blocks.iter().enumerate() {
        assert!(MerkleTree::verify_block(block, i, &tree.leaf_hashes), "Block integrity failure");
    }

    // 7. Verification: Bit-Identity (SHA-256)
    let mut final_data = Vec::new();
    for b in decompressed_blocks {
        final_data.extend_from_slice(&b);
    }
    
    let mut final_hasher = Sha256::new();
    final_hasher.update(&final_data);
    let final_hash = final_hasher.finalize();

    assert_eq!(original_hash, final_hash, "SHA-256 mismatch! Not bit-identical.");
    println!("Integration Test Success: 100% Bit-Identical recovery confirmed.");
}
