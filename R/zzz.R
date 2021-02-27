rr4r_env <- new.env(parent = emptyenv())

.onLoad <- function(libname, pkgname) {
  cap <- getOption("rr4r.lru_cache_capacity", 100L)
  cap <- as.integer(cap)

  if (length(cap) != 1 || !is.integer(cap) || cap <= 0) {
    stop("Invalid option specified in rr4r.lru_cache_capacity", call. = FALSE)
  }

  rr4r_env$RR4R <- RR4R$new(cap)

  invisible()
}
