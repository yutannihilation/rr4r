use extendr_api::prelude::*;
use regex::Regex;

/// Detect the presence or absence of a pattern in a string.
///
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
                return NA_LOGICAL;
            }
            re.is_match(&s).into()
        })
        .collect::<Vec<_>>()
}

/// Extract matching patterns from a string.
///
/// @param x
///   A character vector to look for the patterns in.
/// @param pattern
///   A pattern to look for.
/// @examples
/// shopping_list <- c("apples x4", "bag of flour", "bag of sugar", "milk x2")
/// rr4r_extract(shopping_list, "\\d")
/// rr4r_extract(shopping_list, "[a-z]+")
/// rr4r_extract(shopping_list, "[a-z]{1,4}")
/// rr4r_extract(shopping_list, "\\b[a-z]{1,4}\\b")
/// @export
#[extendr]
fn rr4r_extract(x: Robj, pattern: String) -> Vec<Option<&'static str>> {
    if x.is_na() {
        return vec![NA_STRING];
    }
    let re = Regex::new(&pattern).unwrap();

    let x_str_iter = x.as_str_iter().unwrap();
    x_str_iter
        .map(|s| {
            if s.is_na() {
                return NA_STRING;
            }
            if let Some(cap) = re.captures(&s) {
                if let Some(m) = cap.get(0) {
                    Some(m.as_str())
                } else {
                    NA_STRING
                }
            } else {
                NA_STRING
            }
        })
        .collect::<Vec<_>>()
}

// Macro to generate exports
extendr_module! {
    mod rr4r;
    fn rr4r_detect;
    fn rr4r_extract;
}
