pub mod configs;
pub mod draw_unit;
pub mod terminal;
pub mod user_event_gateway;

mod surface;

use crate::ui::{
    configs::UIConfig,
    terminal::{Terminal, TerminalI},
    user_event_gateway::{UserEventGateway, UserEventGatewayI},
};

#[derive(engine_macros::Delegate)]
pub struct UserInterface {
    #[delegate(TerminalI)]
    terminal: Terminal,

    #[delegate(UserEventGatewayI)]
    user_event_gateway: UserEventGateway,
}

impl UserInterface {
    pub fn new(config: UIConfig) -> Self {
        Self {
            terminal: Terminal::new(config),
            user_event_gateway: UserEventGateway::default(),
        }
    }
}
