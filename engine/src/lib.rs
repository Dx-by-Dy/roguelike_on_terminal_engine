#[macro_use]
mod delegate_macroses;

pub mod component;
pub mod data_master;
pub mod game;
pub mod transformation_master;
pub mod ui;

pub type Gtr = fn(&mut crate::game::Game);

#[macro_export]
macro_rules! addr {
    ($func:ident) => {
        $func as usize
    };
}
