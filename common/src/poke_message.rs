use serde::{Deserialize, Serialize};
use crate::serialized::Serialized;

#[derive(Serialize, Deserialize)]
pub struct Poke {
    pub action : String
}

pub trait Pokeing {
    fn new(new_action: String) -> Poke;
}

impl Pokeing for Poke {
    fn new(new_action: String) -> Poke {
        Poke {
            action: new_action
        }
    }
}

impl Serialized for Poke {
    fn serialized(&self) -> String {
        serde_json::to_string(&self).unwrap().to_string()
    }
}