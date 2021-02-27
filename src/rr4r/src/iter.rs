use extendr_api::prelude::*;
use std::iter::{FromIterator, IntoIterator};

impl FromIterator<Robj> for List {
    fn from_iter<I: IntoIterator<Item = &str>>(iter: I) -> Self {}
}
