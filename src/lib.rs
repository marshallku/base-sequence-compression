use std::io::{self, Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

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
pub fn compress_sequence(sequence: &str) -> Vec<u8> {
    let mut compressed = Vec::with_capacity(sequence.len() / 4 + 1);
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for base in sequence.chars() {
        let bits = match base {
            'A' => A_BITS,
            'a' => A_BITS,
            'C' => C_BITS,
            'c' => C_BITS,
            'T' => T_BITS,
            't' => T_BITS,
            'G' => G_BITS,
            'g' => G_BITS,
            _ => continue,
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
        current_byte <<= 8 - bit_count;
        compressed.push(current_byte);
    }

    // Apply ZLIB compression
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&compressed).unwrap();
    encoder.finish().unwrap()
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
pub fn decompress_sequence(compressed: &[u8], sequence_length: usize) -> io::Result<String> {
    let mut decoder = ZlibDecoder::new(compressed);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    let mut sequence = String::new();

    for &byte in &decompressed_data {
        let mut current_byte = byte;
        for _ in 0..4 {
            if sequence.len() >= sequence_length {
                break;
            }
            let nucleotide = match (current_byte >> 6) & 0b11 {
                A_BITS => 'A',
                C_BITS => 'C',
                T_BITS => 'T',
                G_BITS => 'G',
                _ => unreachable!(),
            };
            sequence.push(nucleotide);
            current_byte <<= 2;
        }
    }

    Ok(sequence)
}
