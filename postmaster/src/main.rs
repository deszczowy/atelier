use concierge::*;

mod postmaster;
use postmaster::{Postmaster, Mailing};

fn run(message: String) {

    println!("Postmaster RUN!");
    
    let mut postmaster = Postmaster::new();
    postmaster.from(message);
    
    if postmaster.send() {
        println!("Email sent");
    } else {
        println!("Could not send email");
    }
}

fn main() {

    let concierge = Concierge::new();
    concierge.expect("postmaster".to_string(), &run);

}
