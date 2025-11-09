#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub x: u16,
    pub y: u16,
}

impl Size {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
