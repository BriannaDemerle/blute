pub mod board;
pub mod display;
pub mod shop;
pub mod terminal;

pub type Grid<T> = Vec<Vec<T>>;

pub fn to_grid(s: String, row_length: usize) -> Grid<String> {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(row_length)
        .map(|cs| cs.chunks(1).map(|c| c.iter().collect::<String>()).collect())
        .collect()
}

pub fn empty_string_grid(size: (usize, usize)) -> Grid<String> {
    vec![vec![" ".to_string(); size.0]; size.1]
}
