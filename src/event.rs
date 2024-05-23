#[derive(PartialEq)]
pub enum KeyCode {
    Left,
    Right,
    Up,
    Down,
    Char(char),
    NotImplemented,
}

#[derive(PartialEq)]
pub enum KeyModifiers {
    CONTROL,
    SHIFT,
    NONE,
}

#[derive(PartialEq)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyEvent {
    pub const fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub const fn new_with_modifiers(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

#[derive(PartialEq)]
pub enum Event {
    Key(KeyEvent),
}

pub trait EngineKeyCode {
    fn to_key(&self) -> KeyCode;
}
