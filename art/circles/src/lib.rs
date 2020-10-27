use random::*;
use painting::*;

pub trait Circles {
    fn generate(&mut self);

    fn caclulate_center(&mut self, circle: &mut painting::Circle);
    fn caclulate_radius(&mut self, circle: &mut painting::Circle, divider: i32);
    fn calculate_colors(&mut self, circle: &mut painting::Circle, palette: i32, color_pick_pattern: i32);
    fn average_colors_sqrt(&mut self, part_one: u8, part_two: u8) -> u8;
    fn average_colors_line(&mut self, part_one: u8, part_two: u8) -> u8;
    fn average_colors(&mut self, part_one: u8, part_two: u8, color_pick_pattern: i32) -> u8;

    fn calculate_palete_with_blue_wildcard(&mut self, circle: &mut painting::Circle, color_pick_pattern: i32);
    fn calculate_palete_with_green_wildcard(&mut self, circle: &mut painting::Circle, color_pick_pattern: i32);
    fn calculate_palete_with_red_wildcard(&mut self, circle: &mut painting::Circle, color_pick_pattern: i32);
}

impl Circles for Painting {

    fn generate(&mut self) {

        let palette = self.randomizer.spit(3);
        let color_pick = self.randomizer.spit(2);
        let divider = self.randomizer.spit_range(2, 4);

        println!("Palette {}, Divider {}", palette, divider);
        
        match color_pick {
            0 => { println!("Color pick method - Square root"); }
            1 => { println!("Color pick method - Line"); }
            _ => { println!("Color pick method - Unknown")}
        };

        let mut circle = painting::Circle{
            x0: 0,
            y0: 0,
            red: 0,
            green: 0,
            blue: 0,
            radius: 0
        };

        for _i in 0..100 {
            self.caclulate_center(&mut circle);
            self.calculate_colors(&mut circle, palette, color_pick);
            self.caclulate_radius(&mut circle, divider);
            self.draw_circle(&circle);
        }
    }

    fn calculate_colors(&mut self, circle: &mut painting::Circle, palette: i32, color_pick_pattern: i32) {
        match palette {
            0 => { self.calculate_palete_with_blue_wildcard(circle, color_pick_pattern); },
            1 => { self.calculate_palete_with_green_wildcard(circle, color_pick_pattern); },
            2 => { self.calculate_palete_with_red_wildcard(circle, color_pick_pattern); },
            _ => {}
        }
    } 

    fn calculate_palete_with_blue_wildcard(&mut self, circle: &mut painting::Circle, color_pick_pattern: i32) {
        circle.red = self.randomizer.spit(255) as u8;
        circle.green = self.randomizer.spit(255) as u8;

        circle.blue = self.average_colors(circle.red, circle.green, color_pick_pattern);
    }

    fn calculate_palete_with_green_wildcard(&mut self, circle: &mut painting::Circle, color_pick_pattern: i32) {
        circle.red = self.randomizer.spit(255) as u8;
        circle.blue = self.randomizer.spit(255) as u8;

        circle.green = self.average_colors(circle.red, circle.blue, color_pick_pattern);
    }

    fn calculate_palete_with_red_wildcard(&mut self, circle: &mut painting::Circle, color_pick_pattern: i32) {
        circle.blue = self.randomizer.spit(255) as u8;
        circle.green = self.randomizer.spit(255) as u8;

        circle.red = self.average_colors(circle.blue, circle.green, color_pick_pattern);
    }

    fn average_colors_sqrt(&mut self, part_one: u8, part_two: u8) -> u8 {
        ((part_one as f64).sqrt() + (part_two as f64).sqrt() / 2.0) as u8
    }

    fn average_colors_line(&mut self, part_one: u8, part_two: u8) -> u8 {
        ((part_one as u32 + part_two as u32) / 2) as u8
    }

    fn average_colors(&mut self, part_one: u8, part_two: u8, color_pick_pattern: i32) -> u8 {
        match color_pick_pattern {
            0 => { self.average_colors_sqrt(part_one, part_two) }
            1 => { self.average_colors_line(part_one, part_two) }
            _ => { color_pick_pattern as u8 }
        }
    }

    fn caclulate_center(&mut self, circle: &mut painting::Circle) {
        circle.x0 = self.randomizer.spit(self.width as i32);
        circle.y0 = self.randomizer.spit(self.height as i32);
    }

    fn caclulate_radius(&mut self, circle: &mut painting::Circle, divider: i32) {
        let seed = (self.width / (divider as u32)) as i32;
        circle.radius = self.randomizer.spit(seed);
    }
}
