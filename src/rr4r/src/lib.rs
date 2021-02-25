use extendr_api::prelude::*;
use regex::Regex;

/// Detect the presence or absence of a pattern in a string.
/// @param x
///   A character vector to look for the patterns in.
/// @param pattern
///   A pattern to look for.
/// @examples
/// fruit <- c("apple", "banana", "pear", "pinapple")
/// rr4r_detect(fruit, "a")
/// rr4r_detect(fruit, "^a")
/// rr4r_detect(fruit, "a$")
/// rr4r_detect(fruit, "b")
/// rr4r_detect(fruit, "[aeiou]")
/// @export
#[extendr]
fn rr4r_detect(x: Vec<String>, pattern: String) -> Vec<bool> {
    let re = Regex::new(&pattern).unwrap();
    x.iter().map(|e| re.is_match(e)).collect()
}

// Macro to generate exports
extendr_module! {
    mod rr4r;
    fn rr4r_detect;
}
