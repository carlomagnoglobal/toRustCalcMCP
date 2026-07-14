# Getting Started with rcalc

**rcalc** is an arbitrary-precision calculator — exact math with no rounding errors.

## Install (1 minute)

### From source:
```bash
git clone https://github.com/carlomagnoglobal/toRustCalcMCP.git
cd toRustCalcMCP
cargo build --release
./target/release/toRustCalcMCP '2^100'
```

### Via Homebrew (macOS):
```bash
# coming soon
```

### Via cargo:
```bash
# coming soon
```

## Try It (2 minutes)

### Browser-based REPL (Easiest):
```bash
./target/release/rcalc-web
# Then open: http://localhost:8888
```
Type in the browser: `2^100`, `1/3 + 1/6`, `sin(pi()/6)`, etc.
Use arrow keys for command history. Ctrl+L to clear.

### Interactive mode (Terminal):
```bash
./target/release/toRustCalcMCP
> 2 + 3
5
> 1/3 + 1/6
~0.5
> sqrt(2)
~1.41421356237309504881
> quit
```

### One-liner:
```bash
./target/release/toRustCalcMCP '2^256'
# Output: exact 78-digit number (no approximation!)
```

### Exact rationals:
```bash
./target/release/toRustCalcMCP -m frac '1/3 * 3'
# Output: 1 (exactly, not 0.999...)
```

### Pipe mode:
```bash
echo -e '10 + 5\n20 * 3\nsqrt(2)' | ./target/release/toRustCalcMCP -p
```

### Script file:
```bash
# Create math.cal with:
#   x = 10
#   y = 20
#   x + y

./target/release/toRustCalcMCP -f math.cal
```

## Key Features

✅ **Exact arithmetic** — `1/3 * 3 = 1` exactly, no rounding  
✅ **Big numbers** — `2^256` computed to the last digit  
✅ **Complex numbers** — `sqrt(-1) = i`  
✅ **351 builtins** — math, strings, lists, file I/O, and more  
✅ **User functions** — `define f(x) = x^2`  
✅ **MCP server** — JSON-RPC 2.0 for AI integration  

## Next Steps

- **Interactive help:** Type `help intro` in the REPL
- **Full guide:** See [USAGE.md](USAGE.md) (250+ examples)
- **Quick reference:** See [QUICKSTART.md](QUICKSTART.md)

## Common Commands

| Task | Command |
|------|---------|
| Basic math | `./target/release/toRustCalcMCP '10 + 5'` |
| Fractions | `./target/release/toRustCalcMCP -m frac '1/3 + 1/6'` |
| Functions | `./target/release/toRustCalcMCP 'sin(pi()/6)'` |
| Lists | `./target/release/toRustCalcMCP 'list(1,2,3) \| sort'` |
| Help | `./target/release/toRustCalcMCP` then `help` |
| Version | `./target/release/toRustCalcMCP -v` |

## Example: Fibonacci

```bash
./target/release/toRustCalcMCP << 'EOF'
define fib(n) {
    if (n <= 1) return n
    return fib(n-1) + fib(n-2)
}
fib(10)
EOF
```

Output: `55`

---

**Questions?** Check the interactive `help` system or read [USAGE.md](USAGE.md).
