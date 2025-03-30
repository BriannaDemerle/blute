use std::{ops::Deref, rc::Rc};

use crate::{
    flowers::{
        acnh_flowers::{ACNHMum, ACNHRose},
        flower::{Flower, FlowerContext, FlowerData, FlowerType},
    },
    genetics::Genotype,
    terminal::{refresh_with, AnsiColor, TextBlueprint},
};

type AnyFlower = Rc<dyn Flower>;
type Grid<T> = Vec<Vec<T>>;

pub struct Board {
    board: Grid<Option<AnyFlower>>,
    flower_context: FlowerContext,
    board_size: (u8, u8),
    cursor_pos: (u8, u8),
}

impl Board {
    pub fn selected_blueprint() -> TextBlueprint {
        TextBlueprint::new().with_background_color(AnsiColor::from_code(236)).to_owned()
    }

    pub fn new(width: u8, height: u8) -> Option<Board> {
        if width == 0 || height == 0 {
            return None
        }

        let empty_vec: Vec<Option<AnyFlower>> = vec![None; width as usize];
        let flower_context = FlowerContext::new().expect("Could not create flower context.");

        Some(Board {
            board: vec![empty_vec; height as usize],
            flower_context,
            board_size: (width, height),
            cursor_pos: (0, 0)
        })
    }

    fn blank_flower() -> String {
        TextBlueprint::new()
            .with_text_color(AnsiColor::from_gray_value(12).expect("Couldn't get blank color"))
            .apply("-")
    }

    pub fn get_flower(&self, x: u8, y: u8) -> Option<Option<AnyFlower>> {
        Some(self.board.get(y as usize)?.get(x as usize)?.clone())
    }

    pub fn get_flower_mut(&mut self, x: u8, y: u8) -> Option<&mut Option<AnyFlower>> {
        Some(self.board.get_mut(y as usize)?.get_mut(x as usize)?)
    }

    pub fn set_flower(&mut self, x: u8, y: u8, new_flower: Option<AnyFlower>) {
        if let Some(f) = self.get_flower_mut(x, y) {
            *f = new_flower;
        }
    }

    pub fn flower_string(&self, f: Option<AnyFlower>, position: (u8, u8)) -> String {
        let unselected_flower_string = &f.map_or(Self::blank_flower(), |flower| {
            flower.to_string(&self.flower_context)
        });

        if self.cursor_pos == position {
            Self::selected_blueprint().apply(&unselected_flower_string)
        } else {
            unselected_flower_string.to_string()
        }
        
    }

    pub fn set_cursor(&mut self, new_pos: (u8, u8)) {
        self.cursor_pos = new_pos;
    }

    pub fn move_cursor(&mut self, d_pos: (i8, i8)) {
        let new_x = (self.cursor_pos.0 as i8 + d_pos.0).clamp(0, self.board_size.0 as i8 - 1) as u8;
        let new_y = (self.cursor_pos.1 as i8 + d_pos.1).clamp(0, self.board_size.1 as i8 - 1) as u8;
        self.cursor_pos = (new_x, new_y);
    }

    pub fn show_board(&self) {
        let coordinate_board = self.board
            .iter()
            .enumerate()
            .rev()
            .map(|(i, fs)| (i, fs.iter().enumerate()));

        let output = coordinate_board
            .map(|(y, fs)| {
                fs.map(|(x, f)| {
                    self.flower_string(f.clone(), (x as u8, y as u8))})
                        .collect::<Vec<String>>()
                        .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        refresh_with(&output);
    }
}
