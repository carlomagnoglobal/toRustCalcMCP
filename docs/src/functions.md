# Complete Function Reference

rcalc includes **351 builtin functions** covering:

- **Arithmetic & Math** — 60+ functions
- **Strings** — 17 functions  
- **Lists** — 14 functions
- **Matrices** — 8 functions
- **Sets** — 7 functions
- **File I/O** — 24 functions
- **System & Environment** — 12 functions
- **Type & Introspection** — 8 functions
- **And many more...**

## Quick Search

Use `help <name>` in the interactive mode to search:

```bash
rcalc
> help sin           # Find all sine-related functions (19 results)
> help list          # Find all list functions
> help string        # Find all string functions
```

## By Category

See the help topics for detailed information:

- [Builtin Functions](topics/builtin.md) — Overview and categories
- [Math Functions](topics/number.md) — Number operations
- [String Functions](topics/string.md) — Text manipulation
- [List Functions](topics/list.md) — Collections
- [File I/O](topics/file.md) — File operations
- [System Access](topics/config.md) — System functions

## All Functions

For the complete list of all 351 functions with signatures and descriptions, run:

```bash
rcalc
> help
```

This will display:
- Topic list
- All 351 builtin functions
- Search guide

## Examples by Function Type

### Math
```bash
rcalc 'sin(pi()/6)'        # sin(π/6) ≈ 0.5
rcalc 'gcd(462, 1071)'     # 21
rcalc 'factorial(10)'      # 3628800
rcalc 'sqrt(2)'            # ~1.414...
rcalc 'isprime(1000003)'   # 1
```

### Strings
```bash
rcalc 'strlen("hello")'            # 5
rcalc 'substr("hello", 1, 3)'      # "ell"
rcalc 'toupper("hello")'           # "HELLO"
rcalc 'split("a,b,c", ",")'        # ["a","b","c"]
```

### Lists
```bash
rcalc 'sort(list(3,1,2))'          # [1,2,3]
rcalc 'sum(list(1,2,3,4,5))'       # 15
rcalc 'mean(list(1,2,3,4,5))'      # 3
rcalc 'zip(list(1,2), list("a","b"))' # [[1,"a"],[2,"b"]]
```

### Type Checking
```bash
rcalc 'typeof(5)'                  # "number"
rcalc 'typeof("hello")'            # "string"
rcalc 'typeof(list(1,2))'          # "list"
```

---

For more detailed information, see [Builtin Functions](topics/builtin.md).
