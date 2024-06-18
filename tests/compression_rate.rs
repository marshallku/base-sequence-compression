use base_sequence_compression::compress_sequence;

#[cfg(test)]
mod tests {
    use super::*;

    fn calculate_compression_rate(original: &str, compressed: &[u8]) -> f64 {
        let original_size = original.len();
        let compressed_size = compressed.len();
        if original_size == 0 {
            return 0.0;
        }
        100.0 - ((compressed_size as f64 / original_size as f64) * 100.0)
    }

    #[test]
    fn test_compression_rate() {
        let dna_sequence = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTAC";
        let compressed = compress_sequence(dna_sequence);
        let compression_rate = calculate_compression_rate(dna_sequence, &compressed);

        println!("Compression rate: {:.2}%", compression_rate);

        assert!(compression_rate < 100.0);
    }
}
