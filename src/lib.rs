/// The bit pattern for the base 'A' (00).
pub const A_BITS: u8 = 0b00;
/// The bit pattern for the base 'C' (01).
pub const C_BITS: u8 = 0b01;
/// The bit pattern for the base 'T' (10).
pub const T_BITS: u8 = 0b10;
/// The bit pattern for the base 'G' (11).
pub const G_BITS: u8 = 0b11;

/// Compresses a DNA sequence into a vector of bytes.
///
/// The DNA sequence is compressed by representing each base (A, C, T, G)
/// with 2 bits. The compressed data starts with a 4-byte (u32) integer
/// representing the length of the original sequence.
///
/// # Arguments
///
/// * `sequence` - A string slice that holds the DNA sequence.
///
/// # Returns
///
/// A vector of bytes containing the compressed DNA sequence.
///
/// # Panics
///
/// This function will panic if the sequence contains invalid characters
/// other than 'A', 'C', 'T', or 'G'.
pub fn compress_sequence(sequence: &str) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    // Prepend the length of the original DNA sequence as a 4-byte (u32) integer
    let length: u32 = sequence.len() as u32;
    compressed.extend_from_slice(&length.to_be_bytes());

    for base in sequence.chars() {
        let bits = match base {
            'A' => A_BITS,
            'C' => C_BITS,
            'T' => T_BITS,
            'G' => G_BITS,
            _ => panic!("Invalid SEQUENCE!"),
        };

        current_byte = (current_byte << 2) | bits;
        bit_count += 2;

        if bit_count == 8 {
            compressed.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    if bit_count > 0 {
        compressed.push(current_byte << (8 - bit_count));
    }

    compressed
}

/// Decompresses a vector of bytes into a DNA sequence string.
///
/// The compressed data starts with a 4-byte (u32) integer representing
/// the length of the original DNA sequence, followed by the compressed
/// sequence data. Each base (A, C, T, G) is represented by 2 bits.
///
/// # Arguments
///
/// * `compressed` - A slice of bytes containing the compressed DNA sequence.
///
/// # Returns
///
/// A string containing the decompressed DNA sequence.
///
/// # Panics
///
/// This function will panic if the decompressed bits do not match the valid
/// bit patterns for 'A', 'C', 'T', or 'G'.
pub fn decompress_sequence(compressed: &[u8]) -> String {
    // Extract the length of the original DNA sequence from the first 4 bytes
    let length =
        u32::from_be_bytes([compressed[0], compressed[1], compressed[2], compressed[3]]) as usize;
    let mut sequence = String::new();
    let mut bits = 0;
    let mut bit_count = 0;

    for &byte in &compressed[4..] {
        bits = (bits << 8) | byte as usize;
        bit_count += 8;

        while bit_count >= 2 && sequence.len() < length {
            let base_bits = (bits >> (bit_count - 2)) & 0b11;
            let base = match base_bits as u8 {
                A_BITS => 'A',
                C_BITS => 'C',
                T_BITS => 'T',
                G_BITS => 'G',
                _ => panic!("Invalid bits!"),
            };
            sequence.push(base);
            bit_count -= 2;
        }
    }

    sequence
}
