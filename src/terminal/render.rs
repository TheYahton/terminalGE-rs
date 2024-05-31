use crate::drawing::{Color, Display};
use std::io::Write;

pub struct Render {
    body: Vec<char>,
    colors: Vec<Color>,
    stdout: std::io::Stdout,
    width: u16,
    height: u16,
    aspect: f64,
    pixel_aspect: f64,
}

impl Render {
    pub fn new(width: u16, height: u16, pixel_width: u16, pixel_height: u16) -> Self {
        Self {
            body: vec![' '; (width * height) as usize],
            colors: vec![Color(0, 0, 0); (width * height) as usize],
            stdout: std::io::stdout(),
            width,
            height,
            aspect: width as f64 / height as f64,
            pixel_aspect: pixel_width as f64 / pixel_height as f64,
        }
    }

    pub fn clear(&mut self) {
        self.body.fill(' ');
        self.colors.fill(Color(0, 0, 0));
    }

    #[cfg(feature = "ascii")]
    pub fn draw(&mut self) {
        let _ = self
            .stdout
            .write_fmt(format_args!("{}", String::from_iter(self.body.clone())));
    }

    #[cfg(feature = "blocks")]
    pub fn draw(&mut self) {
        let mut prev_color: Color = Color(0, 0, 0);

        for i in 0..self.body.len() {
            if prev_color != self.colors[i] {
                prev_color = self.colors[i].clone();
                let Color(r, g, b) = self.colors[i];
                let _ = self
                    .stdout
                    .write_fmt(format_args!("\x1b[38;2;{};{};{}m{}", r, g, b, self.body[i]));
            } else {
                let _ = self.stdout.write_fmt(format_args!("{}", self.body[i]));
            }
        }
    }

    pub fn put(&mut self, text: &[u8]) {
        let start_index: usize = ((self.height - 2) * self.width) as usize;
        for index in start_index..start_index + text.len() {
            self.body[index] = text[index - start_index] as char;
            self.colors[index] = Color(255, 255, 255);
        }
    }
}

const GRADIENT: [char; 70] = [
    '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p', 'q', 'w',
    'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u', 'n', 'x', 'r', 'j',
    'f', 't', '/', '\\', '|', '(', ')', '1', '{', '}', '[', ']', '?', '-', '_', '+', '~', '<', '>',
    'i', '!', 'l', 'I', ';', ':', ',', '\"', '^', '`', '\'', '.', ' ',
];
const LEN: usize = GRADIENT.len() - 1;
#[cfg(feature = "ascii")]
impl Display for Render {
    fn plot(&mut self, x: i64, y: i64, color: &Color) {
        let rationed_y = y as f64 / (self.aspect * self.pixel_aspect);
        if x >= self.width as i64 || rationed_y >= self.height as f64 || x < 0 || rationed_y <= 0.0
        {
            return;
        }
        let bright = ((color.0 as u64 + color.1 as u64 + color.2 as u64) as f64 / 765.0
            * LEN as f64) as usize;

        self.body[x as usize + rationed_y as usize * self.width as usize] = GRADIENT[LEN - bright];
    }
}

#[cfg(feature = "blocks")]
impl Display for Render {
    fn plot(&mut self, x: i64, y: i64, color: &Color) {
        if x >= self.width as i64 || y / 2 >= self.height as i64 || x < 0 || y < 0 {
            return;
        }

        let index: usize = (x + (y / 2) * self.width as i64) as usize;
        let current_symbol: char = self.body[index];
        let next_symbol: char;

        next_symbol = if current_symbol == ' ' {
            if y % 2 == 0 {
                '▀'
            } else {
                '▄'
            }
        } else if current_symbol == '▀' {
            if y % 2 == 1 {
                '█'
            } else {
                current_symbol
            }
        } else if current_symbol == '▄' {
            if y % 2 == 0 {
                '█'
            } else {
                current_symbol
            }
        } else {
            current_symbol
        };

        self.body[index] = next_symbol;
        self.colors[index] = color.clone();
    }
}
