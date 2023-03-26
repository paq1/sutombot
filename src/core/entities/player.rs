use serde::{Deserialize, Serialize};
use crate::core::entities::party::Party;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub parties: Vec<Party>
}