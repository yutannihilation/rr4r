use extendr_api::prelude::*;
use regex::Regex;

/// Detect pattern
/// @export
#[extendr]
fn rr4r_detect(x: &str, pattern: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(x)
}

// Macro to generate exports
extendr_module! {
    mod rr4r;
    fn rr4r_detect;
}
