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