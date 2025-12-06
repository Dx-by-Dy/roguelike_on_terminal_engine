use crate::{Gtr, component::Component, game::Game};

#[derive(Component, Debug, Clone, Copy)]
pub struct Transformation {
    pub gtr: fn(&mut Game),
}

impl Transformation {
    pub fn new(gtr: Gtr) -> Self {
        Self { gtr }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TransformationInit {
    pub gtr: fn(&mut Game),
    pub init_gtr: bool,
    pub set_in_datamaster: bool,
    pub timestamp: Option<usize>,
}

inventory::collect!(TransformationInit);
