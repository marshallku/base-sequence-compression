pub fn compress_sequence(sequence: &str) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for base in sequence.chars() {
        let bits = match base {
            'A' => 0b00,
            'C' => 0b01,
            'T' => 0b10,
            'G' => 0b11,
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

#[cfg(test)]
mod tests {}
