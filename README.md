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
- `;`-separated statements; the value of each is printed (calc behaviour).
- Numeric literals: integers, `a/b` rationals, decimals, `1.2e-3`, `0x`/`0b`.
- ~40 builtins covering arithmetic, number theory, and elementary functions —
  run `calc_functions` or `rcalc -h` to list them. Highlights: `abs sgn int frac
  floor ceil round min max avg gcd lcm mod fact comb perm fib isprime nextprime
  isqrt sqrt cbrt hypot power num den pi e exp ln log log2 sin cos tan asin acos
  atan atan2`.

## Precision model

Numbers are exact rationals. Irrational results are approximated to within the
session `epsilon` (default `1e-20`), exactly like calc. `sqrt`/`hypot` are
computed at arbitrary precision via Newton's method (e.g. `sqrt(2)` to 50 digits
with `epsilon=1e-50` is correct to 50 digits). `pi`/`e` are exact to 60 digits.
A leading `~` in real-mode output marks an inexact (rounded/non-terminating)
rendering, as in calc.

## Scope — what this port does and does not cover

calc upstream is ~92,000 lines of C with ~350 builtins and a full,
Turing-complete scripting language. This port is a faithful **core**, not a
1:1 reproduction. Implemented: the exact-rational engine, the expression
language above, ~40 builtins, the `rcalc` CLI, and the complete MCP server +
schema.

**Not yet ported** (clear next steps):
- The remaining ~310 builtins (matrices, lists/assoc arrays, blocks, bitwise
  ops, full transcendental suite at arbitrary precision, `config()` surface).
- User-defined functions, `define`, control flow (`if`/`for`/`while`), and
  reading `.cal` resource files (`-f`).
- Complex numbers and the full `mat`/`list`/`obj` type system.
- Arbitrary-precision `sin/cos/exp/ln/...` (currently f64-precision; `sqrt`,
  `pi`, `e` are already arbitrary/high precision).

These are additive: the lexer/parser/evaluator and the builtin registry are
structured so each remaining piece slots in without reworking the core.

## License

LGPL-2.1, matching calc upstream.
