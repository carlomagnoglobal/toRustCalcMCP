OPERATORS

ARITHMETIC

  +       Addition:           2 + 3 = 5
  -       Subtraction:        5 - 2 = 3
  *       Multiplication:     3 * 4 = 12
  /       Exact division:     5 / 2 = 5/2 (in frac mode)
  //      Integer division:   5 // 2 = 2
  %       Modulo:             5 % 2 = 1
  ^       Power:              2 ^ 10 = 1024

UNARY

  -       Negation:           -5, -x
  !       Logical NOT:        !0 = 1, !1 = 0

COMPARISON

  <       Less than:          2 < 3 = 1
  <=      Less or equal:      2 <= 2 = 1
  >       Greater than:       3 > 2 = 1
  >=      Greater or equal:   3 >= 3 = 1
  ==      Equal:              5 == 5 = 1
  !=      Not equal:          5 != 3 = 1

LOGICAL

  &&      AND (short-circuit):  0 && x  = 0 (x not evaluated)
  ||      OR (short-circuit):   1 || x  = 1 (x not evaluated)

ASSIGNMENT

  =       Assign:             x = 5, y = x + 10

PRECEDENCE TABLE (High to Low)

  1. ( )      Parentheses, function calls, array indexing
  2. - ! +    Unary minus, NOT
  3. ^        Power (right-associative)
  4. * / // % Multiply, divide, integer divide, modulo
  5. + -      Add, subtract
  6. < <= > >= == !=   Comparisons
  7. &&       Logical AND
  8. ||       Logical OR
  9. =        Assignment (right-associative)

POWER IS RIGHT-ASSOCIATIVE

  2 ^ 3 ^ 2 = 2 ^ (3 ^ 2) = 2 ^ 9 = 512
  not (2 ^ 3) ^ 2 = 8 ^ 2 = 64

EXACT DIVISION

rcalc always performs exact arithmetic:

  1 / 3               = 1/3 (exact rational, not 0.333...)
  1 / 3 * 3           = 1   (exactly, no rounding)

Integer division (//) truncates:

  5 // 2              = 2
  -5 // 2             = -3

MODULO

  5 % 2               = 1
  -5 % 2              = -1

Note: behavior depends on the signs of operands.

NOTES

- Short-circuit evaluation: && and || evaluate left-to-right and stop early
  if the result is known (0 for &&, 1 for ||).
- All integer arithmetic is exact (no overflow, arbitrary precision).
- Rational arithmetic is exact (e.g., 1/3 + 1/6 = 1/2).
- Transcendental functions (sin, exp) approximate to epsilon precision.