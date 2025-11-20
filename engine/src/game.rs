use std::{thread::sleep, time::Duration};

use crate::{
    component::transformation::Transformation,
    data_master::{DataMaster, DataMasterI},
    transformation_master::{TransformationMaster, TransformationMasterI},
    ui::{
        UserInterface, configs::UIConfig, terminal::TerminalI,
        user_event_gateway::UserEventGatewayI,
    },
};

#[derive(engine_macros::Delegate)]
pub struct Game {
    #[delegate(TerminalI, UserEventGatewayI)]
    user_interface: UserInterface,

    #[delegate(TransformationMasterI)]
    transformation_master: TransformationMaster,

    #[delegate(DataMasterI)]
    data_master: DataMaster,
}

impl Game {
    pub fn new(ui_config: UIConfig) -> Self {
        Self {
            user_interface: UserInterface::new(ui_config),
            transformation_master: TransformationMaster::default(),
            data_master: DataMaster::default(),
        }
    }

    pub fn run(mut self) {
        loop {
            while let Some(pointer) = self.transformation_master.next() {
                (self
                    .data_master
                    .get_mut::<Transformation>(pointer)
                    .copied()
                    .unwrap()
                    .tr)(&mut self)
            }

            sleep(Duration::from_millis(10));
        }
    }
}
