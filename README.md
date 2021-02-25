
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

### `rr4r_detect()`

``` r
library(rr4r)

fruit <- c("apple", "banana", "pear", "pinapple", NA)
rr4r_detect(fruit, "a")
#> [1] TRUE TRUE TRUE TRUE   NA
rr4r_detect(fruit, "^a")
#> [1]  TRUE FALSE FALSE FALSE    NA
rr4r_detect(fruit, "a$")
#> [1] FALSE  TRUE FALSE FALSE    NA
rr4r_detect(fruit, "b")
#> [1] FALSE  TRUE FALSE FALSE    NA
rr4r_detect(fruit, "[aeiou]")
#> [1] TRUE TRUE TRUE TRUE   NA
```
