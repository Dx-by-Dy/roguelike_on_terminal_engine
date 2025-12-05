use {
    crate::{
        component::{
            positions::{SurfacePosition, TerminalPosition},
            ref_to_surface::RefToSurface,
            size::Size,
        },
        ui::{
            configs::{MountingPoint, SurfaceConfig, UIConfig},
            draw_unit::DrawUnit,
            surface::Surface,
        },
    },
    crossterm::{ExecutableCommand, QueueableCommand, cursor, style::PrintStyledContent, terminal},
    std::{
        collections::HashMap,
        io::{Stdout, Write, stdout},
    },
};

pub trait TerminalI {
    fn get_surface_size(&self, ref_to_surf: RefToSurface) -> Result<Size, TerminalError>;
    fn redraw(&mut self);
    fn add_surface(&mut self, surf_conf: SurfaceConfig) -> RefToSurface;
    fn update_surface(
        &mut self,
        pos: SurfacePosition,
        surface_ref: RefToSurface,
        unit: DrawUnit,
    ) -> Result<(), TerminalError>;
    fn degrade_surface(
        &mut self,
        pos: SurfacePosition,
        surface_ref: RefToSurface,
    ) -> Result<DrawUnit, TerminalError>;
    fn change_mounting_point(
        &mut self,
        mounting_point: MountingPoint,
        surface_ref: RefToSurface,
    ) -> Result<(), TerminalError>;
}

#[derive(Debug)]
pub enum TerminalError {
    NotExistSurface,
}

pub struct Terminal {
    out: Stdout,
    size: Size,
    surfaces: HashMap<usize, Surface>,
}

impl Terminal {
    pub fn new(config: UIConfig) -> Self {
        let mut out = stdout();

        crossterm::terminal::enable_raw_mode().unwrap();

        out.execute(crossterm::terminal::EnterAlternateScreen)
            .unwrap();
        out.execute(crossterm::cursor::Hide).unwrap();
        out.execute(crossterm::terminal::SetSize(
            config.terminal_size.x,
            config.terminal_size.y,
        ))
        .unwrap();
        out.execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        Self {
            out,
            size: config.terminal_size,
            surfaces: HashMap::default(),
        }
    }

    fn transform_position(
        surface: &Surface,
        pos: SurfacePosition,
        term_size: Size,
    ) -> Option<TerminalPosition> {
        surface.transform_position(pos, term_size)
    }
}

impl TerminalI for Terminal {
    fn redraw(&mut self) {
        for (_, surface) in &mut self.surfaces {
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

    fn get_surface_size(&self, ref_to_surf: RefToSurface) -> Result<Size, TerminalError> {
        self.surfaces
            .get(&ref_to_surf.value)
            .map(|surf| surf.get_size())
            .ok_or(TerminalError::NotExistSurface)
    }

    fn add_surface(&mut self, surf_conf: SurfaceConfig) -> RefToSurface {
        let mut i = 0;
        while self.surfaces.contains_key(&i) {
            i += 1;
        }
        self.surfaces.insert(i, Surface::new(surf_conf));
        RefToSurface::new(i)
    }

    fn update_surface(
        &mut self,
        pos: SurfacePosition,
        surface_ref: RefToSurface,
        unit: DrawUnit,
    ) -> Result<(), TerminalError> {
        self.surfaces
            .get_mut(&surface_ref.value)
            .map(|surf| surf.forward(pos, unit))
            .ok_or(TerminalError::NotExistSurface)
    }

    fn degrade_surface(
        &mut self,
        pos: SurfacePosition,
        surface_ref: RefToSurface,
    ) -> Result<DrawUnit, TerminalError> {
        self.surfaces
            .get_mut(&surface_ref.value)
            .map(|surf: &mut Surface| surf.backward(pos))
            .ok_or(TerminalError::NotExistSurface)
    }

    fn change_mounting_point(
        &mut self,
        mounting_point: MountingPoint,
        surface_ref: RefToSurface,
    ) -> Result<(), TerminalError> {
        self.surfaces
            .get_mut(&surface_ref.value)
            .map(|surf| surf.change_mounting_point(mounting_point))
            .ok_or(TerminalError::NotExistSurface)
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
