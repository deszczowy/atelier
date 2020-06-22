#![allow(unused)]
use rand::prelude::*;
use std::io;
use std::collections::HashSet;

use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{FontCollection, Scale};

use common::date::*;

const WHITE : Rgb<u8> = Rgb([255u8, 255u8, 255u8]);
const BLACK : Rgb<u8> = Rgb([0u8, 0u8, 0u8]);


/*
************************************************* C E L L
*/
struct Cell {
    value: u8,
    valid: bool
}

trait IsSudokuCell {
    fn new() -> Cell;
}

impl IsSudokuCell for Cell {
    
    fn new() -> Cell {
        Cell {
            value: 0,
            valid: false
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

/*
************************************************* B O X
*/

struct Square {
    start_row : u8,
    start_column : u8,
    end_row : u8,
    end_column : u8
}

trait IsSquare {
    fn new() -> Square;
    fn calculate(&mut self, row: u8, column: u8);
}

impl IsSquare for Square {
    
    fn new() -> Square {
        Square {
            start_row : 0,
            start_column : 0,
            end_row : 0,
            end_column : 0
        }
    }

    fn calculate(&mut self, row: u8, column: u8) {

        //print!("Box calculation from row {} and column {} ", row, column);
        
        let div : f32 = 2.7;

        self.start_row = (row as f32 / div) as u8;
        self.start_row = self.start_row * 3;
        self.end_row = self.start_row + 2;
        
        self.start_column = (column as f32 / div) as u8;
        self.start_column = self.start_column * 3;
        self.end_column = self.start_column + 2;

        //println!("-> rows {}-{}, columns {}-{}. ", self.start_row, self.end_row, self.start_column, self.end_column);
        
    }
}


/*
************************************************* B O A R D
*/

pub struct Board {
    cells : [[Cell; 9]; 9],
    current_row : u8
}

pub trait IsSudokuBoard { // todo: split into process and printing traits
    fn new() -> Board;
    fn zero(&mut self);

    fn check_column(&self, row: u8, column: u8) -> bool;
    fn check_box(&self, row: u8, column: u8) -> bool;
    fn fits(&self, row: u8, column: u8) -> bool;
    fn print(&self);
    fn print_row(&self, row:u8);

    fn get(&self, row: u8, column: u8) -> u8;
    fn get_valid(&self, row: u8, column: u8) -> bool;
    fn set(&mut self, row:u8, column: u8, number: u8);
    fn set_valid(&mut self, row:u8, column: u8);

    fn test_chaos(&mut self, row:u8);
    fn make_order(&mut self, row:u8, column:u8);

    fn test_laggers(&mut self, row:u8);
    fn resolve_lagger(&mut self, row:u8, column:u8);

    fn row_valid(&self, row:u8) -> bool;
    fn fill_row(&mut self, row:u8);

    fn lets_go(&mut self);

    fn validate_row(&self, row:u8) -> bool;
    fn validate_column(&self, column:u8) -> bool;
    fn validate_box(&self, row:u8, column:u8) -> bool;
    fn validate_result(&self) -> bool;

    fn as_text(&self);
    fn mask_board(&mut self);
}

impl IsSudokuBoard for Board {
    fn new() -> Board {
        Board {
            cells: Default::default(),
            current_row: 0
        }
    }
    fn zero(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                self.cells[i as usize][j as usize].value = 0;
                self.cells[i as usize][j as usize].valid = false;
            }
        }
    }

    fn check_column(&self, row: u8, column: u8) -> bool {

        let mut result = true;
        for i in 0..9 {
            if i != row && self.cells[i as usize][column as usize].value == self.cells[row as usize][column as usize].value  {
                result = false;
                break;
            }
        }
        result

    }

    fn check_box(&self, row: u8, column: u8) -> bool {

        let mut result = true;

        let mut square = Square::new();
        square.calculate(row, column);

        for i in square.start_row..square.end_row + 1 {
            for j in square.start_column..square.end_column + 1 {
                if i != row && j != column && self.cells[i as usize][j as usize].value == self.cells[row as usize][column as usize].value  {
                    result = false;
                    break;
                }
            }
        }

        result
    }

    fn fits(&self, row: u8, column: u8) -> bool {
        self.check_column(row, column) && self.check_box(row, column)
    }

    fn print(&self) {
        println!("");

        for i in 0..9 {
            // board
            self.print_row(i);
            println!("");
        }

        print!("> ");
        let mut input = String::new();
        //io::stdin().read_line(&mut input).expect("Error reading input");
        println!("{}", input);
    }

    fn print_row(&self, row:u8) {
        for j in 0..9 {
            if self.cells[row as usize][j as usize].value == 0 {
                    print!("|    ");
            } else {
                    print!("| {} ", self.cells[row as usize][j as usize].value);
                if self.cells[row as usize][j as usize].valid {
                    print!(".");
                } else {
                    print!(" ");
                }
            }

        }
        print!("|    ");
    }

    fn get(&self, row: u8, column: u8) -> u8 {
        self.cells[row as usize][column as usize].value
    }

    fn get_valid(&self, row: u8, column: u8) -> bool {
        self.cells[row as usize][column as usize].valid
    }

    fn set(&mut self, row:u8, column: u8, number: u8) {
        self.cells[row as usize][column as usize].value = number;
    }

    fn set_valid(&mut self, row:u8, column: u8){
        self.cells[row as usize][column as usize].valid = true;
    }

    fn test_chaos(&mut self, row:u8) {
        for i in 0..9 {
            if self.fits(row, i){
                self.set_valid(row, i);
            } else {
                self.make_order(row, i);
            }
        }
    }

    fn make_order(&mut self, row:u8, column:u8) {
        // hold original cell value
        let carry = self.get(row, column);
        let mut test = 0;

        // testing of next values
        for i in column +1 .. 9 {
            // put next value into invalid cell
            test = self.get(row, i);
            self.set(row, column, test);

            if self.fits(row, column) {
                self.set_valid(row, column);
                // put invalid value into found cell
                self.set(row, i, carry);
                break;
            } else {
                 // revert value when vaid value not found
                self.set(row, column, carry);
            }
        }
    }

    fn test_laggers(&mut self, row:u8) {
        for i in (0..9).rev() {
            if !self.get_valid(row, i) {
                self.resolve_lagger(row, i);
            }
        }
    }

    fn resolve_lagger(&mut self, row:u8, column:u8) {
        print!("ROW now: ");
        self.print_row(row);
        println!("");
        
        print!("LAGGER at row {} col {} ", row, column);
        // hold original cell value
        let carry = self.get(row, column);
        let mut test = 0;
        print!("of value {} ", carry);

        // testing of next values
        for i in (0..column).rev() {
            print!(":test in {} ", i);
            // put next value into invalid cell
            test = self.get(row, i);
            print!("with {} ", test);
            self.set(row, column, test);

            if self.fits(row, column) {
                print!("fits! ");
                self.set(row, i, carry);


                if self.fits(row, i) {
                    
                    // both fits, so valid them all
                    self.set_valid(row, column);
                    self.set_valid(row, i);
                    // and stop
                    break;
                } else {
                    // revert values into tested cell
                    self.set(row, i, test);
                    self.set(row, column, carry);
                }
            } else {
                    // revert values into tested cell
                    self.set(row, i, test);
                    self.set(row, column, carry);
            }

            print!("ROW after: ");
            self.print_row(row);
            println!("");
        }
    }

    fn row_valid(&self, row:u8) -> bool {
        let mut ok = true;
        for i in 0..9 {
            if !self.get_valid(row, i) {
                ok = false;
                break;
            }
        }
        ok
    }

    fn fill_row(&mut self, row:u8) {
        let mut posible : Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut selected : u8;
        let mut index : u8;
        let mut generator = thread_rng();
        let mut column : u8 = 0;

        while posible.len() > 0 && column < 9 {

            if posible.len() == 1 {
                index = 0;
            } else {
                index = generator.gen_range(0, posible.len()) as u8;
            }

            selected = posible[index as usize];
            self.set(row, column, selected);
            posible.remove(index as usize);

            column += 1;
        }
    }

    fn lets_go(&mut self) {
        let mut cnt = 0;
        self.zero();
        for i in 0..9 {
            println!("");
            println!("");
            println!("ROW {}", i);

            cnt = 0;
            while !self.row_valid(i) && cnt < 10{

            self.fill_row(i);
            //self.print();
            self.test_chaos(i);
            //self.print();
            self.test_laggers(i);
            //self.print();
            cnt += 1;
            }
        }
        println!("");
        self.print();
    }

    fn validate_row(&self, row:u8) -> bool {
        let mut numbers: HashSet<u8> = [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
        let mut test : u8 = 0;

        for i in 0..9 {
            test = self.get(row, i);
            if numbers.contains(&test) {
                numbers.remove(&test);
            }
        }

        numbers.len() == 0
    }

    fn validate_column(&self, column:u8) -> bool {
        let mut numbers: HashSet<u8> = [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
        let mut test : u8 = 0;

        for i in 0..9 {
            test = self.get(i, column);
            if numbers.contains(&test) {
                numbers.remove(&test);
            }
        }

        numbers.len() == 0
    }

    fn validate_box(&self, row:u8, column:u8) -> bool {
        let mut numbers: HashSet<u8> = [1, 2, 3, 4, 5, 6, 7, 8, 9].iter().cloned().collect();
        let mut test : u8 = 0;

        let mut square = Square::new();
        square.calculate(row, column);

        for i in square.start_row..square.end_row + 1 {
            for j in square.start_column..square.end_column + 1 {
                test = self.get(i, j);
                if numbers.contains(&test) {
                    numbers.remove(&test);
                }
            }
        }
        
        numbers.len() == 0
    }

    fn validate_result(&self) -> bool {
        let mut ok = true;

        for i in 0..9 {
            ok = self.validate_row(i);
            if !ok {
                break;
            }
        }

        if ok {
            for i in 0..9 {
                ok = self.validate_column(i);
                if !ok {
                    break;
                }
            }
        }

        if ok {
            for i in (1..8).step_by(3) {
                for j in (1..8).step_by(3) {
                    ok = self.validate_box(i, j);
                    if !ok {
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
        }

        ok
    }

    fn as_text(&self) {

    } // result string

    fn mask_board(&mut self) {
        let mut generator = thread_rng();
        let rank = generator.gen_range(0, 24) as u8 + 17; // r in [17, 41]

        println!("Mask for rank {}", rank);

        let mut visible = HashSet::new();

        while visible.len() < rank as usize {
            let position = generator.gen_range(0, 80) as u8;
            visible.insert(position);
        }

        println!("{:?}", visible);

        let mut count = 0;
        for i in 0..9 {
            for j in 0..9 {
                if !visible.contains(&count) {
                    self.set(i, j, 0);
                }
                count += 1;
            }
        }
    }
}




/*
************************************************* P R I N T E R
*/


pub struct Printer {
    sheet : RgbImage,
    sizex : f32,
    sizey : f32,
    file_name : String,
    file_path : String
}

pub trait Printing {
    fn new() -> Printer;

    // sudoku table printing
    fn print(&mut self, sudoku_board: &Board);
    fn clean_canvas(&mut self);
    fn print_frame(&mut self);
    fn print_grid(&mut self);
    fn print_boxes(&mut self);

    // texts
    fn print_header(&mut self);
    fn print_footer(&mut self);

    // numbers
    fn print_number(&mut self, row:u8, column:u8, number:u8);
    fn print_numbers(&mut self, source: &Board);
    
    // io
    fn set_path(&mut self, new_path: String);
    fn save(&self);
}

impl Printing for Printer {
    fn new() -> Printer {
        Printer {
            sheet : RgbImage::new(880, 880),
            
            sizex : 880.0,
            sizey : 880.0,

            file_name : "".to_string(),
            file_path : "".to_string()
        }
    }

    fn print(&mut self, sudoku_board: &Board) {
        self.file_name = format!("{}{}{}", self.file_path, time_stamp(), ".png");

        self.clean_canvas();
        self.print_frame();
        self.print_grid();
        self.print_boxes();

        self.print_header();
        self.print_footer();
        self.print_numbers(&sudoku_board);
    }

    fn clean_canvas(&mut self) {
        draw_filled_rect_mut(&mut self.sheet, Rect::at(0, 0).of_size(self.sizex as u32, self.sizey as u32), WHITE);
    }

    fn print_frame(&mut self) {
        draw_line_segment_mut(&mut self.sheet, (79f32, 79f32), (801f32, 79f32), BLACK);
        draw_line_segment_mut(&mut self.sheet, (801f32, 79f32), (801f32, 801f32), BLACK);
        draw_line_segment_mut(&mut self.sheet, (79f32, 801f32), (801f32, 801f32), BLACK);
        draw_line_segment_mut(&mut self.sheet, (79f32, 79f32), (79f32, 801f32), BLACK);
    }

    fn print_grid(&mut self) {
        let start_x = 80f32;
        let start_y = 80f32;
        let end_x = self.sizex - start_x;
        let end_y = self.sizey - start_y;

        // horizontal lines
        for i in 0..10 {
            let new_y = start_y + (80f32 * i as f32);
            draw_line_segment_mut(&mut self.sheet, (start_x, new_y), (end_x, new_y), BLACK);
        }

        // vertical lines
        for i in 0..10 {
            let new_x = start_x + (80f32 * i as f32);
            draw_line_segment_mut(&mut self.sheet, (new_x, start_y), (new_x, end_y), BLACK);
        }
    }

    fn print_boxes(&mut self) {
        // vertical
        draw_line_segment_mut(&mut self.sheet, (319f32, 80f32), (319f32, 800f32), BLACK);
        draw_line_segment_mut(&mut self.sheet, (559f32, 80f32), (559f32, 800f32), BLACK);
        
        // horizontal
        draw_line_segment_mut(&mut self.sheet, (80f32, 319f32), (800f32, 319f32), BLACK);
        draw_line_segment_mut(&mut self.sheet, (80f32, 559f32), (800f32, 559f32), BLACK);
    }

    fn print_number(&mut self, row:u8, column:u8, number:u8) {
        let font = Vec::from(include_bytes!("../../assets/fonts/monofonto.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();

        let cx = (80u32 * (column +1) as u32) + 30u32;
        let cy = (80u32 * (row +1) as u32) + 15u32;
        let nr = number.to_string();

        let scale = Scale {
            x: 50.0,
            y: 50.0,
        };
        draw_text_mut(&mut self.sheet, BLACK, cx, cy, scale, &font, &nr);
    }

    fn print_header(&mut self) {
        let font = Vec::from(include_bytes!("../../assets/fonts/monofonto.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();

        let cx = (880u32 - 160u32) / 2u32;
        let cy = 10u32;
        let nr = "Sudoku!".to_string();

        let scale = Scale {
            x: 60.0,
            y: 60.0,
        };
        draw_text_mut(&mut self.sheet, BLACK, cx, cy, scale, &font, &nr);
    }

    fn print_footer(&mut self) {
        let font = Vec::from(include_bytes!("../../assets/fonts/monofonto.ttf") as &[u8]);
        let font = FontCollection::from_bytes(font)
            .unwrap()
            .into_font()
            .unwrap();

        let mut cx = (880u32 - 160u32) / 2u32;
        let mut cy = (880u32 - 60u32);
        let mut footnote = format!("{}", "@sudoku_break");

        let mut scale = Scale {
            x: 30.0,
            y: 30.0,
        };
        draw_text_mut(&mut self.sheet, BLACK, cx, cy, scale, &font, &footnote);

        footnote = format!("{}", perfect_date());
        cx = (880u32 - 60u32) / 2u32;
        cy = (880u32 - 30u32);
        scale.x = 15.0;
        scale.y = 15.0;
        draw_text_mut(&mut self.sheet, BLACK, cx, cy, scale, &font, &footnote);
    }

    fn print_numbers(&mut self, source:&Board) {
        for i in 0..9 {
            for j in 0..9 {
                let number = source.get(i,j);

                if number > 0 {
                    self.print_number(i, j, number);
                }
            }
        }
    }

    fn set_path(&mut self, new_path: String) {
        self.file_path = new_path;
    }

    fn save(&self) {
        self.sheet.save(self.file_name.clone()).expect("Oh.. Can not save sudoku table printing...");
    }
}