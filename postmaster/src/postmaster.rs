use tuner::*;

use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};

use std::path::Path;

use common::letter::{Letter, BeLetter};
use common::log::*;

const LIB_NAME : &str = "postmaster";

pub struct Postmaster {
    letter: Letter,
}

pub trait Mailing {
    fn new() -> Postmaster;
    fn from(&mut self, message: String);
    fn send(&self) -> bool;
}

impl Mailing for Postmaster {

    fn new() -> Postmaster {
        Postmaster {
            letter: Letter::new()
        }
    }
    
    fn from(&mut self, message: String) {
        let d = serde_json::from_str(&message);

        self.letter = match d {
            Ok(data) => data,
            Err(error) => {
                write_log(format!("Unable to read message: {:?}", error), LIB_NAME);
                Letter::new()
            },
        };
    }

    fn send(&self) -> bool {

        let cfg = Config::new("../config/postmaster.config".to_string()).unwrap();

        write_log(format!("Sending mail to {}", self.letter.recipient), LIB_NAME);

        let email = Email::builder()
            .to(self.letter.recipient.to_string())
            .from(cfg["mail_address"].as_str().unwrap().to_string())
            .subject(&self.letter.subject)
            .text(&self.letter.message)
            .attachment_from_file(Path::new(&self.letter.attachment), None, &mime::IMAGE_PNG)
            .unwrap()
            .build()
            .unwrap();

        let creds = Credentials::new(
            cfg["mail_address"].as_str().unwrap().to_string(),
            cfg["mail_password"].as_str().unwrap().to_string()
        );

        let mut mailer = SmtpClient::new_simple(&cfg["smtp_host"].as_str().unwrap().to_string())
            .unwrap()
            .credentials(creds)
            .transport();

        let result = mailer.send(email.into());

        if !result.is_ok() {
            write_log(format!("Error: {:?}", result), LIB_NAME);
        }
        result.is_ok()
    }
}