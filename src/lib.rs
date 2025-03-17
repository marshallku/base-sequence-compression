use std::io::{self, Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
#[allow(unused_imports)]
use wasm_bindgen::prelude::wasm_bindgen;

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
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

/// Compresses a FASTA file content into a vector of bytes.
///
/// The FASTA file content is expected to have a header line followed by
/// the DNA sequence. The DNA sequence is compressed by representing each
/// base (A, C, T, G) with 2 bits. The compressed data starts with a 4-byte
/// (u32) integer representing the length of the original sequence.
///
/// # Arguments
///
/// * `content` - A string slice that holds the FASTA file content.
///
/// # Returns
///
/// A vector of bytes containing the compressed FASTA file content.
pub fn compress_fasta(content: &str) -> Vec<u8> {
    let mut lines = content.lines();
    let header = lines.next().unwrap_or("").to_string();
    let sequence: String = lines.map(|line| line.trim()).collect();

    let sequence_length = sequence.len() as u32;
    let compressed_data = compress_sequence(&sequence);

    let mut output = Vec::new();

    // Write header length (4 bytes)
    output.extend_from_slice(&(header.len() as u32).to_le_bytes());

    // Write header
    output.extend_from_slice(header.as_bytes());

    // Write sequence length (4 bytes)
    output.extend_from_slice(&sequence_length.to_le_bytes());

    // Write compressed data length (4 bytes)
    output.extend_from_slice(&(compressed_data.len() as u32).to_le_bytes());

    // Write compressed data
    output.extend_from_slice(&compressed_data);

    output
}

/// Decompresses a vector of bytes into a FASTA file content.
///
/// The compressed data starts with a 4-byte (u32) integer representing
/// the length of the header, followed by the header, the sequence length,
/// and the compressed sequence data. Each base (A, C, T, G) is represented
/// by 2 bits.
///
/// # Arguments
///
/// * `data` - A slice of bytes containing the compressed FASTA file content.
/// * `length` - Length of the original sequence.
///
/// # Returns
///
/// A string containing the decompressed FASTA file content.
///
/// # Errors
///
/// Returns an error if the file is too short or if the file is missing
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn decompress_fasta(data: &[u8]) -> String {
    if data.len() < 12 {
        return "".to_string();
    }

    let header_len = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;

    if data.len() < 12 + header_len {
        return "".to_string();
    }

    let header =
        match String::from_utf8(data[4..4 + header_len].to_vec()).map_err(|e| e.to_string()) {
            Ok(header) => header,
            Err(_) => return "".to_string(),
        };

    let sequence_length =
        u32::from_le_bytes(data[4 + header_len..8 + header_len].try_into().unwrap()) as usize;

    let compressed_len =
        u32::from_le_bytes(data[8 + header_len..12 + header_len].try_into().unwrap()) as usize;

    if data.len() < 12 + header_len + compressed_len {
        return "".to_string();
    }

    let compressed_data = &data[12 + header_len..12 + header_len + compressed_len];
    let decompressed = decompress_sequence(compressed_data, sequence_length).unwrap_or_default();

    let mut result =
        String::with_capacity(header.len() + decompressed.len() + (decompressed.len() / 60) * 2);
    result.push_str(&header);
    result.push('\n');

    for chunk in decompressed.as_bytes().chunks(60) {
        result.extend(chunk.iter().map(|&b| b as char));
        result.push('\n');
    }

    result
}
