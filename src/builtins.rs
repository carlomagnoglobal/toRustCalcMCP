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

fn real_part(v: &Value) -> Result<Num, String> {
    match v {
        Value::Number(r) => Ok(r.clone()),
        Value::Complex(r, _) => Ok(r.clone()),
        _ => Err("not a number".to_string()),
    }
}

fn imag_part(v: &Value) -> Result<Num, String> {
    match v {
        Value::Number(_) => Ok(Num::zero()),
        Value::Complex(_, i) => Ok(i.clone()),
        _ => Err("not a number".to_string()),
    }
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
    argc_range("round", a, 1, 2)?;
    let x = n(a, 0)?;
    let places = if a.len() == 2 {
        int(a, 1)?.to_i64().ok_or("round: places out of range")?
    } else {
        0
    };
    if places == 0 {
        Ok(Value::Number(x.round()))
    } else {
        Ok(Value::Number(number::round_decimal(x, places)))
    }
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
    let x = n(a, 0)?;
    let eps = it.epsilon();
    if x.is_negative() {
        let (real, imag) = number::sqrt_complex(x, &Num::zero(), &eps)?;
        Ok(Value::Complex(real, imag))
    } else {
        Ok(Value::Number(number::sqrt(x, &eps)?))
    }
}

// Real part of a complex number
fn f_re(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("re", a, 1)?;
    Ok(Value::Number(real_part(&a[0])?))
}

// Imaginary part of a complex number
fn f_im(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("im", a, 1)?;
    Ok(Value::Number(imag_part(&a[0])?))
}

// Argument (phase angle) of a complex number
fn f_arg(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("arg", a, 1)?;
    let r = real_part(&a[0])?;
    let i = imag_part(&a[0])?;
    if r.is_zero() && i.is_zero() {
        return Ok(Value::Number(Num::zero()));
    }
    // atan2(imag, real)
    let angle_f64 = i.to_f64()
        .and_then(|im| r.to_f64().map(|re| im.atan2(re)))
        .ok_or("overflow in arg")?;
    let angle = Num::from_float(angle_f64)
        .ok_or("non-finite result in arg")?;
    Ok(Value::Number(angle))
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

// Get or set input/output base
fn f_base(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() {
        // No arguments: return current obase
        Ok(Value::Number(Num::from_integer(BigInt::from(it.cfg.obase))))
    } else if a.len() == 1 {
        // One argument: set both ibase and obase
        let base_val = int(a, 0)?;
        let base_u32 = base_val.to_u32().ok_or("base out of range")?;
        if base_u32 < 2 || base_u32 > 36 {
            return Err("base must be between 2 and 36".to_string());
        }
        it.cfg.ibase = base_u32;
        it.cfg.obase = base_u32;
        Ok(Value::Number(Num::from_integer(BigInt::from(base_u32))))
    } else if a.len() == 2 {
        // Two arguments: set ibase and obase separately
        let ibase_val = int(a, 0)?;
        let obase_val = int(a, 1)?;
        let ibase_u32 = ibase_val.to_u32().ok_or("ibase out of range")?;
        let obase_u32 = obase_val.to_u32().ok_or("obase out of range")?;
        if ibase_u32 < 2 || ibase_u32 > 36 {
            return Err("ibase must be between 2 and 36".to_string());
        }
        if obase_u32 < 2 || obase_u32 > 36 {
            return Err("obase must be between 2 and 36".to_string());
        }
        it.cfg.ibase = ibase_u32;
        it.cfg.obase = obase_u32;
        Ok(Value::Number(Num::from_integer(BigInt::from(obase_u32))))
    } else {
        Err("base() expects 0, 1, or 2 arguments".to_string())
    }
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

// Inverse sine
fn f_asin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("asin", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::asin(n(a, 0)?, &eps)?))
}

// Inverse cosine
fn f_acos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acos", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::acos(n(a, 0)?, &eps)?))
}

// Inverse tangent
fn f_atan(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("atan", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::atan(n(a, 0)?, &eps)?))
}

// Two-argument arctangent
fn f_atan2(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("atan2", a, 2)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::atan2(n(a, 0)?, n(a, 1)?, &eps)?))
}

// Hyperbolic sine
fn f_sinh(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sinh", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::sinh(n(a, 0)?, &eps)?))
}

// Hyperbolic cosine
fn f_cosh(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cosh", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::cosh(n(a, 0)?, &eps)?))
}

// Hyperbolic tangent
fn f_tanh(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("tanh", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::tanh(n(a, 0)?, &eps)?))
}

// Inverse hyperbolic sine
fn f_asinh(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("asinh", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::asinh(n(a, 0)?, &eps)?))
}

// Inverse hyperbolic cosine
fn f_acosh(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acosh", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::acosh(n(a, 0)?, &eps)?))
}

// Inverse hyperbolic tangent
fn f_atanh(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("atanh", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::atanh(n(a, 0)?, &eps)?))
}

// Cosine + sine
fn f_cas(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cas", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::cas(n(a, 0)?, &eps)?))
}

// Euler's formula: cis(x) = cos(x) + i*sin(x)
fn f_cis(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cis", a, 1)?;
    let eps = it.epsilon();
    let (real, imag) = number::cis(n(a, 0)?, &eps)?;
    Ok(Value::Complex(real, imag))
}

// Complex conjugate
fn f_conj(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("conj", a, 1)?;
    match &a[0] {
        Value::Number(n) => Ok(Value::Number(n.clone())),
        Value::Complex(r, i) => {
            let (real, imag) = number::conj_complex(r, i);
            Ok(Value::Complex(real, imag))
        }
        _ => Err("conj: argument must be a number".to_string()),
    }
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

// Bitwise AND
fn f_and(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("and", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;
    Ok(Value::Number(Num::from_integer(&x & &y)))
}

// Bitwise OR
fn f_or(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("or", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;
    Ok(Value::Number(Num::from_integer(&x | &y)))
}

// Bitwise XOR
fn f_xor(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("xor", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;
    Ok(Value::Number(Num::from_integer(&x ^ &y)))
}

// Bitwise complement (two's complement for negative numbers)
fn f_comp(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("comp", a, 1)?;
    let x = int(a, 0)?;
    Ok(Value::Number(Num::from_integer(!&x)))
}

// Left shift
fn f_lshift(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("lshift", a, 2)?;
    let x = int(a, 0)?;
    let shift = int(a, 1)?;
    let shift_u32 = shift.to_u32().ok_or("shift amount too large")?;
    Ok(Value::Number(Num::from_integer(&x << shift_u32)))
}

// Right shift
fn f_rshift(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rshift", a, 2)?;
    let x = int(a, 0)?;
    let shift = int(a, 1)?;
    let shift_u32 = shift.to_u32().ok_or("shift amount too large")?;
    Ok(Value::Number(Num::from_integer(&x >> shift_u32)))
}

// Test if bit n is set
fn f_bit(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("bit", a, 2)?;
    let x = int(a, 0)?;
    let n = int(a, 1)?;
    let n_u32 = n.to_u32().ok_or("bit position too large")?;
    let is_set = (&x >> n_u32) & BigInt::from(1) != BigInt::from(0);
    Ok(Value::Number(Num::from_integer(BigInt::from(if is_set { 1 } else { 0 }))))
}

// Position of highest set bit (most significant bit)
fn f_highbit(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("highbit", a, 1)?;
    let mut x = int(a, 0)?;
    if x.is_zero() {
        return Ok(Value::Number(Num::from_integer(BigInt::from(-1))));
    }
    if x.is_negative() {
        x = -x - BigInt::from(1);
    }
    let bits = x.bits();
    Ok(Value::Number(Num::from_integer(BigInt::from(bits as i64 - 1))))
}

// Position of lowest set bit (least significant bit)
fn f_lowbit(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("lowbit", a, 1)?;
    let x = int(a, 0)?;
    if x.is_zero() {
        return Ok(Value::Number(Num::from_integer(BigInt::from(-1))));
    }
    // Find position of lowest set bit
    let mut pos = 0i64;
    let mut val = if x.is_negative() { -&x } else { x };
    while (&val & BigInt::from(1)).is_zero() {
        val = val >> 1;
        pos += 1;
    }
    Ok(Value::Number(Num::from_integer(BigInt::from(pos))))
}

// Count of set bits (population count / Hamming weight)
fn f_fcnt(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fcnt", a, 1)?;
    let x = int(a, 0)?;
    if x.is_negative() {
        return Err("fcnt: undefined for negative numbers".to_string());
    }
    let mut count = 0u32;
    let mut val = x;
    while val.is_positive() {
        if (&val & BigInt::from(1)).is_positive() {
            count += 1;
        }
        val = val >> 1;
    }
    Ok(Value::Number(Num::from_integer(BigInt::from(count))))
}

// Number of digits in given base
fn f_digits(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("digits", a, 1, 2)?;
    let x = int(a, 0)?;
    let base = if a.len() == 2 {
        int(a, 1)?.to_u32().ok_or("base too large")?
    } else {
        10
    };
    if base < 2 || base > 36 {
        return Err("base must be between 2 and 36".to_string());
    }
    if x.is_zero() {
        return Ok(Value::Number(Num::from_integer(BigInt::from(1))));
    }
    let mut count = 0u32;
    let mut val = x.abs();
    let base_bi = BigInt::from(base);
    while val.is_positive() {
        val = val / &base_bi;
        count += 1;
    }
    Ok(Value::Number(Num::from_integer(BigInt::from(count))))
}

// Create a list from arguments
fn f_list(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    Ok(Value::List(a.to_vec()))
}

// Get size of a list
fn f_size(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("size", a, 1)?;
    match &a[0] {
        Value::List(items) => {
            Ok(Value::Number(Num::from_integer(BigInt::from(items.len()))))
        }
        _ => Err("size() requires a list".to_string()),
    }
}

// Append item(s) to a list (mutates in-place and returns the list)
fn f_append(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("append", a, 2, 100)?;
    match a[0].clone() {
        Value::List(mut items) => {
            for v in &a[1..] {
                items.push(v.clone());
            }
            Ok(Value::List(items))
        }
        _ => Err("append() requires a list as first argument".to_string()),
    }
}

// Get the first item of a list
fn f_first(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("first", a, 1)?;
    match &a[0] {
        Value::List(items) => {
            items.first()
                .cloned()
                .ok_or("list is empty".to_string())
        }
        _ => Err("first() requires a list".to_string()),
    }
}

// Get the last item of a list
fn f_last(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("last", a, 1)?;
    match &a[0] {
        Value::List(items) => {
            items.last()
                .cloned()
                .ok_or("list is empty".to_string())
        }
        _ => Err("last() requires a list".to_string()),
    }
}

// Get a sublist (slice)
fn f_slice(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("slice", a, 2, 3)?;
    match &a[0] {
        Value::List(items) => {
            let start_num = n(a, 1)?;
            if !start_num.is_integer() {
                return Err("start index must be an integer".to_string());
            }
            let start = start_num.numer();
            let start_idx = if start < &BigInt::from(0) {
                let len = items.len() as i64;
                ((len + start.to_i64().unwrap_or(0)) as usize)
            } else {
                start.to_usize().unwrap_or(0)
            };

            let end_idx = if a.len() == 3 {
                let end_num = n(a, 2)?;
                if !end_num.is_integer() {
                    return Err("end index must be an integer".to_string());
                }
                let end = end_num.numer();
                if end < &BigInt::from(0) {
                    let len = items.len() as i64;
                    ((len + end.to_i64().unwrap_or(0)) as usize)
                } else {
                    end.to_usize().unwrap_or(items.len())
                }
            } else {
                items.len()
            };

            let result: Vec<Value> = items.iter()
                .skip(start_idx)
                .take(end_idx.saturating_sub(start_idx))
                .cloned()
                .collect();
            Ok(Value::List(result))
        }
        _ => Err("slice() requires a list".to_string()),
    }
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
    builtins.insert("re".to_string(), f_re as BuiltinFn);
    builtins.insert("im".to_string(), f_im as BuiltinFn);
    builtins.insert("arg".to_string(), f_arg as BuiltinFn);
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
    builtins.insert("base".to_string(), f_base as BuiltinFn);
    builtins.insert("exp".to_string(), f_exp as BuiltinFn);
    builtins.insert("ln".to_string(), f_ln as BuiltinFn);
    builtins.insert("log".to_string(), f_log as BuiltinFn);
    builtins.insert("log2".to_string(), f_log2 as BuiltinFn);
    builtins.insert("sin".to_string(), f_sin as BuiltinFn);
    builtins.insert("cos".to_string(), f_cos as BuiltinFn);
    builtins.insert("tan".to_string(), f_tan as BuiltinFn);
    builtins.insert("asin".to_string(), f_asin as BuiltinFn);
    builtins.insert("acos".to_string(), f_acos as BuiltinFn);
    builtins.insert("atan".to_string(), f_atan as BuiltinFn);
    builtins.insert("atan2".to_string(), f_atan2 as BuiltinFn);
    builtins.insert("sinh".to_string(), f_sinh as BuiltinFn);
    builtins.insert("cosh".to_string(), f_cosh as BuiltinFn);
    builtins.insert("tanh".to_string(), f_tanh as BuiltinFn);
    builtins.insert("asinh".to_string(), f_asinh as BuiltinFn);
    builtins.insert("acosh".to_string(), f_acosh as BuiltinFn);
    builtins.insert("atanh".to_string(), f_atanh as BuiltinFn);
    builtins.insert("cas".to_string(), f_cas as BuiltinFn);
    builtins.insert("cis".to_string(), f_cis as BuiltinFn);
    builtins.insert("conj".to_string(), f_conj as BuiltinFn);
    builtins.insert("round".to_string(), f_round as BuiltinFn);
    // Bitwise operations
    builtins.insert("and".to_string(), f_and as BuiltinFn);
    builtins.insert("or".to_string(), f_or as BuiltinFn);
    builtins.insert("xor".to_string(), f_xor as BuiltinFn);
    builtins.insert("comp".to_string(), f_comp as BuiltinFn);
    builtins.insert("lshift".to_string(), f_lshift as BuiltinFn);
    builtins.insert("rshift".to_string(), f_rshift as BuiltinFn);
    builtins.insert("bit".to_string(), f_bit as BuiltinFn);
    builtins.insert("highbit".to_string(), f_highbit as BuiltinFn);
    builtins.insert("lowbit".to_string(), f_lowbit as BuiltinFn);
    builtins.insert("fcnt".to_string(), f_fcnt as BuiltinFn);
    builtins.insert("digits".to_string(), f_digits as BuiltinFn);
    // List operations
    builtins.insert("list".to_string(), f_list as BuiltinFn);
    builtins.insert("size".to_string(), f_size as BuiltinFn);
    builtins.insert("append".to_string(), f_append as BuiltinFn);
    builtins.insert("first".to_string(), f_first as BuiltinFn);
    builtins.insert("last".to_string(), f_last as BuiltinFn);
    builtins.insert("slice".to_string(), f_slice as BuiltinFn);
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
        ("sqrt", "sqrt(x)", "square root (returns complex for negative x)"),
        ("re", "re(z)", "real part of complex number"),
        ("im", "im(z)", "imaginary part of complex number"),
        ("arg", "arg(z)", "argument (phase angle) of complex number"),
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
        ("base", "base([ibase[,obase]])", "get/set input and output base (2-36)"),
        ("exp", "exp(x)", "e^x"),
        ("ln", "ln(x)", "natural logarithm"),
        ("log", "log(x)", "base-10 logarithm"),
        ("log2", "log2(x)", "base-2 logarithm"),
        ("sin", "sin(x)", "sine (radians)"),
        ("cos", "cos(x)", "cosine (radians)"),
        ("tan", "tan(x)", "tangent (radians)"),
        ("asin", "asin(x)", "inverse sine"),
        ("acos", "acos(x)", "inverse cosine"),
        ("atan", "atan(x)", "inverse tangent"),
        ("atan2", "atan2(y,x)", "two-argument inverse tangent"),
        ("sinh", "sinh(x)", "hyperbolic sine"),
        ("cosh", "cosh(x)", "hyperbolic cosine"),
        ("tanh", "tanh(x)", "hyperbolic tangent"),
        ("asinh", "asinh(x)", "inverse hyperbolic sine"),
        ("acosh", "acosh(x)", "inverse hyperbolic cosine"),
        ("atanh", "atanh(x)", "inverse hyperbolic tangent"),
        ("cas", "cas(x)", "cosine + sine"),
        ("cis", "cis(x)", "cos(x) + i*sin(x) (returns complex)"),
        ("conj", "conj(x)", "complex conjugate"),
        ("round", "round(x[,places])", "round to decimal places"),
        ("and", "and(x,y)", "bitwise AND"),
        ("or", "or(x,y)", "bitwise OR"),
        ("xor", "xor(x,y)", "bitwise XOR"),
        ("comp", "comp(x)", "bitwise complement"),
        ("lshift", "lshift(x,n)", "left shift by n bits"),
        ("rshift", "rshift(x,n)", "right shift by n bits"),
        ("bit", "bit(x,n)", "is bit n set? (1 or 0)"),
        ("highbit", "highbit(x)", "position of highest set bit"),
        ("lowbit", "lowbit(x)", "position of lowest set bit"),
        ("fcnt", "fcnt(x)", "count of set bits"),
        ("digits", "digits(x[,base])", "number of digits (base 10 or specified)"),
        ("list", "list(x,...)", "create a list from items"),
        ("size", "size(list)", "number of items in list"),
        ("append", "append(list,x,...)", "append items to list"),
        ("first", "first(list)", "get first item"),
        ("last", "last(list)", "get last item"),
        ("slice", "slice(list,start[,end])", "get sublist from start to end"),
    ]
}
