use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Party {
    pub taille_du_mot: u32,
    pub nombre_essaies: u32,
    pub nombre_essaies_total: u32
}

impl Party {
    pub fn new(taille: u32, total: u32, current: u32) -> Self {
        Self {
            taille_du_mot: taille,
            nombre_essaies: current,
            nombre_essaies_total: total
        }
    }
}