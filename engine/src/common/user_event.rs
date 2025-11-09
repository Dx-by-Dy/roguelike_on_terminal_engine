pub mod key_event;

use {crate::common::size::Size, key_event::KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserEvent {
    KeyEvent(KeyEvent),
    Resize(Size),
}

impl From<crossterm::event::Event> for UserEvent {
    fn from(value: crossterm::event::Event) -> Self {
        match value {
            crossterm::event::Event::Key(key_event) => Self::KeyEvent(key_event.into()),
            crossterm::event::Event::Resize(x, y) => Self::Resize(Size { x, y }),
            _ => panic!("Failed to transform {:?} to UserEvent", value),
        }
    }
}
