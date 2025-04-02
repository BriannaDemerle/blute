#![allow(unused)]

mod context;
mod flowers;
mod genetics;
mod ui;

use std::rc::Rc;

use context::Context;
use flowers::{
    acnh_flowers::{ACNHMum, ACNHRose},
    flower::{Flower, FlowerContext},
};
use genetics::{Gene, Genotype, MendelianGene};
use getch_rs::Key;
use ui::board::Board;
use ui::terminal::{AnsiColor, AnsiEffect, KeyStack, TextBlueprint, new_keystack};

fn main() {
    let mut context = Context::new();

    // clear terminal
    print!("\x1Bc");

    loop {
        if context.update() {
            break;
        }
    }
}
