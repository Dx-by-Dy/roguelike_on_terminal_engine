use {
    crate::component::{pointer::Pointer, timestamp::Timestamp},
    std::collections::BTreeMap,
};

pub trait TransformationMasterI {
    fn add_transformation(&mut self, timestamp: Timestamp, transformation_pointer: Pointer);
    fn current_timestamp(&self) -> Timestamp;
    fn current_pointer(&self) -> Pointer;
}

#[derive(Default)]
pub struct TransformationMaster {
    current_timestamp_index: usize,
    current_pointer: Pointer,
    current_timestamp: Timestamp,
    index: BTreeMap<Timestamp, Vec<Pointer>>,
}

impl TransformationMaster {
    pub(crate) fn next(&mut self) -> Option<Pointer> {
        match self.index.get(&self.current_timestamp) {
            Some(v) => match v.get(self.current_timestamp_index) {
                Some(pointer) => {
                    self.current_timestamp_index += 1;
                    self.current_pointer = *pointer;
                    Some(self.current_pointer)
                }
                None => {
                    self.index.remove(&self.current_timestamp);
                    self.current_timestamp_index = 0;
                    self.current_timestamp = self.current_timestamp.next();
                    None
                }
            },
            None => {
                self.current_timestamp = self.current_timestamp.next();
                None
            }
        }
    }
}

impl TransformationMasterI for TransformationMaster {
    fn add_transformation(&mut self, timestamp: Timestamp, transformation_pointer: Pointer) {
        self.index
            .entry(timestamp)
            .or_default()
            .push(transformation_pointer);
    }

    fn current_timestamp(&self) -> Timestamp {
        self.current_timestamp
    }

    fn current_pointer(&self) -> Pointer {
        self.current_pointer
    }
}
