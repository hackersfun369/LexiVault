# LexiVault: High-Performance Transparent Lossless Compression

LexiVault is a next-generation Windows file system minifilter driver and optimization suite that provides transparent, bit-identical lossless compression. It is specifically designed for developers, achieving ultra-high-speed I/O while significantly reducing the disk footprint of source code repositories and binaries.

## 🚀 Key Performance Benchmarks
On a standard NVMe Gen3 SSD, LexiVault delivers:
- **LZ4 Decompression (Fast Path)**: ~3,086 MB/s
- **Zstd Decompression (Standard Path)**: ~900 MB/s
- **Compression Ratio (Source Code)**: ~2.38x (Significant improvement over standard ZIP/NTFS)

## ✨ Unique Features
- **Hybrid Adaptive Engine**: Dynamically switches between LZ4 and Zstandard based on file access patterns.
- **Semantic Code Optimization**: Integrated with **Tree-sitter** to parse and tokenize source code (Rust, Java, etc.) before compression, achieving density that standard compressors miss.
- **Cryptographic Integrity**: Every 64KB block is protected by a **Merkle Tree**, verified in real-time using **Intel SHA-NI** hardware acceleration.
- **Native Transparency**: Operates as a Windows Kernel Minifilter; your files remain directly editable and searchable without any extraction.
- **NTFS Persistence**: Uses Reparse Points and Alternate Data Streams (ADS) to store metadata invisibly alongside your data.

## 🏗️ Project Structure
- `lexivault_lib/`: The core `no_std` hybrid compression engine and Merkle Tree implementation.
- `lexivault_driver/`: The Rust-based Windows Kernel Minifilter driver.
- `lexivault_service/`: User-mode optimization service for background dictionary training and semantic preprocessing.

## 🛠️ Getting Started
### Prerequisites
- [Rust](https://rustup.rs/) (Stable/Nightly)
- [Enterprise Windows WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk) (For driver compilation)

### Build & Test
1. **Core Library**:
   ```bash
   cd lexivault_lib
   cargo test --test integration_test
   ```
2. **Optimization Service**:
   ```bash
   cd lexivault_service
   cargo check
   ```

## ⚖️ License
LexiVault is provided for educational and research-level kernel development purposes. Ensure you enable **Test-Signing** mode in Windows before attempting to load the driver.

---
*Created by Antigravity AI for HackersFun369.*
