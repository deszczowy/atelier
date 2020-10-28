use painting::*;

use concierge::*;
use tuner::*;
use common::log::*;
use common::serialized::Serialized;
use common::poke_message::*;
use common::arguments::*;

fn run(message: String, one_shot: bool) {
    
    let width = 1600 as u32;
    let height = 700 as u32;
    let frame = 5 as i32;
    
    println!("ARTIST!");

    let p : Poke = match serde_json::from_str(&message) {
        Ok(data) => data,
        Err(error) => panic!("Unable to read message: {:?}", error),
    };

    let target = "./".to_string();
    if !one_shot {
        let cfg = Config::new("../config/artist.config".to_string()).unwrap();
        let target = cfg["target"].as_str().unwrap().to_string();
    }

    if p.action == "CIRCLES" {
        use circles::*;
        let mut c = Painting::new(width, height);
        c.initialize();
        c.generate();
        c.put_a_frame(frame, painting::BLACK);
        c.save_file("test.png".to_string());
    } 
    else

    if p.action == "STRIPES" {
        use stripes::*;
        let mut c = Painting::new(width, height);
        c.initialize();
        c.generate();
        c.put_a_frame(frame, painting::BLACK);
        c.save_file("test.png".to_string());
    }

    if one_shot {
        println!("One shot done!");
    }

}

fn main() {
    if is_one_shot() {
        println!("One shot run!");
        let message = "{ \"action\":\"STRIPES\"}".to_string();
        run(message, true);
    } else {
        let concierge = Concierge::new();
        concierge.expect("art".to_string(), &run);
    }
}
