use std::str::FromStr;

use crate::flowers::flower::{Flower, FlowerData, Phenotype};
use crate::genetics::{Genotype, MendelianGenotype};
use crate::terminal::{AnsiColor, TextBlueprint};

use super::flower::{ACNHFlower, FlowerContext, FlowerType};

pub struct ACNHRose {
    genotype: MendelianGenotype
}

impl ACNHRose {
    pub const GENOTYPE_LENGTH: u8 = 4;
    pub const FLOWER_TYPE: FlowerType = FlowerType::ACNHFlowerType(ACNHFlower::Rose);

    pub fn get_color(color_name: &str) -> Option<AnsiColor> {
        match color_name {
            "Red" => Some(AnsiColor::from_code(160)),
            "White" => Some(AnsiColor::from_code(7)),
            "Purple" => Some(AnsiColor::from_code(90)),
            "Yellow" => Some(AnsiColor::from_code(221)),
            "Pink" => Some(AnsiColor::from_code(211)),
            "Orange" => Some(AnsiColor::from_code(208)),
            "Black" => Some(AnsiColor::from_code(8)),
            "Blue" => Some(AnsiColor::from_code(27)),
            _ => None
        }
    }
}

impl Flower for ACNHRose {
    type Genome = MendelianGenotype;

    fn info(&self) -> FlowerData {
        FlowerData::new(
            String::from_str("ACNH Rose").expect("Could not generate flower data for ACNH Rose"),
            4
        )
    }

    fn new(genes: Vec<u8>) -> Option<Self>
            where Self: Sized {
        if let Some(genotype) = MendelianGenotype::new(genes) {
            Some(Self {
                genotype
            })
        } else {
            None
        }
    }

    fn new_random() -> Self
            where Self: Sized {
        Self {
            genotype: MendelianGenotype::random(Self::GENOTYPE_LENGTH)
        }
    }

    fn genotype(&self) -> &Self::Genome {
        &self.genotype
    }

    fn phenotype(&self, flower_context: &FlowerContext) -> Phenotype {
        let index = self.genotype.as_index();
        let string = flower_context
            .get_phenotype_string(Self::FLOWER_TYPE, index)
            .expect("Could not get phenotype");
        let color = ACNHRose::get_color(string.as_str()).expect("Could not get rose color");
        
        let rose_blueprint = TextBlueprint::new().with_text_color(color).to_owned();
        Phenotype::new(rose_blueprint, '@')
    }
}

