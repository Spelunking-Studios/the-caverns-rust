use serde::{Serialize, Deserialize};
use ron::{to_string, from_string};

pub struct DataStore {
    
}

pub struct Field<T> {
    value: T
}

impl Field<T> {
    pub fn set<T>(&mut self, new_value: T) {
        self.value = new_value;
    }
}
