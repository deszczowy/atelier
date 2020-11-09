use random::*;
use painting::*;

//-- Ornament data 
pub struct OrnamentData {
    pub horizontal_margin_height: u32,
    pub vertical_margin_width: u32,
    pub axis_x_range: u32,
    pub axis_y_range: u32,
    pub axis_y_positive_part: u32,
    pub line_thickness: u32
}

pub trait OrnametCalculation {
    fn caclulate_axes(&mut self, canvas_width: u32, canvas_height: u32);
}

impl OrnametCalculation for OrnamentData {
    fn caclulate_axes(&mut self, canvas_width: u32, canvas_height: u32) {
        self.axis_x_range = canvas_width - (2u32 * self.vertical_margin_width);
        self.axis_y_range = canvas_height - (2u32 * self.horizontal_margin_height); 
        self.axis_y_positive_part = self.axis_y_range / 2u32;
    }
}

//-- Ornament printing
pub trait Ornament {
    fn raise_color_by_step(&self, color: &mut TheColor, step: u8);
    fn fall_color_by_step(&self, color: &mut TheColor, step: u8);
    fn print_background(&mut self, ornament: &OrnamentData);
    fn print_background_triangles(&mut self, list: &mut Vec<i32>, triangle: &mut Triangle, color: &mut TheColor);
    fn generate_random_nodes(&mut self, points: &mut Vec<ThePointF>);
    fn generate(&mut self);
}

impl Ornament for Painting {
    fn raise_color_by_step(&self, color: &mut TheColor, step: u8) {
        color.r += step;
        color.g += step;
        color.b += step;
    }
    fn fall_color_by_step(&self, color: &mut TheColor, step: u8) {
        color.r -= step;
        color.g -= step;
        color.b -= step;
    }

    fn print_background_triangles(&mut self, list: &mut Vec<i32>, triangle: &mut Triangle, color: &mut TheColor) {
        let bc_step = 3u8;

        while list.len() > 0 && list.len() % 4 == 0 {
            triangle.p0.x = list.remove(0);
            triangle.p0.y = list.remove(0);
            triangle.p1.x = list.remove(0);
            triangle.p1.y = list.remove(0);

            self.fall_color_by_step(color, bc_step);
            self.draw_triangle(&triangle, &color);
        }
    }

    fn print_background(&mut self, ornament: &OrnamentData) {

        let mut color = TheColor{
            r : self.randomizer.spit_range(200, 250) as u8,
            g : self.randomizer.spit_range(200, 250) as u8,
            b : self.randomizer.spit_range(200, 250) as u8
        };

        let bcx = self.randomizer.spit_range(
            ornament.vertical_margin_width as i32, 
            (self.width - ornament.vertical_margin_width) as i32
        );
        let bcy = self.randomizer.spit_range(
            ornament.horizontal_margin_height as i32, 
            (self.height - ornament.horizontal_margin_height) as i32
        );

        let mut triangle = Triangle{
            p0: ThePoint {x: 0, y: 0 },
            p1: ThePoint {x: 0, y: 0},
            p2: ThePoint {x: bcx, y: bcy},
        };

        let mut bv = Vec::new();

        bv.extend([0i32, 0i32, self.width as i32, 0i32].iter().copied());
        bv.extend([self.width as i32, 0i32, self.width as i32, self.height as i32].iter().copied());
        bv.extend([self.width as i32, self.height as i32, 0i32, self.height as i32].iter().copied());
        bv.extend([0i32, self.height as i32, 0i32, 0i32].iter().copied());
        
        self.print_background_triangles(&mut bv, &mut triangle, &mut color);
    }

    fn generate_random_nodes(&mut self, points: &mut Vec<ThePointF>) {
        points.push(ThePointF{x:0.0, y:0.0}); // first node at zero
        for i in 1..9 {
            points.push(
                ThePointF{ 
                    x: i as f32, 
                    y: self.randomizer.spit_range_f(-1.0, 1.0) as f32
                }
            );
        }
        points.push(ThePointF{x: 9.0, y:0.0}); // last node also at zero
    }

    fn generate(&mut self) {

        let mut nodes = Vec::new();

        let mut ornament = OrnamentData {
            horizontal_margin_height: 40u32,
            vertical_margin_width: 40u32,
            axis_x_range: 0u32,
            axis_y_range: 0u32,
            axis_y_positive_part: 0u32,
            line_thickness: 8u32
        };

        ornament.caclulate_axes(self.width, self.height);
        self.print_background(&ornament);

        let high_color_part = self.randomizer.spit(3);
        // main print
        let mut color = TheColor {
            r : if high_color_part == 0 {self.randomizer.spit_range(200, 250) as u8} else {self.randomizer.spit(100) as u8},
            g : if high_color_part == 1 {self.randomizer.spit_range(200, 250) as u8} else {self.randomizer.spit(100) as u8},
            b : if high_color_part == 2 {self.randomizer.spit_range(200, 250) as u8} else {self.randomizer.spit(100) as u8}
        };
    
        let step = 20u8;
        let step_high = 1u8;

        println!("w{} h{}  vm{} hm{}  x{} y{} +y{}", self.width, self.height, ornament.vertical_margin_width, ornament.horizontal_margin_height, ornament.axis_x_range, ornament.axis_y_range, ornament.axis_y_positive_part);
        
        // random points
        self.generate_random_nodes(&mut nodes);
        
        for rp in &nodes {
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
            
            for p in &nodes {
                //println!("{:?}", p);
                
                let mut coef = 1f32;
                
                for pj in &nodes {
                    if pj.x != p.x {
                        coef *= (x - pj.x) / (p.x - pj.x);
                    }
                }
                
                sum += p.y * coef;
            }

            let mut proposed_y = ((sum * y_mult) + ornament.axis_y_positive_part as f32 + ornament.horizontal_margin_height as f32) as i32;

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
            self.draw_line(&v0, &v1, &color, ornament.line_thickness);
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

            self.draw_line(&v0, &v1, &color, ornament.line_thickness);

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
            let y0 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &function[i].y.clone();
            let x1 = &function[i+1].x;
            let y1 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &function[i+1].y.clone();

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            self.draw_line(&v0, &v1, &color, ornament.line_thickness);

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
            let y0 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &function[j].y.clone();
            let x1 = &function[i+1].x;
            let y1 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &function[j-1].y.clone();

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            //println!("f[{:?}] v0{:?} f[{:?}] v1{:?}", i, v0, j, v1);

            self.draw_line(&v0, &v1, &color, ornament.line_thickness);

            i += 1;
            j -= 1;        
        }

    }
}