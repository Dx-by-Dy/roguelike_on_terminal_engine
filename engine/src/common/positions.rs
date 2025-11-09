#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfacePosition {
    pub x: u16,
    pub y: u16,
}

impl SurfacePosition {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalPosition {
    pub x: u16,
    pub y: u16,
}

impl TerminalPosition {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
