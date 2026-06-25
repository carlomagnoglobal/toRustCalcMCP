INTRODUCTION TO RCALC

rcalc is a Rust port of the arbitrary-precision calculator calc. It performs
exact arithmetic on integers and rational numbers, with support for complex
numbers, transcendental functions, user-defined functions, lists, strings,
matrices, and more.

GETTING STARTED

  rcalc [options] [expression...]

To start interactively:

  rcalc
  > 2 + 3
  5
  > sqrt(2)
  ~1.41421356237309504881
  > quit

BASIC ARITHMETIC

Integer arithmetic is exact (no rounding):

  1 / 3 + 1 / 6           = 1/2 (exactly, in frac mode)
  2 ^ 256                 = exact 78-digit number
  1 / 3 * 3               = 1 (exactly)

VARIABLES AND ASSIGNMENT

Assign values to variables:

  x = 10
  y = 20
  x + y                   = 30

BUILTIN FUNCTIONS

Call built-in functions:

  sin(pi()/6)             = ~0.5
  abs(-5)                 = 5
  list(1,2,3)             = [1, 2, 3]

For a complete list: help builtin

MODES

Real mode (default):      shows decimals with ~ for inexact
Fractional mode (-m frac): shows exact rationals (e.g., 1/2)
Integer mode (-m int):     truncates to integer

QUITTING

  quit
  exit
  Ctrl+D (EOF)

For more topics:  help <topic>
Examples:         help usage   help define   help operator