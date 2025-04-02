use std::io::{StdoutLock, Write};
use std::sync::MutexGuard;
use std::thread::JoinHandle;
use std::vec::IntoIter;

use getch_rs::Key;

use crate::flowers::flower::FlowerContext;
use crate::ui::shop::Shop;
use crate::ui::terminal::{self, KeyStack, KeyStackInner, new_keystack};
use crate::ui::{Grid, board::Board, display::Display, shop, to_grid};

pub struct Context {
    flower_context: FlowerContext,
    display: Display,
    board: Board,
    shop: Shop,

    keys: KeyStack,
    _guard: JoinHandle<()>,

    stdout_lock: StdoutLock<'static>,
}

impl Context {
    pub fn new() -> Context {
        let display = Display::new();
        let (keys, _guard) = new_keystack();
        let board = Board::new(display.board_size()).expect("Could not create board");
        let flower_context = FlowerContext::new().expect("Could not create flower context.");
        let shop = Shop::new(&flower_context);
        Context {
            flower_context,
            display,
            board,
            shop,
            keys,
            _guard,
            stdout_lock: std::io::stdout().lock(),
        }
    }

    pub fn to_string(&self) -> String {
        self.display.to_string()
    }

    pub fn refresh(&mut self) {
        self.display.refresh(&mut self.stdout_lock);
    }

    pub fn get_keystack(&self) -> MutexGuard<'_, KeyStackInner> {
        self.keys.lock().expect("Couldn't get keys.")
    }

    pub fn board(&mut self) -> &mut Board {
        &mut self.board
    }

    fn handle_keys(&mut self, keys: IntoIter<Key>) {
        for key in keys {
            match key {
                // board cursor movement
                Key::Char('w') => self.board.move_cursor((0, 1)),
                Key::Char('a') => self.board.move_cursor((-1, 0)),
                Key::Char('s') => self.board.move_cursor((0, -1)),
                Key::Char('d') => self.board.move_cursor((1, 0)),
                // shop cursor movement
                Key::Up => self.shop.move_cursor((0, 1)),
                Key::Left => self.shop.move_cursor((-1, 0)),
                Key::Down => self.shop.move_cursor((0, -1)),
                Key::Right => self.shop.move_cursor((1, 0)),
                // flowers
                Key::Char(' ') => self.board.set_flower_at_cursor(self.shop.selected_flower()),
                Key::Char('c') => self.board.set_flower_at_cursor(None),
                _ => {}
            }
        }
    }

    /// Game update. Returns `true` if the loop should quit.
    pub fn update(&mut self) -> bool {
        let mut key_stack_mutex = self.get_keystack();
        if key_stack_mutex.should_quit() {
            true
        } else {
            let keys = key_stack_mutex.poll_keys();
            if keys.len() != 0 {
                drop(key_stack_mutex);
                self.handle_keys(keys);
                self.display
                    .stamp(self.board.to_grid(&self.flower_context), (1, 1));
                self.display.stamp(self.shop.to_grid(&self.flower_context), (22, 3));
                self.refresh();
            }
            false
        }
    }
}
