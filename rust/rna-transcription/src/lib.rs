#[derive(Debug, PartialEq)]
enum DNANucleotide {
    A,
    C,
    G,
    T,
}

#[derive(Debug, PartialEq)]
enum RNANucleotide {
    A,
    C,
    G,
    U,
}

#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: Vec<DNANucleotide>,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: Vec<RNANucleotide>,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut dna_vec = Vec::with_capacity(dna.len());
        for (index, c) in dna.chars().enumerate() {
            match c {
                'A' => dna_vec.push(DNANucleotide::A),
                'C' => dna_vec.push(DNANucleotide::C),
                'G' => dna_vec.push(DNANucleotide::G),
                'T' => dna_vec.push(DNANucleotide::T),
                _ => return Err(index),
            }
        }
        Ok(DNA { dna: dna_vec })
    }

    pub fn to_rna(self) -> RNA {
        let rna = self
            .dna
            .into_iter()
            .map(|c| match c {
                DNANucleotide::A => RNANucleotide::U,
                DNANucleotide::C => RNANucleotide::G,
                DNANucleotide::G => RNANucleotide::C,
                DNANucleotide::T => RNANucleotide::A,
            })
            .collect();
        RNA { rna }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut rna_vec = Vec::with_capacity(rna.len());
        for (index, c) in rna.chars().enumerate() {
            match c {
                'A' => rna_vec.push(RNANucleotide::A),
                'C' => rna_vec.push(RNANucleotide::C),
                'G' => rna_vec.push(RNANucleotide::G),
                'U' => rna_vec.push(RNANucleotide::U),
                _ => return Err(index),
            }
        }
        Ok(RNA { rna: rna_vec })
    }
}
