use extendr_api::prelude::*;
use lru::LruCache;
use regex::Regex;

#[extendr]
pub struct RR4R {
    cache: LruCache<String, Regex>,
}

impl RR4R {
    fn get_or_compile_regex(&mut self, pattern: &String) -> &Regex {
        if !self.cache.contains(pattern) {
            let re = Regex::new(pattern.as_str()).unwrap();
            self.cache.put(pattern.clone(), re);
        }

        self.cache.get(pattern).unwrap()
    }
}

#[extendr]
impl RR4R {
    fn new(cap: u16) -> Self {
        let cache = LruCache::new(cap.into());
        Self { cache }
    }

    fn rr4r_detect(&mut self, x: Robj, pattern: String) -> Vec<Bool> {
        if x.is_na() {
            return vec![NA_LOGICAL];
        }
        let re = self.get_or_compile_regex(&pattern);

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

    fn rr4r_extract(&mut self, x: Robj, pattern: String) -> Vec<Option<&'static str>> {
        if x.is_na() {
            return vec![NA_STRING];
        }
        let re = self.get_or_compile_regex(&pattern);

        let x_str_iter = x.as_str_iter().unwrap();
        x_str_iter
            .map(|s| {
                if s.is_na() {
                    return NA_STRING;
                }

                if let Some(m) = re.find(&s) {
                    Some(m.as_str())
                } else {
                    NA_STRING
                }
            })
            .collect::<Vec<_>>()
    }
}

// Macro to generate exports
extendr_module! {
    mod rr4r;
    impl RR4R;
}
