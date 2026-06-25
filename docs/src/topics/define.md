DEFINING FUNCTIONS

Define your own functions with the define keyword.

SIMPLE ONE-LINE FORM

Define a function in one line:

  define f(x) = x * x
  f(5)                    = 25

Parameters can be multiple:

  define add(a, b) = a + b
  add(3, 4)               = 7

BLOCK FORM

For multi-statement functions, use braces:

  define fact(n) {
      if (n <= 1) {
          return 1
      }
      return n * fact(n - 1)
  }
  fact(5)                 = 120

RECURSION

Functions can call themselves:

  define fib(n) {
      if (n <= 1) return n
      return fib(n-1) + fib(n-2)
  }
  fib(10)                 = 55

LOCAL VARIABLES

Variables assigned inside a function are local to that call:

  define test() {
      x = 5
      return x * 2
  }
  test()                  = 10
  x                       = undefined (if not set globally)

To use a global variable:

  x = 100
  define use_global() = x * 2
  use_global()            = 200

RETURN VALUES

Use return or the last expression:

  define f(x) {
      y = x * 2
      return y + 1
  }

  define g(x) {
      y = x * 2
      y + 1          # Last expression is returned
  }

Both return the same value.

CALLING CONVENTION

Arguments are passed by value (immutable copies). Functions cannot modify
caller's variables:

  define inc(x) {
      x = x + 1
      return x
  }

  a = 10
  inc(a)                  = 11
  a                       = 10  (unchanged)

SCOPE

All variables are global by default. Function parameters are local (shadowing
globals during the call).