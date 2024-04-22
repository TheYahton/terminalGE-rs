use crate::vec::Vec2;

pub struct Player {
    pub position: Vec2,
    speed: f64,
    pub keys: [bool; 4],
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vec2::zero(),
            speed: 1.0,
            keys: [false; 4],
        }
    }

    pub fn movement(&mut self) {
        let mut delta: Vec2 = Vec2::zero();

        if self.keys[0] {
            delta.y += -1.0;
        }
        if self.keys[1] {
            delta.y += 1.0;
        }
        if self.keys[2] {
            delta.x += -1.0;
        }
        if self.keys[3] {
            delta.x += 1.0;
        }

        self.position += delta * self.speed;
        self.keys = [false; 4];
    }
}
