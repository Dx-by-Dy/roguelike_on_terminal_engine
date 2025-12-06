use crate::{
    component::{timestamp::Timestamp, transformation::TransformationInit},
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
        self.init();

        loop {
            while let Some(gtr) = self.transformation_master.next() {
                gtr(&mut self)
            }

            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn init(&mut self) {
        for tr_init in inventory::iter::<TransformationInit> {
            if let Some(value) = tr_init.timestamp {
                self.add_gtr(Timestamp::new(value), (*tr_init).gtr);
            }
            if tr_init.set_in_datamaster {
                self.data_master
                    .set_name((*tr_init).gtr as usize)
                    .finalize();
            }
            if tr_init.init_gtr {
                ((*tr_init).gtr)(self);
            }
        }
    }
}
