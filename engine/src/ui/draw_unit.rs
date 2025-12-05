use crate::component::Component;
use crossterm::style::{StyledContent, Stylize};
use engine_macros::Component;

#[derive(Component, Debug, Clone, Copy)]
pub struct DrawUnit {
    ch: char,
    modificator: UnitModificator,
}

impl DrawUnit {
    pub const fn new(ch: char, modificator: UnitModificator) -> Self {
        Self { ch, modificator }
    }

    pub const fn symbol(&self) -> char {
        self.ch
    }
}

impl Default for DrawUnit {
    fn default() -> Self {
        Self {
            ch: ' ',
            modificator: UnitModificator::DFLT_UNIT_MODIFICATOR,
        }
    }
}

impl Into<StyledContent<char>> for DrawUnit {
    fn into(self) -> StyledContent<char> {
        self.modificator.apply(self.ch)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitModificator {
    #[allow(dead_code)]
    map: u64,
}

impl UnitModificator {
    pub const MODIFICATORS: [UnitModificator; 1] = [UnitModificator { map: 0 }];
    pub const DFLT_UNIT_MODIFICATOR: UnitModificator = Self::MODIFICATORS[0];

    pub const fn new(map: u64) -> Self {
        Self { map }
    }

    pub fn apply(&self, ch: char) -> StyledContent<char> {
        let content = ch.stylize();
        for (kind, _modificator) in Self::MODIFICATORS.into_iter().enumerate() {
            match kind {
                0 => {}
                _ => unreachable!(),
            }
        }
        content
    }
}
