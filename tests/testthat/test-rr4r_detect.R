# These codes are from stringr's tests: https://github.com/tidyverse/stringr/blob/1f03eb02b68922d24bb37b326c85dbae57f7d2f1/tests/testthat/test-detect.r#L3-L14

test_that("rr4r_detect() works", {
  expect_true(rr4r_detect("abc", "ab[c]"))
  expect_equal(rr4r_detect(NA, "x"), NA)
  expect_equal(rr4r_detect(character(), "x"), logical())
})
