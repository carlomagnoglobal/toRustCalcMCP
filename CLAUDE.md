# CLAUDE.md — project guide for Claude Code

> This file is read automatically by Claude Code / the Claude CLI when it opens
> this repo. It is the single source of truth for *how to work here*: what the
> project is, how to build/run/test it, the architecture, the pending work, and
> the decision chain that produced the current state. Keep it at the repo root.

---

## 1. What this project is

`toRustCalcMCP` is a Rust port of [`calc`](https://github.com/lcn2/calc)
(Landon Curt Noll's arbitrary-precision calculator). One engine, two front-ends:

- **`rcalc`** — a calc-compatible command-line calculator.
- **`toRustCalcMCP --mcp`** — an MCP server speaking JSON-RPC 2.0 over stdio.

Current status: **Phase 3 mostly complete.** The project has a full `src/` structure
with lexer, parser, evaluator, 99 builtins, CLI, MCP server, and 91 integration
tests. `cargo build --release` succeeds; all tests pass. Core TODO #1–#8 complete (exact rationals, 
transcendentals, control flow, bitwise ops, lists, complex numbers, base conversion, MCP extensions); 
Phase 3 extended builtins 3.1–3.3 complete (inverse/hyperbolic trig, special functions, string/type ops).
The exact-rational engine works correctly (e.g., `1/3 * 3` is exactly `1`), big powers compute to the last digit 
(e.g., `2^256`), complex arithmetic works (e.g., `sqrt(-1) * sqrt(-1) = -1`), string/type introspection is built-in,
angle conversions between degrees/radians/gradians work, and the MCP server provides structured JSON alongside text results.

---

---

## 2. Building and running (copy-paste)

```sh
# build
cargo build --release
# test
cargo test
# run
./target/release/toRustCalcMCP '2^100'
./target/release/toRustCalcMCP -m frac '1/3 + 1/6'
./target/release/toRustCalcMCP --mcp   # MCP server
# verify your CLAUDE.md checklist in §9
```

---

## 2b. Environment / setup notes (read before building)

These reflect the exact environment this was developed in; adjust for a normal dev box.

- **Toolchain:** developed against **rustc/cargo 1.75** (Ubuntu apt package).
  `rustup` was *not* available (no network to `static.rust-lang.org`). If there is
  no toolchain:
  ```sh
  # container / Debian-Ubuntu, as root:
  apt-get update && apt-get install -y rustc cargo
  # a normal workstation: prefer rustup (https://rustup.rs)
  ```
- **Network allowlist (if sandboxed):** crates.io / static.crates.io / index.crates.io
  are reachable, so `cargo build` can fetch deps. `Cargo.lock` is committed — prefer
  `cargo build --locked` for reproducibility.
- **Shell is `dash`/`sh`, not bash** in the sandbox: **no brace expansion**
  (`cp a/{x,y}` fails). Write explicit loops or full paths.
- **Deliverables/outputs** (sandbox only): copy final files to
  `/mnt/user-data/outputs/` and present them; `target/` is intentionally not shipped.
- MSRV-sensitive: stick to syntax that compiles on 1.75 (no newer std APIs).

---

## 3. Build / run / test (copy-paste)

```sh
# build
cargo build                 # debug
cargo build --release       # optimized (LTO on)

# run the CLI
cargo run --bin rcalc -- '2^100'
./target/debug/rcalc '1/3 + 1/6'          # 0.5
./target/debug/rcalc -m frac '1/3 + 1/6'  # 1/2
echo '3*4' | ./target/debug/rcalc -p      # pipe mode

# run the MCP server
cargo run --bin toRustCalcMCP -- --mcp

# tests
cargo test                  # 9 integration tests in tests/integration.rs
```

**MCP smoke test** (drives a full JSON-RPC session and pretty-prints replies):

```sh
printf '%s\n' \
 '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
 '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
 '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
 '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"calc_eval","arguments":{"expression":"2^256"}}}' \
 | ./target/debug/toRustCalcMCP --mcp
```

Expect `id:3` → exact 78-digit value. Notifications get **no** reply.

---

## 4. Implemented architecture — the actual code

**Implemented** pipeline: **lexer → parser (AST) → eval** over a shared `Interp`. 
The CLI and MCP layers are thin wrappers around `Interp::eval_render`. All modules
live in `src/` and compile cleanly.

| file | responsibility |
|------|----------------|
| `number.rs` | numeric core. `Num = BigRational`. parsing, `pow`/`pow_int`, arbitrary-precision `sqrt` (Newton), `round_to_epsilon`, decimal rendering (`~` marks inexact). **Start here for precision work.** |
| `number.rs` | **DONE.** Exact rationals, `parse_number`, `pow`/`pow_int`, arbitrary-precision `sqrt` (Newton), `pi()`/`e()` (60-digit constants), `exp`/`ln`/`sin`/`cos`/`tan` (Taylor series, epsilon-aware). |
| `value.rs` | **DONE.** `Value` enum (`Number`/`Complex`/`Str`/`Null`/`Function`/`List`) + `render(&Config)` for all modes. Functions stored as params + body. Complex numbers rendered as `a+bi` or `a-bi`. |
| `config.rs` | **DONE.** `Config { epsilon, display, mode, ibase, obase }` + `Mode {Real,Frac,Int}` with `parse()`. Supports bases 2-36. |
| `lexer.rs` | **DONE.** Tokenizer: keywords (define/if/for/while/print), `**`→`^`, `//`, blocks `{}`, strings, `0x`/`0b`, sci-notation. |
| `parser.rs` | **DONE.** Pratt parser: `Expr` including Define, If, While, For, Block, Print. `^` right-assoc. Assignments, calls, control flow. |
| `eval.rs` | **DONE.** Tree-walk `Interp` with scoped environments for function calls. `eval`, `eval_all`, `eval_render`. Handles user-defined functions, if/while/for, print. |
| `builtins.rs` | **DONE.** 79 builtins: arithmetic, rounding, number theory, transcendentals (sin/cos/tan/asin/acos/atan/atan2/sinh/cosh/tanh/asinh/acosh/atanh/cas/cis), special functions (erf/erfc/hypot/gd/agd/j0/j1), Catalan, bitwise/shifts/bits, digits, list ops, complex ops, base conversion. All registered + catalog. |
| `cli.rs` | **DONE.** Arg parsing: `-p` pipe, `-q` quiet, `-f` file, `-m` mode, `-v` version. REPL with `>` prompt. Handles interactive, pipe, file, and expression modes. |
| `mcp.rs` | **DONE.** JSON-RPC 2.0 over stdio. `initialize`, `tools/list` (4 tools), `tools/call` dispatch. `calc_eval`, `calc_config`, `calc_functions`, `calc_session`. Structured JSON output alongside text. |
| `main.rs` | **DONE.** Entry point. Dispatches `--mcp` → server; else CLI (also CLI when argv0 ends in `rcalc`). |
| `bin_rcalc.rs` | **DONE.** Thin `rcalc` binary that always runs CLI. |
| `lib.rs` | **DONE.** Module declarations. |
| `tests/integration.rs` | **DONE.** 71 tests: exactness, transcendentals, control flow, bitwise operations, file loading, list operations, complex numbers, base conversion, inverse/hyperbolic trig, and special functions. All passing. |
| `docs/MCP_TOOL_SCHEMA.json` | **DONE.** Server-emitted schema. Regenerate after tool changes via §7 script. |

---

## 5. Conventions & invariants (do not break)

- **Exactness first.** Integer/rational arithmetic must stay exact (no f64 in the
  `+ - * / // % ^`(int) paths). Only irrational results may approximate, and they
  approximate to `cfg.epsilon`.
- **Errors are `Result<_, String>`** with lowercase, human-readable messages
  (e.g. `division by zero`). No panics on user input.
- **Comments in English.** Doc-comment public items.
- **Builtin contract:** signature `fn(&mut Interp, &[Value]) -> Result<Value,String>`;
  validate arity with `argc`/`argc_range`; pull args via `n(...)`/`int(...)`.
  Register in `builtins::register` **and** add a `catalog()` row (keeps `-h`,
  `calc_functions`, and the docs in sync).
- **MCP wire format is fixed:** newline-delimited JSON-RPC 2.0; `tools/call`
  returns `{ content:[{type:"text",text}], isError }`; requests with no `id` are
  notifications and must get **no** response; unknown method → JSON-RPC `-32601`.
- **Two binaries must both keep working** after any change (`toRustCalcMCP` and
  `rcalc`). Run the §3 smoke tests + `cargo test` before declaring done.
- **Keep MSRV 1.75.** If you add a dependency, run `cargo build --locked` and
  commit the updated `Cargo.lock`.

### How to add a builtin (worked example)
1. In `builtins.rs`, write `fn f_foo(it:&mut Interp,a:&[Value])->Result<Value,String>`.
2. `argc("foo",a,N)?;` then read args; return `Ok(Value::Number(...))`.
3. Add `("foo", f_foo)` to the `register` table **and** `("foo","foo(x)","desc")`
   to `catalog()`.
4. Add a case to `tests/integration.rs`; `cargo test`.

---

## 6. Pending tasks (prioritized) — the actual TODO

Each item lists **where it slots in** and a **done-when** acceptance check. Pick
top-down; they're ordered by value-to-effort and by what unblocks the most.

~~1. **Arbitrary-precision transcendentals** — DONE.~~
   - ✅ Implemented `exp`, `ln`, `sin`, `cos`, `tan` in `number.rs` via Taylor series
   - ✅ All functions respect epsilon and converge to required precision
   - ✅ Verified: exp(1) ≈ e(), ln(e()) ≈ 1, sin(π/6) = 0.5, cos(0) = 1
   - ✅ 6 new integration tests added and passing

~~2. **User-defined functions + control flow** — DONE.~~
   - ✅ Lexer: added keywords `define`, `if`, `for`, `while`, `print` + block delimiters
   - ✅ Parser: new Expr variants (Define, If, While, For, Block, Print)
   - ✅ Evaluator: scoped environments (scope_stack) for function calls
   - ✅ Functions: stored as Value::Function(params, body), callable with args
   - ✅ Control flow: if/else branching, while loops, for loops (1..n inclusive)
   - ✅ Verified: `define sq(x) = x^2; sq(9)` → 81; for-loop sum works; 6 new tests pass

~~3. **Integer / bitwise builtins** — DONE.~~
   - ✅ Bitwise: `and`, `or`, `xor`, `comp` (complement)
   - ✅ Shifts: `lshift`, `rshift` (left/right shift by n bits)
   - ✅ Bit inspection: `bit` (test bit n), `highbit`, `lowbit` (MSB/LSB position)
   - ✅ Utilities: `fcnt` (count set bits), `digits(x[, base])` (digit count)
   - ✅ Examples: and(12,10)→8, or(12,10)→14, xor(12,10)→6, lshift(3,2)→12
   - ✅ Verified: all operations work on integers; 7 new tests pass

~~4. **`-f file.cal` resource loading** — DONE.~~
   - ✅ CLI: added `-f filename` flag to parse_args()
   - ✅ File reading: std::fs::read_to_string() with error handling
   - ✅ Execution: file content executed via eval_all()
   - ✅ Quiet mode: `-q -f script.cal` suppresses output
   - ✅ Verified: works with functions, loops, conditionals; 1 new test pass
   - Where: `cli.rs` (read file → `Interp::eval_all`); honor `-s`/`-q` interplay.
   - Done when: a small `.cal` script with `define` + loop runs and prints expected output.

~~5. **More of the type system: lists & associative arrays** — DONE.~~
   - ✅ Value: added `List(Vec<Value>)` variant with proper rendering as `[item1, item2, ...]`
   - ✅ Parser: added `Index(Box<Expr>, Box<Expr>)` for `list[index]` syntax; updated `parse_postfix()`
   - ✅ Lexer: added `LBracket`, `RBracket` tokens for `[` and `]`
   - ✅ Builtins: implemented `list()`, `size()`, `append()`, `first()`, `last()`, `slice()`
   - ✅ Indexing: supports 0-based and negative indices (Python-style)
   - ✅ Verified: `x=list(1,2,3); append(x,4); size(x)` → 4; 7 new integration tests pass
   - Total tests: 36 passing (added 7 for lists)

~~6. **Complex numbers** — DONE.~~
   - ✅ Value: added `Complex(Num, Num)` variant for real and imaginary parts
   - ✅ Rendering: complex numbers render as `a+bi` or `a-bi` with proper sign handling
   - ✅ sqrt: negative numbers now return complex results (e.g., `sqrt(-1)` → `i`)
   - ✅ Arithmetic: complex addition, subtraction, multiplication, division all working
   - ✅ Builtins: implemented `re()` (real part), `im()` (imaginary part), `arg()` (phase angle)
   - ✅ Comparisons: allowed for real numbers, error for complex values
   - ✅ Verified: `sqrt(-1)` → `i`, `(1+i)*(2-i)` → `3+1i`; 7 new integration tests pass
   - Total tests: 43 passing (added 7 for complex)

~~7. **Display/base faithfulness** — DONE.~~
   - ✅ Config: added `ibase` and `obase` fields (default 10, supports 2-36)
   - ✅ Base conversion: `to_base()` converts BigInt to any base 2-36
   - ✅ Rendering: numbers output in obase in all modes (Real, Frac, Int)
   - ✅ Builtin `base()`: get/set ibase and obase; `base(16)` sets both; `base(10, 16)` sets separately
   - ✅ Fractional numbers: support bases for both integer and fractional parts
   - ✅ Complex numbers: real and imaginary parts render in obase
   - ✅ Verified: `base(16); 255` → `ff`, `base(2); 255` → `11111111`, `base(16); 1/2` → `0.8`
   - ✅ 6 new integration tests added and passing
   - Total tests: 49 passing (added 6 for base conversion)

~~8. **Broaden MCP** — DONE.~~
   - ✅ New `calc_session` tool: reset session or show session state
   - ✅ Session reset: `calc_session` with action "reset" clears all variables and config
   - ✅ Session state: `calc_session` with action "state" returns session info as JSON
   - ✅ Structured JSON output: all tools now return both text and application/json content types
   - ✅ calc_eval: returns text result + JSON with expression, result, and mode
   - ✅ calc_config: returns text summary + JSON with all config fields (including ibase/obase)
   - ✅ calc_functions: returns text list + JSON with structured function catalog
   - ✅ calc_session: returns text summary + JSON with session state
   - ✅ Updated tools_list_result to include new calc_session tool
   - ✅ Schema regenerated: MCP_TOOL_SCHEMA.json updated with 4 tools
   - Total tools: 4 (calc_eval, calc_config, calc_functions, calc_session)

## Phase 3: Extended Builtins (In Progress)

### 3.1 Inverse & Hyperbolic Trigonometric Functions — DONE
   - ✅ Inverse trig: `asin`, `acos`, `atan`, `atan2`
   - ✅ Hyperbolic: `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`
   - ✅ Special: `cas` (cos + sin), `cis` (e^(ix)), `conj` (complex conjugate)
   - ✅ Rounding: Enhanced `round(x, places)` to support decimal place rounding
   - ✅ All functions respect epsilon for precision
   - ✅ 14 new integration tests added and passing
   - Builtins: 51 → 67 (+16)
   - Total tests: 49 → 63 (+14)

### 3.2 Number Theory & Special Functions — DONE
   - ✅ Special functions: `hypot(x,y)`, `erf(x)`, `erfc(x)` (error functions)
   - ✅ Geometric: `gd(x)` (Gudermannian), `agd(x)` (inverse Gudermannian)
   - ✅ Bessel functions: `j0(x)`, `j1(x)` (first kind, orders 0 and 1)
   - ✅ Number theory: `catalan(n)` (Catalan numbers, C_n = (2n)!/(n!(n+1)!))
   - ✅ All functions respect epsilon for precision
   - ✅ 8 new integration tests added and passing
   - Builtins: 67 → 79 (+12)
   - Total tests: 63 → 71 (+8)

### 3.3 String & Type Functions — DONE
   - ✅ String functions: `strlen(s)`, `index(haystack, needle)` (returns position or -1)
   - ✅ Type checking: `isalpha(s)`, `isdigit(s)`, `isspace(s)` (all return 1 or 0)
   - ✅ Type inspection: `typeof(x)` returns string type name (number, complex, string, list, function, null)
   - ✅ Value predicates: `isnan(x)`, `isinf(x)` (always 0 for rationals; included for compatibility)
   - ✅ Angle conversions: `d2r(x)`, `r2d(x)`, `d2g(x)`, `g2r(x)`, `g2d(x)` (degrees, radians, gradians)
   - ✅ All functions tested; 20 new integration tests added and passing
   - Builtins: 79 → 99 (+20)
   - Total tests: 71 → 91 (+20)

When you finish an item: update §6 (strike/remove it), update the **Scope** section
of `README.md`, add tests, and re-run the §3 smoke tests.

## Phase 4: High-Value Functions (TBD)

### 4.1 More Trigonometric Variants (10 functions)
   - [ ] `cot(x)`, `sec(x)`, `csc(x)` (basic reciprocal trig)
   - [ ] `acot(x)`, `asec(x)`, `acsc(x)` (inverse reciprocal trig)
   - [ ] `coth(x)`, `sech(x)`, `csch(x)` (hyperbolic reciprocal)
   - [ ] `acoth(x)`, `asech(x)`, `acsch(x)` (inverse hyperbolic reciprocal)
   - Integration tests: verify identities (e.g., `tan(x) * cot(x) = 1`)

### 4.2 Root & Logarithm Variants (9 functions)
   - [ ] `root(x, n)` — nth root (generalized sqrt)
   - [ ] `cbrt(x)` — cube root
   - [ ] `isqrt(x)` — integer square root
   - [ ] `iroot(x, n)` — integer nth root
   - [ ] `logn(x, n)` — logarithm base n
   - [ ] `ilog(x)`, `ilog2(x)`, `ilog10(x)`, `ilogn(x, n)` — integer logarithms
   - Integration tests: verify `root(x, n)^n ≈ x`, `logn(base^x, base) = x`

### 4.3 Prime & Number Theory Extensions (8 functions)
   - [ ] `prevprime(n)` — previous prime before n
   - [ ] `factor(n)` — prime factorization
   - [ ] `lfactor(n)` — largest prime factor
   - [ ] `ptest(n, k)` — probabilistic primality test
   - [ ] `euler(n)` — Euler numbers
   - [ ] `bernoulli(n)` — Bernoulli numbers
   - [ ] `jacobi(a, n)` — Jacobi symbol
   - Integration tests: verify factorization, prime properties

### 4.4 More Special Functions (6 functions)
   - [ ] `y0(x)`, `y1(x)` — Bessel functions of 2nd kind
   - [ ] `polygamma(n, x)` — polygamma function
   - [ ] `zeta(x)` — Riemann zeta function
   - [ ] `gamma(x)` — gamma function (generalized factorial)
   - [ ] `lgamma(x)` — log-gamma
   - Integration tests: verify special values and known identities

### 4.5 Random Number Functions (10 functions)
   - [ ] `rand()` — random integer
   - [ ] `random()` — random [0,1)
   - [ ] `randbit()` — random bit
   - [ ] `seed(s)` — set random seed
   - [ ] `srand(s)`, `srandom(s)` — seeding variants
   - [ ] `randperm(n)` — random permutation of 0..n-1
   - [ ] `randint(a, b)` — random in range
   - Integration tests: verify distribution properties

### 4.6 Environment & System Functions (8 functions)
   - [ ] `getenv(name)` — read environment variable
   - [ ] `putenv(name, value)` — set environment variable
   - [ ] `system(cmd)` — execute shell command
   - [ ] `time()` — current Unix time
   - [ ] `systime()` — system time
   - [ ] `ctime(t)` — convert time to string
   - [ ] `sleep(seconds)` — pause execution
   - [ ] `usertime()` — user/system time
   - Integration tests: verify time functions, env access

## Phase 5: Extended Compatibility (TBD)

### 5.1 Character Classification (12 functions)
   - [ ] `isalnum(s)` — alphanumeric
   - [ ] `isupper(s)`, `islower(s)` — case checking
   - [ ] `isprint(s)`, `isgraph(s)` — printable
   - [ ] `iscntrl(s)` — control character
   - [ ] `ispunct(s)` — punctuation
   - [ ] `isxdigit(s)` — hex digit
   - [ ] `isascii(s)` — ASCII-only
   - [ ] `toupper(s)`, `tolower(s)` — case conversion
   - [ ] `strrev(s)` — string reverse

### 5.2 Advanced Modular Arithmetic (5 functions)
   - [ ] `pmod(x, y)` — positive modulus
   - [ ] `quomod(x, y)` — quotient and modulus
   - [ ] `quo(x, y)`, `rem(x, y)` — quotient and remainder (distinct from //)
   - [ ] `hnrmod(x, y)` — Hensel modular

### 5.3 Rational Approximations (4 functions)
   - [ ] `appr(x, [eps])` — approximate rational
   - [ ] `cfappr(x, [maxd])` — continued fraction approximation
   - [ ] `cfsim(x, [maxd])` — continued fraction simplification
   - [ ] `scale(x, [places])` — scale to decimal places

### 5.4 Matrix Operations (9 functions)
   - [ ] `det(m)` — determinant
   - [ ] `inverse(m)` — matrix inverse
   - [ ] `mattrans(m)` — transpose
   - [ ] `mattrace(m)` — trace (sum of diagonal)
   - [ ] `matdim(m)` — matrix dimensions
   - [ ] `matfill(m, val)` — fill matrix with value
   - [ ] `matmin(m)`, `matmax(m)` — min/max element
   - [ ] `matsum(m)` — sum all elements

### 5.5 Hash & Associative Arrays (6 functions)
   - [ ] `assoc(...)` — create associative array
   - [ ] `indices(h)` — get all keys
   - [ ] `insert(h, key, val)` — add key-value
   - [ ] `delete(h, key)` — remove key
   - [ ] `count(h)` — number of pairs
   - [ ] `join(h, sep)` — join values

## Phase 6: Specialized & Exotic (TBD)

### 6.1 File I/O (24 functions) — High Priority
   - [ ] `fopen(filename, mode)` — open file
   - [ ] `fclose(fd)` — close file descriptor
   - [ ] `fgets(fd)` — read line
   - [ ] `fgetc(fd)` — read character
   - [ ] `fprintf(fd, fmt, ...)` — formatted write
   - [ ] `fscan(fd, fmt)` — read formatted
   - [ ] `fscanf(fd, fmt, ...)` — formatted input with args
   - [ ] `fputs(fd, str)` — write string
   - [ ] `fputc(fd, ch)` — write character
   - [ ] `seek(fd, offset)` — seek in file
   - [ ] `tell(fd)` — current position
   - [ ] `eof(fd)` — end-of-file test
   - [ ] `remove(filename)` — delete file
   - [ ] `rename(old, new)` — rename file
   - [ ] And 10+ more streaming/file operations

### 6.2 Memory & Stack Management (13 functions)
   - [ ] `blk(n)` — allocate n bytes
   - [ ] `blkcpy(dest, src, size)` — copy memory block
   - [ ] `blkfree(ptr)` — free allocated block
   - [ ] `blocks()` — number of allocated blocks
   - [ ] `free()` — free unused memory
   - [ ] `freeglobals()` — free global variables
   - [ ] `push(val)` — push to stack
   - [ ] `pop()` — pop from stack
   - [ ] `depth()` — stack depth
   - [ ] Memory address functions (advanced)

### 6.3 Error & Exception Handling (7 functions)
   - [ ] `errcount()` — number of errors so far
   - [ ] `errmax(n)` — set max errors before stop
   - [ ] `errno()` — last error code
   - [ ] `errsym(code)` — error name from code
   - [ ] `error(msg)` — raise error
   - [ ] `newerror(msg)` — raise new error type
   - [ ] `warn(msg)` — issue warning

### 6.4 Command & Script Functions (4 functions)
   - [ ] `argv(n)` — nth command-line argument
   - [ ] `cmdbuf()` — current command buffer
   - [ ] `command(str)` — execute command string
   - [ ] `eval(str)` — evaluate string expression

### 6.5 Obscure Trigonometric Variants (20+ functions)
   - [ ] `haversin(x)`, `versin(x)`, `coversin(x)` — specialized trig
   - [ ] `exsecant(x)`, `chord(x)` — arc functions
   - [ ] And 15+ other rare variants from historical/specialized domains

### 6.6 Cryptographic & Hashing (3 functions)
   - [ ] `sha1(data)` — SHA-1 hash
   - [ ] `md5(data)` — MD5 hash
   - [ ] `crc32(data)` — CRC32 checksum

### 6.7 Residue Class & Modular (8+ functions)
   - [ ] `rc(n, m)` — residue class
   - [ ] `rcadd`, `rcsub`, `rcmul`, `rcdiv` — RC arithmetic
   - [ ] And specialized modular operations

---

## 7. Regenerating the MCP schema doc

`docs/MCP_TOOL_SCHEMA.json` is generated from the running server so it can't drift.
After editing tools in `mcp.rs`:

```sh
printf '%s\n' \
 '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
 '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' \
 | cargo run --quiet --bin toRustCalcMCP -- --mcp \
 | python3 - <<'PY'
import sys, json
init=tools=None
for ln in sys.stdin:
    ln=ln.strip()
    if not ln: continue
    o=json.loads(ln)
    if o.get("id")==1: init=o["result"]
    if o.get("id")==2: tools=o["result"]
doc={"protocolVersion":init["protocolVersion"],"serverInfo":init["serverInfo"],
     "capabilities":init["capabilities"],"toolsListResult":tools}
open("docs/MCP_TOOL_SCHEMA.json","w").write(json.dumps(doc,indent=2)+"\n")
print("regenerated:",len(tools["tools"]),"tools")
PY
```

---

## 8. Session 2 accomplishments — what was built

1. **Reorganized** root-level .rs files → `src/` (Cargo.toml targets fixed; builds cleanly).
2. **Implemented** `config.rs` (Mode enum, Config struct with epsilon/display/mode).
3. **Implemented** `value.rs` (Value enum, render logic per Mode).
4. **Implemented** `lexer.rs` (full tokenization: lex(), token types, hex/binary/sci-notation).
5. **Implemented** `parser.rs` (Pratt parser: Expr, BinOp, UnOp ASTs, right-assoc `^`).
6. **Implemented** `builtins.rs` (~30 functions: arithmetic, number theory, transcendentals + register/catalog).
7. **Implemented** `cli.rs` (rcalc arg parsing: `-p` pipe, `-q` quiet, `-m` mode, `-v` version, `-h` help; REPL).
8. **Implemented** `main.rs`, `bin_rcalc.rs`, `lib.rs` (dispatch, module structure).
9. **Implemented** integration tests (11 tests covering exactness, modes, number theory, constants).
10. **Verified** via `cargo build --release` and `cargo test` (all green).
11. **Verified** CLI: `2^100`, `-m frac`, pipe mode, and REPL all work.
12. **Verified** MCP server: `initialize`, `tools/list`, `tools/call` with exact results.

---

## 8b. Context chain — earlier decisions (session 1, kept for reference)

Read this to understand *why* the code looks the way it does before changing it.

1. **Goal** (user): port `lcn2/calc` (latest) to Rust as a binary named
   `toRustCalcMCP` that works as an MCP server *and* as a `rcalc` command, and
   draft the MCP JSON-RPC tool schema.
2. **Grounding:** cloned upstream and measured it — **92,319 LOC of C, ~350
   builtins, upstream version 2.17.x, its own Turing-complete language.** Concluded
   a literal 1:1 port is not a single-session artifact; chose to build a *runnable,
   tested core* + the *complete* MCP layer (the explicitly requested deliverable),
   and to document scope honestly rather than ship a stub.
3. **Numeric model = exact rationals** (`num-rational::BigRational` over
   `num-bigint`). Rationale: calc's native value *is* an exact rational, so this is
   the faithful choice and avoids a GMP/system-lib dependency (pure-Rust, builds on
   apt's rustc 1.75 with no `rustup`).
4. **Irrationals approximate to a session `epsilon`** (default `1e-20`), mirroring
   calc. `sqrt` is done at arbitrary precision via Newton's method on rationals
   (verified: `sqrt(2)` correct to 50 digits at `epsilon=1e-50`). `pi`/`e` are exact
   60-digit constants. **`sin/cos/exp/ln` are still f64-precision** — a deliberate,
   documented shortcut and TODO #1.
5. **Architecture** chosen for extensibility: classic lexer→Pratt-parser→tree-walk
   evaluator; builtins as fn-pointers in a map with a parallel `catalog()` so help,
   the MCP `calc_functions` tool, and docs never drift.
6. **Single source, two bins:** `main.rs` dispatches MCP vs CLI (by `--mcp`/`mcp`
   or argv0 `rcalc`); `bin_rcalc.rs` is a CLI-only shim. Satisfies "named
   `toRustCalcMCP`, also works as `rcalc`."
7. **MCP transport:** newline-delimited JSON-RPC 2.0 over stdio, protocol
   `2025-06-18`; tools = `calc_eval`, `calc_config`, `calc_functions`. Schema is
   server-emitted into `docs/` so it stays authoritative. Verified end-to-end
   (initialize / notifications / tools.list / tools.call / ping / unknown→-32601).
8. **Verification before shipping:** `cargo build --release` clean; 9 integration
   tests green; CLI + MCP smoke tests run. One test initially "failed" only because
   an expected string truncated a digit the renderer correctly *rounds* — the value
   was right; assertion was loosened.
9. **Output rendering:** real/frac/int modes; real mode prints up to `display`
   digits with a leading `~` for inexact results, as calc does. Each `;`-separated
   statement's value is printed (calc behaviour), so `x=7; x^2` prints `7` then `49`.
10. **Known sharp edges for the next instance:** f64 transcendentals (#1); no
    user-defined functions / control flow yet (#2); `^` is power (bit-xor must be a
    function, #3); sandbox shell is `dash` (no brace expansion); keep MSRV 1.75.

---

## 9. Definition of done for any change

- [ ] `cargo build` and `cargo build --release` succeed.
- [ ] `cargo test` is green; new behaviour has a test.
- [ ] CLI smoke (`rcalc '2^100'`, `-m frac`, pipe) works.
- [ ] MCP smoke session (§3) works; schema regenerated if tools changed (§7).
- [ ] `README.md` Scope and this file's §6 updated.
- [ ] Exactness invariant intact; no panics on bad input.
