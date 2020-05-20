mod sudoku;
use sudoku::{Board, IsSudokuBoard, Printer, Printing};

use concierge::*;
use tuner::*;
use common::letter::Letter;
use common::serialized::Serialized;
use common::poke_message::*;


fn run(message: String) {
    println!("SUDOKU");

    let p : Poke = match serde_json::from_str(&message) {
        Ok(data) => data,
        Err(error) => panic!("Unable to read message: {:?}", error),
    };

    if p.action == "RUN" {

        let cfg = Config::new("../config/sudoku.config".to_string()).unwrap();
        let target = cfg["target"].as_str().unwrap().to_string();
        let tag = cfg["tag"].as_str().unwrap().to_string();

        let mut board = Board::new();

        let mut at = 0;
        while !board.validate_result() {
            at += 1;
            board.lets_go();
        }

        println!("Done in {} attempts!", at);

        board.mask_board();

        let mut printer = Printer::new();
        printer.print(&board);
        printer.save();

        let mail = Letter{
            subject: tag,
            message: "".to_string(),
            recipient: target,
            attachment: printer.file_name
        };

        let concierge = Concierge::new();
        println!("{:?}", mail);
        concierge.leave_message("postmaster".to_string(), mail.serialized());

    }
}

fn main() {

    let concierge = Concierge::new();
    concierge.expect("sudoku".to_string(), &run);
}