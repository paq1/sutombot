use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
}

impl CreatePlayer {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}
