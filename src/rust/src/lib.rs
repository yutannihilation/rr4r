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

    fn resize(&mut self, cap: u16) {
        self.cache.resize(cap.into())
    }

    fn clear(&mut self) {
        self.cache.clear()
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

    fn rr4r_extract(&mut self, x: Robj, pattern: String) -> Vec<Option<String>> {
        if x.is_na() {
            return vec![None];
        }
        let re = self.get_or_compile_regex(&pattern);

        let x_str_iter = x.as_str_iter().unwrap();
        x_str_iter
            .map(|s| {
                if s.is_na() {
                    return None;
                }

                if let Some(m) = re.find(&s) {
                    Some(m.as_str().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn rr4r_extract_all(&mut self, x: Robj, pattern: String) -> Robj {
        if x.is_na() {
            return List(NA_STRING).into();
        }
        let re = self.get_or_compile_regex(&pattern);

        let x_str_iter = x.as_str_iter().unwrap();
        let list_inner: Vec<Robj> = x_str_iter
            .map(|s| {
                if s.is_na() {
                    return NA_STRING.into_robj();
                }

                re.captures_iter(&s)
                    .map(|cap| Some(cap[0].to_string()))
                    .collect_robj()
            })
            .collect();

        list_inner.into_robj()
    }

    fn rr4r_extract_groups(&mut self, x: Robj, pattern: String) -> Robj {
        // This function doesn't need to handle the case when `x` is NA specially

        let re = self.get_or_compile_regex(&pattern);
        let group_names: Vec<_> = re
            .capture_names()
            // First group is always for the whole match and doesn't have name
            .skip(1)
            // For groups that doesn't have names, automatically assign names like "X1"
            .enumerate()
            .map(|(i, s)| {
                if s.is_some() {
                    s.unwrap().to_string()
                } else {
                    format!("X{}", i + 1)
                }
            })
            .collect();
        let ncol = group_names.len();

        let mut tmp: Vec<Vec<Option<String>>> = Vec::with_capacity(group_names.len());
        for _ in 0..ncol {
            tmp.push(Vec::with_capacity(x.len()));
        }

        if x.is_na() {
            for i in 0..ncol {
                tmp.get_mut(i).unwrap().push(None);
            }
        } else {
            let x_str_iter = x.as_str_iter().unwrap();
            for s in x_str_iter {
                if s.is_na() {
                    for i in 0..ncol {
                        tmp.get_mut(i).unwrap().push(None);
                    }
                    continue;
                }

                if let Some(cap) = re.captures(&s) {
                    for i in 0..ncol {
                        if let Some(m) = cap.get(i + 1) {
                            tmp.get_mut(i).unwrap().push(Some(m.as_str().to_string()));
                        } else {
                            tmp.get_mut(i).unwrap().push(None);
                        }
                    }
                } else {
                    for i in 0..ncol {
                        tmp.get_mut(i).unwrap().push(None);
                    }
                }
            }
        }

        // Create a list from Vec
        let result: Robj = tmp
            .into_iter()
            .map(|v| v.to_owned().into_robj())
            // To create a list, we need to craete Vec<Robj> first, then convert it to Robj.
            .collect::<Vec<Robj>>()
            .into_robj();

        result.set_names(group_names).unwrap()
    }

    fn rr4r_replace(
        &mut self,
        x: Robj,
        pattern: String,
        replacement: String,
    ) -> Vec<Option<String>> {
        if x.is_na() {
            return vec![None];
        }
        let re = self.get_or_compile_regex(&pattern);
        let replacement = replacement.as_str();

        let x_str_iter = x.as_str_iter().unwrap();
        x_str_iter
            .map(|s| {
                if s.is_na() {
                    return None;
                }
                Some(re.replace(s, replacement).to_string())
            })
            .collect()
    }
}

// Macro to generate exports
extendr_module! {
    mod rr4r;
    impl RR4R;
}
