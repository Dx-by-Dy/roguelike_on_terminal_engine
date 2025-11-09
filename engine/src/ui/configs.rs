use crate::common::{point::Point, size::Size};

pub struct UIConfig {
    pub terminal_config: TerminalConfig,
}

pub struct TerminalConfig {
    pub terminal_size: Size,
    pub surface_confs: Vec<SurfaceConfig>,
}

pub struct SurfaceConfig {
    pub size: Size,
    pub mounting_point: MountingPoint,
}

impl SurfaceConfig {
    pub fn new(size: Size, mounting_point: MountingPoint) -> Self {
        Self {
            size,
            mounting_point,
        }
    }
}

pub struct MountingPoint {
    pub point: Point,
    pub kind: MountingPointKind,
}

impl MountingPoint {
    pub fn new(point: Point, kind: MountingPointKind) -> Self {
        Self { point, kind }
    }
}

pub enum MountingPointKind {
    UpperLeftCorner,
    DownLeftCorner,
    UpperRightCorner,
    DownRightCorner,
}
