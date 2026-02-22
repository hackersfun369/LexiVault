use sha2::{Sha256, Digest};
use alloc::vec::Vec;

pub struct MerkleTree {
    pub root_hash: [u8; 32],
    pub leaf_hashes: Vec<[u8; 32]>,
}

impl MerkleTree {
    pub fn build(blocks: &[Vec<u8>]) -> Self {
        let leaf_hashes: Vec<[u8; 32]> = blocks.iter()
            .map(|block| {
                let mut hasher = Sha256::new();
                hasher.update(block);
                hasher.finalize().into()
            })
            .collect();

        let root_hash = Self::calculate_root(&leaf_hashes);
        
        MerkleTree {
            root_hash,
            leaf_hashes,
        }
    }

    fn calculate_root(hashes: &[[u8; 32]]) -> [u8; 32] {
        if hashes.is_empty() {
            return [0u8; 32];
        }
        if hashes.len() == 1 {
            return hashes[0];
        }

        let mut current_level = hashes.to_vec();
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(chunk[1]);
                } else {
                    hasher.update(chunk[0]);
                }
                next_level.push(hasher.finalize().into());
            }
            current_level = next_level;
        }
        current_level[0]
    }

    pub fn verify_block(block: &[u8], index: usize, leaf_hashes: &[[u8; 32]]) -> bool {
        if index >= leaf_hashes.len() {
            return false;
        }
        let mut hasher = Sha256::new();
        hasher.update(block);
        let hash: [u8; 32] = hasher.finalize().into();
        hash == leaf_hashes[index]
    }
}
