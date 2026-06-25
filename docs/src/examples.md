# Examples

## Basic Calculations

```bash
# Simple arithmetic
rcalc '2 + 3'                          # 5
rcalc '10 * 5'                         # 50
rcalc '100 / 7'                        # ~14.28571...

# Exact rationals
rcalc -m frac '1/3 + 1/6'             # 1/2
rcalc -m frac '22/7 - pi()'           # ~0.00126448...
```

## Big Numbers (Exact)

```bash
# 2^256 computed exactly
rcalc '2^256'
# Output: 115792089237316195423570985008687907853269984665640564039457584007913129639936

# Factorial (exact)
rcalc 'fact(30)'
# Output: 265252859812191058636308480000000
```

## Math Functions

```bash
# Trigonometry
rcalc 'sin(pi()/6)'                    # ~0.5
rcalc 'cos(0)'                         # 1
rcalc 'atan2(1, 1)'                    # ~0.785398... (π/4)

# Logarithms
rcalc 'ln(e())'                        # ~1
rcalc 'log(1000)'                      # ~3
rcalc 'log2(8)'                        # 3

# Special functions
rcalc 'gamma(5)'                       # 24 (4!)
rcalc 'erf(1)'                         # ~0.8427...
```

## Complex Numbers

```bash
# Square root of negative
rcalc 'sqrt(-1)'                       # 1i
rcalc 'sqrt(-4)'                       # 2i

# Complex operations
rcalc 'sqrt(-1) * sqrt(-1)'            # -1
rcalc 're(2 + 3i)'                     # 2 (real part)
rcalc 'im(2 + 3i)'                     # 3 (imaginary part)
```

## Lists & Collections

```bash
# Create and manipulate lists
rcalc 'list(1,2,3,4,5)'                # [1, 2, 3, 4, 5]
rcalc 'sort(list(5,3,1,4,2))'          # [1, 2, 3, 4, 5]
rcalc 'reverse(list(1,2,3))'           # [3, 2, 1]

# List operations
rcalc 'sum(list(1,2,3,4,5))'           # 15
rcalc 'mean(list(1,2,3,4,5))'          # 3
rcalc 'max(list(1,5,3,2,4))'           # 5

# Searching
rcalc 'find(list(10,20,30), 20)'       # 1
rcalc 'contains(list(1,2,3), 2)'       # 1
```

## Strings

```bash
# String operations
rcalc 'strlen("hello")'                # 5
rcalc 'substr("hello", 0, 3)'          # "hel"
rcalc 'index("hello world", "world")'  # 6

# Transformations
rcalc 'toupper("hello")'               # "HELLO"
rcalc 'tolower("HELLO")'               # "hello"
rcalc 'replace("hello", "l", "L")'     # "heLLo"

# Splitting & joining
rcalc 'split("a,b,c", ",")'            # ["a", "b", "c"]
```

## Number Theory

```bash
# Prime operations
rcalc 'isprime(997)'                   # 1
rcalc 'nextprime(1000)'                # 1009
rcalc 'prevprime(1000)'                # 997

# Factorization
rcalc 'factor(60)'                     # [2, 2, 3, 5]
rcalc 'gcd(462, 1071)'                 # 21
rcalc 'lcm(12, 18)'                    # 36
```

## Base Conversion

```bash
# Hexadecimal
rcalc 'hex(255)'                       # ff

# Binary
rcalc 'bin(15)'                        # 1111

# Octal
rcalc 'oct(64)'                        # 100

# Custom bases
rcalc -m int 'base(16); 255'           # ff
rcalc -m int 'base(2); 255'            # 11111111
```

## User-Defined Functions

```bash
# Simple function
rcalc << 'EOF'
define double(x) = x * 2
double(5)
EOF
# Output: 10

# Recursive function
rcalc << 'EOF'
define fib(n) {
    if (n <= 1) return n
    return fib(n-1) + fib(n-2)
}
fib(10)
EOF
# Output: 55
```

## Control Flow

```bash
# If/else
rcalc << 'EOF'
x = 15
if (x > 10) {
    print "large"
} else {
    print "small"
}
EOF
# Output: large

# Loops
rcalc << 'EOF'
sum = 0
for (i = 1; 5) {
    sum = sum + i
}
print sum
EOF
# Output: 15
```

## Pipe Mode

```bash
# Multiple expressions
echo -e '2+3\n10*5\nsqrt(2)' | rcalc -p
# Output:
# 5
# 50
# ~1.41421356237309504881

# With different modes
echo '1/3 + 1/6' | rcalc -p -m frac
# Output: 1/2
```

## Statistical Operations

```bash
# Create sample data
rcalc << 'EOF'
data = list(2, 4, 6, 8, 10)
print "Mean:"
mean(data)
print "Median:"
median(data)
print "Variance:"
variance(data)
print "Std Dev:"
stdev(data)
EOF
```

## Matrix Operations

```bash
rcalc << 'EOF'
m = list(list(1,2), list(3,4))
# Determinant
det(m)          # -2
# Transpose
mattrans(m)     # [[1,3], [2,4]]
# Trace
mattrace(m)     # 5
EOF
```

---

For more examples and detailed explanations, see the [Help Topics](topics/intro.md) and [Function Reference](functions.md).
