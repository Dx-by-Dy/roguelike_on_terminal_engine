use std::io::{Stdout, Write, stdout};

use crossterm::{ExecutableCommand, QueueableCommand, cursor, style::PrintStyledContent, terminal};

use crate::{
    component::{
        positions::{SurfacePosition, TerminalPosition},
        size::Size,
    },
    ui::{
        configs::{MountingPoint, TerminalConfig},
        draw_unit::DrawUnit,
        surface::Surface,
    },
};

pub struct Terminal {
    out: Stdout,
    size: Size,
    surfaces: Vec<Surface>,
}

impl Terminal {
    pub fn new(config: TerminalConfig) -> Self {
        // let mut out = stdout();

        // crossterm::terminal::enable_raw_mode().unwrap();

        // out.execute(crossterm::terminal::EnterAlternateScreen)
        //     .unwrap();
        // out.execute(crossterm::cursor::Hide).unwrap();
        // out.execute(crossterm::terminal::SetSize(
        //     config.terminal_size.x,
        //     config.terminal_size.y,
        // ))
        // .unwrap();
        // out.execute(terminal::Clear(terminal::ClearType::All))
        //     .unwrap();

        Self {
            out: stdout(),
            size: config.terminal_size,
            surfaces: config
                .surface_confs
                .into_iter()
                .map(|s_conf| Surface::new(s_conf))
                .collect(),
        }
    }

    pub fn redraw(&mut self) {
        for surface in &mut self.surfaces {
            while let Some((surf_pos, unit)) = surface.next() {
                if let Some(term_pos) = Self::transform_position(surface, surf_pos, self.size) {
                    self.out
                        .queue(cursor::MoveTo(term_pos.x, term_pos.y))
                        .unwrap()
                        .queue(PrintStyledContent(unit.into()))
                        .unwrap();
                }
            }
        }
        self.out.flush().unwrap();
    }

    pub fn update_surface(&mut self, pos: SurfacePosition, surface_id: usize, unit: DrawUnit) {
        self.surfaces[surface_id].forward(pos, unit);
    }

    pub fn degrade_surface(&mut self, pos: SurfacePosition, surface_id: usize) -> Option<DrawUnit> {
        self.surfaces[surface_id].backward(pos)
    }

    pub fn change_mounting_point(&mut self, mounting_point: MountingPoint, surface_id: usize) {
        self.surfaces[surface_id].change_mounting_point(mounting_point);
    }

    fn transform_position(
        surface: &Surface,
        pos: SurfacePosition,
        term_size: Size,
    ) -> Option<TerminalPosition> {
        surface.transform_position(pos, term_size)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().unwrap();
        self.out
            .execute(crossterm::terminal::LeaveAlternateScreen)
            .unwrap();
        self.out.execute(crossterm::cursor::EnableBlinking).unwrap();
    }
}
