
use painting::*;

fn main() {
    
    let width = 800 as u32;
    let height = 350 as u32;

    let choice = 2 as u8;

    if (choice == 1) {
        use circles::*;

        let mut c = Painting::new(width, height);
        c.initialize();
        c.generate();
        c.put_a_frame(20, painting::BLACK);
        c.save_file("test.png".to_string());
    }

    if (choice == 2) {
        use stripes::*;

        let mut c = Painting::new(width, height);
        c.initialize();
        c.generate();
        c.put_a_frame(5, painting::BLACK);
        c.save_file("test.png".to_string());
    }
    

}
