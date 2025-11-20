use crate::component::user_event::UserEvent;

pub trait UserEventGatewayI {
    fn drain_events(&mut self);
    fn get_one_event(&mut self) -> Option<UserEvent>;

    fn get_one_event_and_drain(&mut self) -> Option<UserEvent> {
        let one = self.get_one_event();
        self.drain_events();
        one
    }
}

#[derive(Default, Clone, Copy)]
pub struct UserEventGateway {}

impl UserEventGatewayI for UserEventGateway {
    fn drain_events(&mut self) {
        while let Some(_) = self.get_one_event() {}
    }

    fn get_one_event(&mut self) -> Option<UserEvent> {
        if crossterm::event::poll(std::time::Duration::ZERO).unwrap() {
            Some(crossterm::event::read().unwrap().into())
        } else {
            return None;
        }
    }
}
