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