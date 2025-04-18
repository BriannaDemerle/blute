use crate::flowers::acnh_flowers::{ACNHHyacinth, ACNHMum, ACNHRose};
use crate::genetics::{Gene, GeneType, Genotype, MendelianGene};
use crate::ui::terminal::{AnsiColor, AnsiEffect, TextBlueprint};

use serde::Deserialize;
use serde_json::de::from_reader;
use serde_json::from_str;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{self, Path};
use std::rc::Rc;

pub type AnyFlower = Rc<dyn Flower>;

#[derive(Debug, Clone, Copy)]
pub enum ACNHFlowerType {
    Rose,
    Mum,
    Hyacinth,
}

#[derive(Debug, Clone, Copy)]
pub enum FlowerType {
    ACNH(ACNHFlowerType),
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FlowerData {
    name: String,
    flower_type: FlowerType,
    gene_print: Vec<GeneType>,
}

impl FlowerData {
    pub fn new(name: String, gene_print: Vec<GeneType>, flower_type: FlowerType) -> Self {
        Self {
            name,
            flower_type,
            gene_print,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn gene_print(&self) -> Vec<GeneType> {
        self.gene_print.clone()
    }

    pub fn flower_type(&self) -> FlowerType {
        self.flower_type
    }
}

pub trait Flower {
    fn info(&self) -> FlowerData;

    fn genotype(&self) -> Genotype;
    fn phenotype(&self, flower_context: &FlowerContext) -> Phenotype;
    fn to_string(&self, flower_context: &FlowerContext) -> String {
        self.phenotype(flower_context).to_string()
    }

    fn can_cross(&self, other: &Self) -> bool
    where
        Self: Sized,
    {
        self.genotype().can_cross(&other.genotype())
    }

    fn cross(&self, other: &Self) -> Genotype
    where
        Self: Sized,
    {
        self.genotype()
            .cross_with(&other.genotype())
            .expect("Couldn't cross-breed flowers...")
    }
}

impl std::fmt::Debug for dyn Flower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Flower (info: {:?}, genotype: {:?})",
            self.info(),
            self.genotype()
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ACNHPhenotypes {
    acnh_rose: Vec<String>,
    acnh_mum: Vec<String>,
    acnh_hyacinth: Vec<String>
}

impl ACNHPhenotypes {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let obj: ACNHPhenotypes = from_str(include_str!("acnh_phenotypes.json"))?;
        Ok(obj)
    }

    pub fn get_color(&self, flower_type: ACNHFlowerType, index: usize) -> Option<String> {
        match flower_type {
            ACNHFlowerType::Rose => self.acnh_rose.get(index).cloned(),
            ACNHFlowerType::Mum => self.acnh_mum.get(index).cloned(),
            ACNHFlowerType::Hyacinth => self.acnh_hyacinth.get(index).cloned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlowerContext {
    acnh_phenotypes: ACNHPhenotypes,
    seeds: Vec<Vec<AnyFlower>>
}

impl FlowerContext {
    fn acnh_seeds() -> Vec<Vec<AnyFlower>> {
        let m_0 = Gene::Mendelian(MendelianGene::HomozygousRecessive);
        let m_1 = Gene::Mendelian(MendelianGene::Heterozygous);
        let m_2 = Gene::Mendelian(MendelianGene::HomozygousDominant);

        vec![
            // ACNH Roses
            vec![
                Rc::new(ACNHRose(Genotype::new(vec![m_2, m_0, m_0, m_1]))),
                Rc::new(ACNHRose(Genotype::new(vec![m_0, m_0, m_1, m_0]))),
                Rc::new(ACNHRose(Genotype::new(vec![m_0, m_2, m_0, m_0])))
            ],
            // ACNH Mums
            vec![
                Rc::new(ACNHMum(Genotype::new(vec![m_2, m_0, m_0]))),
                Rc::new(ACNHMum(Genotype::new(vec![m_0, m_0, m_1]))),
                Rc::new(ACNHMum(Genotype::new(vec![m_0, m_2, m_0])))
            ],
            // ACNH Hyacinths
            vec![
                Rc::new(ACNHHyacinth(Genotype::new(vec![m_2, m_0, m_1]))),
                Rc::new(ACNHHyacinth(Genotype::new(vec![m_0, m_0, m_1]))),
                Rc::new(ACNHHyacinth(Genotype::new(vec![m_0, m_2, m_0]))),
            ]
        ]
    }

    fn all_seeds() -> Vec<Vec<AnyFlower>> {
        vec![Self::acnh_seeds()].iter().flatten().cloned().collect()
    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            acnh_phenotypes: ACNHPhenotypes::new()?,
            seeds: Self::all_seeds()
        })
    }

    pub fn get_phenotype_string(&self, flower_type: FlowerType, index: usize) -> Option<String> {
        match flower_type {
            FlowerType::ACNH(f) => self.acnh_phenotypes.get_color(f, index),
        }
    }

    pub fn get_seeds(&self) -> Vec<Vec<AnyFlower>> {
        self.seeds.clone()
    }
}
