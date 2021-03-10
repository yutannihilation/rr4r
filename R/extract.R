#' Extract matching patterns from a string.
#'
#' @param x
#'   A character vector to look for the patterns in.
#' @param pattern
#'   A pattern to look for.
#' @param i
#'   Index of a capture group to extract. If not specified, extract the whole
#'   match.
#' @examples
#' ### These examples are taken from stringr's doc (?str_extract) ###
#'
#' shopping_list <- c("apples x4", "bag of flour", "bag of sugar", "milk x2")
#' rr4r_extract(shopping_list, "\\d")
#' rr4r_extract(shopping_list, "[a-z]+")
#' rr4r_extract(shopping_list, "[a-z]{1,4}")
#' rr4r_extract(shopping_list, "\\b[a-z]{1,4}\\b")
#'
#' # Extract all matches
#' rr4r_extract_all(shopping_list, "[a-z]+")
#' rr4r_extract_all(shopping_list, "\\b[a-z]+\\b")
#' rr4r_extract_all(shopping_list, "\\d")
#' @rdname extract
#' @export
rr4r_extract <- function(x, pattern, i = NULL) {
  if (is.null(i)) {
    rr4r_env$RR4R$rr4r_extract(x, pattern)
  } else {
    rr4r_env$RR4R$rr4r_extract_part(x, pattern, i)
  }
}

#' @rdname extract
#' @export
rr4r_extract_all <- function(x, pattern, i = NULL) {
  if (is.null(i)) {
    i <- 0L
  }
  rr4r_env$RR4R$rr4r_extract_all(x, pattern, i)
}

#' @rdname extract
#' @export
rr4r_extract_groups <- function(x, pattern) {
  tibble::new_tibble(
    rr4r_env$RR4R$rr4r_extract_groups(x, pattern),
    nrow = length(x)
  )
}
