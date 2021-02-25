test_that("rr4r_extract() works", {
  expect_equal(rr4r_extract(c("abc", "ab", "abcd", NA), "ab[cd]"), c("abc", NA, "abc", NA))
})
