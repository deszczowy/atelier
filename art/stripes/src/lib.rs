use random::*;
use painting::*;

pub struct Four {
    x0: i32,
    x1: i32,
    x2: i32,
    x3: i32
}

pub trait Stripes {
    fn generate(&mut self);
    fn fill_convex(&self, packed: &mut Four, st:i32, sb:i32, dt:i32, db:i32);
    fn fill_crossed(&self, packed: &mut Four, st:i32, sb:i32, dt:i32, db:i32);
}

impl Stripes for Painting {
    fn generate(&mut self) {

        // statring range for top line 
        let d0 = (self.width / 3) as i32;
        let d1 = ((2f32 / 3f32) * (self.width as f32)) as i32;

        //println!("(d0, d1) = ({}, {})", d0, d1);

        let mut going_up = false;

        // selecting filling model
        // 0 - all crossing
        // 1 - all convex
        // 2 - mixed convex up
        // 3 - mixed convex down
        let model = self.randomizer.spit(4);

        match model {
            0 => { println!("Model static - all crossed"); },
            1 => { println!("Model static - all convex"); }
            2 => { println!("Model mixed - convex up"); }
            3 => { println!("Model mixed - convex down"); }
            _ => { println!("Model unknown"); }
        }

        // colors initialization
        let color_part_top = self.randomizer.spit_range(40, 100) as i32;
        let mut c_red: u8;
        let mut c_green: u8;
        let mut c_blue: u8;

        // direction of color progression. 
        // up - from dark to light
        // down - from light to dark
        // in this selection color_part_top is top or bottom of the color scale
        let direction_up = self.randomizer.spit(2) == 1;

        if direction_up {
            println!("Color progression - up");
            c_red = self.randomizer.spit(color_part_top) as u8;
            c_green = self.randomizer.spit(color_part_top) as u8;
            c_blue = self.randomizer.spit(color_part_top) as u8;
        } else {
            println!("Color progression - down");
            c_red = self.randomizer.spit_range(color_part_top, 255) as u8;
            c_green = self.randomizer.spit_range(color_part_top, 255) as u8;
            c_blue = self.randomizer.spit_range(color_part_top, 255) as u8;
        }

        println!("Base color - RGB({}, {}, {})", c_red, c_green, c_blue);

        let c_red_step = self.randomizer.spit(5) as u8;
        let c_green_step = self.randomizer.spit(5) as u8;
        let c_blue_step = self.randomizer.spit(5) as u8;

        self.fill_canvas(c_red, c_green, c_blue);

        let mut d_top = self.randomizer.spit_range(d0, d1);
        let mut d_bottom = 1i32;
        let mut shift_top = 1i32;
        let mut shift_bottom = 1i32;

        let mut pack = Four {
            x0: 0,
            x1: 0,
            x2: 0,
            x3: 0
        };

        let mut counter = 30;

        while d_top > 0 && d_bottom > 0 && counter > 0{

            if going_up {
                d_top = d_bottom -1;
                shift_top = self.randomizer.spit(self.width as i32 - d_top);
            } else {
                d_bottom = d_top -1;
                shift_bottom = self.randomizer.spit(self.width as i32 - d_bottom);
            }

            if direction_up {
                c_red += c_red_step;
                c_green += c_green_step;
                c_blue += c_blue_step;
            } else {
                c_red -= c_red_step;
                c_green -= c_green_step;
                c_blue -= c_blue_step;
            }

            //println!("dt = {}, db = {}", d_top, d_bottom);
            //println!("st = {}, sb = {}", shift_top, shift_bottom);
            
            if (model == 0) || (going_up && model == 2) || (!going_up && model == 3) {
                self.fill_convex(&mut pack, shift_top, shift_bottom, d_top, d_bottom);
            } 
            else 
            if (model == 1) || (going_up && model == 3) || (!going_up && model == 2) {
                self.fill_crossed(&mut pack, shift_top, shift_bottom, d_top, d_bottom);
            }

            let p_0 = painting::ThePoint{ x: pack.x0, y: 0 }; // top border
            let p_1 = painting::ThePoint{ x: pack.x1, y: 0 };
            let p_2 = painting::ThePoint{ x: pack.x2, y: self.height as i32}; // bottom border
            let p_3 = painting::ThePoint{ x: pack.x3, y: self.height as i32};

            going_up = !going_up;
            counter -= 1;

            let stripe = painting::Trapezoid{
                p0: p_0,
                p1: p_1,
                p2: p_2,
                p3: p_3,
                red: c_red,
                green: c_green,
                blue: c_blue
            };
    
            self.draw_trapezoid(&stripe);
        }
    }

    fn fill_convex(&self, packed: &mut Four, st:i32, sb:i32, dt:i32, db:i32){
        packed.x0 = st;
        packed.x1 = st + dt;
        packed.x2 = sb;
        packed.x3 = sb + db;
    }

    fn fill_crossed(&self, packed: &mut Four, st:i32, sb:i32, dt:i32, db:i32) {
        packed.x0 = st;
        packed.x1 = st + dt;
        packed.x2 = sb + db;
        packed.x3 = sb;
    }
}