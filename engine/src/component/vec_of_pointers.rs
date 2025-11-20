use crate::component::{Component, pointer::Pointer};

#[derive(Component, Clone)]
pub struct VecOfPointers {
    pub pointers: Vec<Pointer>,
}

impl VecOfPointers {
    pub fn new(pointers: Vec<Pointer>) -> Self {
        Self { pointers }
    }
}
