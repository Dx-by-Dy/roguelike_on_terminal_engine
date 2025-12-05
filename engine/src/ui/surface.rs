use std::collections::HashSet;

use crate::{
    component::{
        point::Point,
        positions::{SurfacePosition, TerminalPosition},
        size::Size,
    },
    ui::{
        configs::{MountingPoint, MountingPointKind, SurfaceConfig},
        draw_unit::DrawUnit,
    },
};

impl MountingPoint {
    fn transform_position(
        &self,
        surf_position: SurfacePosition,
        surf_size: Size,
        term_size: Size,
    ) -> Option<TerminalPosition> {
        match self.kind {
            MountingPointKind::UpperLeftCorner => {
                Self::upper_left_corner_transform(self.point, surf_position, term_size)
            }
            MountingPointKind::DownLeftCorner => {
                Self::down_left_corner_transform(self.point, surf_position, surf_size, term_size)
            }
            MountingPointKind::UpperRightCorner => {
                Self::upper_right_corner_transform(self.point, surf_position, surf_size, term_size)
            }
            MountingPointKind::DownRightCorner => {
                Self::down_right_corner_transform(self.point, surf_position, surf_size)
            }
        }
    }

    fn upper_left_corner_transform(
        point: Point,
        surf_position: SurfacePosition,
        term_size: Size,
    ) -> Option<TerminalPosition> {
        let new_x = point.x + surf_position.x as i16;
        let new_y = point.y + surf_position.y as i16;
        if new_x < term_size.x as i16 && new_y < term_size.y as i16 {
            Some(TerminalPosition {
                x: new_x as u16,
                y: new_y as u16,
            })
        } else {
            None
        }
    }

    fn down_left_corner_transform(
        point: Point,
        surf_position: SurfacePosition,
        surf_size: Size,
        term_size: Size,
    ) -> Option<TerminalPosition> {
        let new_x = point.x + surf_position.x as i16;
        let new_y = point.y + surf_position.y as i16 - surf_size.y as i16;
        if new_x < term_size.x as i16 && new_y >= 0 {
            Some(TerminalPosition {
                x: new_x as u16,
                y: new_y as u16,
            })
        } else {
            None
        }
    }

    fn upper_right_corner_transform(
        point: Point,
        surf_position: SurfacePosition,
        surf_size: Size,
        term_size: Size,
    ) -> Option<TerminalPosition> {
        let new_x = point.x + surf_position.x as i16 - surf_size.x as i16;
        let new_y = point.y + surf_position.y as i16;
        if new_x >= 0 && new_y < term_size.y as i16 {
            Some(TerminalPosition {
                x: new_x as u16,
                y: new_y as u16,
            })
        } else {
            None
        }
    }

    fn down_right_corner_transform(
        point: Point,
        surf_position: SurfacePosition,
        surf_size: Size,
    ) -> Option<TerminalPosition> {
        let new_x = point.x + surf_position.x as i16 - surf_size.x as i16;
        let new_y = point.y + surf_position.y as i16 - surf_size.y as i16;
        if new_x >= 0 && new_y >= 0 {
            Some(TerminalPosition {
                x: new_x as u16,
                y: new_y as u16,
            })
        } else {
            None
        }
    }
}

pub struct Surface {
    mounting_point: MountingPoint,
    size: Size,
    data: Vec<Vec<SurfaceUnit>>,
    changed_pos: HashSet<SurfacePosition>,
}

impl Surface {
    pub fn new(surf_conf: SurfaceConfig) -> Self {
        Self {
            data: vec![
                vec![SurfaceUnit::default(); surf_conf.size.x as usize];
                surf_conf.size.y as usize
            ],
            changed_pos: HashSet::new(),
            size: surf_conf.size,
            mounting_point: surf_conf.mounting_point,
        }
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn forward(&mut self, pos: SurfacePosition, unit: DrawUnit) {
        self.changed_pos.insert(pos);
        self.data[pos.y as usize][pos.x as usize].forward(unit);
    }

    pub fn backward(&mut self, pos: SurfacePosition) -> DrawUnit {
        self.changed_pos.insert(pos);
        self.data[pos.y as usize][pos.x as usize].backward()
    }

    pub fn change_mounting_point(&mut self, mounting_point: MountingPoint) {
        self.mounting_point = mounting_point
    }

    pub fn transform_position(
        &self,
        pos: SurfacePosition,
        terminal_size: Size,
    ) -> Option<TerminalPosition> {
        self.mounting_point
            .transform_position(pos, self.size, terminal_size)
    }
}

impl Iterator for Surface {
    type Item = (SurfacePosition, DrawUnit);

    fn next(&mut self) -> Option<Self::Item> {
        match self.changed_pos.iter().next().copied() {
            Some(pos) => {
                self.changed_pos.remove(&pos);
                Some((pos, self.data[pos.y as usize][pos.x as usize].get_unit()))
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
struct SurfaceUnit {
    history: Vec<DrawUnit>,
}

impl SurfaceUnit {
    pub fn forward(&mut self, unit: DrawUnit) {
        self.history.push(unit);
    }

    pub fn backward(&mut self) -> DrawUnit {
        self.history.pop().unwrap_or_default()
    }

    pub fn get_unit(&mut self) -> DrawUnit {
        self.history.last().copied().unwrap_or_default()
    }
}

impl Default for SurfaceUnit {
    fn default() -> Self {
        Self { history: vec![] }
    }
}
