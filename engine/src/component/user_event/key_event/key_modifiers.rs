#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyModifiers {
    Shift,
    Control,
    Alt,
    None,
}

impl From<crossterm::event::KeyModifiers> for KeyModifiers {
    fn from(value: crossterm::event::KeyModifiers) -> Self {
        match value {
            crossterm::event::KeyModifiers::ALT => Self::Alt,
            crossterm::event::KeyModifiers::CONTROL => Self::Control,
            crossterm::event::KeyModifiers::NONE => Self::None,
            crossterm::event::KeyModifiers::SHIFT => Self::Shift,
            _ => panic!("Failed to transform {:?} to KeyModifiers", value),
        }
    }
}
