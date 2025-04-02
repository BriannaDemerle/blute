use std::str::FromStr;

use crate::flowers::flower::{
    ACNHFlowerType, Flower, FlowerContext, FlowerData, FlowerType, Phenotype,
};
use crate::genetics::{GeneType, Genotype};

use crate::AnsiColor;
use crate::ui::terminal::TextBlueprint;

fn get_acnh_flower_color(color_name: &str) -> Option<AnsiColor> {
    match color_name {
        "Red" => Some(AnsiColor::from_code(160)),
        "White" => Some(AnsiColor::from_code(7)),
        "Purple" => Some(AnsiColor::from_code(90)),
        "Yellow" => Some(AnsiColor::from_code(221)),
        "Pink" => Some(AnsiColor::from_code(211)),
        "Orange" => Some(AnsiColor::from_code(208)),
        "Black" => Some(AnsiColor::from_code(8)),
        "Blue" => Some(AnsiColor::from_code(27)),
        "Green" => Some(AnsiColor::from_code(106)),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACNHRose(pub Genotype);

impl Flower for ACNHRose {
    fn info(&self) -> FlowerData {
        let gene_print = vec![GeneType::Mendelian; 4];
        FlowerData::new(
            "Rose (acnh)".to_string(),
            gene_print,
            FlowerType::ACNH(ACNHFlowerType::Rose),
        )
    }

    fn genotype(&self) -> Genotype {
        self.0.clone()
    }

    fn phenotype(&self, flower_context: &FlowerContext) -> Phenotype {
        let color_string = flower_context
            .get_phenotype_string(
                self.info().flower_type(),
                self.0
                    .into_index()
                    .expect("could not get color index for acnh rose"),
            )
            .expect("could not get color for acnh rose");
        let color =
            get_acnh_flower_color(&color_string).expect("could not get ansi color for acnh rose");
        let blueprint = TextBlueprint::new().with_text_color(color).to_owned();
        Phenotype::new(blueprint, '@')
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACNHMum(pub Genotype);

impl Flower for ACNHMum {
    fn info(&self) -> FlowerData {
        let gene_print = vec![GeneType::Mendelian; 3];
        FlowerData::new(
            "Mum (acnh)".to_string(),
            gene_print,
            FlowerType::ACNH(ACNHFlowerType::Mum),
        )
    }

    fn genotype(&self) -> Genotype {
        self.0.clone()
    }

    fn phenotype(&self, flower_context: &FlowerContext) -> Phenotype {
        let color_string = flower_context
            .get_phenotype_string(
                self.info().flower_type(),
                self.0
                    .into_index()
                    .expect("could not get color index for acnh mum"),
            )
            .expect("could not get color for acnh mum");
        let color =
            get_acnh_flower_color(&color_string).expect("could not get ansi color for acnh mum");
        let blueprint = TextBlueprint::new().with_text_color(color).to_owned();
        Phenotype::new(blueprint, '⚛')
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ACNHHyacinth(pub Genotype);

impl Flower for ACNHHyacinth {
    fn info(&self) -> FlowerData {
        let gene_print = vec![GeneType::Mendelian; 3];
        FlowerData::new(
            "Hyacinth (acnh)".to_string(),
            gene_print,
            FlowerType::ACNH(ACNHFlowerType::Hyacinth),
        )
    }

    fn genotype(&self) -> Genotype {
        self.0.clone()
    }

    fn phenotype(&self, flower_context: &FlowerContext) -> Phenotype {
        let color_string = flower_context
            .get_phenotype_string(
                self.info().flower_type(),
                self.0
                    .into_index()
                    .expect("could not get color index for acnh hyacinth"),
            )
            .expect("could not get color for acnh hyacinth");
        let color =
            get_acnh_flower_color(&color_string).expect("could not get ansi color for acnh hyacinth");
        let blueprint = TextBlueprint::new().with_text_color(color).to_owned();
        Phenotype::new(blueprint, '⁑')
    }
}