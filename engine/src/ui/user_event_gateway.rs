use crate::common::user_event::UserEvent;

#[derive(Default, Clone, Copy)]
pub struct UserEventGateway {}

impl Iterator for UserEventGateway {
    type Item = UserEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if crossterm::event::poll(std::time::Duration::ZERO).unwrap() {
            Some(crossterm::event::read().unwrap().into())
        } else {
            return None;
        }
    }
}
