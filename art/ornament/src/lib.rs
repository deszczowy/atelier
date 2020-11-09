use random::*;
use painting::*;

//-- Ornament data 
pub struct OrnamentData {
    pub horizontal_margin_height: u32,
    pub vertical_margin_width: u32,
    pub axis_x_range: u32,
    pub axis_y_range: u32,
    pub axis_y_positive_part: u32,
    pub line_thickness: u32,

    pub main_color: TheColor,
    pub high_color_part: u8,
    pub main_color_step: u8,
    pub high_color_step: u8,

    pub plot: Vec<ThePoint>,
    pub plot_start: usize,
    pub plot_end: usize,
}

pub trait OrnametCalculation {
    fn caclulate_axes(&mut self, canvas_width: u32, canvas_height: u32);
    fn generate_main_color(&mut self, randomizer: &mut random::RandomGenerator);
    fn bump_color(&mut self);
    fn calculate_bound_points(&mut self);
    fn calculate_polynomial(&mut self, points: &Vec<ThePointF>);
}

impl OrnametCalculation for OrnamentData {
    fn caclulate_axes(&mut self, canvas_width: u32, canvas_height: u32) {
        self.axis_x_range = canvas_width - (2u32 * self.vertical_margin_width);
        self.axis_y_range = canvas_height - (2u32 * self.horizontal_margin_height); 
        self.axis_y_positive_part = self.axis_y_range / 2u32;
    }

    fn generate_main_color(&mut self, randomizer: &mut random::RandomGenerator) {
        self.main_color.r = if self.high_color_part == 0 {randomizer.spit_range(200, 250) as u8} else {randomizer.spit(100) as u8};
        self.main_color.g = if self.high_color_part == 1 {randomizer.spit_range(200, 250) as u8} else {randomizer.spit(100) as u8};
        self.main_color.b = if self.high_color_part == 2 {randomizer.spit_range(200, 250) as u8} else {randomizer.spit(100) as u8};
    }

    fn bump_color(&mut self) {
        self.main_color.r += if self.high_color_part == 0 {self.high_color_step} else {self.main_color_step};
        self.main_color.g += if self.high_color_part == 1 {self.high_color_step} else {self.main_color_step};
        self.main_color.b += if self.high_color_part == 2 {self.high_color_step} else {self.main_color_step};
    }

    fn calculate_bound_points(&mut self) {
        self.plot_start = 10;
        self.plot_end = self.plot.len() -10;
    }

    fn calculate_polynomial(&mut self, points: &Vec<ThePointF>){

        let mut x = 0f32;
        let mut sum : f32;

        let x_multiplier = (self.axis_x_range / 9u32) as f32;
        let y_multiplier = (self.axis_y_range / 5u32) as f32;

        while x < 9 as f32 {

            sum = 0.0;
            
            for n in points {                
                let mut coef : f32 = 1.0;
                
                for pj in points {
                    if pj.x != n.x {
                        coef *= (x - pj.x) / (n.x - pj.x);
                    }
                }
                
                sum += n.y * coef;
            }

            let mut proposed_y = (
                (sum * y_multiplier) + 
                (self.axis_y_positive_part + self.horizontal_margin_height) as f32
            ) as u32;

            if proposed_y < self.horizontal_margin_height {
                proposed_y = self.horizontal_margin_height;
            } else
            if proposed_y > (self.horizontal_margin_height + self.axis_y_range) {
                proposed_y = (self.horizontal_margin_height + self.axis_y_range);
            }
            
            self.plot.push(
                ThePoint{
                    x: ((x * x_multiplier) + self.vertical_margin_width as f32) as i32, 
                    y: proposed_y as i32
                }
            );
            
            x += 0.1f32;
        }
    }
}

//-- Ornament printing
pub trait Ornament {
    fn raise_color_by_step(&self, color: &mut TheColor, step: u8);
    fn fall_color_by_step(&self, color: &mut TheColor, step: u8);
    fn print_background(&mut self, ornament: &OrnamentData);
    fn print_background_triangles(&mut self, list: &mut Vec<i32>, triangle: &mut Triangle, color: &mut TheColor);
    fn generate_random_nodes(&mut self, points: &mut Vec<ThePointF>);
    fn print_plot_normal(&mut self, plot: &Vec<ThePoint>, ornament: &OrnamentData);
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

    fn print_plot_normal(&mut self, plot: &Vec<ThePoint>, ornament: &OrnamentData) {

    }

    fn generate(&mut self) {

        let mut nodes = Vec::new();
        
        let mut ornament = OrnamentData {
            horizontal_margin_height: 40u32,
            vertical_margin_width: 40u32,
            axis_x_range: 0u32,
            axis_y_range: 0u32,
            axis_y_positive_part: 0u32,
            line_thickness: 8u32,

            main_color: TheColor{r:0, g:0, b:0},
            high_color_part: self.randomizer.spit(3) as u8,
            main_color_step: 20u8,
            high_color_step: 1u8, 

            plot: Vec::new(),
            plot_start: 0,
            plot_end: 0
        };

        ornament.caclulate_axes(self.width, self.height);
        ornament.generate_main_color(&mut self.randomizer);

        self.print_background(&ornament);
        self.generate_random_nodes(&mut nodes);
        ornament.calculate_polynomial(&nodes);
        ornament.calculate_bound_points();
        
        println!("w{} h{}  vm{} hm{}  x{} y{} +y{}", self.width, self.height, ornament.vertical_margin_width, ornament.horizontal_margin_height, ornament.axis_x_range, ornament.axis_y_range, ornament.axis_y_positive_part);
        
        ornament.bump_color();
        let mut v0 = &ornament.plot[ornament.plot_start];
        let mut v1 = &ornament.plot[ornament.plot_start +1];
        for i in ornament.plot_start +2..ornament.plot_end +1 {
            self.draw_line(&v0, &v1, &ornament.main_color, ornament.line_thickness);
            v0 = v1;
            v1 = &ornament.plot[i];
        }


        ornament.bump_color();
        let mut i = ornament.plot_start;
        let mut j = ornament.plot_end;
        while i < ornament.plot_end && j > 0 {

            let x0 = &ornament.plot[i].x;
            let y0 = &ornament.plot[j].y;
            let x1 = &ornament.plot[i+1].x;
            let y1 = &ornament.plot[j-1].y;

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            self.draw_line(&v0, &v1, &ornament.main_color, ornament.line_thickness);

            i += 1;
            j -= 1;        
        }

        
        ornament.bump_color();
        let mut i = ornament.plot_start;
        while i < ornament.plot_end {
            let x0 = &ornament.plot[i].x;
            let y0 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &ornament.plot[i].y.clone();
            let x1 = &ornament.plot[i+1].x;
            let y1 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &ornament.plot[i+1].y.clone();

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            self.draw_line(&v0, &v1, &ornament.main_color, ornament.line_thickness);

            i += 1;
        }

        
        ornament.bump_color();
        let mut i = ornament.plot_start;
        let mut j = ornament.plot_end;

        while i < ornament.plot_end && j > 0 {

            let x0 = &ornament.plot[i].x;
            let y0 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &ornament.plot[j].y.clone();
            let x1 = &ornament.plot[i+1].x;
            let y1 = (2 * (ornament.axis_y_positive_part + ornament.horizontal_margin_height)) as i32 - &ornament.plot[j-1].y.clone();

            let v0 = ThePoint{ x: x0.clone(), y: y0.clone()};
            let v1 = ThePoint{ x: x1.clone(), y: y1.clone()};

            //println!("f[{:?}] v0{:?} f[{:?}] v1{:?}", i, v0, j, v1);

            self.draw_line(&v0, &v1, &ornament.main_color, ornament.line_thickness);

            i += 1;
            j -= 1;        
        }

    }
}