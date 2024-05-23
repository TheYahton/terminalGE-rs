use crate::{
    event::{self, Event, KeyCode, KeyEvent},
    vec::Vec2,
};

pub struct Player {
    pub position: Vec2,
    speed: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vec2::zero(),
            speed: 1.0,
        }
    }

    pub fn movement(&mut self, event: &Option<Event>) {
        let key_event: &KeyEvent;
        if let Some(event) = event {
            match event {
                event::Event::Key(k) => key_event = k,
            }
        } else {
            return;
        }
        let mut delta: Vec2 = Vec2::zero();
        let key_code = &key_event.code;
        match key_code {
            KeyCode::Up => delta.y -= 1.0,
            KeyCode::Down => delta.y += 1.0,
            KeyCode::Left => delta.x -= 1.0,
            KeyCode::Right => delta.x += 1.0,
            _ => (),
        }

        self.position += delta * self.speed;
    }
}
