use tuner::*;
use common::log::*;

use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher, Op};
use uuid::Uuid;
use std::sync::mpsc::channel;

pub struct Concierge {
    root: String,
}

const LIB_NAME : &str = "concierge";

// todo: close channel with wipe out

pub trait Serve {
    fn new() -> Concierge;
    fn retrieve_message(&self, path: String) -> std::io::Result<String>;
    fn call(&self, channel_name: String) -> std::io::Result<PathBuf>;
    fn expect(&self, channel_name: String, action: &dyn Fn(String, bool)) -> std::io::Result<()>;
    fn leave_message(&self, channel_name: String, message: String) -> std::io::Result<()>;
}

impl Serve for Concierge {

    fn new() -> Concierge {
        let cfg = Config::new("../config/concierge.config".to_string()).unwrap();
        write_log(format!("Concierge start at root {}", cfg["channels_root"]), LIB_NAME);

        Concierge {            
            root: cfg["channels_root"].as_str().unwrap().to_string(),
        }
    }

    fn retrieve_message(&self, path: String) -> std::io::Result<String> {
        write_log(format!("Retrieve message from {}", path), LIB_NAME);
        let contents = fs::read_to_string(path)?;
        write_log(format!("Contents = {:?}", contents), LIB_NAME);
        Ok(contents)
    }

    fn call(&self, channel_name: String) -> std::io::Result<PathBuf> {
        write_log(format!("Calling into channel {}", channel_name), LIB_NAME);
        let mut message_location = PathBuf::from(&self.root);
        message_location.push(&channel_name);
        write_log(format!("{:?}", message_location), LIB_NAME);

        fs::create_dir_all(&message_location)?;
        Ok(message_location)
    }

    fn expect(&self, channel_name: String, action: &dyn Fn(String, bool)) -> std::io::Result<()> {
        write_log(format!("Listening on {}", channel_name), LIB_NAME);
        let (tx, rx) = channel();
        let mut watcher = raw_watcher(tx).unwrap();

        let channel = self.call(channel_name).unwrap();
        watcher.watch(channel.as_path(), RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(operation), cookie: _}) => {

                write_log(format!("Operation {:?} on file {:?}", path, operation), LIB_NAME);

                if operation == Op::CLOSE_WRITE {
                    write_log("Run action!".to_string(), LIB_NAME);
                    let m = self.retrieve_message(path.to_str().unwrap().to_string());
                    action(m.unwrap(), false);
                }
                
            },
            Ok(event) => write_log(format!("broken event: {:?}", event), LIB_NAME),
            Err(e) => write_log(format!("watch error: {:?}", e), LIB_NAME),
            }
        }
    }

    // todo: some mechanics for regulation sending messages into channel which is listened to
    // if receiving message triggers sending back to the channel you will find yourself into
    // infinite loop of messages, till resources run out
    
    fn leave_message(&self, channel_name: String, message: String) -> std::io::Result<()> {
        let message_id = Uuid::new_v4().to_simple().to_string();
        
        let mut path = self.call(channel_name).unwrap();
        path.push(&message_id);

        write_log(format!("Leaving message with id {} in {:?}", message_id, path), LIB_NAME);

        let mut message_instance = File::create(path.as_path())?;
        message_instance.write_all(message.as_bytes());
        Ok(())
    }
}