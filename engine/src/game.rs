use crate::{
    common::{positions::SurfacePosition, user_event::UserEvent},
    ui::{UI, configs::UIConfig, draw_unit::DrawUnit},
};

pub struct Game {
    ui: UI,
}

impl Game {
    pub fn new(ui_config: UIConfig) -> Self {
        Self {
            ui: UI::new(ui_config),
        }
    }

    pub fn get_user_event(&mut self) -> Option<UserEvent> {
        self.ui.get_user_event()
    }

    pub fn update_surface(&mut self, pos: SurfacePosition, surface_id: usize, unit: DrawUnit) {
        self.ui.update_surface(pos, surface_id, unit);
    }

    pub fn degrade_surface(&mut self, pos: SurfacePosition, surface_id: usize) -> Option<DrawUnit> {
        self.ui.degrade_surface(pos, surface_id)
    }

    pub fn redraw(&mut self) {
        self.ui.redraw();
    }
}

// impl Drop for Game {
//     fn drop(&mut self) {
//         self.stdout
//             .execute(crossterm::terminal::LeaveAlternateScreen)
//             .unwrap();
//     }
// }
