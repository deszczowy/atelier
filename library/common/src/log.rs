use std::fs::OpenOptions;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use super::date::*;

pub fn write_log(message: String, tag: &str) {
    write_to_file(message, tag);
}

fn write_to_file(message: String, tag: &str) {
    let file_name = prepare_log_file_name(tag);
    let entry = prepare_log_entry(message);

    //println!("{}", file_name);

    println!("{}: {}", tag, entry);
    append_file(file_name, entry);
}

fn prepare_log_entry(message: String) -> String {
    format!("{} | {}", time_stamp(), message)
}

fn prepare_log_file_name(tag: &str) -> String {
    format!("../logs/{}.log", tag)
}

fn append_file(file_name: String, new_content: String) {
    if Path::new(&file_name).exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap();

        if let Err(e) = writeln!(file, "{}", new_content) {
            println!("Couldn't write to file: {}", e);
        }
    } else {
        match File::create(file_name) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(new_content.as_bytes()) {
                    println!("Couldn't write to file: {}", e);
                }
            },
            Err(e) => println!("Couldn't write to file: {}", e)
        }
    }

    
}