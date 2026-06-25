ERROR HANDLING

Detect and report errors.

QUERYING ERRORS

  errcount()                    Count of errors so far
  errmax()                      Maximum errors before stop
  errmax(n)                     Set max errors
  errno()                       Last error code
  errsym(code)                  Error name from code

Example:

  if (errcount() > 5) {
      print "Too many errors, quitting"
      quit
  }

RAISING ERRORS

  error(message)                Raise error with message
  warn(message)                 Issue warning (non-fatal)

Example:

  if (x < 0) {
      error("x must be non-negative")
  }

  warn("This calculation may be imprecise")

REGISTERING CUSTOM ERRORS

  newerror(code, message)       Register new error type

Example:

  newerror(100, "Custom error")
  errno()                       = 100