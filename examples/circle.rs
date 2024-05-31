use terminalge_rs::{
    drawing::{self, Color},
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{get_event, Terminal},
};

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
        drawing::circle(&mut self.screen.render, 15, 15, 15, &Color(255, 255, 0));
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
