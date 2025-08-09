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

## Prerequisites

- Rust (1.70 or later)
- Python 3.7 or later
- pip

## Installation

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd fast_dicom_reader
```

2. Install maturin for building Python extensions:
```bash
pip install maturin
```

3. Build and install the Python extension:
```bash
maturin develop
```

4. Build the CLI tool:
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

**CLI Features:**
- **Progress tracking**: Real-time progress bar showing processing status
- **Parallel processing**: Configurable thread count for optimal performance
- **Error handling**: Continues processing even if individual files fail
- **Smart threading**: Automatically uses (CPU cores - 1) threads by default

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

## Python Development

### Python API

```python
import fast_dicom_reader

# Read a single DICOM file
result = fast_dicom_reader.read_single_dicom('path/to/file.dcm')
print(f"File: {result.path}")
print(f"Tags: {len(result.tags)}")
print(f"Pixel data shape: {result.pixel_data_shape}")

# Process multiple files in parallel
results = fast_dicom_reader.process_dicom_folder('path/to/folder/', num_threads=4)
for result in results:
    print(f"Processed: {result.path}")
```

### Python Functions

#### `read_single_dicom(path: str) -> PyDicomData`
Reads a single DICOM file and returns a `PyDicomData` object.

**Parameters:**
- `path`: Path to the DICOM file

**Returns:**
- `PyDicomData` object containing:
  - `path`: File path
  - `tags`: Dictionary of DICOM tags and values
  - `pixel_data_shape`: Shape of pixel data (if available)
  - `pixel_data`: Pixel data as a flat array (if available)

#### `process_dicom_folder(folder_path: str, num_threads: Optional[int] = None) -> List[PyDicomData]`
Processes multiple DICOM files in parallel using the same logic as the CLI.

**Parameters:**
- `folder_path`: Path to folder containing DICOM files
- `num_threads`: Number of threads to use (defaults to CPU cores - 1)

**Returns:**
- List of `PyDicomData` objects

### Python Testing

```bash
# Test Python functionality
python3 -c "import fast_dicom_reader; print('Python bindings working!')"

# Test with sample data
python3 -c "import fast_dicom_reader; result = fast_dicom_reader.read_single_dicom('data/dicom_example.dcm'); print(f'Tags: {len(result.tags)}')"
```

## Supported DICOM Tags

The library extracts a comprehensive set of DICOM tags including:

- Patient information (name, ID, birth date)
- Study information (description, date, time)
- Series information (modality, description)
- Image information (rows, columns, pixel spacing)
- Technical parameters (kVp, mA, exposure time)
- And many more...

## Performance

- **Single file processing**: ~10-50ms per file (depending on size)
- **Parallel processing**: Scales linearly with number of CPU cores
- **Memory efficient**: Processes files without loading entire dataset into memory
- **Progress tracking**: Real-time feedback during batch processing

## Dependencies

### Rust Dependencies
- `dicom`: DICOM file parsing
- `pyo3`: Python bindings
- `rayon`: Parallel processing
- `clap`: Command-line argument parsing
- `ndarray`: Multi-dimensional arrays
- `serde`: Serialization/deserialization
- `indicatif`: Progress bars and CLI feedback
- `num_cpus`: CPU core detection

### Python Dependencies
- `numpy`: Numerical computing (optional dependency)

## License

[Add your license information here]

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Acknowledgments

- Built with the excellent `dicom` crate for DICOM parsing
- Python bindings powered by PyO3
- Parallel processing with Rayon
- CLI progress tracking with Indicatif 