# Fast DICOM Reader

A high-performance DICOM file reader written in Rust with Python bindings, designed for fast processing of medical imaging data.

## Features

- **Fast DICOM parsing**: Built with Rust for optimal performance
- **Python bindings**: Easy integration with Python workflows
- **Parallel processing**: Multi-threaded processing of multiple DICOM files
- **Comprehensive tag extraction**: Extracts popular standard DICOM tags
- **Pixel data support**: Handles various pixel data formats (8-bit, 16-bit, 32-bit)
- **CLI interface**: Command-line tool for batch processing with progress tracking
- **Cross-platform**: Works on macOS, Linux, and Windows

## Installation

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd fast_dicom_reader
```
3. Build the CLI tool:
```bash
cargo build --release
```

## Rust Development

### Project Structure

```
fast_dicom_reader/
├── src/
│   ├── lib.rs          # Python bindings and main library
│   ├── main.rs         # CLI interface with progress tracking
│   ├── dicom_utils.rs  # DICOM processing logic
│   ├── os_utils.rs     # File system utilities
│   └── consts.rs       # Constants and configurations
├── data/               # Sample DICOM files
├── Cargo.toml         # Rust dependencies
└── README.md          # This file
```

### Rust Commands

#### Building
```bash
# Build CLI tool (debug)
cargo build

# Build CLI tool (release)
cargo build --release

# Build Python extension
maturin build --release
```

#### Testing
```bash
# Run Rust tests
cargo test

# Test CLI functionality
cargo run -- read --path data/
```

#### Running
```bash
# Run CLI tool (debug)
cargo run -- read --path /path/to/dicom/folder/

# Run CLI tool (release)
./target/release/fast_dicom_reader read --path /path/to/dicom/folder/
```

### CLI Interface

The CLI provides a simple interface for processing DICOM files in bulk:

```bash
# Process a folder of DICOM files
cargo run -- read --path /path/to/dicom/folder/

# Use specific number of threads
cargo run -- read --path /path/to/dicom/folder/ --threads 8

# Build and run the release version
cargo build --release
./target/release/fast_dicom_reader read --path /path/to/dicom/folder/
```

### CLI Commands

#### `read`
Process DICOM files in a directory with progress tracking and parallel processing.

**Options:**
- `--path <PATH>`: Directory path to scan for DICOM files
- `--threads <THREADS>`: Number of threads to use (optional, defaults to CPU cores - 1)

**Example Output:**
```
Processing DICOM files in: /path/to/dicom/folder/
CPU cores detected: 8
Using 7 threads for parallel processing
Found 150 DICOM files to process
[00:00:15] [####################] 150/150
Processing complete!
```