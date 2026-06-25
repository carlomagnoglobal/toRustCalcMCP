NUMBERS

rcalc performs exact arithmetic on integers and rationals.

EXACT INTEGERS

All integer arithmetic is exact (arbitrary precision):

  2 ^ 256             = 115792089237316195423570985008687907853269984665640564039457584007913129639936
                        (exact 78-digit number, no overflow)
  factorial(100)      = exact product of 1*2*3*...*100

EXACT RATIONALS

Ratios of integers are exact:

  1 / 3               = 1/3 (not 0.333...)
  1 / 3 + 1 / 6       = 1/2 (exactly)
  1 / 3 * 3           = 1   (exactly, no rounding error)

INEXACT (IRRATIONAL) NUMBERS

Transcendental functions produce inexact results (marked with ~):

  sqrt(2)             = ~1.41421356237309504881
  sin(pi()/6)         = ~0.5
  ln(e())             = ~0.99999999999999999999

The ~ prefix indicates the value is approximated to epsilon precision
(default epsilon = 1e-20).

EPSILON & PRECISION

Epsilon controls convergence of iterative algorithms (sqrt, transcendental
series). Default is 1e-20, good for 20 decimal digits of precision.

Set epsilon:

  epsilon(1e-50)      # Higher precision (slower)
  sqrt(2)             = (computed to 50 digits)

Display digits:

  2 / 3               = ~0.66666666666666666667  (20 digits, default)

Set display:

  display(50)         # Show up to 50 digits
  2 / 3               = (shows more digits)

NOTATION

Literals:
  123                 Integer
  123.456             Decimal (converted to exact rational internally)
  1/2                 Rational (exact)
  0x1A                Hexadecimal (26)
  0b1010              Binary (10)
  1.5e-10             Scientific notation

MODES

  real  (default):    Show decimals with ~ for inexact
  frac:               Show exact rationals (1/2 not 0.5)
  int:                Show integer truncation (10/3 shows as 3)

Set mode:

  rcalc -m frac '1/3 + 1/6'    # Output: 1/2
  rcalc -m int '10 / 3'         # Output: 3

BASES

Input base (ibase) and output base (obase) support 2-36:

  base(16)            # Set both to hexadecimal
  255                 = ff

  base(10, 2)         # ibase=10, obase=2
  255                 = 11111111

COMPLEX NUMBERS

Roots of negative numbers produce complex results:

  sqrt(-1)            = 1i
  sqrt(-4)            = 2i

Complex arithmetic works as expected.

OPERATIONS

All standard operations preserve exactness:

  +  - * /            Exact for rationals
  ^ (integer power)   Exact
  ^                   Inexact for fractional exponents
  sqrt, sin, etc.     Inexact (irrational results)

OVERFLOW

rcalc does not overflow. Numbers grow as large as memory allows:

  1000!               Computes exactly (2568 digits)
  2^1000000           Computes (no limit)