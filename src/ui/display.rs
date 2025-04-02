use std::io::StdoutLock;
use std::io::Write;

use crate::ui::{Grid, to_grid};

pub struct Display {
    display_size: (usize, usize),
    display: Grid<String>,
    board_size: (usize, usize),
}

impl Display {
    pub fn new() -> Display {
        let display_string: String = Self::grid_template()
            .chars()
            .take_while(|&c| c != '#')
            .collect();
        let display_size = (
            display_string
                .chars()
                .position(|c| c == '\n')
                .expect("Could not find a new line in display template...")
                + 1,
            display_string.matches("\n").count(),
        );
        let board_length = display_string
            .char_indices()
            .filter(|c| c.1 == '+')
            .nth(1)
            .expect("Could not find a second + in display template...")
            .0
            / 2
            - 1;
        let display = to_grid(display_string, display_size.0);

        Display {
            display_size,
            display,
            board_size: (board_length, board_length),
        }
    }

    pub fn stamp_row(&mut self, row: &Vec<String>, start: (usize, usize)) {
        self.display
            .get_mut(start.1)
            .expect(format!("Could not get mutable reference to the display at row: {}", start.1).as_str())
            .iter_mut()
            .skip(start.0)
            .zip(row.iter())
            .for_each(|(old, new)| {
                *old = new.to_string();
            });
    }

    pub fn stamp(&mut self, grid: Grid<String>, top_left: (usize, usize)) {
        grid.iter().enumerate().for_each(|(index, strings)| {
            self.stamp_row(strings, (top_left.0, top_left.1 + index));
        });
    }

    pub fn reset_display(&mut self) {
        self.display = to_grid(Self::grid_template(), self.display_size.0);
    }

    fn grid_template() -> String {
        include_str!("grid_template.txt").to_string()
    }

    pub fn board_size(&self) -> (usize, usize) {
        self.board_size
    }

    pub fn to_string(&self) -> String {
        self.display
            .iter()
            .map(|strings| {
                strings
                    .iter()
                    .map(|cs| cs.chars())
                    .flatten()
                    .collect::<String>()
            })
            .collect()
    }

    pub fn refresh(&mut self, lock: &mut StdoutLock<'static>) {
        // absolute buffonery: for each line, clear it and print on that line using ansi trickery :3
        for (i, s) in self.display.iter().enumerate() {
            let text: String = s.iter().flat_map(|cs| cs.chars()).collect();
            writeln!(lock, "\x1B[{}H{}", i + 1, text);
        }
    }
}
