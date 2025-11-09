use crate::{common::traits::Entity, ui::draw_unit::DrawUnit};

pub trait DrawableEntity: Entity {
    fn get_unit(&self) -> DrawUnit;
    fn get_surface_id(&self) -> usize;
}
