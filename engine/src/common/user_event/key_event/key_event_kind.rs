#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}

impl From<crossterm::event::KeyEventKind> for KeyEventKind {
    fn from(value: crossterm::event::KeyEventKind) -> Self {
        match value {
            crossterm::event::KeyEventKind::Press => Self::Press,
            crossterm::event::KeyEventKind::Repeat => Self::Repeat,
            crossterm::event::KeyEventKind::Release => Self::Release,
        }
    }
}
