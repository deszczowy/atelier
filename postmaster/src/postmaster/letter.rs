use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Letter {
    pub subject: String,
    pub message: String,
    pub attachment: String,
    pub recipient: String,
}

pub trait BeLetter {
    fn new() -> Letter;
}

impl BeLetter for Letter {
    fn new() -> Letter {
        Letter {
            subject: "".to_string(),
            message: "".to_string(),
            attachment: "".to_string(),
            recipient: "".to_string()    
        }
    }
}

/*
ToDo: 
    - list of files to attach
*/