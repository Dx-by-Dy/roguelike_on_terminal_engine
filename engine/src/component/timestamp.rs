use crate::component::Component;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub value: usize,
}

impl Timestamp {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn next(self) -> Self {
        Self {
            value: self.value + 1,
        }
    }
}
