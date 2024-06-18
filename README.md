# Base Sequence Compression

Base Sequence Compression is a Rust library for compressing and decompressing DNA sequences. This library efficiently encodes DNA sequences, reducing storage space while preserving the integrity of the original data. The DNA bases (A, C, T, G) are encoded using 2 bits each, allowing for significant compression.

## Features

- **Compression and Decompression**: Compress DNA sequences into a binary format and decompress them back to their original form.
- **Error Handling**: Handles invalid DNA sequences by panicking, ensuring data integrity.
- **Compression Rate Calculation**: Calculate the compression rate to evaluate the efficiency of the compression.

## Usage

### Adding the Library to Your Project

Add the following to your `Cargo.toml`:

```toml
[dependencies]
base_sequence_compression = "0.1.0"
```

### Example

Here's an example of how to use the library:

```rust
use base_sequence_compression::{compress_sequence, decompress_sequence};

fn main() {
    let dna_sequence = "ACGTACGTACGT";
    let compressed = compress_sequence(dna_sequence);
    let decompressed = decompress_sequence(&compressed);

    assert_eq!(dna_sequence, decompressed);
    println!("Compression successful!");
}
```

## API

### `compress_sequence`

Compresses a DNA sequence into a binary format.

```rust
pub fn compress_sequence(sequence: &str) -> Vec<u8>
```

- **Input**: `&str` - The DNA sequence to be compressed.
- **Output**: `Vec<u8>` - The compressed binary data.

### `decompress_sequence`

Decompresses binary data back into the original DNA sequence.

```rust
pub fn decompress_sequence(compressed: &[u8]) -> String
```

- **Input**: `&[u8]` - The compressed binary data.
- **Output**: `String` - The decompressed DNA sequence.

## Tests

The library includes a comprehensive set of tests to ensure the correctness of the compression and decompression functions. To run the tests, use the following command:

```sh
cargo test
```

### Test Cases

- **Compression and Decompression**: Tests compressing and decompressing various sequences, including edge cases.
- **Invalid Sequence Handling**: Ensures that invalid sequences are handled appropriately by panicking.
- **Compression Rate**: Evaluates the efficiency of the compression algorithm.

## Example Test

Here's an example test that checks the compression and decompression of a DNA sequence:

```rust
#[test]
fn test_compress_decompress() {
    let dna_sequence = "ACGTACGTACGT";
    let compressed = compress_sequence(dna_sequence);
    let decompressed = decompress_sequence(&compressed);

    assert_eq!(dna_sequence, decompressed);
}
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

## Acknowledgements

Thanks to the contributors and the open-source community for their invaluable feedback and support.

---

**Repository**: [Base Sequence Compression](https://github.com/yourusername/base-sequence-compression)

Feel free to reach out for any questions or suggestions regarding the library.
