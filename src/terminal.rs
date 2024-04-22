use std::{io::Write, time::Duration};

use crossterm::{
    event::Event,
    terminal::{window_size, WindowSize},
    ExecutableCommand,
};

use crate::display::Display;

pub struct Terminal {
    body: Vec<char>,
    stdout: std::io::Stdout,
    width: u16,
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

        Terminal {
            body: vec![' '; (width * height) as usize],
            stdout: std::io::stdout(),
            width: width,
            height: height,
            aspect: width as f64 / height as f64,
            pixel_aspect: size.height as f64 / size.width as f64,
            time_point: std::time::SystemTime::now(),
        }
    }

    pub fn is_event_available() -> std::io::Result<bool> {
        crossterm::event::poll(std::time::Duration::from_secs(0))
    }

    pub fn read_event() -> Result<Event, std::io::Error> {
        crossterm::event::read()
    }

    pub fn show_cursor(&mut self) {
        let _ = self.stdout.execute(crossterm::cursor::Show);
    }

    pub fn hide_cursor(&mut self) {
        let _ = self.stdout.execute(crossterm::cursor::Hide);
    }

    pub fn fill(&mut self) {
        // self.body = vec![' '; self.width as usize * self.height as usize]; // works better when opt-level 1
        self.body.fill(' '); // works better when opt-level 2+
                             // self.body.clear(); // doesn't work ;(
    }

    pub fn draw_line(&mut self, x0: i64, x1: i64, y0: i64, y1: i64) {
        let deltax: i64 = (x1 - x0).abs();
        let deltay: i64 = (y1 - y0).abs();
        let mut error: i64 = 0;
        let deltaerr: i64 = deltay + 1;
        let mut y: i64 = y0;
        let mut diry: i64 = y1 - y0;
        if diry > 0 {
            diry = 1;
        }
        if diry < 0 {
            diry = -1;
        }
        for x in x0..x1 {
            self.plot(x, y, '@');
            error = error + deltaerr;
            if error >= deltax + 1 {
                y = y + diry;
                error = error - (deltax + 1);
            }
        }
    }

    pub fn circle(&mut self, x1: i64, y1: i64, radius: i64) {
        let mut x: i64 = 0;
        let mut y: i64 = radius;
        let mut delta = 1 - 2 * y;
        let mut _error = 0;
        while y >= x {
            self.plot(x1 + x, y1 + y, '@');
            self.plot(x1 + x, y1 - y, '@');
            self.plot(x1 - x, y1 + y, '@');
            self.plot(x1 - x, y1 - y, '@');
            self.plot(x1 + y, y1 + x, '@');
            self.plot(x1 + y, y1 - x, '@');
            self.plot(x1 - y, y1 + x, '@');
            self.plot(x1 - y, y1 - x, '@');
            _error = 2 * (delta + y) - 1;
            if (delta < 0) && (_error <= 0) {
                x += 1;
                delta += 2 * x + 1;
                continue;
            }
            if (delta > 0) && (_error > 0) {
                y -= 1;
                delta -= 2 * y + 1;
                continue;
            }
            x += 1;
            y -= 1;
            delta += 2 * (x - y);
        }
    }

    pub fn full_circle(&mut self) {
        for _x in 0..self.width {
            for _y in 0..self.height {
                let x: f64 = ((_x as f64) / (self.width as f64) * 2.0 - 1.0)
                    * self.aspect
                    * self.pixel_aspect;
                let y: f64 = (_y as f64) / (self.height as f64) * 2.0 - 1.0;
                if x * x + y * y < 1.0 {
                    self.body[_x as usize + _y as usize * self.width as usize] = '@';
                }
            }
        }
    }

    pub fn print(&mut self) {
        self.cursor_move(0, 0);
        let _ = self
            .stdout
            .write_fmt(format_args!("{}", String::from_iter(self.body.clone())));
        // let s = self.body.clone().into_iter().collect::<String>();
        // print!("{}", s);
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

impl Display for Terminal {
    fn plot(&mut self, x: i64, y: i64, symbol: char) {
        let rationed_x = x as f64 * self.aspect * self.pixel_aspect;
        if rationed_x >= self.width as f64 || y >= self.height as i64 || x < 0 || y < 0 {
            return;
        }
        self.body[rationed_x as usize + y as usize * self.width as usize] = symbol
    }
}
