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

Current status: **a faithful core, not 1:1 parity.** Upstream calc is ~92k lines
of C with ~350 builtins and a Turing-complete language. We implemented the
exact-rational engine, the expression language, ~40 builtins, the CLI, and the
**complete** MCP layer + schema. The build is clean and `cargo test` is green
(9 integration tests). Remaining work is enumerated in §6.

---

## 2. Environment / setup notes (read before building)

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

## 4. Architecture map (`src/`)

Pipeline: **lexer → parser (AST) → eval** over a shared `Interp`. The CLI and MCP
layers are thin wrappers around `Interp::eval_render`.

| file | responsibility |
|------|----------------|
| `number.rs` | numeric core. `Num = BigRational`. parsing, `pow`/`pow_int`, arbitrary-precision `sqrt` (Newton), `round_to_epsilon`, decimal rendering (`~` marks inexact). **Start here for precision work.** |
| `value.rs` | `Value` enum (`Number`/`Str`/`Null`) + render-by-`Config`. |
| `config.rs` | `Config { epsilon, display, mode }` + `Mode {Real,Frac,Int}`. |
| `lexer.rs` | `lex(&str) -> Vec<Tok>`. `**`→`^`, `//`, comments (`#`), strings, `0x/0b`, sci-notation. |
| `parser.rs` | AST (`Expr`, `BinOp`, `UnOp`) + Pratt parser. Binding powers live in `infix_bp`; `^` is right-assoc. |
| `eval.rs` | `Interp` (config + vars + builtins map). `eval`, `eval_all` (per-statement values), `eval_render`. **Add control flow / user functions here + parser.** |
| `builtins.rs` | ~40 builtins as `fn(&mut Interp,&[Value])->Result<Value,String>`, the `register()` map, and `catalog()` (drives help + `calc_functions`). **Add new builtins here.** |
| `cli.rs` | `rcalc` arg parsing (calc-style flags) + REPL/pipe. |
| `mcp.rs` | JSON-RPC 2.0 stdio loop, handshake, `tools/list` schema (`tools_list_result`), `tools/call` dispatch. **Edit schema + tools here.** |
| `main.rs` | dispatch: `--mcp`/`mcp` → server; else CLI (also CLI when argv0 == `rcalc`). |
| `bin_rcalc.rs` | thin `rcalc` binary → always CLI. |
| `tests/integration.rs` | exactness, number theory, sqrt precision, MCP handshake/dispatch/schema. |
| `docs/MCP_TOOL_SCHEMA.json` | authoritative schema, **emitted by the server** (regenerate, don't hand-edit — see §7). |

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

1. **Arbitrary-precision transcendentals** — replace the f64 fallback for
   `exp/ln/sin/cos/tan/...`.
   - Where: `builtins.rs` (`f64fn!` macro + `via_f64`); add series/CORDIC helpers
     in `number.rs` honoring `cfg.epsilon` (model them on the existing `sqrt`).
   - Done when: `rcalc 'exp(1)'` matches `e()` to `display` digits; a test pins
     `sin(pi()/6)`/`ln(e())`/`exp(0)` at high `epsilon`.

2. **User-defined functions + control flow** (`define f(x)=…`, `if/for/while`,
   blocks `{…}`, `print`).
   - Where: extend `Tok`/`Expr` (`lexer.rs`,`parser.rs`); add a `Func` value and a
     call frame / scoped env in `eval.rs`; statements return values per calc.
   - Done when: `define sq(x)=x^2; sq(9)` → 81; a `for`-loop sum test passes; the
     REPL supports multi-line `define`.

3. **Integer / bitwise builtins** (`and,or,xor,comp,shift,bit,highbit,lowbit,
   digits,places,fcnt,...`).
   - Where: `builtins.rs` (+ `catalog`). Note: in calc `^` is **power**; bit-xor is
     a function, not the operator — keep that.
   - Done when: parity-checked against upstream `calc` for a sampled table; tests added.

4. **`-f file.cal` resource loading** for the CLI.
   - Where: `cli.rs` (read file → `Interp::eval_all`); honor `-s`/`-q` interplay.
   - Done when: a small `.cal` script with `define` + loop runs and prints expected output.

5. **More of the type system: lists & associative arrays** (`list()`, `[]`,
   `append`, `size`, `mat`/matrices later).
   - Where: new `Value` variants + indexing in `parser.rs`/`eval.rs`; builtins.
   - Done when: `x=list(1,2,3); append(x,4); size(x)` → 4.

6. **Complex numbers** (`a+bi`, `re`, `im`, `arg`, complex `sqrt`).
   - Where: extend `Num`/`Value` (or add `Complex`); thread through ops.
   - Done when: `sqrt(-1)` → `i`; arithmetic on complex values has tests.

7. **Display/base faithfulness** (`base()`/`obase`, `config()` surface, exact `~`
   semantics, scientific output) to match calc output byte-for-byte where sane.
   - Where: `value.rs`/`number.rs` rendering; `config.rs`; expose via `calc_config`.

8. **Broaden MCP** (optional): a `calc_session` reset tool; structured (JSON)
   results alongside text; resources for the function catalog.
   - Where: `mcp.rs` (`tools_list_result` + `handle_tool_call`); regenerate schema (§7).

When you finish an item: update §6 (strike/remove it), update the **Scope** section
of `README.md`, add tests, and re-run the §3 smoke tests.

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

## 8. Context chain — decisions made this session (and why)

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
