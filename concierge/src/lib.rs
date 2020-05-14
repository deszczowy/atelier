use tuner::*;

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

// todo: close channel with wipe out

trait Serve {
    fn new(configuration_file: String) -> Concierge;
    fn call(&self, channel_name: String) -> std::io::Result<PathBuf>;
    fn expect(&self, channel_name: String, action: &dyn Fn(String)) -> std::io::Result<()>;
    fn leave_message(&self, channel_name: String, message: String) -> std::io::Result<()>;
}

impl Serve for Concierge {

    fn new(configuration_file: String) -> Concierge {
        let cfg = Config::new(configuration_file).unwrap();

        Concierge {            
            root: cfg["channels_root"].to_string(),
        }
    }

    fn call(&self, channel_name: String) -> std::io::Result<PathBuf> {
        let mut message_location = PathBuf::from(&self.root);
        message_location.push(&channel_name);

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

                if operation == Op::CREATE {
                action(
                    format!("{:?}", cookie)
                );
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