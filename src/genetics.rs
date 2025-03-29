use rand::{distr::Uniform, prelude::*};

/// The trait for anything that can cross-breed
pub trait Genotype {
    type Allele;

    fn new(genes: Vec<Self::Allele>) -> Option<Self>
    where
        Self: Sized;

    fn length(&self) -> usize;

    fn cross(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    fn as_index(&self) -> usize;
}

/// Represents a 0, 1, or 2. Useful for Mendelian Genotypes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trit {
    trit: u8,
}

impl Trit {
    pub fn new(trit: u8) -> Option<Self> {
        if trit > 2 { None } else { Some(Self { trit }) }
    }

    fn new_unchecked(trit: u8) -> Self {
        Self { trit }
    }

    pub fn from_bools(bools: [bool; 2]) -> Self {
        match bools {
            [true, true] => Self::new_unchecked(2),
            [false, false] => Self::new_unchecked(0),
            _ => Self::new_unchecked(1),
        }
    }

    pub fn as_bools(&self) -> [bool; 2] {
        match self.trit {
            0 => [false, false],
            1 => [true, false],
            2 => [true, true],
            _ => panic!("Trit has invalid value..."),
        }
    }

    pub fn cross(&self, other: Trit, rng: &mut ThreadRng) -> Trit {
        let b1 = *self.as_bools().choose(rng).unwrap();
        let b2 = *other.as_bools().choose(rng).unwrap();
        Trit::from_bools([b1, b2])
    }
}

/// Represents the genome of a plant with only two alleles for each gene
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MendelianGenotype {
    genes: Vec<Trit>,
}

impl FromIterator<Trit> for MendelianGenotype {
    fn from_iter<T: IntoIterator<Item = Trit>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl MendelianGenotype {
    pub fn new(genes: Vec<u8>) -> Option<Self> {
        genes
            .iter()
            .map(|&g| Trit::new(g))
            .collect::<Option<Vec<Trit>>>()
            .map(|trits| Self { genes: trits })
    }

    pub fn random(length: u8) -> Self {
        let mut rng = rand::rng();
        let trit_distr = Uniform::new_inclusive(0, 2).unwrap();
        let trits = (&mut rng)
            .sample_iter(trit_distr)
            .take(length as usize)
            .collect();
        Self::new(trits).unwrap()
    }

    pub fn cross(&self, other: &MendelianGenotype) -> Option<Self> {
        let mut rng = rand::rng();

        if self.genes.len() != other.genes.len() {
            None
        } else {
            Some(
                self.genes
                    .iter()
                    .zip(other.genes.iter())
                    .map(|(&t1, &t2)| t1.cross(t2, &mut rng))
                    .collect(),
            )
        }
    }

    pub fn as_u8_vec(&self) -> Vec<u8> {
        self.genes.iter().map(|t| t.trit).collect()
    }
}

impl Genotype for MendelianGenotype {
    type Allele = u8;

    fn new(genes: Vec<Self::Allele>) -> Option<Self>
    where
        Self: Sized,
    {
        MendelianGenotype::new(genes)
    }
    fn cross(&self, other: &Self) -> Option<Self> {
        self.cross(other)
    }

    fn length(&self) -> usize {
        self.genes.len()
    }

    fn as_index(&self) -> usize {
        self.genes
            .iter()
            .rev()
            .zip(0u32..)
            .map(|(t, p)| t.trit as usize * 3usize.pow(p))
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_trits() {
        let tt = Trit::new(2).expect("tt failed");
        let ff = Trit::new(0).expect("ff failed");
        let tf = Trit::new(1).expect("tf failed");

        let mut rng = rand::rng();

        assert_eq!(tt.cross(ff, &mut rng), tf, "tt cross ff didnt equal tf");
        assert_eq!(tt.cross(tt, &mut rng), tt, "tt cross tt didnt equal tt");
        assert_eq!(ff.cross(ff, &mut rng), ff, "ff cross ff didnt equal ff");
    }

    #[test]
    fn cross_genomes() {
        let m1 = MendelianGenotype::new(vec![0, 0, 2, 2]).expect("m1 failed");
        let m2 = MendelianGenotype::new(vec![2, 0, 2, 0]).expect("m2 failed");
        let expected = MendelianGenotype::new(vec![1, 0, 2, 1]).expect("expected failed");
        assert_eq!(m1.cross(&m2).expect("crossing failed"), expected);
    }
}
