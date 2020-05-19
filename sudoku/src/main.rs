mod sudoku;
use sudoku::{Board, IsSudokuBoard, Printer, Printing};

fn run() {
    println!("SUDOKU");

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

}

fn main() {
    run();
}