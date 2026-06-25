# Quick Start (2 minutes)

## Install

```bash
cargo build --release
# Binary: target/release/toRustCalcMCP
```

## Run

### Single calculation
```bash
./target/release/toRustCalcMCP '2^100'
# Output: 1267650600228229401496703205376
```

### Interactive mode
```bash
./target/release/toRustCalcMCP
> 2 + 3
5
> sqrt(2)
~1.41421356237309504881
> quit
```

### Different output modes
```bash
# Exact rationals
./target/release/toRustCalcMCP -m frac '1/3 + 1/6'
# Output: 1/2

# Integer truncation
./target/release/toRustCalcMCP -m int '10 / 3'
# Output: 3
```

### Pipe mode
```bash
echo '2+3' | ./target/release/toRustCalcMCP -p
# Output: 5
```

## 5 Quick Examples

```bash
# Exact arithmetic
./target/release/toRustCalcMCP '1/3 * 3'     # 1 (exactly)

# Big numbers
./target/release/toRustCalcMCP '2^256'       # exact 78-digit number

# Math functions
./target/release/toRustCalcMCP 'sin(pi()/6)' # ~0.5

# Lists
./target/release/toRustCalcMCP 'sort(list(3,1,2))' # [1, 2, 3]

# Functions
./target/release/toRustCalcMCP 'gcd(462, 1071)'    # 21
```

## Get Help

```bash
# In interactive mode
> help              # Show all topics
> help intro        # Introduction
> help sin          # Search for "sin" (19 results)
```

## More Information

- [Full User Guide](topics/usage.md)
- [All 351 Functions](functions.md)
- [Help Topics](topics/intro.md)
