use serde::{Deserialize, Serialize};
use serde_json::Result;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Task {
    execution_pattern: String,
    channel: String
}

#[derive(Serialize, Deserialize)]
struct Offers {
    interval: u32,
    list: Vec<Task>
}

fn send_order(employee: String) {
    println!("{}", employee);
}

fn run() {
    let jobs = fs::read_to_string("../config/employer.config").unwrap();
    let mut schedule = JobScheduler::new();
    let offers: Offers = serde_json::from_str(&jobs).unwrap();

    for job in offers.list {

        if job.channel == "postman" { 
            schedule.add(
                Job::new(
                    job.execution_pattern.parse().unwrap(), || 
                        { 
                            send_order("postman".to_string()); 
                        } 
                )
            );
        }
            
        if job.channel == "sudoku" {
            schedule.add(
                Job::new(
                    job.execution_pattern.parse().unwrap(), || 
                    { 
                        send_order("sudoku".to_string()); 
                    } 
                )
            );
        }

    }

    loop {
        schedule.tick();
        std::thread::sleep(Duration::from_millis(offers.interval));
    }
}

fn main() {
    println!("Employer:: Start");
    run();
}
