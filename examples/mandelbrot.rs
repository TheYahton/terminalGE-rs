use terminalge_rs::{
    drawing::{self, Color},
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{get_event, Terminal},
};

#[derive(Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn squared(&self) -> Self {
        Self {
            real: self.real * self.real - self.imag * self.imag,
            imag: 2.0 * self.real * self.imag,
        }
    }

    pub fn abs(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Complex) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

fn mandelbrot(x: f64, y: f64) -> f64 {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(x, y);
    for i in 1..1000 {
        z = z.squared() + c;
        if z.abs() >= 2.0 {
            return i as f64 / 1000.0;
        }
    }
    return 1.0;
}

pub struct App {
    pub screen: Terminal,
    running: bool,
    current_event: Option<Event>,
}

impl App {
    pub fn new() -> Self {
        App {
            screen: Terminal::new(),
            running: true,
            current_event: None,
        }
    }

    pub fn check_exit(&mut self) {
        if let Some(event) = &self.current_event {
            match event {
                event::Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => self.running = false,
                event::Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => self.running = false,
                _ => (),
            }
        }
    }

    pub fn events(&mut self) {
        self.current_event = get_event();
    }

    fn update(&mut self) {
        self.screen.tick();
        self.check_exit();
        self.screen.cursor_move(0, 0);
        self.screen.render.draw();
    }

    fn draw(&mut self) {
        self.screen.render.clear();
        for i in 0..self.screen.width {
            for j in 0..self.screen.height {
                let (mut x, y) = (
                    i as f64 / self.screen.width as f64 * 2.0 - 1.0,
                    j as f64 / self.screen.height as f64 * 2.0 - 1.0,
                );
                x *= self.screen.width as f64 / self.screen.height as f64;
                let bright = (mandelbrot(x, y) * 255.0) as u8;
                let color = &Color(bright, bright, bright);
                drawing::pixel(&mut self.screen.render, i.into(), j.into(), color);
            }
        }

        self.screen.print_fps();
    }

    pub fn run(&mut self) {
        self.screen.hide_cursor();
        self.screen.raw_mode();

        // Main loop
        while self.running {
            self.events();
            self.update();
            self.draw();
        }
        self.screen.exit();
    }
}

fn main() {
    let mut app: App = App::new();
    app.run();
}
