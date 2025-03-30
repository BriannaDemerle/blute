#![allow(unused)]

mod board;
mod flowers;
mod genetics;
mod terminal;

use std::rc::Rc;

use board::Board;
use flowers::{
    acnh_flowers::{ACNHMum, ACNHRose},
    flower::{Flower, FlowerContext},
};
use genetics::{Gene, Genotype};
use getch_rs::Key;
use terminal::{AnsiColor, AnsiEffect, KeyStack, TextBlueprint, new_keystack, refresh_with};

fn main() {
    let mut board = Board::new(5, 5).expect("Can't have a board with a 0 width/length");
    let (key_stack, _guard) = new_keystack();

    'running: loop {
        let mut key_stack_mutex = key_stack.lock().unwrap();
        if key_stack_mutex.should_quit() {
            break 'running
        }
        let keys =  key_stack_mutex.poll_keys();
        if keys.len() == 0 {
            continue;
        }
        for key in keys {
            match key{
                Key::Char('w') => board.move_cursor((0, 1)), 
                Key::Char('a') => board.move_cursor((-1, 0)), 
                Key::Char('s') => board.move_cursor((0, -1)), 
                Key::Char('d') => board.move_cursor((1, 0)),
                _ => {} 
            }
        }
        board.show_board();
    }
}
