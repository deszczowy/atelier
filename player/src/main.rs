use http::*;
use crawler::*;

fn main() {
    let mut page = http::get("http://google.pl".to_string());
    let mut links = crawler::scrab_links(page.as_str());

    println!("{:?}", links);
    println!("Done!");
}
