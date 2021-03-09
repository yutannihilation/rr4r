
<!-- README.md is generated from README.Rmd. Please edit that file -->

# rr4r: Rust regex for R

<!-- badges: start -->

[![R-CMD-check](https://github.com/yutannihilation/rr4r/workflows/R-CMD-check/badge.svg)](https://github.com/yutannihilation/rr4r/actions)
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

### `rr4r_extract()`

``` r
shopping_list <- c("apples x4", "bag of flour", "bag of sugar", "milk x2")

rr4r_extract(shopping_list, "\\d")
#> [1] "4" NA  NA  "2"
rr4r_extract(shopping_list, "[a-z]+")
#> [1] "apples" "bag"    "bag"    "milk"
rr4r_extract(shopping_list, "[a-z]{1,4}")
#> [1] "appl" "bag"  "bag"  "milk"
rr4r_extract(shopping_list, "\\b[a-z]{1,4}\\b")
#> [1] NA     "bag"  "bag"  "milk"
```

### `rr4r_extract_all()`

``` r
# Extract all matches
rr4r_extract_all(shopping_list, "[a-z]+")
#> [[1]]
#> [1] "apples" "x"     
#> 
#> [[2]]
#> [1] "bag"   "of"    "flour"
#> 
#> [[3]]
#> [1] "bag"   "of"    "sugar"
#> 
#> [[4]]
#> [1] "milk" "x"
rr4r_extract_all(shopping_list, "\\b[a-z]+\\b")
#> [[1]]
#> [1] "apples"
#> 
#> [[2]]
#> [1] "bag"   "of"    "flour"
#> 
#> [[3]]
#> [1] "bag"   "of"    "sugar"
#> 
#> [[4]]
#> [1] "milk"
rr4r_extract_all(shopping_list, "\\d")
#> [[1]]
#> [1] "4"
#> 
#> [[2]]
#> character(0)
#> 
#> [[3]]
#> character(0)
#> 
#> [[4]]
#> [1] "2"
```

### `rr4r_extract_groups()`

``` r
library(dplyr, warn.conflicts = FALSE)

tibble(
  x = c("2020-01-02", "2021-12-31", "2022-08-09")
) %>% 
  mutate(
    rr4r_extract_groups(x, 
      pattern = "(?P<year>\\d{4})-(?P<month>\\d{2})-(?P<day>\\d{2})"
    )
  )
#> # A tibble: 3 x 4
#>   x          year  month day  
#>   <chr>      <chr> <chr> <chr>
#> 1 2020-01-02 2020  01    02   
#> 2 2021-12-31 2021  12    31   
#> 3 2022-08-09 2022  08    09
```

### `rr4r_replace()`

``` r
# TODO
# fruits <- c("one apple", "two pears", "three bananas")
# rr4r_replace(fruits, "[aeiou]", "-")

rr4r_replace("a1", "\\D", toupper)
#> [1] "A1"
```
