use crate::{component::Component, game::Game};

#[derive(Component, Debug, Clone, Copy)]
pub struct Transformation {
    pub tr: fn(&mut Game),
}

impl Transformation {
    pub fn new(tr: fn(&mut Game)) -> Self {
        Self { tr }
    }
}
