use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Poke {
    action : String
}

pub trait Pokeing {
    fn new(new_action: String) -> Poke;
    fn serialized(&self) -> String;
}

impl Pokeing for Poke {
    fn new(new_action: String) -> Poke {
        Poke {
            action: new_action
        }
    }

    fn serialized(&self) -> String {
        serde_json::to_string(&self).unwrap().to_string()
    }
}