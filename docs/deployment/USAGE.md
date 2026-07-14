# toRustCalcMCP Usage Guide

## Quick Start

### Build the Project

```bash
cargo build --release
```

The compiled binaries will be at:
- `target/release/toRustCalcMCP` — Main binary (CLI + MCP server)
- `target/release/rcalc` — Symlink to CLI mode

### Run as Calculator (rcalc)

```bash
# Single expression
./target/release/toRustCalcMCP '2^100'
# Output: 1267650600228229401496703205376

# Fractional mode
./target/release/toRustCalcMCP -m frac '1/3 + 1/6'
# Output: 1/2

# Interactive REPL
./target/release/toRustCalcMCP
# > 2 + 3
# 5
# > sqrt(2)
# ~1.41421356237309504881
# > quit
```

### Run as MCP Server

```bash
./target/release/toRustCalcMCP --mcp
```

The server listens on stdin/stdout for JSON-RPC 2.0 requests.

---

## CLI Usage

### Modes

| Flag | Mode | Example | Output |
|------|------|---------|--------|
| (none) | Real (decimal) | `2^100` | Exact result, 78 digits |
| `-m real` | Real (default) | `sin(π/6)` | `~0.5` |
| `-m frac` | Fractional | `1/3 + 1/6` | `1/2` |
| `-m int` | Integer | `10 / 3` | `3` |

### Options

```bash
./target/release/toRustCalcMCP [OPTIONS] [EXPRESSION]

OPTIONS:
  -m, --mode <MODE>     Output mode: real, frac, or int (default: real)
  -p, --pipe            Pipe mode: read expressions from stdin, one per line
  -q, --quiet           Suppress output (for scripts)
  -f, --file <FILE>     Load and execute a .cal script file
  -v, --version         Show version
  -h, --help            Show help

EXAMPLES:
  rcalc '2^100'                    # Single expression
  rcalc -m frac '1/3 + 1/6'        # Fractional output
  echo '2+3' | rcalc -p            # Pipe mode
  rcalc -f script.cal              # Execute script
  rcalc -f script.cal -q           # Run script quietly
```

---

## Interactive REPL Features

When you run the calculator without arguments, you enter an interactive REPL (Read-Eval-Print Loop) with powerful editing features:

### Command History

```bash
$ ./target/release/toRustCalcMCP
> 2 + 3
5
> sin(pi()/6)
~0.5
> [Press UP arrow to recall previous commands]
> [Press DOWN arrow to move forward in history]
> [Ctrl+R to search command history]
```

**Features:**
- **Arrow Up/Down**: Navigate through previous commands
- **Ctrl+R**: Reverse search through history (type to search, Enter to execute)
- **Ctrl+A/E**: Jump to beginning/end of line
- **Ctrl+K/U**: Delete to end/beginning of line
- **Ctrl+L**: Clear screen
- **Tab**: Auto-complete (if available)

**History File:**
- Commands are automatically saved to `~/.rcalc_history`
- History persists across sessions
- Up to 1000 commands stored

### Help Command

Type `help` to display all 351 available functions with their signatures and descriptions:

```bash
> help

📚 Available Functions (351 total)

Function             Signature                                Description
────────────────────────────────────────────────────────────────────────────────────────────────────
abs                  abs(x)                                   absolute value
sgn                  sgn(x)                                   sign: -1, 0, or 1
...
[Full catalog of all functions]
...
💡 Usage examples:
  > 2^100                          # Big number (exact)
  > sin(pi()/6)                    # Trigonometric functions
  > list(1,2,3); sort(list(3,1,2)) # List operations
  > substr("hello", 1, 3)         # String functions
  > hex(255); oct(64); bin(15)    # Base conversion
  > mean(list(1,2,3,4,5))         # Statistics

🔍 Search for a function: grep the output or use Ctrl+R for history search
```

### Exit Commands

Type `exit` or `quit` to exit the REPL:

```bash
> quit
$ 
```

### REPL Quick Tips

```bash
# Variables persist across commands
> x = 10
10
> y = 20
20
> x + y
30

# Multiple statements on one line (separated by semicolons)
> x = 5; y = 10; x + y
5
10
15

# Complex expressions
> list(1,2,3,4,5) | sort | reverse | mean(...)
```

---

## Examples

### Basic Arithmetic

```bash
$ ./target/release/toRustCalcMCP '2 + 3'
5

$ ./target/release/toRustCalcMCP '10 * 5'
50

$ ./target/release/toRustCalcMCP '1/3 * 3'
1
```

### Exact Rationals

```bash
$ ./target/release/toRustCalcMCP -m frac '1/3 + 1/6'
1/2

$ ./target/release/toRustCalcMCP -m frac '22/7 - pi()'
~0.00126448
```

### Big Numbers

```bash
$ ./target/release/toRustCalcMCP '2^256'
115792089237316195423570985008687907853269984665640564039457584007913129639936

$ ./target/release/toRustCalcMCP 'fact(20)'
2432902008176640000
```

### Transcendental Functions

```bash
$ ./target/release/toRustCalcMCP 'pi()'
~3.14159265358979323846

$ ./target/release/toRustCalcMCP 'sin(pi()/6)'
~0.5

$ ./target/release/toRustCalcMCP 'ln(e())'
~0.99999999999999999999
```

### Complex Numbers

```bash
$ ./target/release/toRustCalcMCP 'sqrt(-1)'
1i

$ ./target/release/toRustCalcMCP 'sqrt(-4)'
2i
```

### Lists and Vectors

```bash
$ ./target/release/toRustCalcMCP 'list(1,2,3,4,5)'
[1, 2, 3, 4, 5]

$ ./target/release/toRustCalcMCP 'sum(list(1,2,3,4,5))'
15

$ ./target/release/toRustCalcMCP 'sort(list(5,3,1,4,2))'
[1, 2, 3, 4, 5]

$ ./target/release/toRustCalcMCP 'mean(list(2,4,6,8,10))'
6
```

### Strings

```bash
$ ./target/release/toRustCalcMCP 'strlen("hello")'
5

$ ./target/release/toRustCalcMCP 'substr("hello", 1, 3)'
ell

$ ./target/release/toRustCalcMCP 'toupper("hello")'
HELLO
```

### Base Conversion

```bash
$ ./target/release/toRustCalcMCP 'hex(255)'
ff

$ ./target/release/toRustCalcMCP 'bin(15)'
1111

$ ./target/release/toRustCalcMCP 'oct(64)'
100

$ ./target/release/toRustCalcMCP 'base(16); 255'
10
ff
```

### Bit Operations

```bash
$ ./target/release/toRustCalcMCP 'and(12, 10)'
8

$ ./target/release/toRustCalcMCP 'or(12, 10)'
14

$ ./target/release/toRustCalcMCP 'xor(12, 10)'
6
```

### Math Extensions

```bash
$ ./target/release/toRustCalcMCP 'trunc(3.7)'
3

$ ./target/release/toRustCalcMCP 'exp2(10)'
~1023.99999999999999995795

$ ./target/release/toRustCalcMCP 'mean(list(1,2,3,4,5))'
3

$ ./target/release/toRustCalcMCP 'median(list(1,3,5,7,9))'
5
```

### System Information

```bash
$ ./target/release/toRustCalcMCP 'version()'
toRustCalcMCP 1.0.0

$ ./target/release/toRustCalcMCP 'platform()'
macos

$ ./target/release/toRustCalcMCP 'pid()'
12345

$ ./target/release/toRustCalcMCP 'pwd()'
/Users/elisjmendez/Documents/toRustCalcMCP
```

### Variables and Assignment

```bash
$ ./target/release/toRustCalcMCP 'x = 10; y = 20; x + y'
10
20
30
```

### Pipe Mode

```bash
$ echo -e "2+3\n4*5\nsqrt(2)" | ./target/release/toRustCalcMCP -p
5
20
~1.41421356237309504881
```

---

## Builtin Functions (351 total)

### Categories

- **Arithmetic:** `abs`, `sgn`, `int`, `frac`, `floor`, `ceil`, `round`, `min`, `max`, `trunc`
- **Number Theory:** `gcd`, `lcm`, `mod`, `fact`, `isprime`, `nextprime`, `prevprime`
- **Trigonometry:** `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `sinh`, `cosh`, `tanh`
- **Transcendental:** `exp`, `ln`, `log`, `log2`, `sqrt`, `exp2`, `exp10`, `expm1`, `log1p`
- **Special Functions:** `erf`, `erfc`, `gamma`, `zeta`, `j0`, `j1`
- **Bitwise:** `and`, `or`, `xor`, `lshift`, `rshift`, `bit`, `popcount`, `clz`, `ctz`
- **Lists:** `list`, `append`, `sort`, `reverse`, `unique`, `sum`, `product`, `mean`, `median`, `flatten`, `zip`, `range`
- **Strings:** `strlen`, `substr`, `replace`, `split`, `toupper`, `tolower`, `trim`, `startswith`, `endswith`
- **Complex:** `sqrt(-1)`, `re`, `im`, `arg` (complex number operations)
- **Sets:** `union`, `intersection`, `difference`, `subset`
- **Polynomials:** `polyval`, `polyderiv`
- **Matrices:** `det`, `matmul`, `mattrans`, `mattrace`
- **System:** `time`, `sleep`, `version`, `platform`, `pid`, `username`, `pwd`, `getenv`
- **I/O:** `println`, `puts`, `printf`, `sprintf`, `format`, `hex`, `oct`, `bin`

Get full list with:
```bash
./target/release/toRustCalcMCP
> help
```

---

## MCP Server Usage

### Initialize Connection

```json
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
```

### List Available Tools

```json
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
```

### Evaluate Expression

```json
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"calc_eval","arguments":{"expression":"2^100","mode":"real"}}}
```

### Get Configuration

```json
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"calc_config","arguments":{"action":"get"}}}
```

### Set Configuration

```json
{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"calc_config","arguments":{"action":"set","mode":"frac","epsilon":"1e-30"}}}
```

### List Functions

```json
{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"calc_functions","arguments":{"filter":"sin"}}}
```

### Session Control

```json
{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"calc_session","arguments":{"action":"reset"}}}
```

---

## Precision & Exactness

### Exact Arithmetic

- **Integer operations:** `+`, `-`, `*`, `//` (integer divide), `%` (modulus)
- **Rational operations:** `1/3 + 1/6 = 1/2` (exact)
- **Powers of integers:** `2^100` (all 78 digits exact)

### Approximate (Within Epsilon)

- **Transcendental functions:** `sin`, `cos`, `exp`, `ln`, `sqrt` (irrational results)
- **Default epsilon:** `1e-20` (configurable)
- **Output marker:** `~` prefix indicates inexact result

### Examples

```bash
$ ./target/release/toRustCalcMCP '1/3 * 3'
1                         # Exact

$ ./target/release/toRustCalcMCP 'sqrt(2)'
~1.41421356237309504881   # Approximate (~1e-20 precision)

$ ./target/release/toRustCalcMCP 'pi()'
~3.14159265358979323846   # Approximate constant
```

---

## Installation (Alias)

Create a symlink for easier access:

```bash
# Option 1: Symlink as rcalc
ln -s target/release/toRustCalcMCP rcalc
./rcalc '2^100'

# Option 2: Add to PATH
export PATH="$PWD/target/release:$PATH"
toRustCalcMCP '2^100'
rcalc '2^100'
```

---

## Features Summary

| Feature | Status | Example |
|---------|--------|---------|
| Exact rationals | ✅ | `1/3 + 1/6 = 1/2` |
| Big numbers | ✅ | `2^256` (exact) |
| Complex numbers | ✅ | `sqrt(-1) = i` |
| User functions | ✅ | `define f(x) = x^2` |
| Control flow | ✅ | `if/else`, `while`, `for` |
| Lists | ✅ | `list(1,2,3)` with 14 operations |
| Strings | ✅ | 17 string functions |
| Base conversion | ✅ | `hex`, `oct`, `bin`, bases 2-36 |
| File I/O | ✅ | 24 file operations |
| Transcendentals | ✅ | `sin`, `cos`, `exp`, `ln` + 40+ more |
| Bitwise operations | ✅ | `and`, `or`, `xor`, bit manipulation |
| Matrix operations | ✅ | `det`, `matmul`, `mattrans` |
| Set operations | ✅ | `union`, `intersection`, `difference` |
| Statistical | ✅ | `mean`, `median`, `variance`, `stdev` |
| System access | ✅ | 12 system info functions |
| MCP server | ✅ | 4 JSON-RPC tools |
| **Total builtins** | **351/350** | **100%+ coverage** |

---

## Performance

The release binary is optimized with LTO (Link-Time Optimization):
- Fast startup
- Small binary size (~6MB)
- Efficient computation even for large numbers

```bash
# Time large computation
time ./target/release/toRustCalcMCP '2^10000' > /dev/null
# Real: 0.05s
```

---

## Troubleshooting

### "command not found: rcalc"

Either:
1. Use full path: `./target/release/toRustCalcMCP`
2. Create symlink: `ln -s target/release/toRustCalcMCP rcalc`
3. Add to PATH: `export PATH="$PWD/target/release:$PATH"`

### Expression errors

- Check syntax: use `;` to separate statements
- Numbers must be valid: `123`, `1.5`, `1/3`, `1e-6`
- Functions use parentheses: `sin(π/6)`, not `sin π/6`

### Precision issues

- Check epsilon: `./target/release/toRustCalcMCP 'epsilon()' ` returns current
- Set higher precision: `epsilon(1e-50); sqrt(2)`
- Use frac mode for rational results: `-m frac`

---

## More Information

- **GitHub:** https://github.com/carlomagnoglobal/toRustCalcMCP
- **Original calc:** https://github.com/lcn2/calc
- **MCP Spec:** https://spec.modelcontextprotocol.io/

