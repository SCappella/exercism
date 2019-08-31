use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !['A', 'C', 'G', 'T'].contains(&nucleotide) {
        return Err(nucleotide);
    }

    if let Some(c) = dna.chars().find(|c| !['A', 'C', 'G', 'T'].contains(c)) {
        return Err(c);
    }
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for &c in &['A', 'C', 'G', 'T'] {
        counts.insert(c, 0usize);
    }

    for c in dna.chars() {
        if let Some(count) = counts.get_mut(&c) {
            *count += 1
        } else {
            return Err(c);
        }
    }
    Ok(counts)
}
