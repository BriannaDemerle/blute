use std::{rc::Rc, vec::IntoIter};

use rand::{distr::Uniform, prelude::*, rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneType {
    Mendelian,
    Bloodlike,
    Quadruplet,
}

impl GeneType {
    pub fn get_random(&self, rng: &mut ThreadRng) -> Gene {
        Gene::random(*self, rng)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gene {
    Mendelian(MendelianGene),
    Bloodlike(BloodlikeGene),
    Quadruplet(QuadrupletGene),
}

impl Gene {
    pub fn random(gene_type: GeneType, rng: &mut ThreadRng) -> Gene {
        match gene_type {
            GeneType::Mendelian => Self::Mendelian(MendelianGene::random(rng)),
            GeneType::Bloodlike => Self::Bloodlike(BloodlikeGene::random(rng)),
            GeneType::Quadruplet => Self::Quadruplet(QuadrupletGene::random(rng)),
        }
    }

    pub fn gene_type(&self) -> GeneType {
        match self {
            Gene::Mendelian(_) => GeneType::Mendelian,
            Gene::Bloodlike(_) => GeneType::Mendelian,
            Gene::Quadruplet(_) => GeneType::Quadruplet,
        }
    }

    pub fn cross_with(&self, other: &Self, rng: &mut ThreadRng) -> Option<Self> {
        match self {
            Gene::Mendelian(g) => {
                if let Gene::Mendelian(o) = other {
                    Some(Gene::Mendelian(g.cross(o, rng)))
                } else {
                    None
                }
            }
            Gene::Bloodlike(g) => {
                if let Gene::Bloodlike(o) = other {
                    Some(Gene::Bloodlike(g.cross(o, rng)))
                } else {
                    None
                }
            }
            Gene::Quadruplet(g) => {
                if let Gene::Quadruplet(o) = other {
                    Some(Gene::Quadruplet(g.cross(o, rng)))
                } else {
                    None
                }
            }
        }
    }

    /// Only Some for Mendelian genes
    pub fn into_usize(&self) -> Option<usize> {
        if let Self::Mendelian(m) = *self {
            Some(m.into_usize())
        } else {
            None
        }
    }
}

/// Boring genes
/// AA Aa aa
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MendelianGene {
    HomozygousRecessive,
    Heterozygous,
    HomozygousDominant,
}

impl MendelianGene {
    pub const CHOOSE_TABLE: [MendelianGene; 4] = [
        MendelianGene::Heterozygous,
        MendelianGene::Heterozygous,
        MendelianGene::HomozygousDominant,
        MendelianGene::HomozygousRecessive,
    ];

    pub fn into_usize(&self) -> usize {
        match self {
            Self::HomozygousDominant => 2,
            Self::Heterozygous => 1,
            Self::HomozygousRecessive => 0,
        }
    }

    pub fn from_bools(bools: [bool; 2]) -> Self {
        match bools {
            [true, true] => Self::HomozygousDominant,
            [false, false] => Self::HomozygousRecessive,
            _ => Self::Heterozygous,
        }
    }

    pub fn to_bools(&self) -> [bool; 2] {
        match self {
            MendelianGene::HomozygousDominant => [true, true],
            MendelianGene::Heterozygous => [true, false],
            MendelianGene::HomozygousRecessive => [false, false],
        }
    }

    fn random(rng: &mut ThreadRng) -> Self
    where
        Self: Sized,
    {
        *Self::CHOOSE_TABLE
            .choose(rng)
            .expect("Could not choose random Mendelian Gene")
    }

    fn cross(&self, other: &Self, rng: &mut ThreadRng) -> Self
    where
        Self: Sized,
    {
        let b1 = *self
            .to_bools()
            .choose(rng)
            .expect("Could not choose random bool");
        let b2 = *other
            .to_bools()
            .choose(rng)
            .expect("Could not choose random bool");
        Self::from_bools([b1, b2])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BloodlikeAllele {
    AntigenA,
    AntigenB,
    NoAntigen,
}

/// Blood type-esque genes
/// IaIa Iai IbIb Ibi IaIb ii
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloodlikeGene {
    BloodHomozygousA,
    BloodHeterozygousA,
    BloodHomozygousB,
    BloodHeterozygousB,
    BloodAB,
    BloodO,
}

impl BloodlikeGene {
    pub const CHOOSE_TABLE: [BloodlikeGene; 9] = [
        BloodlikeGene::BloodHomozygousA,
        BloodlikeGene::BloodHeterozygousA,
        BloodlikeGene::BloodHeterozygousA,
        BloodlikeGene::BloodHomozygousB,
        BloodlikeGene::BloodHeterozygousB,
        BloodlikeGene::BloodHeterozygousB,
        BloodlikeGene::BloodAB,
        BloodlikeGene::BloodAB,
        BloodlikeGene::BloodO,
    ];

    fn to_alleles(&self) -> [BloodlikeAllele; 2] {
        match self {
            BloodlikeGene::BloodHomozygousA => {
                [BloodlikeAllele::AntigenA, BloodlikeAllele::AntigenA]
            }
            BloodlikeGene::BloodHeterozygousA => {
                [BloodlikeAllele::AntigenA, BloodlikeAllele::NoAntigen]
            }
            BloodlikeGene::BloodHomozygousB => {
                [BloodlikeAllele::AntigenB, BloodlikeAllele::AntigenB]
            }
            BloodlikeGene::BloodHeterozygousB => {
                [BloodlikeAllele::AntigenB, BloodlikeAllele::NoAntigen]
            }
            BloodlikeGene::BloodAB => [BloodlikeAllele::AntigenA, BloodlikeAllele::AntigenB],
            BloodlikeGene::BloodO => [BloodlikeAllele::NoAntigen, BloodlikeAllele::NoAntigen],
        }
    }

    fn from_alleles(alleles: [BloodlikeAllele; 2]) -> Self {
        match alleles {
            [BloodlikeAllele::AntigenA, BloodlikeAllele::AntigenA] => Self::BloodHomozygousA,
            [BloodlikeAllele::AntigenB, BloodlikeAllele::AntigenB] => Self::BloodHomozygousB,
            [BloodlikeAllele::AntigenA, BloodlikeAllele::AntigenB] => Self::BloodAB,
            [BloodlikeAllele::NoAntigen, BloodlikeAllele::NoAntigen] => Self::BloodO,
            [_, BloodlikeAllele::AntigenA] | [BloodlikeAllele::AntigenA, _] => {
                Self::BloodHeterozygousA
            }
            [_, BloodlikeAllele::AntigenB] | [BloodlikeAllele::AntigenB, _] => {
                Self::BloodHeterozygousA
            }
        }
    }

    fn random(rng: &mut ThreadRng) -> Self
    where
        Self: Sized,
    {
        *Self::CHOOSE_TABLE
            .choose(rng)
            .expect("Could not choose random bloodlike gene")
    }

    fn cross(&self, other: &Self, rng: &mut ThreadRng) -> Self
    where
        Self: Sized,
    {
        let a1 = *self
            .to_alleles()
            .choose(rng)
            .expect("Could not choose random bloodlike allele");
        let a2 = *other
            .to_alleles()
            .choose(rng)
            .expect("Could not choose random bloodlike allele");
        Self::from_alleles([a1, a2])
    }
}

/// Fictitious (probably)
/// AAAA AAAa AAaa Aaaa aaaa
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadrupletGene {
    HomozygousDominant,
    SemihomozygousDominant,
    Heterozygous,
    SemihomozygousRecessive,
    HomozygousRecessive,
}

impl QuadrupletGene {
    pub fn to_alleles(&self) -> [bool; 4] {
        match self {
            QuadrupletGene::HomozygousDominant => [true, true, true, true],
            QuadrupletGene::SemihomozygousDominant => [true, true, true, false],
            QuadrupletGene::Heterozygous => [true, true, false, false],
            QuadrupletGene::SemihomozygousRecessive => [true, false, false, false],
            QuadrupletGene::HomozygousRecessive => [false, false, false, false],
        }
    }

    pub fn from_alleles(alleles: [bool; 4]) -> Self {
        match alleles.iter().map(|&b| b as u8).sum() {
            0 => QuadrupletGene::HomozygousDominant,
            1 => QuadrupletGene::SemihomozygousDominant,
            2 => QuadrupletGene::Heterozygous,
            3 => QuadrupletGene::SemihomozygousRecessive,
            4 => QuadrupletGene::HomozygousRecessive,
            _ => panic!("More than 4 booleans in [bool; 4]..."),
        }
    }

    pub fn cross(&self, other: &Self, rng: &mut ThreadRng) -> Self
    where
        Self: Sized,
    {
        let b1s: Vec<bool> = self.to_alleles().choose_multiple(rng, 2).cloned().collect();
        let b2s: Vec<bool> = other
            .to_alleles()
            .choose_multiple(rng, 2)
            .cloned()
            .collect();
        let bools: Vec<bool> = b1s.iter().chain(b2s.iter()).cloned().collect();

        Self::from_alleles(
            bools
                .as_slice()
                .try_into()
                .expect("Should have been 4 values"),
        )
    }

    pub fn random(rng: &mut ThreadRng) -> Self
    where
        Self: Sized,
    {
        let bools = [
            rng.random_bool(0.5),
            rng.random_bool(0.5),
            rng.random_bool(0.5),
            rng.random_bool(0.5),
        ];
        Self::from_alleles(bools)
    }
}

/// All of the genes in a flower
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Genotype {
    genes: Vec<Gene>,
}

impl FromIterator<Gene> for Genotype {
    fn from_iter<T: IntoIterator<Item = Gene>>(iter: T) -> Self {
        let genes = iter.into_iter().collect();
        Self::new(genes)
    }
}

impl Genotype {
    pub fn new(genes: Vec<Gene>) -> Self {
        Genotype { genes }
    }

    pub fn new_random(gene_print: Vec<GeneType>) -> Self {
        let mut rng = rng();
        let genotype = gene_print.iter().map(|t| t.get_random(&mut rng)).collect();

        Self { genes: genotype }
    }

    pub fn gene_print(&self) -> Vec<GeneType> {
        self.genes.iter().map(|g| g.gene_type()).collect()
    }

    pub fn can_cross(&self, other: &Self) -> bool {
        self.gene_print() == other.gene_print()
    }

    pub fn cross_with(&self, other: &Self) -> Option<Self> {
        let mut rng = rng();
        self.genes
            .iter()
            .zip(other.genes.iter())
            .map(|(&g1, g2)| g1.cross_with(g2, &mut rng))
            .collect()
    }

    /// only Some for if all Mendelian genes
    pub fn into_index(&self) -> Option<usize> {
        self.genes
            .iter()
            .enumerate()
            .map(|(i, &g)| g.into_usize().map(|b| b * 3usize.pow(i as u32)))
            .sum()
    }
}
