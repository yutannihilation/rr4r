#' Extract matching patterns from a string.
#'
#' @param x
#'   A character vector to look for the patterns in.
#' @param pattern
#'   A pattern to look for.
#' @examples
#' shopping_list <- c("apples x4", "bag of flour", "bag of sugar", "milk x2")
#' rr4r_extract(shopping_list, "\\d")
#' rr4r_extract(shopping_list, "[a-z]+")
#' rr4r_extract(shopping_list, "[a-z]{1,4}")
#' rr4r_extract(shopping_list, "\\b[a-z]{1,4}\\b")
#' @export
rr4r_extract <- function(x, pattern) {
  rr4r_env$RR4R$rr4r_extract(x, pattern)
}
