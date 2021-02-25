# These codes are from stringr's tests: https://github.com/tidyverse/stringr/blob/1f03eb02b68922d24bb37b326c85dbae57f7d2f1/tests/testthat/test-detect.r#L3-L14

test_that("special cases are correct", {
  expect_equal(rr4r_detect(NA, "x"), NA)
  expect_equal(rr4r_detect(character(), "x"), logical())
})

test_that("vectorised patterns work", {
  expect_equal(rr4r_detect("ab", c("a", "b", "c")), c(T, T, F))
  expect_equal(rr4r_detect(c("ca", "ab"), c("a", "c")), c(T, F))

  # negation works
  expect_equal(rr4r_detect("ab", c("a", "b", "c"), negate = TRUE), c(F, F, T))
})
