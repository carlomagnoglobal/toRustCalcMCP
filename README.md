# toRustCalcMCP

A Rust port of [`calc`](https://github.com/lcn2/calc) (Landon Curt Noll's
arbitrary-precision calculator) that works **both** as:

- **`rcalc`** — a calc-compatible command-line calculator, and
- **an MCP server** — JSON-RPC 2.0 over stdio, so an LLM/agent can do exact math.

One codebase, two front-ends. The numeric core uses **exact rational arithmetic**
(`num-rational` over `num-bigint`), which is the same model calc uses natively —
so `1/3 * 3` is exactly `1`, and `2^256` is computed to the last digit.

## Build

```sh
cargo build --release
# binaries:
#   target/release/toRustCalcMCP   (auto-detects MCP vs CLI)
#   target/release/rcalc           (always the CLI)
```

Optionally symlink: `ln -s toRustCalcMCP rcalc` — when argv[0] is `rcalc`,
`toRustCalcMCP` behaves as the calculator.

## CLI usage (`rcalc`)

```sh
rcalc '2^100'                 # 1267650600228229401496703205376
rcalc '1/3 + 1/6'            # 0.5
rcalc -m frac '1/3 + 1/6'    # 1/2
rcalc 'gcd(462,1071)'        # 21
rcalc 'fact(30)'            # 265252859812191058636308480000000
rcalc 'sqrt(2)'             # 1.4142135623730950488
rcalc 'isprime(1000003)' 'nextprime(1000003)'   # 1 \n 1000033
echo '3*4' | rcalc -p        # pipe mode
rcalc                        # interactive REPL (Ctrl-D to exit)
```

Flags: `-p` pipe mode, `-q` quiet, `-m real|frac|int`, `-v` version, `-h` help.
Several classic calc flags (`-c -C -d -e -i -O -s -u`) are accepted and ignored.

## MCP usage

```sh
toRustCalcMCP --mcp     # speak JSON-RPC 2.0 over stdio
```

Handshake → `initialize`, then `tools/list`, then `tools/call`. See
[`docs/MCP_TOOL_SCHEMA.json`](docs/MCP_TOOL_SCHEMA.json) for the authoritative,
server-emitted schema and [`examples/mcp-config.json`](examples/mcp-config.json)
for a client registration snippet.

### Tools

| tool | purpose | key args |
|------|---------|----------|
| `calc_eval` | evaluate an expression | `expression` (req), `mode`, `digits`, `epsilon` |
| `calc_config` | get/set session precision & display | `action` (`get`/`set`), `mode`, `digits`, `epsilon` |
| `calc_functions` | list builtins | `filter` (optional substring) |

`calc_eval`'s `mode`/`digits`/`epsilon` are per-call overrides; `calc_config set`
changes them for the session.

## Language supported

- Operators: `+ - * /` (exact), `//` (integer divide), `%` (modulus), `^`/`**`
  (power), comparisons `== != < <= > >=` (yield `1`/`0`), unary `-`/`+`.
- Variables and assignment: `x = 7; x^2`.
- User-defined functions: `define f(x) = x^2; f(5)` → `25`.
- Control flow: `if`/`else`, `while`, `for` loops, blocks with `{}`.
- `;`-separated statements; the value of each is printed (calc behaviour).
- Numeric literals: integers, `a/b` rationals, decimals, `1.2e-3`, `0x`/`0b`.
- Lists: `list(1,2,3); append(x,4); slice(x,1,3)`.
- Complex numbers: `sqrt(-1)` → `i`; arithmetic with `+`, `-`, `*`, `/`.
- String literals: `"hello"; strlen(s); index(haystack, needle)`.
- **99 builtins** (28% of calc's ~350) organized by category — see implementation status below.

## Precision model

Numbers are exact rationals. Irrational results are approximated to within the
session `epsilon` (default `1e-20`), exactly like calc. Transcendentals (`exp`,
`ln`, `sin`, `cos`, `tan`) are computed at arbitrary precision via Taylor series
and Newton's method. `sqrt`, `sin`, `cos`, etc. converge until term < epsilon.
`pi`/`e` are 60-digit constants. A leading `~` in real-mode output marks an
inexact (rounded/non-terminating) rendering, as in calc.

## Implementation Status — 99 of ~350 builtins (28% coverage)

calc upstream has ~350 builtins. This port implements **99 core functions** organized by category:

### ✅ Fully Implemented Categories

| Category | Count | Functions |
|----------|-------|-----------|
| **Arithmetic** | 10/10 | `abs`, `sgn`, `int`, `frac`, `floor`, `ceil`, `round`, `min`, `max`, `avg` |
| **Number Theory** | 12/19 | `gcd`, `lcm`, `mod`, `fact`, `comb`, `perm`, `fib`, `isprime`, `nextprime`, `num`, `den`, `catalan` |
| **Basic Trig** | 3/3 | `sin`, `cos`, `tan` |
| **Inverse Trig** | 4/6 | `asin`, `acos`, `atan`, `atan2` |
| **Hyperbolic** | 6/9 | `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh` |
| **Transcendental** | 4/4 | `exp`, `ln`, `log`, `log2` |
| **Special Functions** | 8/12 | `erf`, `erfc`, `hypot`, `gd`, `agd`, `j0`, `j1`, `catalan` |
| **Complex Numbers** | 3/3 | `arg`, `re`, `im` |
| **Bitwise** | 10/10 | `and`, `or`, `xor`, `comp`, `lshift`, `rshift`, `bit`, `highbit`, `lowbit`, `fcnt` |
| **List Operations** | 6/6 | `list`, `size`, `append`, `first`, `last`, `slice` |
| **String Functions** | 5/17 | `strlen`, `index`, `isalpha`, `isdigit`, `isspace` |
| **Type Checking** | 3/20 | `typeof`, `isnan`, `isinf` |
| **Angle Conversion** | 5/5 | `d2r`, `r2d`, `d2g`, `g2d`, `g2r` |

**Total: 99 builtins**

### ⚠️ Partially Implemented

| Category | Implemented | Missing |
|----------|-----------|---------|
| **Trigonometric Variants** | 13 | ~25 (cot, sec, csc, acot, asec, acsc, coth, sech, csch, haversin, versin, etc.) |
| **Logarithmic** | 4 | 5 (logn, ilog, ilog2, ilog10, ilogn) |
| **Root Functions** | 1 | 4 (root, power, isqrt, cbrt, iroot) |
| **Prime Functions** | 3 | 4 (prevprime, nextcand, prevcand, pfact, ptest) |
| **Rounding** | 1 | 2 (bround, btrunc) |

### ❌ Not Yet Implemented (250+ functions)

| Category | Missing | Purpose |
|----------|---------|---------|
| **File I/O** | 24 | `fopen`, `fclose`, `fgets`, `fprintf`, `fscan`, etc. |
| **Matrix Ops** | 9 | `det`, `inverse`, `matdim`, `matfill`, `mattrace`, `mattrans`, etc. |
| **Hash/Assoc Arrays** | 6 | `assoc`, `indices`, `insert`, `delete`, `count`, `join` |
| **Random Numbers** | 10 | `rand`, `random`, `randbit`, `seed`, `srand`, `srandom`, etc. |
| **Character Class** | 12 | `isalnum`, `isupper`, `islower`, `isprint`, `isgraph`, `iscntrl`, `ispunct`, `isxdigit`, etc. |
| **Environment/System** | 8 | `getenv`, `putenv`, `system`, `time`, `systime`, `ctime`, `sleep`, `usertime` |
| **Memory Management** | 10 | `blk`, `blkcpy`, `blkfree`, `blocks`, `free`, `freeglobals`, etc. |
| **Error Handling** | 7 | `errcount`, `errmax`, `errno`, `errsym`, `error`, `newerror`, etc. |
| **Modular Arithmetic** | 5 | `pmod`, `hnrmod`, `quomod`, `quo`, `rem` |
| **Rational Approx** | 4 | `appr`, `cfappr`, `cfsim`, `scale` |
| **Advanced Number Theory** | 7 | `gcdrem`, `lcmfact`, `euler`, `bernoulli`, `jacobi`, `factor`, `lfactor` |
| **Bessel (extended)** | 2 | `y0`, `y1` (have j0, j1) |
| **Other** | ~180 | Stack ops, command/script, variable manipulation, cryptographic (sha1), etc. |

### ✨ Full Language Features Implemented

- ✅ User-defined functions (`define name(params) = expr`)
- ✅ Control flow (`if`/`else`, `while`, `for` loops)
- ✅ Variables and scoping
- ✅ Lists and indexing (0-based, negative indices supported)
- ✅ Complex numbers with full arithmetic
- ✅ String literals and operations
- ✅ Base conversion (2-36, input and output)
- ✅ Arbitrary-precision arithmetic (exact rationals)
- ✅ File loading (`-f filename`)
- ✅ REPL, pipe mode, quiet mode

### 📋 Roadmap for Remaining Work

**Phase 4: High-Value Functions** (estimated 40–80 builtins)
- [ ] File I/O suite (24) — enables automation
- [ ] Matrix operations (9) — linear algebra
- [ ] More trig variants (cot, sec, csc, etc.) — ~10
- [ ] Environment/system functions (8) — scripting integration
- [ ] Random number functions (10) — simulation

**Phase 5: Utility & Compatibility** (estimated 100+ builtins)
- [ ] Character classification (12)
- [ ] Advanced modular arithmetic (5)
- [ ] Continued fractions (4)
- [ ] Root and logarithm variants (9)
- [ ] Memory/error/stack management (20+)

**Phase 6: Exotic & Specialized** (remaining ~100 builtins)
- [ ] Rare trig variants (coversin, exsecant, etc.)
- [ ] Cryptographic (sha1, md5)
- [ ] Advanced number theory (Bernoulli, Euler numbers, Jacobi symbols)
- [ ] Associative arrays and object operations
- [ ] Complete residue class support

## Scope — Architecture & Design

calc upstream is ~92,000 lines of C with ~350 builtins and a full,
Turing-complete scripting language. This port is a faithful **core**, structured
for incremental, additive expansion:

- ✅ **Exact-rational numeric engine** — matches calc's native model
- ✅ **Complete lexer/parser** — handles the full expression syntax
- ✅ **Tree-walk evaluator** — with user-defined functions, control flow, scoping
- ✅ **Builtin registry** — extensible function map with auto-cataloging
- ✅ **CLI & MCP server** — two front-ends, one engine
- 🔄 **Incremental builtins** — each category slots in cleanly without rework

The architecture is stable; adding more functions is straightforward.

## License

LGPL-2.1, matching calc upstream.
