#' Replace matched patterns in a string.
#'
#' @param x
#'   A character vector to look for the patterns in.
#' @param pattern
#'   A pattern to look for.
#' @param replacement
#'   A pattern to replace, or a function to generate the replacement strings.
#' @rdname extract
#' @export
rr4r_replace <- function(x, pattern, replacement) {
  rr4r_env$RR4R$rr4r_replace(x, pattern, replacement)
}

