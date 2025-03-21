use base_sequence_compression::{compress_sequence, decompress_sequence};

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::Path};

    use base_sequence_compression::{compress_fasta, decompress_fasta};

    use super::*;

    #[test]
    fn test_compress_decompress() {
        let dna_sequence = "ACGTACGTACGT";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();

        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_empty_sequence() {
        let dna_sequence = "";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_single_character_a() {
        let dna_sequence = "A";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_single_character_c() {
        let dna_sequence = "C";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_single_character_t() {
        let dna_sequence = "T";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_single_character_g() {
        let dna_sequence = "G";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_non_multiple_of_four_length() {
        let dna_sequence = "ACGTACGTA";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_non_multiple_of_four_length_2() {
        let dna_sequence = "ACGTACGTAC";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_non_multiple_of_four_length_3() {
        let dna_sequence = "ACGTACGTACG";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();
        assert_eq!(dna_sequence, decompressed);
    }

    #[test]
    fn test_lower_case() {
        let dna_sequence = "acgt";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();

        assert_eq!(dna_sequence.to_uppercase(), decompressed);
    }

    #[test]
    fn test_invalid_sequence() {
        let dna_sequence = "ACXGT";
        let compressed = compress_sequence(dna_sequence);
        let sequence_length = dna_sequence.len();
        let decompressed = decompress_sequence(&compressed, sequence_length).unwrap();

        assert_eq!("ACGT", decompressed);
    }

    #[test]
    fn test_fasta_file() {
        let input_path = Path::new("tests/input/test.fasta");

        let content = std::fs::read_to_string(input_path).unwrap();
        let compressed = compress_fasta(&content);
        let decompressed = decompress_fasta(&compressed);

        assert_eq!(content, decompressed);
    }

    #[test]
    fn test_fasta_file_decompress() {
        let input_path = Path::new("tests/input/test.fasta");
        let output_path = Path::new("tests/output/test.bin");

        let input_content = std::fs::read_to_string(input_path).unwrap();
        let mut output_file = File::open(output_path).ok().unwrap();
        let mut output_content = Vec::new();
        output_file.read_to_end(&mut output_content).ok();

        let compressed = compress_fasta(&input_content);
        let decompressed = decompress_fasta(&output_content);

        assert_eq!(compressed, output_content);
        assert_eq!(input_content, decompressed);
    }
}
