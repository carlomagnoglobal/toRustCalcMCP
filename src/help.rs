//! Topic-based help system (ported from calc).

pub const TOPICS: &[&str] = &[
    "intro",
    "usage",
    "builtin",
    "define",
    "statement",
    "expression",
    "operator",
    "variable",
    "number",
    "config",
    "type",
    "list",
    "string",
    "mat",
    "assoc",
    "file",
    "error",
    "resource",
    "mcp",
];

pub fn topic(name: &str) -> Option<&'static str> {
    match name.to_ascii_lowercase().as_str() {
        "intro" | "overview" => Some(INTRO),
        "usage" => Some(USAGE),
        "builtin" | "builtins" => Some(BUILTIN),
        "define" | "function" => Some(DEFINE),
        "statement" | "statements" => Some(STATEMENT),
        "expression" | "expressions" => Some(EXPRESSION),
        "operator" | "operators" => Some(OPERATOR),
        "variable" | "variables" => Some(VARIABLE),
        "number" | "numbers" => Some(NUMBER),
        "config" | "configuration" => Some(CONFIG),
        "type" | "types" => Some(TYPE),
        "list" | "lists" => Some(LIST),
        "string" | "strings" => Some(STRING),
        "mat" | "matrix" | "matrices" => Some(MAT),
        "assoc" | "hash" => Some(ASSOC),
        "file" | "fileio" => Some(FILE),
        "error" | "errors" => Some(ERROR),
        "resource" | "script" => Some(RESOURCE),
        "mcp" => Some(MCP),
        _ => None,
    }
}

const INTRO: &str = r#"
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
"#;

const USAGE: &str = r#"
COMMAND LINE USAGE

SYNOPSIS

  rcalc [options] [expression...]
  rcalc --mcp

OPTIONS

  -p, --pipe              Pipe mode: read expressions from stdin, one per line
  -q, --quiet             Suppress output (useful for scripts)
  -f, --file FILE         Load and execute a .cal script file
  -m, --mode MODE         Output mode: real, frac, or int (default: real)
                          - real: decimal with ~ for inexact
                          - frac: exact rationals (e.g., 1/2)
                          - int: integer truncation
  -v, --version           Show version and exit
  -h, --help              Show this help and exit
  --mcp                   Start MCP server (JSON-RPC 2.0 over stdio)

INVOCATION MODES

Interactive REPL:
  rcalc
  > 2 + 3
  5
  > quit

Single Expression:
  rcalc '2 + 3'
  5

Pipe Mode:
  echo '2+3' | rcalc -p
  5

Script File:
  rcalc -f script.cal

EXAMPLES

  rcalc '2^256'                    # Big number
  rcalc -m frac '1/3 + 1/6'        # Fractional output: 1/2
  echo -e '2+3\n4*5' | rcalc -p   # Pipe multiple expressions
  rcalc -f math.cal -q             # Run script silently

ENVIRONMENT VARIABLES

  CALCRC                  Path to startup script (auto-loaded in interactive mode)
  HOME                    Used for history file (~/.rcalc_history)

INTERACTIVE MODE

When run without arguments, rcalc enters an interactive REPL with:

  > prompt for input
  Arrow Up/Down           Recall previous commands from history
  Ctrl+R                  Reverse history search
  Ctrl+A/E                Jump to line start/end
  Tab                     Auto-complete (partial support)
  help                    Show all topics and functions
  help <topic>            Show topic documentation
  help <name>             Search for function (e.g., help sin)
  quit, exit              Exit the REPL
  Ctrl+D                  EOF exit

SCRIPT FILES

Scripts are plain text with one statement per line. Comments start with #:

  # This is a comment
  x = 10
  y = 20
  x + y
  define f(n) = n * n
  f(5)

Use -f to load: rcalc -f script.cal

MCP SERVER

Start rcalc as a JSON-RPC 2.0 MCP server for integration with AI tools:

  rcalc --mcp

The server provides 4 tools:
  - calc_eval: evaluate expressions
  - calc_config: get/set session config
  - calc_functions: list builtin functions
  - calc_session: reset or inspect session state

See: help mcp
"#;

const BUILTIN: &str = r#"
BUILTIN FUNCTIONS

rcalc includes 351 built-in functions covering:

MATH
  Basics:  abs, sgn, int, frac, floor, ceil, round, min, max, trunc
  Roots:   sqrt, root, cbrt, isqrt, iroot
  Logs:    ln, log, log2, logn, ilog, ilog2, ilog10, ilogn
  Trig:    sin, cos, tan, cot, sec, csc, asin, acos, atan, atan2, acot, asec, acsc
  Hyperbolic: sinh, cosh, tanh, coth, sech, csch, asinh, acosh, atanh, acoth, asech, acsch
  Special: exp, exp2, exp10, expm1, log1p, erf, erfc, gamma, lgamma, polygamma, zeta
           j0, j1, y0, y1 (Bessel), hypot, gd, agd (Gudermannian)
           haversin, versin, coversin, chord, exsecant (obscure trig)
  Powers:  ^ or pow, exp, sqrt
  Rounding: round(x,[places]), ceil, floor

NUMBER THEORY
  Prime:   isprime, nextprime, prevprime, factor, lfactor, ptest
  Division: gcd, lcm, mod, pmod, quo, rem, quomod
  Special: catalan, euler, bernoulli, jacobi

COMPLEX NUMBERS
  re, im, arg (real, imaginary parts, phase angle)
  sqrt of negative numbers returns complex

STRINGS
  Length:  strlen
  Slice:   substr, index
  Modify:  replace, split, repeat, reverse
  Case:    toupper, tolower, swapcase, title
  Padding: lpad, rpad
  Trim:    trim, ltrim, rtrim
  Char:    ord, chr
  Test:    isalpha, isdigit, isspace, isalnum, isupper, islower, isprint, isgraph
           iscntrl, ispunct, isxdigit, isascii
  Parse:   str (convert to string)

LISTS
  Create:  list, range
  Access:  size, first, last, append, slice, find, contains, count
  Order:   sort, rsort, reverse, unique
  Combine: zip, flatten, union, intersection, difference, subset
  Compute: sum, product, min, max, mean, median, variance, stdev, rms, gmean, hmean
  Iterate: cumsum, diff, mode

MATRICES
  Create:  matfill
  Query:   matdim, matmin, matmax, matsum
  Ops:     det, inverse, mattrans, mattrace, matmul
  Linear:  dot, norm

SETS (ASSOCIATIVE ARRAYS)
  Create:  assoc
  Query:   indices, count
  Modify:  insert, delete
  Reduce:  join

FILE I/O
  Open:    fopen, fclose
  Read:    fgets, fgetc, fread, fscan, fscanf
  Write:   fputs, fputc, fwrite, fprintf
  Seek:    seek, tell, rewind
  Check:   eof, fflush, fileno, fsize, exists, isdir
  File:    remove, rename, mkdir

SYSTEM & ENVIRONMENT
  Info:    version, platform, hostname, pid, username, homedir, tmpdir, pwd, arch, uname
  User:    getuid, getenv, putenv
  Time:    time, systime, ctime, sleep, usertime
  IO:      input, getline, println, puts, printf, sprintf, format, debug
  Exec:    system, command, eval

BIT OPERATIONS
  Logic:   and, or, xor, comp
  Shift:   lshift, rshift
  Test:    bit, highbit, lowbit, fcnt (count bits), popcount
  Gray:    gray, igray
  Count:   clz, ctz (leading/trailing zeros)
  Special: hammingdist

CONFIGURATION & INTROSPECTION
  Config:  base (get/set ibase/obase), epsilon
  Type:    typeof, sizeof, isnan, isinf
  Vars:    vars, defined, undefine, del
  Env:     env
  Debug:   dump

HASHING & CRYPTO
  Hash:    sha1, md5, crc32

RANDOM
  Seed:    seed, srand, srandom
  Gen:     rand, random, randbit, randint, randperm

SEARCH & HELP
  Help:    help [topic|function]

To search for a specific function:
  help sin          # Show all functions with "sin" in the name
  help list         # Show all list-related functions
  help string       # Show all string functions

All 351 functions are documented in the catalog.
"#;

const DEFINE: &str = r#"
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
"#;

const STATEMENT: &str = r#"
STATEMENTS

rcalc supports several statement types.

ASSIGNMENT

Assign a value to a variable:

  x = 5
  y = x + 10

Variables persist across statements.

PRINT

Print a value to output:

  print x
  print "Hello"
  println "Done"

(println adds a newline)

CONDITIONAL: IF/ELSE

  if (x > 10) {
      print "x is large"
  } else {
      print "x is small"
  }

The else branch is optional.

LOOP: WHILE

Repeat while a condition is true:

  i = 0
  while (i < 5) {
      print i
      i = i + 1
  }

LOOP: FOR

Loop from start to end (inclusive):

  for (i = 1; 10) {
      print i
  }

This prints 1 through 10.

RETURN

Exit a function with a value:

  define f(x) {
      if (x < 0) return 0
      return x * 2
  }

QUIT/EXIT

Exit the REPL:

  quit
  exit

(Only in interactive mode; in scripts, quits the script.)

BLOCK STATEMENTS

Group multiple statements with braces:

  {
      x = 5
      y = 10
      print x + y
  }

SEMICOLON AS SEPARATOR

Statements are separated by semicolons or newlines:

  x = 5; y = 10; print x + y

Multiple statements on one line are allowed.
"#;

const EXPRESSION: &str = r#"
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
"#;

const OPERATOR: &str = r#"
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
"#;

const VARIABLE: &str = r#"
VARIABLES

A variable stores a value and persists across statements in a session.

DECLARING & ASSIGNING

Simply assign a value:

  x = 10
  y = x + 5

Variables are created on first assignment.

SCOPE

All variables are global within a session. Inside a function, parameters and
local assignments shadow globals but do not modify them:

  x = 100
  define test(x) {
      x = 200
      return x
  }
  test(50)            = 200
  x                   = 100  (unchanged)

LISTING VARIABLES

Use vars() to list all global variables:

  x = 1
  y = 2
  z = 3
  vars()              = [x, y, z]

CHECKING EXISTENCE

Use defined(name) to check if a variable exists:

  defined("x")        = 1  (exists)
  defined("unknown")  = 0  (does not exist)

DELETING VARIABLES

Use undefine(name) or del(name) to remove a variable:

  undefine("x")
  del("y")

After deletion, vars() no longer includes them.

GETTING SIZE

Use sizeof(x) to get the approximate size in bytes:

  x = list(1, 2, 3)
  sizeof(x)           (size in bytes)

TYPE CHECKING

Use typeof(x) to get the type of a value:

  typeof(5)           = "number"
  typeof("hello")     = "string"
  typeof(list(1,2))   = "list"

SPECIAL VARIABLES

Some functions act like variables (read/query):

  vars()              List all global variables
  env()               List environment variables as pairs
  epsilon             Current epsilon (precision target)
  pi()                Constant π (60 digits)
  e()                 Constant e (60 digits)
"#;

const NUMBER: &str = r#"
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
"#;

const CONFIG: &str = r#"
CONFIGURATION

Configure rcalc's behavior via settings.

EPSILON (PRECISION)

Epsilon is the convergence threshold for iterative algorithms:

  epsilon()           Get current epsilon (default 1e-20)
  epsilon(1e-30)      Set to 30 decimal digits of precision

Higher epsilon = faster, lower precision.
Lower epsilon = slower, higher precision.

Set epsilon before evaluating:

  epsilon(1e-50)
  sqrt(2)             Now computed to 50 digits of precision

DISPLAY DIGITS

How many digits to show in real mode:

  display()           Get current (default 20)
  display(50)         Show up to 50 digits

Example:

  display(10)
  2 / 3               = ~0.6666666667  (10 digits)

MODE (OUTPUT FORMAT)

Three output modes:

  real                Decimal: 2/3 = ~0.66666...
  frac                Fraction: 2/3 = 2/3
  int                 Integer: 2/3 = 0

Set via flag:

  rcalc -m frac '1/3 + 1/6'    Output: 1/2

Or in REPL:

  > mode frac
  > 1/3 + 1/6
  1/2

Quering:

  mode()              Get current mode

BASES (IBASE / OBASE)

Input base (ibase) and output base (obase) for number literals and display.
Both support 2-36.

Query:

  ibase()             Get input base (default 10)
  obase()             Get output base (default 10)

Set both:

  base(16)            Set ibase and obase to 16

Set separately:

  base(10, 16)        ibase=10, obase=16 (input decimal, output hex)
  256                 = 100

Hexadecimal input (0x prefix) always works regardless of ibase:

  0xFF                = 255

Binary input (0b prefix) always works:

  0b1111              = 15

CONFIGURATION SUMMARY

  epsilon([value])    Get/set precision target
  display([digits])   Get/set display width
  mode([mode])        Get/set output mode
  base([base])        Set both ibase and obase
  base([in], [out])   Set ibase and obase separately
  ibase()             Get input base
  obase()             Get output base

VIA MCP

If using rcalc --mcp, the calc_config tool handles configuration:

  {"method": "tools/call",
   "params": {"name": "calc_config",
             "arguments": {"action": "set", "mode": "frac", "epsilon": "1e-50"}}}

PERSISTENCE

Changes made in a session persist until you quit or reset.
Use the MCP calc_session tool to reset:

  {"method": "tools/call",
   "params": {"name": "calc_session",
             "arguments": {"action": "reset"}}}
"#;

const TYPE: &str = r#"
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
"#;

const LIST: &str = r#"
LISTS

Ordered collections of values.

CREATING LISTS

Use list() function:

  list(1, 2, 3)           = [1, 2, 3]
  list("a", "b", "c")     = ["a", "b", "c"]
  list(1, "two", 3.5)     = [1, "two", 7/2]

Create a range:

  range(1, 5)             = [1, 2, 3, 4, 5]
  range(0, 10, 2)         = [0, 2, 4, 6, 8, 10]

Empty list:

  list()                  = []

INDEXING

Access elements with [n]:

  x = list(10, 20, 30)
  x[0]                    = 10  (first, 0-indexed)
  x[1]                    = 20
  x[2]                    = 30
  x[-1]                   = 30  (last)
  x[-2]                   = 20  (second from last)

Out of bounds returns null.

BASIC OPERATIONS

  size(list)              Length
  first(list)             First element
  last(list)              Last element
  append(list, item, ...) Add items (mutates list)
  slice(list, start[, end]) Sublist from start to end

Example:

  x = list(1, 2, 3)
  append(x, 4, 5)         # x is now [1, 2, 3, 4, 5]
  slice(x, 1, 3)          = [2, 3]

SORTING & REVERSING

  sort(list)              Sort ascending
  rsort(list)             Sort descending
  reverse(list)           Reverse order
  unique(list)            Remove duplicates

Example:

  sort(list(3, 1, 2))     = [1, 2, 3]
  reverse(list(1, 2, 3))  = [3, 2, 1]
  unique(list(1, 1, 2))   = [1, 2]

SEARCHING

  find(list, value)       Index of first match (-1 if not found)
  contains(list, value)   1 if found, 0 otherwise
  count(list, value)      Number of occurrences

Example:

  find(list(10, 20, 30), 20)    = 1
  contains(list(1, 2, 3), 2)    = 1

AGGREGATION

  sum(list)               Sum of elements
  product(list)           Product of elements
  min(list)               Minimum value
  max(list)               Maximum value
  mean(list)              Arithmetic mean
  median(list)            Middle value
  variance(list)          Variance
  stdev(list)             Standard deviation
  rms(list)               Root mean square
  gmean(list)             Geometric mean
  hmean(list)             Harmonic mean
  mode(list)              Most common value

Example:

  sum(list(1, 2, 3, 4))   = 10
  mean(list(1, 2, 3, 4))  = 2.5
  median(list(1, 3, 5))   = 3

COMBINING LISTS

  zip(list1, list2)       Pair corresponding elements
  flatten(list)           Flatten nested lists
  union(list1, list2)     Set union
  intersection(list1, list2) Set intersection
  difference(list1, list2)   Set difference (list1 - list2)
  subset(list1, list2)    1 if list1 ⊆ list2

Example:

  zip(list(1, 2), list("a", "b")) = [[1, "a"], [2, "b"]]
  flatten(list(1, list(2, 3)))    = [1, 2, 3]

TRANSFORMATION

  cumsum(list)            Cumulative sum
  diff(list)              Consecutive differences

Example:

  cumsum(list(1, 2, 3))   = [1, 3, 6]
  diff(list(1, 3, 6))     = [2, 3]

ITERATION

In a script, use for() to iterate:

  for (x = list(1, 2, 3)) {
      print x
  }
"#;

const STRING: &str = r#"
STRINGS

Text values.

STRING LITERALS

Quoted with double quotes:

  "hello"
  "multi-word string"
  ""                      Empty string

ESCAPE SEQUENCES

  \"                      Double quote
  \\                      Backslash
  \n                      Newline
  \t                      Tab
  \r                      Carriage return

Example:

  "say \"hi\""            = say "hi"
  "line1\nline2"          = (two lines)

BASIC OPERATIONS

  strlen(str)             Length (number of characters)
  substr(str, start[, len]) Extract substring
  index(str, needle)      Find substring (position or -1)
  replace(str, old, new)  Replace all occurrences
  repeat(str, n)          Repeat string n times

Example:

  strlen("hello")         = 5
  substr("hello", 1, 2)   = "el"
  index("hello", "ll")    = 2
  replace("hello", "l", "L") = "heLLo"
  repeat("ab", 3)         = "ababab"

SPLITTING & JOINING

  split(str, sep)         Split by separator (returns list)
  join(list, sep)         Join list items with separator

Example:

  split("a,b,c", ",")     = ["a", "b", "c"]
  join(list("a", "b", "c"), ",") = "a,b,c"

CASE CONVERSION

  toupper(str)            Convert to uppercase
  tolower(str)            Convert to lowercase
  swapcase(str)           Swap case of all characters
  title(str)              Title case (capitalize words)

Example:

  toupper("Hello")        = "HELLO"
  tolower("Hello")        = "hello"
  title("hello world")    = "Hello World"

TRIMMING

  trim(str)               Remove whitespace from both ends
  ltrim(str)              Remove whitespace from left
  rtrim(str)              Remove whitespace from right

Example:

  trim("  hello  ")       = "hello"

PADDING

  lpad(str, width[, fill]) Pad left to width (default fill is space)
  rpad(str, width[, fill]) Pad right to width

Example:

  lpad("5", 3, "0")       = "005"
  rpad("hi", 5, "-")      = "hi---"

CHARACTER OPERATIONS

  ord(char)               Character to ASCII code
  chr(code)               ASCII code to character

Example:

  ord("A")                = 65
  chr(65)                 = "A"

TESTING

  startswith(str, prefix) 1 if starts with prefix
  endswith(str, suffix)   1 if ends with suffix

Example:

  startswith("hello", "he") = 1
  endswith("hello", "lo")   = 1

CHARACTER CLASS TESTS

  isalpha(str)            All alphabetic? (1 or 0)
  isdigit(str)            All digits? (1 or 0)
  isalnum(str)            All alphanumeric? (1 or 0)
  isspace(str)            All whitespace? (1 or 0)
  isupper(str)            All uppercase? (1 or 0)
  islower(str)            All lowercase? (1 or 0)
  isprint(str)            All printable? (1 or 0)

Example:

  isalpha("hello")        = 1
  isdigit("123")          = 1
  isdigit("12a")          = 0

PARSING & CONVERSION

  str(value)              Convert value to string

Example:

  str(123)                = "123"
  str(1/3)                = "1/3"
  str(list(1, 2))         = "[1, 2]"
"#;

const MAT: &str = r#"
MATRICES

Matrices are represented as lists of lists (rows).

CREATING MATRICES

  matfill(rows, cols, val) Create matrix filled with val
  [[1, 2], [3, 4]]         Literal 2x2 matrix

Example:

  m = matfill(2, 3, 0)    = [[0, 0, 0], [0, 0, 0]]

QUERYING MATRICES

  matdim(m)               Dimensions [rows, cols]
  matmin(m)               Minimum element
  matmax(m)               Maximum element
  matsum(m)               Sum of all elements

Example:

  matdim([[1, 2], [3, 4]]) = [2, 2]

OPERATIONS

  det(m)                  Determinant (2x2, 3x3)
  inverse(m)              Matrix inverse
  mattrans(m)             Transpose (swap rows/cols)
  mattrace(m)             Trace (sum of diagonal)
  matmul(m1, m2)          Matrix multiplication

Example:

  det([[1, 2], [3, 4]])           = -2
  mattrans([[1, 2], [3, 4]])      = [[1, 3], [2, 4]]
  mattrace([[1, 0], [0, 2]])      = 3

VECTORS

Vectors are 1-row or 1-column lists:

  v = [1, 2, 3]
  dot(v, [4, 5, 6])               = 1*4 + 2*5 + 3*6 = 32
  norm(v)                          = sqrt(1^2 + 2^2 + 3^2) = sqrt(14)

Example:

  dot([1, 2, 3], [4, 5, 6])        = 32
  norm([3, 4])                     = 5
"#;

const ASSOC: &str = r#"
ASSOCIATIVE ARRAYS (HASHES)

Key-value mappings.

CREATING HASHES

Use assoc() with alternating keys and values:

  h = assoc("name", "Alice", "age", 30)

Retrieve values:

  h["name"]               = "Alice"
  h["age"]                = 30

OPERATIONS

  indices(h)              List all keys
  count(h)                Number of key-value pairs
  insert(h, key, value)   Add or update key (mutates h)
  delete(h, key)          Remove key (mutates h)
  join(h, sep)            Join values with separator

Example:

  h = assoc("x", 1, "y", 2)
  indices(h)              = ["x", "y"]
  insert(h, "z", 3)       # h now has 3 key-value pairs
  delete(h, "y")          # h now has 2 key-value pairs
  count(h)                = 2
  join(h, ", ")           = "1, 3"
"#;

const FILE: &str = r#"
FILE I/O

Read and write files.

OPENING & CLOSING

  fd = fopen("file.txt", "r")  Open file (modes: "r"=read, "w"=write, "a"=append)
  fclose(fd)                    Close file

Example:

  fd = fopen("data.txt", "r")
  fgets(fd)                     # Read a line
  fclose(fd)

READING

  fgets(fd)                     Read one line (newline excluded)
  fgetc(fd)                     Read one character
  fread(fd, size)               Read up to size bytes
  fscan(fd, fmt)                Read formatted data (returns list)
  fscanf(fd, fmt, ...)          Read formatted with args

WRITING

  fputs(fd, str)                Write string
  fputc(fd, char)               Write character
  fwrite(fd, data)              Write data
  fprintf(fd, fmt, ...)         Formatted write

Example:

  fd = fopen("out.txt", "w")
  fprintf(fd, "Value: %d\n", 42)
  fclose(fd)

SEEKING & POSITION

  seek(fd, offset)              Move file pointer to offset
  tell(fd)                      Current position
  rewind(fd)                    Seek to start
  eof(fd)                       1 if at end of file

Example:

  seek(fd, 100)                 Move to byte 100
  tell(fd)                      = 100
  rewind(fd)                    Back to start

UTILITIES

  fflush(fd)                    Flush buffered data
  fileno(fd)                    File descriptor number
  fsize(filename)               File size in bytes
  exists(filename)              1 if file exists
  isdir(path)                   1 if directory
  remove(filename)              Delete file
  rename(old, new)              Rename file
  mkdir(path)                   Create directory

Example:

  exists("data.txt")            = 1 or 0
  fsize("data.txt")             = number of bytes
  remove("temp.txt")            Delete temp file
"#;

const ERROR: &str = r#"
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
"#;

const RESOURCE: &str = r#"
SCRIPTS & RESOURCE FILES

Load and execute files.

LOADING A SCRIPT

  rcalc -f script.cal

The file is read and executed line by line. Variables and functions persist.

SCRIPT SYNTAX

Plain text, one statement per line:

  # Comment (starts with #)
  x = 10
  y = 20
  print x + y

Semicolons separate statements on one line:

  x = 5; y = 10; print x + y

MULTI-LINE STATEMENTS

Use blocks for control flow:

  if (x > 10) {
      print "large"
  } else {
      print "small"
  }

  define fib(n) {
      if (n <= 1) return n
      return fib(n-1) + fib(n-2)
  }

COMMENTS

Lines starting with # are comments:

  # This is a comment
  x = 5  # Also a comment after code

LOADING WITHIN A SESSION

Use read(filename) or source(filename) to load a file (if implemented):

  read("lib.cal")

STARTUP SCRIPT

Environment variable CALCRC points to an optional startup script, auto-loaded
on interactive startup:

  export CALCRC=~/.calcrc
  rcalc                         Loads ~/.calcrc first

COMMON PATTERNS

Defining a function library:

  # lib.cal
  define double(x) = x * 2
  define triple(x) = x * 3

Load and use:

  rcalc -f lib.cal
  > double(5)
  10
"#;

const MCP: &str = r#"
MCP SERVER MODE

rcalc can run as a JSON-RPC 2.0 MCP server for integration with AI tools.

STARTING THE SERVER

  rcalc --mcp

The server reads JSON-RPC 2.0 requests from stdin and writes responses to stdout.

PROTOCOL

Requests are newline-delimited JSON. Example:

  {"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}

Response:

  {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-06-18",...}}

INITIALIZATION

First, initialize the connection:

  {"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}

Response includes protocol version, server info, and capabilities.

TOOLS

The server provides 4 tools via tools/call:

  1. calc_eval — Evaluate an expression
  2. calc_config — Get or set session configuration
  3. calc_functions — List builtin functions
  4. calc_session — Reset or inspect session state

TOOL: CALC_EVAL

Evaluate an arbitrary-precision expression:

  {"jsonrpc":"2.0","id":3,"method":"tools/call",
   "params":{"name":"calc_eval",
            "arguments":{"expression":"2^256",
                        "mode":"real",
                        "epsilon":"1e-20"}}}

Response includes both text and JSON output:

  {"jsonrpc":"2.0","id":3,"result":{"content":[
      {"type":"text","text":"<numeric result>"},
      {"type":"application/json","json":{"expression":"...","result":"..."}}
    ],"isError":false}}

Arguments:
  - expression (required): expression string
  - mode (optional): real, frac, or int
  - epsilon (optional): precision string (e.g., "1e-50")
  - digits (optional): display width (e.g., 50)

TOOL: CALC_CONFIG

Get or set session configuration:

  {"jsonrpc":"2.0","id":4,"method":"tools/call",
   "params":{"name":"calc_config",
            "arguments":{"action":"get"}}}

  {"jsonrpc":"2.0","id":5,"method":"tools/call",
   "params":{"name":"calc_config",
            "arguments":{"action":"set","mode":"frac","epsilon":"1e-50"}}}

Arguments for "set":
  - mode: real, frac, or int
  - epsilon: precision string
  - digits: display width
  - ibase: input base (2-36)
  - obase: output base (2-36)

TOOL: CALC_FUNCTIONS

List available builtin functions, optionally filtered:

  {"jsonrpc":"2.0","id":6,"method":"tools/call",
   "params":{"name":"calc_functions",
            "arguments":{"filter":"sin"}}}

Returns a list of functions matching the filter.

TOOL: CALC_SESSION

Reset or inspect the current session:

  {"jsonrpc":"2.0","id":7,"method":"tools/call",
   "params":{"name":"calc_session",
            "arguments":{"action":"reset"}}}

  {"jsonrpc":"2.0","id":8,"method":"tools/call",
   "params":{"name":"calc_session",
            "arguments":{"action":"state"}}}

Arguments:
  - action: "reset" (clear all variables) or "state" (show current state)

EXAMPLE SESSION

  $ rcalc --mcp

  {"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
  {"jsonrpc":"2.0","id":2,"method":"tools/list"}
  {"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"calc_eval","arguments":{"expression":"2^256"}}}
  {"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"calc_config","arguments":{"action":"get"}}}

NOTIFICATIONS

Requests without an "id" are notifications; the server does not respond:

  {"jsonrpc":"2.0","method":"notifications/initialized"}
"#;
