#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
}

impl From<crossterm::event::KeyCode> for KeyCode {
    fn from(value: crossterm::event::KeyCode) -> Self {
        match value {
            crossterm::event::KeyCode::Backspace => Self::Backspace,
            crossterm::event::KeyCode::Enter => Self::Enter,
            crossterm::event::KeyCode::Left => Self::Left,
            crossterm::event::KeyCode::Right => Self::Right,
            crossterm::event::KeyCode::Up => Self::Up,
            crossterm::event::KeyCode::Down => Self::Down,
            crossterm::event::KeyCode::Home => Self::Home,
            crossterm::event::KeyCode::End => Self::End,
            crossterm::event::KeyCode::PageUp => Self::PageUp,
            crossterm::event::KeyCode::PageDown => Self::PageDown,
            crossterm::event::KeyCode::Tab => Self::Tab,
            crossterm::event::KeyCode::BackTab => Self::BackTab,
            crossterm::event::KeyCode::Delete => Self::Delete,
            crossterm::event::KeyCode::Insert => Self::Insert,
            crossterm::event::KeyCode::F(key) => Self::F(key),
            crossterm::event::KeyCode::Char(ch) => Self::Char(ch),
            crossterm::event::KeyCode::Null => Self::Null,
            crossterm::event::KeyCode::Esc => Self::Esc,
            _ => panic!("Failed to transform {:?} to KeyCode", value),
        }
    }
}
