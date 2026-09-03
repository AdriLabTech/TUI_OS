use crate::buffer::Buffer;

pub struct Surface {
    buffer: Buffer,
    offset_x: usize,
    offset_y: usize,
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: Buffer::new(width, height),
            offset_x: 0,
            offset_y: 0,
        }
    }
}
