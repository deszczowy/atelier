use painting::*;

use concierge::*;
use tuner::*;
use common::log::*;
use common::poke_message::*;
use common::arguments::*;

const ARTIST_NAME : &str = "artist";

fn run(message: String, one_shot: bool) {
    
    let width = 1600 as u32;
    let height = 700 as u32;
    let frame = 5 as i32;
    
    write_log("ARTIST!".to_string(), ARTIST_NAME);

    let p : Poke = match serde_json::from_str(&message) {
        Ok(data) => data,
        Err(error) => panic!("Unable to read message: {:?}", error),
    };

    let mut target = "./".to_string();
    if !one_shot {
        let cfg = Config::new("../config/artist.config".to_string()).unwrap();
        target = cfg["target"].as_str().unwrap().to_string();
    }

    write_log(format!("Running {}", p.action), ARTIST_NAME);

    if p.action == "CIRCLES" {
        use circles::*;
        let mut c = Painting::new(width, height);
        c.initialize(target);
        c.generate();
        c.put_a_frame(frame, painting::BLACK);
        c.save_file();
    } 
    else

    if p.action == "STRIPES" {
        use stripes::*;
        let mut c = Painting::new(width, height);
        c.initialize(target);
        c.generate();
        c.put_a_frame(frame, painting::BLACK);
        c.save_file();
    }
    else

    if p.action == "ORNAMENT" {
        use ornament::*;
        let mut c = Painting::new(width, height);
        c.initialize(target);
        c.generate();
        c.put_a_frame(frame, painting::BLACK);
        c.save_test();
    }

    if one_shot {
        println!("One shot done!");
    } else {
        write_log("Done!".to_string(), ARTIST_NAME);
    }

}

fn main() {
    if is_one_shot() {
        println!("One shot run!");
        let message = "{ \"action\":\"ORNAMENT\"}".to_string();
        run(message, true);
    } else {
        let concierge = Concierge::new();
        concierge
            .expect("art".to_string(), &run)
            .expect("Concierge cannot fulfill any artist requests."); // this is from error handling
    }
}
