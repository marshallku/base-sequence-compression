pub const A_BITS: u8 = 0b00;
pub const C_BITS: u8 = 0b01;
pub const T_BITS: u8 = 0b10;
pub const G_BITS: u8 = 0b11;

pub fn compress_sequence(sequence: &str) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

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

pub fn decompress_sequence(compressed: &[u8], length: usize) -> String {
    let mut sequence = String::new();
    let mut bits = 0;
    let mut bit_count = 0;

    for &byte in compressed {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let dna_sequence = "ACGTACGTACGT";
        let compressed = compress_sequence(dna_sequence);
        let decompressed = decompress_sequence(&compressed, dna_sequence.len());

        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    #[should_panic]
    fn test_invalid_sequence() {
        let dna_sequence = "ACXGT";
        compress_sequence(dna_sequence);
    }
}
