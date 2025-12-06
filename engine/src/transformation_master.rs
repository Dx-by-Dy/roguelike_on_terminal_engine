use {
    crate::{Gtr, component::timestamp::Timestamp},
    std::collections::BTreeMap,
};

pub trait TransformationMasterI {
    fn add_gtr(&mut self, timestamp: Timestamp, gtr: Gtr);
    fn current_timestamp(&self) -> Timestamp;
}

#[derive(Default)]
pub struct TransformationMaster {
    current_timestamp_index: usize,
    current_timestamp: Timestamp,
    index: BTreeMap<Timestamp, Vec<Gtr>>,
}

impl TransformationMaster {
    pub(crate) fn next(&mut self) -> Option<Gtr> {
        match self.index.get(&self.current_timestamp) {
            Some(v) => match v.get(self.current_timestamp_index) {
                Some(gtr) => {
                    self.current_timestamp_index += 1;
                    Some(*gtr)
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
    fn add_gtr(&mut self, timestamp: Timestamp, gtr: Gtr) {
        self.index.entry(timestamp).or_default().push(gtr);
    }

    fn current_timestamp(&self) -> Timestamp {
        self.current_timestamp
    }
}
