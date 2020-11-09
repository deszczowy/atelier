use random::*;
use painting::*;

//-- Ornament data 
pub struct OrnamentData {
    pub horizontal_margin_height: u32,
    pub vertical_margin_width: u32,
    pub axis_x_range: u32,
    pub axis_y_range: u32
}

pub trait OrnametCalculation {
    fn caclulate_axes(&mut self, canvas_width: u32, canvas_height: u32);
}

impl OrnametCalculation for OrnamentData {
    fn caclulate_axes(&mut self, canvas_width: u32, canvas_height: u32) {
        self.axis_x_range = canvas_width - (2u32 * self.vertical_margin_width);
        self.axis_y_range = canvas_height - (2u32 * self.horizontal_margin_height);    
    }
}

//-- Ornament printing

pub trait Ornament {
    fn generate(&mut self);
}

impl Ornament for Painting {
    fn generate(&mut self) {

        let mut ornament = OrnamentData {
            horizontal_margin_height: 40u32,
            vertical_margin_width: 40u32,
            axis_x_range: 0u32,
            axis_y_range: 0u32
        };

        ornament.caclulate_axes(self.width, self.height);

        // canvas preparation
        
        
        let yplus = ornament.axis_y_range / 2;

        // bc print
        let bc_step = 3u8;
        let high_color_part = self.randomizer.spit(3);
        let mut color = TheColor{
            r : self.randomizer.spit_range(200, 250) as u8,
            g : self.randomizer.spit_range(200, 250) as u8,
            b : self.randomizer.spit_range(200, 250) as u8
        };

        let bcx = self.randomizer.spit_range(ornament.vertical_margin_width as i32, (self.width - ornament.vertical_margin_width as u32) as i32);
        let bcy = self.randomizer.spit_range(ornament.horizontal_margin_height as i32, (self.height - ornament.horizontal_margin_height) as i32);

        let mut triangle = Triangle{
            p0: ThePoint {x: 0, y: 0 },
            p1: ThePoint {x: self.width as i32, y: 0},
            p2: ThePoint {x: bcx, y: bcy},
            red: color.r,
            green: color.g,
            blue: color.b
        };
        self.draw_triangle(&triangle);

        triangle.p0.x = self.width as i32;
        triangle.p0.y = 0;
        triangle.p1.x = self.width as i32;
        triangle.p1.y = self.height as i32;
        triangle.red -= bc_step;
        triangle.green -= bc_step;
        triangle.blue -= bc_step;
        self.draw_triangle(&triangle);

        triangle.p0.x = self.width as i32;
        triangle.p0.y = self.height as i32;
        triangle.p1.x = 0;
        triangle.p1.y = self.height as i32;
        triangle.red -= bc_step;
        triangle.green -= bc_step;
        triangle.blue -= bc_step;
        self.draw_triangle(&triangle);

        triangle.p0.x = 0;
        triangle.p0.y = self.height as i32;
        triangle.p1.x = 0;
        triangle.p1.y = 0;
        triangle.red -= bc_step;
        triangle.green -= bc_step;
        triangle.blue -= bc_step;
        self.draw_triangle(&triangle);

        // main print
        color.r = if high_color_part == 0 {self.randomizer.spit_range(200, 250) as u8} else {self.randomizer.spit(100) as u8};
        color.g = if high_color_part == 1 {self.randomizer.spit_range(200, 250) as u8} else {self.randomizer.spit(100) as u8};
        color.b = if high_color_part == 2 {self.randomizer.spit_range(200, 250) as u8} else {self.randomizer.spit(100) as u8};
    
        let step = 20u8;
        let step_high = 1u8;

        //self.fill_canvas(color.r, color.g, color.b);
        //self.fill_canvas(255-color.r, 255-color.g, 255-color.b);

        let thickness = 8u32;
        

        println!("w{} h{}  vm{} hm{}  x{} y{} +y{}", self.width, self.height, ornament.vertical_margin_width, ornament.horizontal_margin_height, ornament.axis_x_range, ornament.axis_y_range, yplus);
        
        // random points
        let mut random_points_counter = 0u8;
        let mut random_range_bottom = 0u32;
        let mut random_range_step = (ornament.axis_x_range / 8u32) as u32; 
        let mut random_range_top = random_range_step;
        let mut random_points = Vec::new();

         random_points.push(
             ThePointF{ 
                 x: 0 as f32, 
                 y: 0 as f32
             }
         );

        for i in 1..9 {

            random_points.push(
                ThePointF{ 
                    x: i as f32, 
                    y: self.randomizer.spit_range_f(-1.0, 1.0) as f32
                }
            );

        }

         random_points.push(
             ThePointF{ 
                 x: 9 as f32, 
                 y: 0 as f32
             }
         );

        for rp in &random_points {
            println!("{:?}", rp);
        }

        // polynome
        let mut function = Vec::new();

        let mut x = 0f32;
        let mut sum : f32;

        let x_mult = ((ornament.axis_x_range  as i32) / 9i32) as f32;
        let y_mult = ((ornament.axis_y_range  as i32) / 5i32) as f32;
        
        while x < 9 as f32 {
        
            sum = 0f32;
            
            for p in &random_points {
                //println!("{:?}", p);
                
                let mut coef = 1f32;
                
                for pj in &random_points {
                    if pj.x != p.x {
                        coef *= (x - pj.x) / (p.x - pj.x);
                    }
                }
                
                sum += p.y * coef;
            }

            let mut proposed_y = ((sum * y_mult) + yplus as f32 + ornament.horizontal_margin_height as f32) as i32;

            if proposed_y < ornament.horizontal_margin_height as i32 {
                proposed_y = ornament.horizontal_margin_height as i32;
            } else
            if proposed_y > (ornament.horizontal_margin_height + ornament.axis_y_range) as i32 {
                proposed_y = (ornament.horizontal_margin_height + ornament.axis_y_range) as i32;
            }
            
            function.push(
                ThePoint{
                    x: ((x * x_mult) + ornament.vertical_margin_width as f32) as i32, 
                    y: proposed_y
                }
            );
            
            x += 0.1f32;
        }
        
        for d in 0..10 {
            println!("{:?}", &function[d]);
        }
        println!("...");
        for d in function.len() -20..function.len() {
            println!("{:?}", &function[d]);
        }

        // printing 
        let start_index = 10;
        let end_index = function.len() -10;

        // print normal ->
        color.r += if high_color_part == 0 {step_high} else {step};
        color.g += if high_color_part == 1 {step_high} else {step};
        color.b += if high_color_part == 2 {step_high} else {step};

        println!("n ->");
        let mut v0 = &function[start_index];
        let mut v1 = &function[start_index +1];
        for i in start_index +2..end_index +1 {
            self.draw_line(&v0, &v1, &color, thickness);
            v0 = v1;
            v1 = &function[i];
        }

        // print normal <-
        color.r += if high_color_part == 0 {step_high} else {step};
        color.g += if high_color_part == 1 {step_high} else {step};
        color.b += if high_color_part == 2 {step_high} else {step};
        
        println!("n <-");
        println!("{}", &function.len());

        let mut i = start_index;
        let mut j = end_index;

        while i < end_index && j > 0 {

            let x0 = &function[i].x;
            let y0 = &function[j].y;
            let x1 = &function[i+1].x;
            let y1 = &function[j-1].y;

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            //println!("f[{:?}] v0{:?} f[{:?}] v1{:?}", i, v0, j, v1);

            self.draw_line(&v0, &v1, &color, thickness);

            i += 1;
            j -= 1;        
        }

        
        // print rev ->
        color.r += if high_color_part == 0 {step_high} else {step};
        color.g += if high_color_part == 1 {step_high} else {step};
        color.b += if high_color_part == 2 {step_high} else {step};
        
        println!("rev ->");
        let mut i = start_index;
        while i < end_index {
            let x0 = &function[i].x;
            let y0 = (2 * (yplus + ornament.horizontal_margin_height)) as i32 - &function[i].y.clone();
            let x1 = &function[i+1].x;
            let y1 = (2 * (yplus + ornament.horizontal_margin_height)) as i32 - &function[i+1].y.clone();

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            self.draw_line(&v0, &v1, &color, thickness);

            i += 1;
        }

        // print rev <-
        color.r += if high_color_part == 0 {step_high} else {step};
        color.g += if high_color_part == 1 {step_high} else {step};
        color.b += if high_color_part == 2 {step_high} else {step};
        
        println!("n <-");
        println!("{}", &function.len());

        let mut i = start_index;
        let mut j = end_index;

        while i < end_index && j > 0 {

            let x0 = &function[i].x;
            let y0 = (2 * (yplus + ornament.horizontal_margin_height)) as i32 - &function[j].y.clone();
            let x1 = &function[i+1].x;
            let y1 = (2 * (yplus + ornament.horizontal_margin_height)) as i32 - &function[j-1].y.clone();

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            //println!("f[{:?}] v0{:?} f[{:?}] v1{:?}", i, v0, j, v1);

            self.draw_line(&v0, &v1, &color, thickness);

            i += 1;
            j -= 1;        
        }

    }
}