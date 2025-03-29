use crate::genetics::{Genotype, MendelianGenotype};
use crate::terminal::{AnsiColor, AnsiEffect, TextBlueprint};

use serde::Deserialize;
use serde_json::de::from_reader;
use serde_json::from_str;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{self, Path};

pub struct Phenotype {
    blueprint: TextBlueprint,
    character: char,
}

impl Phenotype {
    pub fn new(blueprint: TextBlueprint, character: char) -> Self {
        Self {
            blueprint,
            character,
        }
    }

    pub fn to_string(&self) -> String {
        self.blueprint.apply(self.character.to_string().as_str())
    }
}

pub struct FlowerData {
    name: String,
    genotype_length: u8,
}

impl FlowerData {
    pub fn new(name: String, genotype_length: u8) -> Self {
        Self {
            name,
            genotype_length,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn genotype_length(&self) -> u8 {
        self.genotype_length
    }
}

pub trait Flower {
    type Genome: Genotype;

    fn new(genes: Vec<u8>) -> Option<Self>
    where
        Self: Sized;

    fn new_random() -> Self
    where
        Self: Sized;

    fn info(&self) -> FlowerData;
    fn genotype(&self) -> &Self::Genome;
    fn phenotype(&self, flower_context: &FlowerContext) -> Phenotype;
    fn to_string(&self, flower_context: &FlowerContext) -> String {
        self.phenotype(flower_context).to_string()
    }
}

pub enum ACNHFlower {
    Rose,
}

#[derive(Debug, Deserialize)]
pub struct ACNHPhenotypes {
    acnh_rose: Vec<String>,
}

impl ACNHPhenotypes {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let obj: ACNHPhenotypes = from_str(include_str!("acnh_phenotypes.json"))?;
        Ok(obj)
    }

    pub fn get_color(&self, flower_type: ACNHFlower, index: usize) -> Option<String> {
        match flower_type {
            ACNHFlower::Rose => self.acnh_rose.get(index).cloned(),
        }
    }
}

pub enum FlowerType {
    ACNHFlowerType(ACNHFlower),
}

pub struct FlowerContext {
    acnh_phenotypes: ACNHPhenotypes,
}

impl FlowerContext {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            acnh_phenotypes: ACNHPhenotypes::new()?,
        })
    }

    pub fn get_phenotype_string(&self, flower_type: FlowerType, index: usize) -> Option<String> {
        match flower_type {
            FlowerType::ACNHFlowerType(f) => self.acnh_phenotypes.get_color(f, index),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::flowers::acnh_flowers::ACNHRose;

    use super::*;

    #[test]
    fn test_roses() {
        
    }
}
