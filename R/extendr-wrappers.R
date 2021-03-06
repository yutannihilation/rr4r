# Generated by extendr: Do not edit by hand
#
# This file was created with the following call:
#   .Call("wrap__make_rr4r_wrappers", use_symbols = TRUE, package_name = "rr4r")

#' @docType package
#' @usage NULL
#' @useDynLib rr4r, .registration = TRUE
NULL

RR4R <- new.env(parent = emptyenv())

RR4R$new <- function(cap) .Call(wrap__RR4R__new, cap)

RR4R$resize <- function(cap) invisible(.Call(wrap__RR4R__resize, self, cap))

RR4R$clear <- function() invisible(.Call(wrap__RR4R__clear, self))

RR4R$rr4r_detect <- function(x, pattern) .Call(wrap__RR4R__rr4r_detect, self, x, pattern)

RR4R$rr4r_extract <- function(x, pattern) .Call(wrap__RR4R__rr4r_extract, self, x, pattern)

RR4R$rr4r_extract_part <- function(x, pattern, i) .Call(wrap__RR4R__rr4r_extract_part, self, x, pattern, i)

RR4R$rr4r_extract_all <- function(x, pattern, i) .Call(wrap__RR4R__rr4r_extract_all, self, x, pattern, i)

RR4R$rr4r_extract_groups <- function(x, pattern) .Call(wrap__RR4R__rr4r_extract_groups, self, x, pattern)

RR4R$rr4r_replace <- function(x, pattern, replacement) .Call(wrap__RR4R__rr4r_replace, self, x, pattern, replacement)

#' @export
`$.RR4R` <- function (self, name) { func <- RR4R[[name]]; environment(func) <- environment(); func }

