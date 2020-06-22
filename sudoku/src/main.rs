mod sudoku;
use sudoku::{Board, IsSudokuBoard, Printer, Printing};

use concierge::*;
use tuner::*;
use common::log::*;
use common::letter::Letter;
use common::serialized::Serialized;
use common::poke_message::*;
use common::arguments::*;


fn run(message: String, one_shot: bool) {
    println!("SUDOKU");

    let p : Poke = match serde_json::from_str(&message) {
        Ok(data) => data,
        Err(error) => panic!("Unable to read message: {:?}", error),
    };

    if p.action == "RUN" {

        let cfg = Config::new("../config/sudoku.config".to_string()).unwrap();
        let target = cfg["target"].as_str().unwrap().to_string();

        let mut board = Board::new();

        let mut at = 0;
        while !board.validate_result() {
            at += 1;
            board.lets_go();
        }

        println!("Done in {} attempts!", at);

        board.mask_board();

        let mut printer = Printer::new();
        printer.set_path(target);
        printer.print(&board);
        printer.save();

        if one_shot {
            println!("One shot done!");
        }
    }
}

fn main() {

    if is_one_shot() {
        println!("One shot run!");
        let message = "{ \"action\":\"RUN\"}".to_string();
        run(message, true);
    } else {
        let concierge = Concierge::new();
        concierge.expect("sudoku".to_string(), &run);
    }
}