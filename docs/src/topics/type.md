TYPES

rcalc supports seven value types.

NUMBER (EXACT RATIONAL)

An exact integer or rational number:

  5               Number
  1/2             Number (exact rational)
  2.5             Number (1/2 is 5/2, exact)
  ~0.5            Number (marked inexact, but is still Number type)

Query:

  typeof(5)       = "number"
  typeof(1/3)     = "number"

COMPLEX

A complex number with real and imaginary parts:

  sqrt(-1)        = 1i (complex)
  2 + 3i          = 2+3i (complex)

Operations:

  re(2+3i)        = 2 (real part)
  im(2+3i)        = 3 (imaginary part)
  arg(2+3i)       = phase angle (argument)

Query:

  typeof(sqrt(-1)) = "complex"

STRING

A sequence of characters:

  "hello"         String
  "say \"hi\""    String with escaped quote

Operations:

  strlen("hello") = 5
  substr("hello", 0, 3) = "hel"
  toupper("hello") = "HELLO"

Query:

  typeof("hello") = "string"

LIST

An ordered collection:

  list(1, 2, 3)   List
  [1, 2, 3]       List (shorthand)

Operations:

  size(list(1,2,3)) = 3
  list(1,2,3)[0] = 1 (first element)
  sort(list(3,1,2)) = [1, 2, 3]

Query:

  typeof(list(1,2)) = "list"

HASH (ASSOCIATIVE ARRAY)

A key-value mapping:

  assoc("name", "Alice", "age", 30)  Hash

Operations:

  insert(h, "city", "NYC")
  indices(h) = ["name", "age", "city"]
  delete(h, "age")

Query:

  typeof(assoc(...)) = "hash"

FUNCTION

A user-defined or built-in function:

  define f(x) = x * 2
  f               = <function>

Query:

  typeof(f) = "function"

NULL

The absence of a value:

  null            = null (uninitialized / void)

Query:

  typeof(null)    = "null"

TYPE PREDICATES

Check types with typeof():

  typeof(x)       = "number", "complex", "string", "list", "hash", "function", "null"

Test properties:

  isnan(x)        = 1 if x is NaN (always 0 for exact rationals)
  isinf(x)        = 1 if x is infinite (always 0 for exact rationals)

CONVERSIONS

Convert between types:

  str(5)          = "5"
  str(1/3)        = "1/3"
  typeof(str(x))  = "string"

SIZE

Get approximate size of a value:

  sizeof(5)       Bytes for a number
  sizeof("hello") Bytes for a string
  sizeof([1,2,3]) Bytes for a list