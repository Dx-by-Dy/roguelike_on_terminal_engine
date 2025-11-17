use crate::component::Component;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pointer {
    pub id: usize,
}

impl Pointer {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn next(self) -> Self {
        Self { id: self.id + 1 }
    }
}
