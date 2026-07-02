# toRustCalcMCP

A Rust port of [`calc`](https://github.com/lcn2/calc) (Landon Curt Noll's
arbitrary-precision calculator) that works as:

- **`rcalc`** — a calc-compatible command-line calculator
- **Web REPL** — browser-based interactive calculator
- **MCP server** — JSON-RPC 2.0 over stdio for LLM/agent integration

**Three interfaces, one engine.** The numeric core uses **exact rational arithmetic**
(`num-rational` over `num-bigint`), which is the same model calc uses natively —
so `1/3 * 3` is exactly `1`, and `2^256` is computed to the last digit.

**👉 [Getting Started in 2 minutes](GETTING_STARTED.md) — new to rcalc? Start here.**
**👉 [Install as Claude Desktop MCP](INSTALL.md) — integrate with Claude Desktop.**

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

## Web REPL usage

```sh
cargo build --release
./target/release/rcalc-web
# Open browser: http://localhost:8888
```

A modern browser-based REPL with:
- Interactive expression evaluation
- Command history (↑/↓ arrow keys)
- Syntax-highlighted output
- Full calc functionality (483 builtin names)
- Responsive design for mobile/desktop

Try it: open http://localhost:8888 and enter `2^256` or `sin(pi()/6)`.

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
- **483 builtin names** (full parity with calc's 350, minus 15 documented interpreter internals, plus extensions) — see implementation status below.

## Precision model

Numbers are exact rationals. Irrational results are approximated to within the
session `epsilon` (default exact `1/10^20`), exactly like calc. Transcendentals (`exp`,
`ln`, `sin`, `cos`, `tan`) are computed at arbitrary precision via Taylor series
and Newton's method. `sqrt`, `sin`, `cos`, etc. converge until term < epsilon.
`pi`/`e` are 60-digit constants. A leading `~` in real-mode output marks an
inexact (rounded/non-terminating) rendering, as in calc.

## Implementation Status — full upstream parity ✅

calc upstream (`lcn2/calc` func.c) defines 350 builtins. This port registers
**483 builtin names** covering **335 of the 350** upstream builtins (the other
148 that were long missing were added in the upstream-parity batches below),
plus dozens of extensions and aliases beyond upstream (statistics, hashing,
bit tricks, system info, and more).

### Intentionally not implemented (15 interpreter internals)

These upstream names are artifacts of calc's C interpreter and have no
meaningful mapping to this port's architecture; they remain unimplemented
rather than shipping fake stubs:

`access`, `calc_tty`, `calclevel`, `calcpath`, `custom`, `dp`, `estr`,
`inputlevel`, `memsize`, `name`, `param`, `prompt`, `protect`, `saveval`,
`stoponerror`

### Known deviations from upstream

- Out-parameter builtins return values instead: `d2dm`/`d2dms` (and the g/h
  family) return `[deg, min]` / `[deg, min, sec]` lists; `quomod` returns
  `[q, r]`; `search`/`rsearch` return an index or null.
- In-place/lvalue builtins return new values: `modify`, `copy`, `swap`
  (builtins receive values, not references).
- `base2()` reads as 0 (no secondary base); setting it errors — the renderer
  has a single output base.
- `free*()` are no-ops: nothing is cached, values are computed on demand.

### Full language features

- ✅ User-defined functions (`define name(params) = expr`), higher-order
  builtins (`select`/`forall`/`modify` call function values)
- ✅ Control flow (`if`/`else`, `while`, `for` loops)
- ✅ Variables and scoping
- ✅ Lists and indexing (0-based, negative indices supported)
- ✅ Complex numbers with full arithmetic
- ✅ String literals and a complete string-function suite
- ✅ Base conversion (2-36, input and output)
- ✅ Arbitrary-precision arithmetic (exact rationals; exact 1/10^20 epsilon)
- ✅ File loading (`-f filename`), file I/O builtins, REPL, pipe mode

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

# Test CI workflow
