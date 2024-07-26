use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Context {
    data: HashMap<String, Value>,
}

impl Context {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Context { data }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.data.insert(key, value);
    }
}
