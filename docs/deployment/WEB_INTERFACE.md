# Web Interface Guide

The rcalc web interface is a browser-based REPL that provides an interactive calculator in your web browser.

## Quick Start

```bash
cargo build --release
./target/release/rcalc-web
```

Then open your browser to **http://localhost:8888**

## Features

### ✅ Core Features
- **Full calc compatibility** — all 351 builtins available
- **Real-time evaluation** — see results instantly
- **Command history** — use ↑/↓ arrow keys to navigate previous commands
- **Syntax highlighting** — color-coded expressions and results
- **Error handling** — clear error messages in red
- **Responsive design** — works on desktop, tablet, and mobile

### ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Enter` | Execute expression |
| `↑` / `↓` | Navigate command history |
| `Ctrl+L` | Clear output |

## Example Calculations

```
> 2^256
115792089237316195423570985008687907853269984665640564039457584007913129639936

> 1/3 + 1/6
0.5

> sin(pi()/6)
~0.5

> list(1,2,3,4,5)
[1, 2, 3, 4, 5]

> sort(list(5,3,1,4,2))
[1, 2, 3, 4, 5]

> gcd(462, 1071)
21

> sqrt(2)
~1.41421356237309504881
```

## Interface Layout

**Header:** Project title and description

**REPL Area:**
- **Output section** — displays calculation history with expressions and results
- **Input field** — enter expressions here with `>` prompt

**Info Panels:**
- **💡 Quick Tips** — common examples
- **⌨️ Keyboard Shortcuts** — available shortcuts

**Footer:** Credits and links

## Tips & Tricks

### Exact Rationals
The calculator performs exact rational arithmetic:
```
> 1/3 * 3
1
```

This is exact — no rounding error! Compare to typical calculators that might give 0.9999999...

### Big Numbers
Calculate huge numbers exactly:
```
> 2^100
1267650600228229401496703205376

> fact(50)
30414093201713378043612608166064768844377641568960512000000000000
```

### Functions
Define and call your own functions:
```
> define double(x) = x * 2
> double(5)
10
```

### Lists
Create and manipulate lists:
```
> x = list(3,1,4,1,5,9,2,6)
> sort(x)
[1, 1, 2, 3, 4, 5, 6, 9]

> sum(x)
33

> mean(x)
~4.125
```

### Help
Get documentation on topics:
```
> help intro
[Shows introduction to rcalc]

> help sin
[Shows all sine-related functions]
```

## Technical Details

**Architecture:**
- Lightweight HTTP server (tiny_http crate)
- Single shared calculator instance
- Synchronous request handling
- No external dependencies for frontend

**Session State:**
- Variables and functions persist across calculations
- Configuration (precision, mode) applies to all subsequent calculations
- Clear output with Ctrl+L without resetting session

**Browser Support:**
- Modern browsers (Chrome, Firefox, Safari, Edge)
- Responsive layout — works on any screen size
- No external CDN dependencies — all assets served locally

## Running Multiple Instances

You can run multiple instances on different ports:

```bash
# Terminal 1
./target/release/rcalc-web  # port 8888

# Terminal 2 (would need to modify source to use different port)
# Currently hardcoded to 8888
```

To use different ports, you can modify the port in `src/bin_web.rs` and rebuild.

## Advanced Usage

### Session Variables
All variables persist within a browser session:
```
> x = 10
10
> y = 20
20
> x + y
30
```

### Complex Numbers
```
> sqrt(-1)
1i

> (1 + i) * (2 - i)
3+1i
```

### Base Conversion
```
> hex(255)
ff

> bin(255)
11111111

> oct(64)
100
```

## Troubleshooting

**"Connection refused" error when opening the browser**
- Make sure the server is running: `./target/release/rcalc-web`
- Check the port is correct: http://localhost:8888 (not 8889, etc.)
- Wait 1-2 seconds after starting the server before opening the browser

**History not working**
- Use arrow UP/DOWN keys, not left/right
- History is per-browser session; refresh clears it

**Results showing "~" prefix**
- This indicates an irrational or inexact result
- The `~` is calc's standard notation for approximations
- Use `-m frac` mode or check precision settings

## See Also

- [GETTING_STARTED.md](GETTING_STARTED.md) — quick start guide
- [USAGE.md](USAGE.md) — comprehensive usage guide
- [QUICKSTART.md](QUICKSTART.md) — quick reference
- [README.md](../../README.md) — project overview
