#[derive(Default, Clone, Copy)]
pub struct UserEventGateway {}

impl Iterator for UserEventGateway {
    type Item = crate::component::user_event::UserEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if crossterm::event::poll(std::time::Duration::ZERO).unwrap() {
            Some(crossterm::event::read().unwrap().into())
        } else {
            return None;
        }
    }
}
