# Error & Exception Handling

7 functions.

| Name | Signature | Description |
|------|-----------|-------------|
| `errcount` | `errcount()` | number of errors occurred |
| `errmax` | `errmax(n)` | set max errors before stopping (0=unlimited) |
| `errno` | `errno()` | last error code |
| `errsym` | `errsym(code)` | error message for error code |
| `error` | `error(msg)` | raise an error with message |
| `newerror` | `newerror(code,msg)` | register a new error type |
| `warn` | `warn(msg)` | issue a warning (not counted as error) |
