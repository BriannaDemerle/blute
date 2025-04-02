use crate::flowers::flower::{AnyFlower, FlowerContext};

use super::{
    Grid, empty_string_grid,
    terminal::{AnsiColor, TextBlueprint},
};

pub struct Shop {
    shop_grid: Grid<AnyFlower>,
    shop_size: (usize, usize),
    cursor_pos: (usize, usize),
}

impl Shop {
    pub fn selected_blueprint() -> TextBlueprint {
        TextBlueprint::new()
            .with_background_color(AnsiColor::from_code(236))
            .to_owned()
    }

    pub fn new(flower_context: &FlowerContext) -> Shop {
        let shop_grid: Grid<AnyFlower> = flower_context.get_seeds()
            .iter()
            .rev()
            .cloned()
            .collect();

        let shop_size = (shop_grid[0].len(), shop_grid.len());

        Shop {
            shop_grid,
            shop_size,
            cursor_pos: (0, 0),
        }
    }

    pub fn set_cursor(&mut self, new_pos: (usize, usize)) {
        self.cursor_pos = new_pos;
    }

    pub fn move_cursor(&mut self, d_pos: (isize, isize)) {
        let new_x =
            (self.cursor_pos.0 as isize + d_pos.0).clamp(0, self.shop_size.0 as isize - 1) as usize;
        let new_y =
            (self.cursor_pos.1 as isize - d_pos.1).clamp(0, self.shop_size.1 as isize - 1) as usize;
        self.cursor_pos = (new_x, new_y);
    }

    pub fn to_grid(&self, flower_context: &FlowerContext) -> Grid<String> {
        let first_row_index = self.cursor_pos.1.clamp(0, self.shop_grid.len().checked_sub(2).unwrap_or_else(|| 0));
        let mut displayed_grid: Grid<String> = self
            .shop_grid
            .iter()
            .enumerate()
            .map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .map(|(x, f)| {
                        if (x, y) == self.cursor_pos {
                            Self::selected_blueprint().apply(f.to_string(flower_context).as_str())
                        } else {
                            f.to_string(flower_context)
                        }
                    })
                    .collect()
            })
            .skip(first_row_index)
            .take(2)
            .collect();

        displayed_grid.insert(1, vec![" ".to_string(); 3]);
        if displayed_grid[0].len() < 4 {
            displayed_grid
                .iter_mut()
                .for_each(|v| v.push(" ".to_string()));
        }
        displayed_grid
            .iter()
            .map(|v| {
                v.iter()
                    .zip(vec![" ".to_string()].iter().cycle())
                    .flat_map(|(s1, s2)| [s1, s2].into_iter().cloned())
                    .collect()
            })
            .collect()
    }

    pub fn selected_flower(&self) -> Option<AnyFlower> {
        self.shop_grid.get(self.cursor_pos.1)?
            .get(self.cursor_pos.0)
            .cloned()
    }
}
