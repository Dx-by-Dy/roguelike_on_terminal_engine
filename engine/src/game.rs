use std::{thread::sleep, time::Duration};

use crate::{
    component::transformation::Transformation,
    data_master::{DataMaster, DataMasterI},
    transformation_master::{TransformationMaster, TransformationMasterI},
    ui::{UserInterface, configs::UIConfig},
};

pub struct Game {
    user_interface: UserInterface,
    transformation_master: TransformationMaster,
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

    // pub fn get_user_event(&mut self) -> Option<UserEvent> {
    //     self.ui.get_user_event()
    // }

    // pub fn update_surface(&mut self, pos: SurfacePosition, surface_id: usize, unit: DrawUnit) {
    //     self.ui.update_surface(pos, surface_id, unit);
    // }

    // pub fn degrade_surface(&mut self, pos: SurfacePosition, surface_id: usize) -> Option<DrawUnit> {
    //     self.ui.degrade_surface(pos, surface_id)
    // }

    // pub fn get_ui(&self) -> &UserInterface {
    //     &self.user_interface
    // }

    // pub fn get_ui_mut(&mut self) -> &mut UserInterface {
    //     &mut self.user_interface
    // }

    // pub fn get_entity_master(&self) -> &EM {
    //     &self.entity_master
    // }

    // pub fn get_entity_master_mut(&mut self) -> &mut EM {
    //     &mut self.entity_master
    // }

    // pub fn get_transformation_master(&self) -> &TM {
    //     &self.transformation_master
    // }

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

            sleep(Duration::from_millis(16));
        }
    }

    pub fn redraw(&mut self) {
        self.user_interface.redraw();
    }
}

impl DataMasterI for Game {
    fn get_pointer_by_name<T: Into<usize>>(
        &self,
        name: T,
    ) -> Option<crate::component::pointer::Pointer> {
        self.data_master.get_pointer_by_name(name)
    }

    fn get<T: crate::component::Component>(
        &self,
        pointer: crate::component::pointer::Pointer,
    ) -> Result<&T, crate::data_master::DataMasterError> {
        self.data_master.get(pointer)
    }

    fn get_mut<T: crate::component::Component>(
        &mut self,
        pointer: crate::component::pointer::Pointer,
    ) -> Result<&mut T, crate::data_master::DataMasterError> {
        self.data_master.get_mut(pointer)
    }

    fn set_by_pointer<T: crate::component::Component>(
        &mut self,
        pointer: crate::component::pointer::Pointer,
        value: T,
    ) -> &mut Self {
        self.data_master.set_by_pointer(pointer, value);
        self
    }

    fn set<T: crate::component::Component>(&mut self, value: T) -> &mut Self {
        self.data_master.set(value);
        self
    }

    fn set_name_by_pointer<T: Into<usize>>(
        &mut self,
        pointer: crate::component::pointer::Pointer,
        name: T,
    ) -> &mut Self {
        self.data_master.set_name_by_pointer(pointer, name);
        self
    }

    fn set_name<T: Into<usize>>(&mut self, name: T) -> &mut Self {
        self.data_master.set_name(name);
        self
    }

    fn remove(&mut self, pointer: crate::component::pointer::Pointer) {
        self.data_master.remove(pointer);
    }

    fn finalize(&mut self) -> crate::component::pointer::Pointer {
        self.data_master.finalize()
    }
}

impl TransformationMasterI for Game {
    fn add_transformation(
        &mut self,
        timestamp: crate::component::timestamp::Timestamp,
        transformation_pointer: crate::component::pointer::Pointer,
    ) {
        self.transformation_master
            .add_transformation(timestamp, transformation_pointer);
    }

    fn current_timestamp(&self) -> crate::component::timestamp::Timestamp {
        self.transformation_master.current_timestamp()
    }

    fn current_pointer(&self) -> crate::component::pointer::Pointer {
        self.transformation_master.current_pointer()
    }
}
