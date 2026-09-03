use crate::cell::Cell;

pub struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::new(' '); width * height];
        Self { width, height, cells }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
}
