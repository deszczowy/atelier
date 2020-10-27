use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub fn scrab_links(text: &str) -> HashSet<&str>{
    lazy_static! {
        static ref URL_REGEX : Regex = Regex::new(
                r"(http|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-])?"
            ).unwrap();
    }
    URL_REGEX.find_iter(text).map(|mat| mat.as_str()).collect()
}