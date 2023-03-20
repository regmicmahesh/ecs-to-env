use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvVariable {
    name: String,
    value: String,
}

impl EnvVariable {
    pub fn to_kv(&self) -> String {
        format!("{}={}", self.name, self.value)
    }

    pub fn to_k(&self) -> String {
        self.name.clone()
    }

    pub fn to_v(&self) -> String {
        self.value.clone()
    }

    pub fn new(name: String, value: String) -> Self {
        EnvVariable {
            name,
            value,
        }
    }
}
