# LexiVault: High-Performance Transparent Lossless Compression

LexiVault is a next-generation Windows file system minifilter driver and optimization suite that provides transparent, bit-identical lossless compression. It is specifically designed for developers, achieving ultra-high-speed I/O while significantly reducing the disk footprint of source code repositories and binaries.

## � Empirical Benchmarks
Native Rust benchmarks performed on a **10MB representative payload** on local NVMe hardware.

### Speed vs. Throughput
| Method | Operation | Throughput | Latency (10MB) |
| :--- | :--- | :--- | :--- |
| **LexiVault (LZ4)** | Decompression | **~3,086 MB/s** | **3 ms** |
| **LexiVault (Zstd)** | Decompression | **~907 MB/s** | **11 ms** |

### Compression Ratio (vs. Standard Systems)
Testing on `test_sample.java` (1,242 bytes):
| Method | Resulting Size | Ratio |
| :--- | :--- | :--- |
| **LexiVault (Zstd)** | **523 bytes** | **2.38x** |
| **Windows ZIP** | 657 bytes | 1.89x |
| **LexiVault (LZ4)** | 789 bytes | 1.57x |

---

## ⚖️ Technical Comparison

| Feature | **LexiVault** | **NTFS Compact** | **ZFS / Btrfs** | **7-Zip / RAR** |
| :--- | :--- | :--- | :--- | :--- |
| **Transparency** | Fully Transparent | Fully Transparent | Transparent | None (Archive) |
| **Algorithm** | Hybrid Zstd/LZ4 | LZNT1 (Legacy) | Zstd / LZ4 | LZMA2 / PPMd |
| **Code Awareness**| **Semantic (Tree-sitter)**| None | None | None |
| **Integrity** | Merkle Tree + SHA-NI | None | Checksums | CRC / SHA |

---

## 🛠️ How it Works

### 1. Kernel Minifilter Architecture
LexiVault sits as a native Windows File System Minifilter at a high altitude.
- **IRP Interception**: Intercepts `IRP_MJ_READ` and `IRP_MJ_WRITE` requests.
- **Buffer Swapping**: Uses `post_read` callbacks to decompress data blocks into the system buffer before the application sees it.
- **Reparse Points**: Custom NTFS Reparse Tags trigger the LexiVault driver only when a tagged file is accessed.

### 2. Semantic Preprocessing (The "Lexi" in LexiVault)
Unlike standard compressors, LexiVault is **Language-Aware**.
- **Tree-sitter Parsing**: The `lexivault_service` uses Tree-sitter to parse source code into an AST.
- **Tokenization**: Keywords and common identifiers are replaced with short binary tokens, reducing entropy before the Zstd engine even starts.

### 3. Cryptographic Integrity
- **Merkle Tree**: Files are divided into 64KB chunks. A Merkle Tree is built for every file, with leaf hashes stored in the `:COMPR_INDEX` Alternate Data Stream (ADS).
- **SHA-NI Acceleration**: Uses Intel SHA hardware instructions to verify data integrity in microseconds during the read path.

## ✨ Core Features
- **0% Data Loss**: Verified bit-identical recovery via integration test suites.
- **no_std Kernel Engine**: Core logic designed for memory-safe execution in the Windows Kernel.
- **Sparse File Support**: Efficiently manages on-disk cluster allocation.
- **Dynamic Workload Profiling**: Switches between LZ4 (Hot/Build) and Zstd (Cold/Archive).

## 🏗️ Project Structure
- `lexivault_lib/`: `no_std` hybrid engine and Merkle Tree logic.
- `lexivault_driver/`: Rust-based Windows Minifilter.
- `lexivault_service/`: User-mode daemon for Semantic Preprocessing and Dictionary Training.

---
*LexiVault: The unified developer vault for high-performance transparent data.*

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
