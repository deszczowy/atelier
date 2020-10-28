use serde::{Deserialize, Serialize};
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
use std::fs;
use concierge::*;
use common::poke_message::*;
use common::log::*;
use common::serialized::Serialized;

const LIB_NAME : &str = "employer";
const STANDARD_ORDER : &str = "RUN";

#[derive(Serialize, Deserialize)]
struct Task {
    execution_pattern: String,
    channel: String
}

#[derive(Serialize, Deserialize)]
struct Offers {
    interval: u64,
    list: Vec<Task>
}

fn send_order(employee: String, order: String) {
    write_log(format!("Sending order to {}", employee), LIB_NAME);
    let concierge = Concierge::new();
    let msg = Poke::new(order).serialized();
    concierge.leave_message(employee, msg);
}

// this is for asking status. Not used right now, but will be.
#[allow(dead_code)]
fn ask(employee: String) {
    write_log(format!("Sending status request to {}", employee), LIB_NAME);
    let concierge = Concierge::new();
    let msg = Poke::new("STATUS".to_string()).serialized();
    concierge.leave_message(employee, msg);
}

fn run() {
    write_log("Employer::Start".to_string(), LIB_NAME);
    let jobs = fs::read_to_string("../config/employer.config").unwrap();
    let mut schedule = JobScheduler::new();
    let offers: Offers = serde_json::from_str(&jobs).unwrap();

    for job in offers.list {

        write_log(format!("Schedule for {}", job.channel), LIB_NAME);

        if job.channel == "sudoku" {
            schedule.add(
                Job::new(
                    job.execution_pattern.parse().unwrap(), || 
                    { 
                        send_order("sudoku".to_string(), STANDARD_ORDER.to_string()); 
                    } 
                )
            );
            write_log("Added".to_string(), LIB_NAME);
        }
    }

    write_log(format!("Running main loop with {}mills duration ", offers.interval), LIB_NAME);

    loop {
        schedule.tick();
        std::thread::sleep(Duration::from_millis(offers.interval));
    }
}

fn main() {
    run();
}
