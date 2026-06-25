EXPRESSIONS

An expression evaluates to a value.

LITERALS

Numbers:
  123
  3.14
  1/2
  0x1A        (hexadecimal)
  0b1010      (binary)
  1.5e-10     (scientific notation)

Strings:
  "hello"
  "escape: \"quote\""

Lists:
  list(1, 2, 3)
  [1, 2, 3]

VARIABLES

Use a variable by name:

  x               # value of x
  x + y           # value of x plus value of y

FUNCTION CALLS

Call a built-in or user-defined function:

  sqrt(2)
  max(3, 5)
  list(1, 2, 3)
  f(x, y)         # user-defined function

OPERATORS & PRECEDENCE

Operators have priority (high to low):

  1. Unary:       - !       (negation, not)
  2. Power:       ^         (right-associative: 2^3^2 = 2^(3^2) = 512)
  3. Multiply:    * / // %  (*, /, integer divide, modulo)
  4. Add:         + -       (addition, subtraction)
  5. Compare:     < <= > >= == !=  (less, equal, etc.)
  6. Logical AND: &&        (short-circuit)
  7. Logical OR:  ||        (short-circuit)
  8. Assign:      =         (lowest precedence, right-associative)

Examples:

  2 + 3 * 4               = 14  (multiply before add)
  2 ^ 3 ^ 2               = 512 (power is right-assoc)
  -2 ^ 2                  = -4  (unary minus after power)
  x = 5                   (assigns 5 to x, result is 5)

GROUPING WITH PARENTHESES

Use parentheses to override precedence:

  (2 + 3) * 4             = 20
  2 ^ (3 + 2)             = 32

COMPOUND EXPRESSIONS

Statements can be chained with semicolons; the result of the last is returned:

  x = 5; y = 10; x + y    = 15  (prints 5, 10, 15)

INDEXING

Access list or string elements:

  list(1, 2, 3)[0]        = 1   (first element, 0-indexed)
  list(1, 2, 3)[-1]       = 3   (last element)
  "hello"[1]              = "e"

TRUTHINESS

In conditions (if, while), values are truthy/falsy:

  0       = false
  1       = true
  ""      = false
  "text"  = true
  []      = false
  [x]     = true