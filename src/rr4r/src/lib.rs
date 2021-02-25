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
fn rr4r_detect(x: Robj, pattern: String) -> Vec<Bool> {
    if x.is_na() {
        return vec![NA_LOGICAL];
    }
    let re = Regex::new(&pattern).unwrap();

    let x_str_iter = x.as_str_iter().unwrap();
    x_str_iter
        .map(|s| {
            if s.is_na() {
                NA_LOGICAL
            } else {
                re.is_match(&s).into()
            }
        })
        .collect::<Vec<_>>()
}

// Macro to generate exports
extendr_module! {
    mod rr4r;
    fn rr4r_detect;
}
