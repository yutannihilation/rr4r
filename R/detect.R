#' Detect the presence or absence of a pattern in a string.
#'
#' @param x
#'   A character vector to look for the patterns in.
#' @param pattern
#'   A pattern to look for.
#' @examples
#' ### These examples are taken from stringr's doc (?str_detect) ###
#'
#' fruit <- c("apple", "banana", "pear", "pinapple")
#' rr4r_detect(fruit, "a")
#' rr4r_detect(fruit, "^a")
#' rr4r_detect(fruit, "a$")
#' rr4r_detect(fruit, "b")
#' rr4r_detect(fruit, "[aeiou]")
#' @rdname detect
#' @export
rr4r_detect <- function(x, pattern) {
  rr4r_env$RR4R$rr4r_detect(x, pattern)
}
