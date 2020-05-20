use tuner::*;

use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher, Op};
use uuid::Uuid;
use std::sync::mpsc::channel;

use std::time::Duration;
use std::thread;

pub struct Concierge {
    root: String,
}

// todo: close channel with wipe out

pub trait Serve {
    fn new() -> Concierge;
    fn retrieve_message(&self, path: String) -> std::io::Result<String>;
    fn call(&self, channel_name: String) -> std::io::Result<PathBuf>;
    fn expect(&self, channel_name: String, action: &dyn Fn(String)) -> std::io::Result<()>;
    fn leave_message(&self, channel_name: String, message: String) -> std::io::Result<()>;
}

impl Serve for Concierge {

    fn new() -> Concierge {
        let cfg = Config::new("../config/concierge.config".to_string()).unwrap();

        Concierge {            
            root: cfg["channels_root"].as_str().unwrap().to_string(),
        }
    }

    fn retrieve_message(&self, path: String) -> std::io::Result<String> {
        println!("Path = {}", path);
        let mut contents = fs::read_to_string(path)?;
        println!("Contents = {:?}", contents);
        Ok(contents)
    }

    fn call(&self, channel_name: String) -> std::io::Result<PathBuf> {
        let mut message_location = PathBuf::from(&self.root);
        message_location.push(&channel_name);
        println!("{:?}", message_location);

        fs::create_dir_all(&message_location)?;
        Ok(message_location)
    }

    fn expect(&self, channel_name: String, action: &dyn Fn(String)) -> std::io::Result<()> {
        let (tx, rx) = channel();
        let mut watcher = raw_watcher(tx).unwrap();

        let channel = self.call(channel_name).unwrap();
        watcher.watch(channel.as_path(), RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
            Ok(RawEvent{path: Some(path), op: Ok(operation), cookie}) => {

                //println!("operation {:?} on file {:?}, cookie {:?} ", path, operation, cookie);

                if operation == Op::CLOSE_WRITE {
                    let m = self.retrieve_message(path.to_str().unwrap().to_string());
                    action(m.unwrap());
                }
                
            },
            Ok(event) => println!("broken event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
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

        let mut message_instance = File::create(path.as_path())?;
        message_instance.write_all(message.as_bytes());
        Ok(())
    }
}