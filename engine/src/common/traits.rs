use crate::common::positions::SurfacePosition;

pub trait Entity: Definable {
    fn get_pos(&self) -> SurfacePosition;
}

pub type Consequence = Box<dyn Event>;
pub type Consequences = Vec<Consequence>;

pub trait Event {
    fn get_timestamp(&self) -> usize;
    fn consequences(&mut self) -> Consequences;
    fn reverse(&mut self);
}

pub trait Definable: Sized {
    fn get_id(&self) -> usize;
}
