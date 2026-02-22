# LexiVault: High-Performance Transparent Lossless Compression

LexiVault is a next-generation Windows file system minifilter driver and optimization suite that provides transparent, bit-identical lossless compression. It is specifically designed for developers, achieving ultra-high-speed Input/Output (I/O) while significantly reducing the disk footprint of source code repositories and binaries.

## 📊 Empirical Benchmarks
Native Rust benchmarks performed on a **10MB representative payload** on local Non-Volatile Memory Express (NVMe) hardware.

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
LexiVault sits as a native Windows File System Minifilter at a high altitude within the operating system's storage stack.
- **I/O Request Packet (IRP) Interception**: The driver intercepts standard filesystem requests like `IRP_MJ_READ` and `IRP_MJ_WRITE` before they reach the physical disk.
- **Buffer Swapping**: Uses "Post-Operation" callbacks to decompress data blocks into the system buffer, ensuring that the calling application (like an Integrated Development Environment (IDE)) sees uncompressed data.
- **Reparse Points**: Custom New Technology File System (NTFS) Reparse Tags trigger the LexiVault driver only when a specifically tagged compressed file is accessed.

### 2. Semantic Preprocessing (The "Lexi" in LexiVault)
Unlike standard compressors, LexiVault is **Language-Aware**.
- **Abstract Syntax Tree (AST) Parsing**: The `lexivault_service` uses the Tree-sitter library to parse source code into a structured AST.
- **Semantic Tokenization**: Repetitive keywords and common identifiers are replaced with short binary tokens, reducing the mathematical complexity (entropy) before the Zstandard engine processes the data.

### 3. Cryptographic Integrity
- **Merkle Tree (Hash Tree)**: Files are divided into 64 Kilobyte (KB) chunks. A Merkle Tree is built for every file, providing a cryptographic "Chain of Trust" that ensures no data corruption occurs silently.
- **Alternate Data Stream (ADS)**: The Merkle leaf hashes and block indices are stored in a hidden sidecar stream named `:COMPR_INDEX`, keeping the main data stream clean and compatible with standard tools.
- **Secure Hash Algorithm (SHA) NI Acceleration**: Uses dedicated Intel Secure Hash Algorithm New Instructions (SHA-NI) to verify data integrity in microseconds without impacting the overall system performance.

## ✨ Core Features Explained
- **0% Data Loss**: Guaranteed bit-identical recovery for every byte, verified via automated Merkle Tree validation at the kernel level.
- **Memory-Safe Kernel Engine**: Built using Rust's `no_std` (no standard library) environment to eliminate common memory vulnerabilities (like buffer overflows) within the Windows Kernel.
- **Sparse File Support**: Dynamically manages on-disk cluster allocation so that Windows only allocates physical space for the actual compressed bytes, maximizing available disk space.
- **Hybrid Workload Profiling**: Automatically uses LZ4 for critical, low-latency files (like Build artifacts) and high-level Zstandard (Zstd) for maximum archival density.

## 🏗️ Project Structure
- `lexivault_lib/`: The core library containing the memory-safe `no_std` hybrid engine and Merkle Tree logic.
- `lexivault_driver/`: The Rust-based Windows Minifilter driver responsible for real-time I/O handling and buffer management.
- `lexivault_service/`: A User-Mode diagnostic and optimization service for background Dictionary Training and AST-based semantic preprocessing.

## 🖼️ Windows Native Integration
LexiVault is designed for permanent, seamless use within the Windows ecosystem.

### 1. Shell Context Menu
Manage your files directly from Windows Explorer.
- **Action**: Right-click any file or folder -> **LexiVault** -> **Compress/Decompress**.
- **Installation**: Apply `lexivault_shell.reg` to register the native context menu.

### 2. LexiVault Management Hub (App)
A permanent system resident for real-time monitoring.
- **System Tray**: Runs as a background taskbar icon for instant status checks.
- **Dashboard**: High-aesthetics visualization of storage savings and I/O performance.
- **Engine Control**: Switch transparency algorithms (LZ4/Zstd) on the fly.

## 🛠️ Build Instructions
### Prerequisites
- [Rust](https://rustup.rs/)
- [Node.js & npm](https://nodejs.org/)
- [Windows WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk) (For kernel driver)

### Distribution Build
1. **CLI & Shell Integration**:
   ```bash
   cd lexivault_cli
   cargo build --release
   ```
2. **Native Management Hub**:
   ```bash
   cd lexivault_ui
   npm install
   npm run tauri build
   ```

## ⚖️ License
LexiVault is provided for educational and research-level kernel development purposes. Ensure you enable **Test-Signing** mode in Windows before attempting to load the driver.

---
*LexiVault: The unified developer vault for high-performance transparent data.*
