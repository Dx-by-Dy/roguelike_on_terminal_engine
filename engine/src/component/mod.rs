use engine_macros::Component;

pub mod point;
pub mod pointer;
pub mod positions;
pub mod ref_to_surface;
pub mod size;
pub mod timestamp;
pub mod transformation;
pub mod user_event;
pub mod vec_of_pointers;

pub trait Component: std::any::Any {}
