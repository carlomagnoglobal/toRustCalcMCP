# Introduction to rcalc

**rcalc** is a Rust port of the arbitrary-precision calculator `calc` (by Landon Curt Noll). It performs exact arithmetic on integers and rational numbers, with support for complex numbers, transcendental functions, user-defined functions, lists, strings, matrices, and more.

## Why rcalc?

Most calculators use floating-point math, which has rounding errors:

```
3 × (1 ÷ 3) = 0.999999...
```

**rcalc uses exact rational arithmetic:**

```
3 × (1 ÷ 3) = 1 (exactly)
2^256 = 115792089237316195423570985008687907853269984665640564039457584007913129639936
```

No approximations. No rounding errors.

## Key Features

### 📐 Exact Arithmetic
- Integer operations: exact, no overflow
- Rationals: `1/3 + 1/6 = 1/2` exactly
- Big numbers: `2^1000` computed to the last digit
- Complex numbers: `sqrt(-1) = i`

### 🔧 351 Builtin Functions
- **Math:** sin, cos, exp, ln, sqrt, gcd, lcm, factorial, prime operations, special functions
- **Strings:** substring, replace, split, trim, case conversion (17 functions)
- **Lists:** sort, reverse, flatten, zip, range, statistical (14 functions)
- **Matrices:** determinant, transpose, multiplication
- **System:** time, hostname, pid, environment variables

### 💻 Two Interfaces

**Command-line calculator:**
```bash
rcalc '2^256'
rcalc -m frac '1/3 + 1/6'
echo '10 + 5' | rcalc -p
```

**MCP server (for AI/LLMs):**
```bash
rcalc --mcp
```
Provides JSON-RPC 2.0 interface for integration with AI tools.

### 👤 User-Defined Functions
```
define fib(n) {
    if (n <= 1) return n
    return fib(n-1) + fib(n-2)
}
fib(10)  → 55
```

### 🎯 Control Flow
- `if / else` conditionals
- `while` loops
- `for` loops
- Variable assignment & scoping

## What You Can Do

### Calculate Exact Results
```
1/3 * 3 = 1
gcd(462, 1071) = 21
```

### Work with Big Numbers
```
2^256 = 115792089237316195423570985008687907853269984665640564039457584007913129639936
factorial(50) = exact 64-digit result
```

### Use Advanced Math
```
sin(π/6) ≈ 0.5
sqrt(-4) = 2i
polyval([1,2,3], 5) = polynomial evaluation
```

### Manipulate Data
```
sort([3,1,2]) = [1,2,3]
mean([1,2,3,4,5]) = 3
split("a,b,c", ",") = ["a","b","c"]
```

### Write Scripts
```
define double(x) = x * 2
x = 10
double(x) + 5
```

## Quick Navigation

- **New to rcalc?** Start with [Quick Start](quick-start.md)
- **Want help with a topic?** See [Help Topics](topics/intro.md)
- **Need a specific function?** Check [All Functions](functions.md)
- **See examples?** Browse [Examples](examples.md)

---

## Next Steps

→ [Quick Start in 2 minutes](quick-start.md)

→ [Full Usage Guide](topics/usage.md)

→ [Browse All 351 Functions](functions.md)
