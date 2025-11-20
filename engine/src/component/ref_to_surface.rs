use crate::component::Component;

#[derive(Debug, Component, Clone, Copy)]
pub struct RefToSurface {
    pub value: usize,
}

impl RefToSurface {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}
