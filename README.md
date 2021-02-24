
<!-- README.md is generated from README.Rmd. Please edit that file -->

# rr4r: Rust regex for R

<!-- badges: start -->
<!-- badges: end -->

rr4r is my practice to use Rust in an R package with the power of
[extendr](https://github.com/extendr/extendr). My current goal is to
create the same interface to stringr’s functions using Rust’s [regex
crate](https://docs.rs/regex/1.4.3/regex/).

## Installation

``` r
remotes::install_github("yutannihilation/rr4r")
```

## Example

``` r
library(rr4r)

rr4r_detect("Today is 2014-01-01", r"(\d{4}-\d{2}-\d{2})")
#> [1] TRUE
```
