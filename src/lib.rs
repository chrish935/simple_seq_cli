use std::collections::HashMap;

/// Transcription
pub fn transcribe_dna_to_rna(dna: &str) -> Result<String, &'static str> {
    let mut rna = String::new();

    for nucleotide in dna.chars() {
        let rna_nucleotide = match nucleotide {
            'A' => 'U',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => return Err("Invalid DNA nucleotide"),
        };
        rna.push(rna_nucleotide);
    }

    Ok(rna)
}

/// Translation
// Genetic code mapping codons to amino acids
const GENETIC_CODE: &[(&str, &str)] = &[
    // M
    ("AUG", "M"),
    // V
    ("GUG", "V"),
    ("GUA", "V"),
    ("GUC", "V"),
    ("GUU", "V"),
    // A
    ("GCU", "A"),
    ("GCC", "A"),
    ("GCA", "A"),
    ("GCG", "A"),
    // D
    ("GAC", "D"),
    ("GAU", "D"),
    // E
    ("GAG", "E"),
    ("GAA", "E"),
    // G
    ("GGG", "G"),
    ("GGA", "G"),
    ("GGC", "G"),
    ("GGU", "G"),
    // F
    ("UUU", "F"),
    ("UUC", "F"),
    // L
    ("UUA", "L"),
    ("UUG", "L"),
    ("CUU", "L"),
    ("CUC", "L"),
    ("CUA", "L"),
    ("CUG", "L"),
    // S
    ("UCU", "S"),
    ("UCC", "S"),
    ("UCA", "S"),
    ("UCG", "S"),
    ("AGU", "S"),
    ("AGC", "S"),
    // Y
    ("UAU", "Y"),
    ("UAC", "Y"),
    // C
    ("UGU", "C"),
    ("UGC", "C"),
    // W
    ("UGG", "W"),
    // P
    ("CCU", "P"),
    ("CCC", "P"),
    ("CCA", "P"),
    ("CCG", "P"),
    // H
    ("CAU", "H"),
    ("CAC", "H"),
    // Q
    ("CAA", "Q"),
    ("CAG", "Q"),
    // R
    ("CGU", "R"),
    ("CGC", "R"),
    ("CGA", "R"),
    ("CGG", "R"),
    ("AGA", "R"),
    ("AGG", "R"),
    // I
    ("AUU", "I"),
    ("AUC", "I"),
    ("AUA", "I"),
    // T
    ("ACU", "T"),
    ("ACC", "T"),
    ("ACA", "T"),
    ("ACG", "T"),
    // N
    ("AAU", "N"),
    ("AAC", "N"),
    // K
    ("AAA", "K"),
    ("AAG", "K"),
    // Stop
    ("UAA", "*"),
    ("UAG", "*"),
    ("UGA", "*"),
];

pub fn translate_rna_to_aa(dna: &str) -> String {
    let mut aa_seq = String::new();
    let codons = GENETIC_CODE.iter().cloned().collect::<HashMap<_, _>>();

    for codon in dna.as_bytes().chunks(3) {
        if codon.len() == 3 {
            let codon_str = std::str::from_utf8(codon).unwrap();
            if let Some(aa) = codons.get(codon_str) {
                aa_seq.push_str(aa);
            } else {
                return String::from("Invalid RNA sequence");
            }
        }
    }

    aa_seq
}
