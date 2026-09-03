use std::fmt;

/// Represents a single character cell in the TUI grid
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub attrs: Attributes,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Attributes {
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
    pub reverse: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Color;

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cell({:?})", self.ch)
    }
}

impl Cell {
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            fg: Color,
            bg: Color,
            attrs: Attributes::default(),
        }
    }
}
