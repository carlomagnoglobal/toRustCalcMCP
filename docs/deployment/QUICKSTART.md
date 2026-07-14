# Quick Start

## Compile

```bash
cargo build --release
```

Binary location: `target/release/toRustCalcMCP`

## Run as Calculator

```bash
# Single expression
./target/release/toRustCalcMCP '2^100'

# Interactive mode
./target/release/toRustCalcMCP

# Fractional output
./target/release/toRustCalcMCP -m frac '1/3 + 1/6'

# Pipe mode
echo '2+3' | ./target/release/toRustCalcMCP -p
```

## Run as MCP Server

```bash
./target/release/toRustCalcMCP --mcp
```

## Common Operations

| Operation | Command |
|-----------|---------|
| Arithmetic | `'2+3'`, `'10*5'`, `'100/7'` |
| Powers | `'2^256'`, `'10^-5'` |
| Functions | `'sin(pi()/6)'`, `'sqrt(2)'`, `'ln(e())'` |
| Variables | `'x=5; y=10; x+y'` |
| Lists | `'list(1,2,3,4,5)'`, `'sum(list(1,2,3))'` |
| Strings | `'strlen("hello")'`, `'substr("hello",0,2)'` |
| Bases | `'hex(255)'`, `'bin(15)'`, `'oct(64)'` |

## Features

- ✅ **351 builtins** (100% calc compatibility)
- ✅ **Exact arithmetic** (1/3 * 3 = exactly 1)
- ✅ **Big numbers** (2^256 computed to the last digit)
- ✅ **Complex numbers** (sqrt(-1) = i)
- ✅ **Lists, strings, matrices**
- ✅ **User functions & control flow**
- ✅ **File I/O, system access**
- ✅ **MCP server** (JSON-RPC 2.0 over stdio)

## REPL Interactive Features

When you run without arguments, you get a full-featured REPL:

```bash
$ ./target/release/toRustCalcMCP
> help                          # Show all 351 functions
> 2 + 3                         # Calculate
5
> [UP arrow]                    # Recall previous commands
> [Ctrl+R]                      # Search history
> quit                          # Exit
```

## Full Documentation

See `USAGE.md` for comprehensive guide with 50+ examples.
