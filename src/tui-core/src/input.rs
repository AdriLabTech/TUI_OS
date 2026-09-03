#[derive(Debug, Clone)]
pub enum InputEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug, Clone)]
pub struct KeyEvent;

#[derive(Debug, Clone)]
pub struct MouseEvent;

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }
}
