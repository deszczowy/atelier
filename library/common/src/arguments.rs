use std::env;

pub fn is_one_shot() -> bool {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        args[1].to_lowercase() == "one".to_string()
    } else {
        false
    }
}