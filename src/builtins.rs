//! Builtin functions (~40 of them).

use crate::eval::BuiltinFn;
use crate::number::{self, Num};
use crate::value::Value;
use crate::eval::Interp;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};

fn argc(name: &str, args: &[Value], expected: usize) -> Result<(), String> {
    if args.len() != expected {
        return Err(format!(
            "{}() expects {} argument{}, got {}",
            name,
            expected,
            if expected == 1 { "" } else { "s" },
            args.len()
        ));
    }
    Ok(())
}

fn argc_range(name: &str, args: &[Value], lo: usize, hi: usize) -> Result<(), String> {
    if args.len() < lo || args.len() > hi {
        return Err(format!(
            "{}() expects {}-{} arguments, got {}",
            name,
            lo,
            hi,
            args.len()
        ));
    }
    Ok(())
}

fn n(args: &[Value], i: usize) -> Result<&Num, String> {
    args[i].as_number()
}

fn int(args: &[Value], i: usize) -> Result<BigInt, String> {
    let rat = n(args, i)?;
    if !rat.is_integer() {
        return Err("not an integer".to_string());
    }
    Ok(rat.numer().clone())
}

// Absolute value
fn f_abs(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("abs", a, 1)?;
    Ok(Value::Number(n(a, 0)?.abs()))
}

// Sign: -1, 0, or 1
fn f_sgn(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sgn", a, 1)?;
    let x = n(a, 0)?;
    let r = if x.is_positive() {
        Num::from_integer(BigInt::from(1))
    } else if x.is_negative() {
        Num::from_integer(BigInt::from(-1))
    } else {
        Num::from_integer(BigInt::from(0))
    };
    Ok(Value::Number(r))
}

// Integer part (truncate toward zero)
fn f_int(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("int", a, 1)?;
    Ok(Value::Number(number::trunc(n(a, 0)?)))
}

// Fractional part
fn f_frac(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("frac", a, 1)?;
    Ok(Value::Number(number::frac(n(a, 0)?)))
}

// Floor
fn f_floor(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("floor", a, 1)?;
    Ok(Value::Number(number::floor(n(a, 0)?)))
}

// Ceiling
fn f_ceil(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ceil", a, 1)?;
    Ok(Value::Number(number::ceil(n(a, 0)?)))
}

// Round to nearest integer
fn f_round(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("round", a, 1)?;
    Ok(Value::Number(n(a, 0)?.round()))
}

// Minimum
fn f_min(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("min", a, 1, 100)?;
    let mut result = n(a, 0)?.clone();
    for v in &a[1..] {
        let x = v.as_number()?;
        if x < &result {
            result = x.clone();
        }
    }
    Ok(Value::Number(result))
}

// Maximum
fn f_max(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("max", a, 1, 100)?;
    let mut result = n(a, 0)?.clone();
    for v in &a[1..] {
        let x = v.as_number()?;
        if x > &result {
            result = x.clone();
        }
    }
    Ok(Value::Number(result))
}

// Average
fn f_avg(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("avg", a, 1, 100)?;
    let mut sum = Num::from_integer(BigInt::from(0));
    for v in a {
        sum += v.as_number()?;
    }
    Ok(Value::Number(sum / Num::from_integer(BigInt::from(a.len()))))
}

// GCD
fn f_gcd(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("gcd", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;
    Ok(Value::Number(Num::from_integer(number::gcd_int(&x, &y))))
}

// LCM
fn f_lcm(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("lcm", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;
    let g = number::gcd_int(&x, &y);
    let lcm = (&x / &g) * &y;
    Ok(Value::Number(Num::from_integer(lcm)))
}

// Modulo
fn f_mod(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("mod", a, 2)?;
    let x = n(a, 0)?;
    let y = n(a, 1)?;
    if y.is_zero() {
        return Err("modulus by zero".to_string());
    }
    let q = number::trunc(&(x / y));
    Ok(Value::Number(x - y * q))
}

// Square root
fn f_sqrt(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sqrt", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::sqrt(n(a, 0)?, &eps)?))
}

// Factorial
fn f_fact(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fact", a, 1)?;
    let n_val = int(a, 0)?;
    if n_val.is_negative() {
        return Err("factorial of negative number".to_string());
    }
    let n_u32 = n_val
        .to_u32()
        .ok_or("factorial argument too large")?;
    let mut result = BigInt::from(1);
    for i in 2..=n_u32 {
        result *= i;
    }
    Ok(Value::Number(Num::from_integer(result)))
}

// Combinations: C(n, k) = n! / (k! * (n-k)!)
fn f_comb(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("comb", a, 2)?;
    let n = int(a, 0)?;
    let k = int(a, 1)?;
    if n.is_negative() || k.is_negative() || k > n {
        return Err("invalid arguments to comb".to_string());
    }
    let n_u32 = n.to_u32().ok_or("n too large")?;
    let k_u32 = k.to_u32().ok_or("k too large")?;
    if k_u32 > n_u32 {
        return Ok(Value::Number(Num::from_integer(BigInt::from(0))));
    }
    let mut num = BigInt::from(1);
    let mut denom = BigInt::from(1);
    for i in 0..k_u32 {
        num *= n_u32 - i;
        denom *= i + 1;
    }
    Ok(Value::Number(Num::new(num, denom)))
}

// Permutations: P(n, k) = n! / (n-k)!
fn f_perm(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("perm", a, 2)?;
    let n = int(a, 0)?;
    let k = int(a, 1)?;
    if n.is_negative() || k.is_negative() || k > n {
        return Err("invalid arguments to perm".to_string());
    }
    let n_u32 = n.to_u32().ok_or("n too large")?;
    let k_u32 = k.to_u32().ok_or("k too large")?;
    let mut result = BigInt::from(1);
    for i in 0..k_u32 {
        result *= n_u32 - i;
    }
    Ok(Value::Number(Num::from_integer(result)))
}

// Fibonacci
fn f_fib(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fib", a, 1)?;
    let n = int(a, 0)?;
    if n.is_negative() {
        return Err("fibonacci of negative number".to_string());
    }
    let n_u32 = n.to_u32().ok_or("fib argument too large")?;
    let (mut a, mut b) = (BigInt::from(0), BigInt::from(1));
    for _ in 0..n_u32 {
        let tmp = a.clone() + &b;
        a = b;
        b = tmp;
    }
    Ok(Value::Number(Num::from_integer(a)))
}

// Is prime
fn f_isprime(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isprime", a, 1)?;
    let n = int(a, 0)?;
    if n.is_negative() {
        return Ok(Value::Number(Num::from_integer(BigInt::from(0))));
    }
    if let Some(n_u64) = n.to_u64() {
        let is_p = is_prime(n_u64);
        Ok(Value::Number(Num::from_integer(BigInt::from(if is_p { 1 } else { 0 }))))
    } else {
        Err("number too large for primality test".to_string())
    }
}

// Next prime
fn f_nextprime(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("nextprime", a, 1)?;
    let n = int(a, 0)?;
    if n.is_negative() {
        return Err("nextprime of negative number".to_string());
    }
    if let Some(mut n_u64) = n.to_u64() {
        n_u64 += 1;
        while !is_prime(n_u64) {
            n_u64 += 1;
            if n_u64 > 1_000_000_000_000 {
                return Err("search limit exceeded".to_string());
            }
        }
        Ok(Value::Number(Num::from_integer(BigInt::from(n_u64))))
    } else {
        Err("number too large".to_string())
    }
}

// Numerator
fn f_num(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("num", a, 1)?;
    Ok(Value::Number(Num::from_integer(n(a, 0)?.numer().clone())))
}

// Denominator
fn f_den(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("den", a, 1)?;
    Ok(Value::Number(Num::from_integer(n(a, 0)?.denom().clone())))
}

// π constant
fn f_pi(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("pi", a, 0)?;
    Ok(Value::Number(number::pi()))
}

// e constant
fn f_e(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("e", a, 0)?;
    Ok(Value::Number(number::e()))
}

// Exponential
fn f_exp(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("exp", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::exp(n(a, 0)?, &eps)?))
}

// Natural logarithm
fn f_ln(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ln", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::ln(n(a, 0)?, &eps)?))
}

// Log base 10
fn f_log(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("log", a, 1)?;
    let x = n(a, 0)?;
    if x.is_negative() || x.is_zero() {
        return Err("log of non-positive number".to_string());
    }
    let xf = x.to_f64().ok_or("overflow")?;
    let r = xf.log10();
    Num::from_float(r).ok_or_else(|| "non-finite".to_string()).map(Value::Number)
}

// Log base 2
fn f_log2(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("log2", a, 1)?;
    let x = n(a, 0)?;
    if x.is_negative() || x.is_zero() {
        return Err("log2 of non-positive number".to_string());
    }
    let xf = x.to_f64().ok_or("overflow")?;
    let r = xf.log2();
    Num::from_float(r).ok_or_else(|| "non-finite".to_string()).map(Value::Number)
}

// Sine
fn f_sin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sin", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::sin(n(a, 0)?, &eps)?))
}

// Cosine
fn f_cos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cos", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::cos(n(a, 0)?, &eps)?))
}

// Tangent
fn f_tan(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("tan", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::tan(n(a, 0)?, &eps)?))
}

// Simple primality test (trial division for small primes, then test a few bases)
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for i in (3..=(n as f64).sqrt() as u64 + 1).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn register(builtins: &mut std::collections::HashMap<String, crate::eval::BuiltinFn>) {
    builtins.insert("abs".to_string(), f_abs as BuiltinFn);
    builtins.insert("sgn".to_string(), f_sgn as BuiltinFn);
    builtins.insert("int".to_string(), f_int as BuiltinFn);
    builtins.insert("frac".to_string(), f_frac as BuiltinFn);
    builtins.insert("floor".to_string(), f_floor as BuiltinFn);
    builtins.insert("ceil".to_string(), f_ceil as BuiltinFn);
    builtins.insert("round".to_string(), f_round as BuiltinFn);
    builtins.insert("min".to_string(), f_min as BuiltinFn);
    builtins.insert("max".to_string(), f_max as BuiltinFn);
    builtins.insert("avg".to_string(), f_avg as BuiltinFn);
    builtins.insert("gcd".to_string(), f_gcd as BuiltinFn);
    builtins.insert("lcm".to_string(), f_lcm as BuiltinFn);
    builtins.insert("mod".to_string(), f_mod as BuiltinFn);
    builtins.insert("sqrt".to_string(), f_sqrt as BuiltinFn);
    builtins.insert("fact".to_string(), f_fact as BuiltinFn);
    builtins.insert("comb".to_string(), f_comb as BuiltinFn);
    builtins.insert("perm".to_string(), f_perm as BuiltinFn);
    builtins.insert("fib".to_string(), f_fib as BuiltinFn);
    builtins.insert("isprime".to_string(), f_isprime as BuiltinFn);
    builtins.insert("nextprime".to_string(), f_nextprime as BuiltinFn);
    builtins.insert("num".to_string(), f_num as BuiltinFn);
    builtins.insert("den".to_string(), f_den as BuiltinFn);
    builtins.insert("pi".to_string(), f_pi as BuiltinFn);
    builtins.insert("e".to_string(), f_e as BuiltinFn);
    builtins.insert("exp".to_string(), f_exp as BuiltinFn);
    builtins.insert("ln".to_string(), f_ln as BuiltinFn);
    builtins.insert("log".to_string(), f_log as BuiltinFn);
    builtins.insert("log2".to_string(), f_log2 as BuiltinFn);
    builtins.insert("sin".to_string(), f_sin as BuiltinFn);
    builtins.insert("cos".to_string(), f_cos as BuiltinFn);
    builtins.insert("tan".to_string(), f_tan as BuiltinFn);
}

pub fn catalog() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("abs", "abs(x)", "absolute value"),
        ("sgn", "sgn(x)", "sign: -1, 0, or 1"),
        ("int", "int(x)", "integer part"),
        ("frac", "frac(x)", "fractional part"),
        ("floor", "floor(x)", "largest integer <= x"),
        ("ceil", "ceil(x)", "smallest integer >= x"),
        ("round", "round(x)", "round to nearest integer"),
        ("min", "min(x,...)", "minimum"),
        ("max", "max(x,...)", "maximum"),
        ("avg", "avg(x,...)", "average"),
        ("gcd", "gcd(x,y)", "greatest common divisor"),
        ("lcm", "lcm(x,y)", "least common multiple"),
        ("mod", "mod(x,y)", "modulus"),
        ("sqrt", "sqrt(x)", "square root"),
        ("fact", "fact(n)", "factorial"),
        ("comb", "comb(n,k)", "combinations"),
        ("perm", "perm(n,k)", "permutations"),
        ("fib", "fib(n)", "nth Fibonacci number"),
        ("isprime", "isprime(n)", "is n prime? (1 or 0)"),
        ("nextprime", "nextprime(n)", "next prime after n"),
        ("num", "num(x)", "numerator"),
        ("den", "den(x)", "denominator"),
        ("pi", "pi()", "π constant (60 digits)"),
        ("e", "e()", "e constant (60 digits)"),
        ("exp", "exp(x)", "e^x"),
        ("ln", "ln(x)", "natural logarithm"),
        ("log", "log(x)", "base-10 logarithm"),
        ("log2", "log2(x)", "base-2 logarithm"),
        ("sin", "sin(x)", "sine (radians)"),
        ("cos", "cos(x)", "cosine (radians)"),
        ("tan", "tan(x)", "tangent (radians)"),
    ]
}
