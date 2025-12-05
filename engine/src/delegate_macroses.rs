#[macro_export]
macro_rules! __delegate_UserEventGatewayI {
    ($field:ident) => {
        #[inline(always)]
        fn drain_events(&mut self) {
            self.$field.drain_events()
        }

        #[inline(always)]
        fn get_one_event(&mut self) -> Option<crate::component::user_event::UserEvent> {
            self.$field.get_one_event()
        }
    };
}

#[macro_export]
macro_rules! __delegate_TerminalI {
    ($field:ident) => {
        #[inline(always)]
        fn redraw(&mut self) {
            self.$field.redraw();
        }

        #[inline(always)]
        fn get_surface_size(
            &self,
            ref_to_surf: crate::component::ref_to_surface::RefToSurface,
        ) -> Result<crate::component::size::Size, crate::ui::terminal::TerminalError> {
            self.$field.get_surface_size(ref_to_surf)
        }

        #[inline(always)]
        fn add_surface(
            &mut self,
            surf_conf: crate::ui::configs::SurfaceConfig,
        ) -> crate::component::ref_to_surface::RefToSurface {
            self.$field.add_surface(surf_conf)
        }

        #[inline(always)]
        fn update_surface(
            &mut self,
            pos: crate::component::positions::SurfacePosition,
            surface_ref: crate::component::ref_to_surface::RefToSurface,
            unit: crate::ui::draw_unit::DrawUnit,
        ) -> Result<(), crate::ui::terminal::TerminalError> {
            self.$field.update_surface(pos, surface_ref, unit)
        }

        #[inline(always)]
        fn degrade_surface(
            &mut self,
            pos: crate::component::positions::SurfacePosition,
            surface_ref: crate::component::ref_to_surface::RefToSurface,
        ) -> Result<crate::ui::draw_unit::DrawUnit, crate::ui::terminal::TerminalError> {
            self.$field.degrade_surface(pos, surface_ref)
        }

        #[inline(always)]
        fn change_mounting_point(
            &mut self,
            mounting_point: crate::ui::configs::MountingPoint,
            surface_ref: crate::component::ref_to_surface::RefToSurface,
        ) -> Result<(), crate::ui::terminal::TerminalError> {
            self.$field
                .change_mounting_point(mounting_point, surface_ref)
        }
    };
}

#[macro_export]
macro_rules! __delegate_TransformationMasterI {
    ($field:ident) => {
        #[inline(always)]
        fn add_transformation(
            &mut self,
            timestamp: crate::component::timestamp::Timestamp,
            transformation_pointer: crate::component::pointer::Pointer,
        ) {
            self.$field
                .add_transformation(timestamp, transformation_pointer);
        }

        #[inline(always)]
        fn current_timestamp(&self) -> crate::component::timestamp::Timestamp {
            self.$field.current_timestamp()
        }

        #[inline(always)]
        fn current_pointer(&self) -> crate::component::pointer::Pointer {
            self.$field.current_pointer()
        }
    };
}

#[macro_export]
macro_rules! __delegate_DataMasterI {
    ($field:ident) => {
        #[inline(always)]
        fn get_pointer_by_name<T: Into<usize>>(
            &self,
            name: T,
        ) -> Option<crate::component::pointer::Pointer> {
            self.$field.get_pointer_by_name(name)
        }

        #[inline(always)]
        fn get<T: crate::component::Component>(
            &self,
            pointer: crate::component::pointer::Pointer,
        ) -> Result<&T, crate::data_master::DataMasterError> {
            self.$field.get(pointer)
        }

        #[inline(always)]
        fn get_unchecked<T: crate::component::Component>(
            &self,
            pointer: crate::component::pointer::Pointer,
        ) -> &T {
            self.$field.get_unchecked(pointer)
        }

        #[inline(always)]
        fn get_mut<T: crate::component::Component>(
            &mut self,
            pointer: crate::component::pointer::Pointer,
        ) -> Result<&mut T, crate::data_master::DataMasterError> {
            self.$field.get_mut(pointer)
        }

        #[inline(always)]
        fn get_mut_unchecked<T: crate::component::Component>(
            &mut self,
            pointer: crate::component::pointer::Pointer,
        ) -> &mut T {
            self.$field.get_mut_unchecked(pointer)
        }

        #[inline(always)]
        fn set_by_pointer<T: crate::component::Component>(
            &mut self,
            pointer: crate::component::pointer::Pointer,
            value: T,
        ) -> &mut Self {
            self.$field.set_by_pointer(pointer, value);
            self
        }

        #[inline(always)]
        fn set<T: crate::component::Component>(&mut self, value: T) -> &mut Self {
            self.$field.set(value);
            self
        }

        #[inline(always)]
        fn set_name_by_pointer<T: Into<usize>>(
            &mut self,
            pointer: crate::component::pointer::Pointer,
            name: T,
        ) -> &mut Self {
            self.$field.set_name_by_pointer(pointer, name);
            self
        }

        #[inline(always)]
        fn set_name<T: Into<usize>>(&mut self, name: T) -> &mut Self {
            self.$field.set_name(name);
            self
        }

        #[inline(always)]
        fn remove(&mut self, pointer: crate::component::pointer::Pointer) {
            self.$field.remove(pointer);
        }

        #[inline(always)]
        fn finalize(&mut self) -> crate::component::pointer::Pointer {
            self.$field.finalize()
        }
    };
}
