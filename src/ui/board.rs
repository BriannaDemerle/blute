use std::{ops::Deref, rc::Rc};

use super::Grid;
use crate::context::Context;
use crate::{
    flowers::{
        acnh_flowers::{ACNHMum, ACNHRose},
        flower::{AnyFlower, Flower, FlowerContext, FlowerData, FlowerType},
    },
    genetics::Genotype,
    ui::terminal::{AnsiColor, TextBlueprint},
};

pub struct Board {
    board: Grid<Option<AnyFlower>>,
    board_size: (usize, usize),
    cursor_pos: (usize, usize),
}

impl Board {
    pub fn selected_blueprint() -> TextBlueprint {
        TextBlueprint::new()
            .with_background_color(AnsiColor::from_code(236))
            .to_owned()
    }

    pub fn new(size: (usize, usize)) -> Option<Board> {
        if size.0 == 0 || size.1 == 0 {
            return None;
        }

        let (width, height) = size;

        let empty_vec: Vec<Option<AnyFlower>> = vec![None; width as usize];

        Some(Board {
            board: vec![empty_vec; height as usize],
            board_size: size,
            cursor_pos: (0, 0),
        })
    }

    fn blank_flower() -> String {
        TextBlueprint::new()
            .with_text_color(AnsiColor::from_gray_value(12).expect("Couldn't get blank color"))
            .apply("-")
    }

    pub fn get_flower(&self, pos: (usize, usize)) -> Option<Option<AnyFlower>> {
        Some(self.board.get(pos.1)?.get(pos.0)?.clone())
    }

    pub fn get_flower_mut(&mut self, pos: (usize, usize)) -> Option<&mut Option<AnyFlower>> {
        Some(self.board.get_mut(pos.1)?.get_mut(pos.0)?)
    }

    pub fn set_flower(&mut self, pos: (usize, usize), new_flower: Option<AnyFlower>) {
        if let Some(f) = self.get_flower_mut(pos) {
            *f = new_flower;
        }
    }

    pub fn set_flower_at_cursor(&mut self, new_flower: Option<AnyFlower>) {
        self.set_flower(self.cursor_pos, new_flower);
    }

    pub fn flower_string(
        &self,
        f: Option<AnyFlower>,
        position: (usize, usize),
        flower_context: &FlowerContext,
    ) -> String {
        let unselected_flower_string = &f.map_or(Self::blank_flower(), |flower| {
            flower.to_string(flower_context)
        });

        if self.cursor_pos == position {
            Self::selected_blueprint().apply(&unselected_flower_string)
        } else {
            unselected_flower_string.to_string()
        }
    }

    pub fn set_cursor(&mut self, new_pos: (usize, usize)) {
        self.cursor_pos = new_pos;
    }

    pub fn move_cursor(&mut self, d_pos: (isize, isize)) {
        let new_x = (self.cursor_pos.0 as isize + d_pos.0).clamp(0, self.board_size.0 as isize - 1)
            as usize;
        let new_y = (self.cursor_pos.1 as isize + d_pos.1).clamp(0, self.board_size.1 as isize - 1)
            as usize;
        self.cursor_pos = (new_x, new_y);
    }

    pub fn to_grid(&self, flower_context: &FlowerContext) -> Grid<String> {
        self.board
            .iter()
            .enumerate()
            .map(|(y, fs)| {
                fs.iter()
                    .enumerate()
                    .map(|(x, f)| self.flower_string(f.clone(), (x, y), flower_context))
                    .zip([" ".to_string()].into_iter().cycle())
                    .flat_map(|(s1, s2)| [s2, s1].into_iter())
                    .collect::<Vec<String>>()
            })
            .rev()
            .collect()
    }
}
