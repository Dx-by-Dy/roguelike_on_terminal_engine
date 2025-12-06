use {
    crate::component::{Component, pointer::Pointer},
    std::{
        any::{Any, TypeId},
        collections::HashMap,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataMasterError {
    NotExistPointer,
    NotExistComponent,
    WrongComponentType,
}

pub trait DataMasterI {
    fn get_pointer_by_name<T: Into<usize>>(&self, name: T) -> Option<Pointer>;
    fn get<T: Component>(&self, pointer: Pointer) -> Result<&T, DataMasterError>;
    fn get_mut<T: Component>(&mut self, pointer: Pointer) -> Result<&mut T, DataMasterError>;
    fn set_by_pointer<T: Component>(&mut self, pointer: Pointer, value: T) -> &mut Self;
    fn set<T: Component>(&mut self, value: T) -> &mut Self;
    fn set_name_by_pointer<T: Into<usize>>(&mut self, pointer: Pointer, name: T) -> &mut Self;
    fn set_name<T: Into<usize>>(&mut self, name: T) -> &mut Self;
    fn remove(&mut self, pointer: Pointer);
    fn finalize(&mut self) -> Pointer;

    fn get_mut_unchecked<T: Component>(&mut self, pointer: Pointer) -> &mut T {
        self.get_mut(pointer).unwrap()
    }

    fn get_unchecked<T: Component>(&self, pointer: Pointer) -> &T {
        self.get(pointer).unwrap()
    }

    fn get_pointer_by_name_unchecked<T: Into<usize>>(&self, name: T) -> Pointer {
        self.get_pointer_by_name(name).unwrap()
    }

    fn set_by_name_unchecked<N: Into<usize>, T: Component>(
        &mut self,
        name: N,
        value: T,
    ) -> &mut Self {
        self.set_by_pointer(self.get_pointer_by_name_unchecked(name), value)
    }
}

#[derive(Default)]
pub struct DataMaster {
    new_pointer: Pointer,
    index: HashMap<Pointer, HashMap<TypeId, Box<dyn Component>>>,
    named_pointers: HashMap<usize, Pointer>,
}

impl DataMasterI for DataMaster {
    fn get_pointer_by_name<T: Into<usize>>(&self, name: T) -> Option<Pointer> {
        self.named_pointers.get(&name.into()).copied()
    }

    fn get<T: Component>(&self, pointer: Pointer) -> Result<&T, DataMasterError> {
        (self
            .index
            .get(&pointer)
            .ok_or(DataMasterError::NotExistPointer)?
            .get(&TypeId::of::<T>())
            .ok_or(DataMasterError::NotExistComponent)?
            .as_ref() as &dyn Any)
            .downcast_ref::<T>()
            .ok_or(DataMasterError::WrongComponentType)
    }

    fn get_mut<T: Component>(&mut self, pointer: Pointer) -> Result<&mut T, DataMasterError> {
        (self
            .index
            .get_mut(&pointer)
            .ok_or(DataMasterError::NotExistPointer)?
            .get_mut(&TypeId::of::<T>())
            .ok_or(DataMasterError::NotExistComponent)?
            .as_mut() as &mut dyn Any)
            .downcast_mut::<T>()
            .ok_or(DataMasterError::WrongComponentType)
    }

    fn set_by_pointer<T: Component>(&mut self, pointer: Pointer, value: T) -> &mut Self {
        self.index
            .entry(pointer)
            .or_default()
            .insert(TypeId::of::<T>(), Box::new(value));
        self
    }

    fn set<T: Component>(&mut self, value: T) -> &mut Self {
        self.set_by_pointer(self.new_pointer, value)
    }

    fn set_name_by_pointer<T: Into<usize>>(&mut self, pointer: Pointer, name: T) -> &mut Self {
        self.named_pointers.insert(name.into(), pointer);
        self
    }

    fn set_name<T: Into<usize>>(&mut self, name: T) -> &mut Self {
        self.set_name_by_pointer(self.new_pointer, name)
    }

    fn remove(&mut self, pointer: Pointer) {
        self.index.remove(&pointer);
    }

    fn finalize(&mut self) -> Pointer {
        let pointer = self.new_pointer;
        self.new_pointer = self.new_pointer.next();
        pointer
    }
}
