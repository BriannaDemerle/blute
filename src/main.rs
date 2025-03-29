#![allow(unused)]

mod flowers;
mod genetics;
mod terminal;

use flowers::{acnh_flowers::ACNHRose, flower::Flower};
use genetics::{Genotype, MendelianGenotype, Trit};
use getch_rs::Key;
use terminal::{AnsiColor, AnsiEffect, KeyStack, TextBlueprint, new_keystack, refresh_with};

fn main() {
    // let purple = AnsiColor::from_rgb_cube(4, 2, 4).unwrap();
    // let blue = AnsiColor::from_rgb_cube(2, 2, 5).unwrap();
    // let blueprint_unpressed = TextBlueprint::new().with_text_color(blue).to_owned();
    // let blueprint_pressed = TextBlueprint::new().with_text_color(purple).to_owned();

    // let (key_stack, _guard) = new_keystack();
    // loop {
    //     let mut key_stack_mutex = key_stack.lock().unwrap();
    //     if key_stack_mutex.should_quit() {
    //         break
    //     }
    //     let i: Vec<Key> = key_stack_mutex.poll_keys().collect();
    //     if i.len() != 0 {
    //         println!("{:?}", i);
    //     }
    // }
    let c = flowers::flower::FlowerContext::new().expect("Couldn't make flower context");
    for i in 0..10 {
        let r = ACNHRose::new_random();
        let n = c.get_phenotype_string(ACNHRose::FLOWER_TYPE, r.genotype().as_index()).unwrap();
        println!("{}: {}", n, r.to_string(&c));
    }
}
