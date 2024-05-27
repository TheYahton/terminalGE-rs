use std::{io::Write, time::Duration};

use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    terminal::{window_size, WindowSize},
    ExecutableCommand,
};

use crate::{
    drawing::{Color, Display},
    event,
};

pub struct Terminal {
    body: Vec<char>,
    stdout: std::io::Stdout,
    pub width: u16,
    pub height: u16,
    aspect: f64,
    pixel_aspect: f64,
    time_point: std::time::SystemTime,
}

impl Terminal {
    pub fn new() -> Self {
        let size: WindowSize = window_size().unwrap();
        let width: u16 = size.columns;
        let height: u16 = size.rows;
        let pixel_aspect = if (size.height != 0) & (size.width != 0) {
            size.height as f64 / size.width as f64
        } else {
            1.0
        };

        Terminal {
            body: vec![' '; (width * height) as usize],
            stdout: std::io::stdout(),
            width,
            height,
            aspect: width as f64 / height as f64,
            pixel_aspect,
            time_point: std::time::SystemTime::now(),
        }
    }

    pub fn show_cursor(&mut self) {
        let _ = self.stdout.execute(crossterm::cursor::Show);
    }

    pub fn hide_cursor(&mut self) {
        let _ = self.stdout.execute(crossterm::cursor::Hide);
    }

    pub fn fill(&mut self) {
        self.body.fill(' ');
    }

    pub fn print(&mut self) {
        self.cursor_move(0, 0);
        let _ = self
            .stdout
            .write_fmt(format_args!("{}", String::from_iter(self.body.clone())));
    }

    pub fn cursor_move(&mut self, x: u16, y: u16) {
        let _ = self.stdout.execute(crossterm::cursor::MoveTo(y, x));
    }

    pub fn raw_mode(&mut self) {
        if crossterm::terminal::is_raw_mode_enabled().unwrap() {
            let _ = crossterm::terminal::disable_raw_mode().unwrap();
        } else {
            let _ = crossterm::terminal::enable_raw_mode().unwrap();
        }
    }

    pub fn tick(&mut self) {
        self.time_point = std::time::SystemTime::now();
    }

    pub fn decay(&mut self, fps: f64) {
        let current_decay: Duration = self.get_tick();
        let time: Duration = Duration::from_secs_f64(1.0 / fps);
        std::thread::sleep(time - current_decay);
    }

    fn get_tick(&self) -> Duration {
        self.time_point.elapsed().unwrap()
    }

    pub fn print_fps(&mut self) {
        let fps: u64 = (1.0 / self.get_tick().as_secs_f64()) as u64;
        let text: String = fps.to_string() + " FPS";
        let text: &[u8] = text.as_bytes();
        let start_index: usize = ((self.height - 2) * self.width) as usize;
        for index in start_index..start_index + text.len() {
            self.body[index] = text[index - start_index] as char;
        }
    }

    pub fn exit(&mut self) {
        self.cursor_move(0, 0);
        self.raw_mode();
        self.show_cursor();
    }
}

#[cfg(feature = "common")]
impl Display for Terminal {
    fn plot(&mut self, x: i64, y: i64, _color: &Color) {
        let rationed_x = x as f64 * self.aspect * self.pixel_aspect;
        if rationed_x >= self.width as f64 || y >= self.height as i64 || x < 0 || y < 0 {
            return;
        }
        self.body[rationed_x as usize + y as usize * self.width as usize] = 'A';
    }
}

#[cfg(feature = "twice")]
impl Display for Terminal {
    fn plot(&mut self, x: i64, y: i64, _color: &Color) {
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
    }
}

impl From<crossterm::event::KeyCode> for event::KeyCode {
    fn from(value: crossterm::event::KeyCode) -> Self {
        match value {
            KeyCode::Up => event::KeyCode::Up,
            KeyCode::Down => event::KeyCode::Down,
            KeyCode::Left => event::KeyCode::Left,
            KeyCode::Right => event::KeyCode::Right,
            KeyCode::Char(chr) => event::KeyCode::Char(chr),
            _ => event::KeyCode::NotImplemented,
        }
    }
}

impl From<crossterm::event::KeyModifiers> for event::KeyModifiers {
    fn from(value: crossterm::event::KeyModifiers) -> Self {
        match value {
            KeyModifiers::SHIFT => event::KeyModifiers::SHIFT,
            KeyModifiers::CONTROL => event::KeyModifiers::CONTROL,
            _ => event::KeyModifiers::NONE,
        }
    }
}

impl From<crossterm::event::KeyEvent> for event::KeyEvent {
    fn from(value: crossterm::event::KeyEvent) -> Self {
        event::KeyEvent::new_with_modifiers(value.code.into(), value.modifiers.into())
    }
}

impl From<crossterm::event::Event> for event::Event {
    fn from(cevent: crossterm::event::Event) -> Self {
        let key: event::KeyEvent = match cevent {
            Event::Key(l) => event::KeyEvent::from(l),
            _ => event::KeyEvent::new(event::KeyCode::NotImplemented),
        };
        event::Event::Key(key)
    }
}

pub fn is_event_available() -> std::io::Result<bool> {
    crossterm::event::poll(std::time::Duration::from_secs(0))
}

pub fn read_event() -> Result<Event, std::io::Error> {
    crossterm::event::read()
}

pub fn get_event() -> Option<event::Event> {
    if is_event_available().unwrap() {
        Some(read_event().unwrap().into())
    } else {
        None
    }
}
