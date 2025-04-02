use getch_rs::{Getch, Key, disable_echo_input};
use std::io::StdoutLock;
use std::thread;
use std::vec::IntoIter;
use std::{
    io::{Error, Write},
    str::FromStr,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

/// The colors that you can use for the TextBlueprint
#[derive(Debug, Clone, Copy)]
pub struct AnsiColor {
    code: u8,
}

impl AnsiColor {
    pub fn from_code(code: u8) -> Self {
        Self { code }
    }

    pub fn from_rgb_cube(r: u8, g: u8, b: u8) -> Option<Self> {
        if r < 6 && g < 6 && b < 6 {
            Some(Self {
                code: 16 + (r * 36 + g * 6 + b),
            })
        } else {
            None
        }
    }

    pub fn from_gray_value(value: u8) -> Option<Self> {
        if value < 24 {
            Some(Self { code: 232 + value })
        } else {
            None
        }
    }

    fn get_code(&self) -> u8 {
        self.code
    }
}

/// The effects that you can use for the TextBlueprint
#[derive(Debug, Clone, Copy)]
pub enum AnsiEffect {
    Bold = 1,
    Italics = 3,
    Underline = 4,
    Strikethrough = 9,
    DoubleUnderline = 21,
}

/// Blueprint to color and fancify your text!
#[derive(Debug, Clone)]
pub struct TextBlueprint {
    text_color: Option<AnsiColor>,
    background_color: Option<AnsiColor>,
    effects: Vec<AnsiEffect>,
}

impl TextBlueprint {
    pub fn new() -> Self {
        Self {
            text_color: None,
            background_color: None,
            effects: vec![],
        }
    }

    pub fn with_text_color(&mut self, color: AnsiColor) -> &mut Self {
        self.text_color = Some(color);
        self
    }

    pub fn with_background_color(&mut self, color: AnsiColor) -> &mut Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_effect(&mut self, effect: AnsiEffect) -> &mut Self {
        self.effects.push(effect);
        self
    }

    pub fn with_effects(&mut self, effects: &Vec<AnsiEffect>) -> &mut Self {
        self.effects.extend(effects);
        self
    }

    pub fn reset() -> String {
        String::from_str("\x1B[0m").unwrap()
    }

    pub fn get_ansi(&self) -> String {
        let mut ansi = String::from_str("\x1B[").unwrap();

        if let Some(color) = self.text_color {
            ansi.push_str(format!("38;5;{};", color.code).as_str());
        }
        if let Some(color) = self.background_color {
            ansi.push_str(format!("48;5;{};", color.code).as_str());
        }
        for code in self.effects.iter() {
            ansi.push_str(format!("{};", *code as u8).as_str());
        }

        if ansi.ends_with(';') {
            ansi.pop();
        }
        ansi.push_str("m");
        ansi
    }

    pub fn apply(&self, text: &str) -> String {
        let mut colored_string = self.get_ansi();
        colored_string.push_str(text);
        colored_string.push_str(Self::reset().as_str());
        colored_string
    }
}

/// The inside of the KeyStack
#[derive(Debug, Clone)]
pub struct KeyStackInner {
    keys: Vec<Key>,
    quit: bool,
}

/// The thread-safe version of KeyStackInner
pub type KeyStack = Arc<Mutex<KeyStackInner>>;

impl KeyStackInner {
    fn new() -> KeyStackInner {
        Self {
            keys: vec![],
            quit: false,
        }
    }

    /// Returns an iterator over the keys pressed since the last poll and clears the poll
    pub fn poll_keys(&mut self) -> IntoIter<Key> {
        let it: IntoIter<Key> = self.keys.clone().into_iter();
        self.keys.clear();
        it
    }

    /// `true` if there was an Escape Key input received, `false` otherwise
    pub fn should_quit(&self) -> bool {
        self.quit
    }
}

/// Generates a KeyStack with its polling thread. Used for getting inputs without them showing up on the terminal
pub fn new_keystack() -> (KeyStack, JoinHandle<()>) {
    // create a thread-safe keystack that can be accessed by the main thread and the new thread
    let mutex = Arc::new(Mutex::new(KeyStackInner::new()));
    let t_mutex = Arc::clone(&mutex);

    // create a thread so we don't block the main thread while waiting for keyboard presses
    let guard = std::thread::spawn(move || {
        // make the get character thing
        let g = Getch::new();
        // keep waiting for a character input
        loop {
            let c = g.getch();
            match c {
                // if the character couldn't be received explode the computer
                Err(e) => {
                    println!("An error occured while getting input: {:?}", e);
                    t_mutex.lock().unwrap().quit = true;
                }
                // if the character is escape key quit the loop
                Ok(Key::Esc) => {
                    t_mutex.lock().unwrap().quit = true;
                }
                // otherwise return the character inputted
                Ok(k) => {
                    t_mutex.lock().unwrap().keys.push(k);
                }
            }
        }
    });

    // return the things the program needs
    (mutex, guard)
}
