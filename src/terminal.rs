mod render;

use std::time::Duration;

use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    terminal::{window_size, WindowSize},
    ExecutableCommand,
};

use crate::event;

pub struct Terminal {
    pub render: render::Render,
    stdout: std::io::Stdout,
    time_point: std::time::SystemTime,
    last_fps: [f64; 120],
    tick_counter: usize,
}

impl Terminal {
    pub fn new() -> Self {
        let size: WindowSize = window_size().unwrap();
        let width: u16 = size.columns;
        let height: u16 = size.rows;
        let (pixel_width, pixel_height) = if (size.height != 0) & (size.width != 0) {
            (size.height, size.width)
        } else {
            (1, 1)
        };

        Terminal {
            render: render::Render::new(width, height, pixel_width, pixel_height),
            stdout: std::io::stdout(),
            time_point: std::time::SystemTime::now(),
            last_fps: [60.0; 120],
            tick_counter: 0,
        }
    }

    pub fn show_cursor(&mut self) {
        let _ = self.stdout.execute(crossterm::cursor::Show);
    }

    pub fn hide_cursor(&mut self) {
        let _ = self.stdout.execute(crossterm::cursor::Hide);
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
        self.last_fps[self.tick_counter % self.last_fps.len()] =
            1.0 / self.get_tick().as_secs_f64();
        self.tick_counter += 1;
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
        let fps: u64 = (self.last_fps.iter().sum::<f64>() / self.last_fps.len() as f64) as u64;
        let text: String = fps.to_string() + " FPS";
        let text: &[u8] = text.as_bytes();
        self.render.put(text);
    }

    pub fn exit(&mut self) {
        self.cursor_move(0, 0);
        self.raw_mode();
        self.show_cursor();
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
