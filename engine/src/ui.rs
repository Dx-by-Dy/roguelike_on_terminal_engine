pub mod draw_unit;
pub mod drawable_entity;

pub(crate) mod user_event_gateway;

pub mod configs;
mod surface;
mod terminal;

use crate::{
    common::{positions::SurfacePosition, user_event::UserEvent},
    ui::{
        configs::{MountingPoint, UIConfig},
        draw_unit::DrawUnit,
        terminal::Terminal,
        user_event_gateway::UserEventGateway,
    },
};

pub struct UI {
    terminal: Terminal,
    user_event_gateway: UserEventGateway,
}

impl UI {
    pub fn new(config: UIConfig) -> Self {
        Self {
            terminal: Terminal::new(config.terminal_config),
            user_event_gateway: UserEventGateway::default(),
        }
    }

    pub fn get_user_event(&mut self) -> Option<UserEvent> {
        self.user_event_gateway.next()
    }

    pub fn update_surface(&mut self, pos: SurfacePosition, surface_id: usize, unit: DrawUnit) {
        self.terminal.update_surface(pos, surface_id, unit);
    }

    pub fn degrade_surface(&mut self, pos: SurfacePosition, surface_id: usize) -> Option<DrawUnit> {
        self.terminal.degrade_surface(pos, surface_id)
    }

    pub fn change_mounting_point(&mut self, mounting_point: MountingPoint, surface_id: usize) {
        self.terminal
            .change_mounting_point(mounting_point, surface_id);
    }

    pub fn redraw(&mut self) {
        self.terminal.redraw();
    }
}
