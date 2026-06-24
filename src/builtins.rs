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

// Nth root
fn f_root(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("root", a, 2)?;
    let x = n(a, 0)?;
    let n_val = int(a, 1)?;
    let n_i64 = n_val.to_i64().ok_or("root: n out of range")?;
    let eps = it.epsilon();
    Ok(Value::Number(number::root(x, n_i64, &eps)?))
}

// Cube root
fn f_cbrt(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cbrt", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::cbrt(n(a, 0)?, &eps)?))
}

// Integer square root
fn f_isqrt(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isqrt", a, 1)?;
    Ok(Value::Number(number::isqrt(n(a, 0)?)?))
}

// Integer nth root
fn f_iroot(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("iroot", a, 2)?;
    let x = n(a, 0)?;
    let n_val = int(a, 1)?;
    let n_i64 = n_val.to_i64().ok_or("iroot: n out of range")?;
    Ok(Value::Number(number::iroot(x, n_i64)?))
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

// Previous prime
fn f_prevprime(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("prevprime", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("prevprime: number too large")?;
    Ok(Value::Number(Num::from_integer(number::prevprime(n)?)))
}

// Prime factorization
fn f_factor(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("factor", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("factor: number too large")?;
    let factors = number::factor(n)?;
    let result_list: Vec<Value> = factors.iter().map(|f| Value::Number(Num::from_integer(f.clone()))).collect();
    Ok(Value::List(result_list))
}

// Largest prime factor
fn f_lfactor(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("lfactor", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("lfactor: number too large")?;
    Ok(Value::Number(Num::from_integer(number::lfactor(n)?)))
}

// Probabilistic primality test
fn f_ptest(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ptest", a, 2)?;
    let n = int(a, 0)?.to_i64().ok_or("ptest: n too large")?;
    let k = int(a, 1)?.to_i64().ok_or("ptest: k too large")?;
    Ok(Value::Number(number::ptest(n, k)?))
}

// Euler numbers
fn f_euler(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("euler", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("euler: index too large")?;
    Ok(Value::Number(Num::from_integer(number::euler(n)?)))
}

// Bernoulli numbers
fn f_bernoulli(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("bernoulli", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("bernoulli: index too large")?;
    Ok(Value::Number(number::bernoulli(n)?))
}

// Jacobi symbol
fn f_jacobi(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("jacobi", a, 2)?;
    let a_val = int(a, 0)?.to_i64().ok_or("jacobi: a too large")?;
    let n_val = int(a, 1)?.to_i64().ok_or("jacobi: n too large")?;
    Ok(Value::Number(number::jacobi(a_val, n_val)?))
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

// Log base n
fn f_logn(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("logn", a, 2)?;
    let x = n(a, 0)?;
    let n = n(a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::logn(x, n, &eps)?))
}

// Integer log base 10
fn f_ilog10(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ilog10", a, 1)?;
    Ok(Value::Number(number::ilog10(n(a, 0)?)?))
}

// Integer log base 2
fn f_ilog2(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ilog2", a, 1)?;
    Ok(Value::Number(number::ilog2(n(a, 0)?)?))
}

// Integer log base e
fn f_ilog(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ilog", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::ilog(n(a, 0)?, &eps)?))
}

// Integer log base n
fn f_ilogn(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ilogn", a, 2)?;
    let x = n(a, 0)?;
    let n = n(a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::ilogn(x, n, &eps)?))
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

// Cotangent
fn f_cot(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cot", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::cot(n(a, 0)?, &eps)?))
}

// Secant
fn f_sec(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sec", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::sec(n(a, 0)?, &eps)?))
}

// Cosecant
fn f_csc(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("csc", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::csc(n(a, 0)?, &eps)?))
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

// Inverse cotangent
fn f_acot(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acot", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::acot(n(a, 0)?, &eps)?))
}

// Inverse secant
fn f_asec(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("asec", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::asec(n(a, 0)?, &eps)?))
}

// Inverse cosecant
fn f_acsc(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acsc", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::acsc(n(a, 0)?, &eps)?))
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

// Hyperbolic cotangent
fn f_coth(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("coth", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::coth(n(a, 0)?, &eps)?))
}

// Hyperbolic secant
fn f_sech(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sech", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::sech(n(a, 0)?, &eps)?))
}

// Hyperbolic cosecant
fn f_csch(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("csch", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::csch(n(a, 0)?, &eps)?))
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

// Inverse hyperbolic cotangent
fn f_acoth(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acoth", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::acoth(n(a, 0)?, &eps)?))
}

// Inverse hyperbolic secant
fn f_asech(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("asech", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::asech(n(a, 0)?, &eps)?))
}

// Inverse hyperbolic cosecant
fn f_acsch(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acsch", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::acsch(n(a, 0)?, &eps)?))
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

// Hypot: sqrt(x^2 + y^2)
fn f_hypot(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hypot", a, 2)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::hypot(n(a, 0)?, n(a, 1)?, &eps)?))
}

// Error function
fn f_erf(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("erf", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::erf(n(a, 0)?, &eps)?))
}

// Complementary error function
fn f_erfc(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("erfc", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::erfc(n(a, 0)?, &eps)?))
}

// Gudermannian function
fn f_gd(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("gd", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::gd(n(a, 0)?, &eps)?))
}

// Inverse Gudermannian function
fn f_agd(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("agd", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::agd(n(a, 0)?, &eps)?))
}

// Bessel J0
fn f_j0(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("j0", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::j0(n(a, 0)?, &eps)?))
}

// Bessel J1
fn f_j1(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("j1", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::j1(n(a, 0)?, &eps)?))
}

// Bessel function Y0
fn f_y0(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("y0", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::y0(n(a, 0)?, &eps)?))
}

// Bessel function Y1
fn f_y1(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("y1", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::y1(n(a, 0)?, &eps)?))
}

// Gamma function
fn f_gamma(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("gamma", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::gamma(n(a, 0)?, &eps)?))
}

// Log-gamma function
fn f_lgamma(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("lgamma", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::lgamma(n(a, 0)?, &eps)?))
}

// Polygamma function
fn f_polygamma(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("polygamma", a, 2)?;
    let order = int(a, 0)?.to_i64().ok_or("polygamma: order out of range")?;
    let eps = it.epsilon();
    Ok(Value::Number(number::polygamma(order, n(a, 1)?, &eps)?))
}

// Riemann zeta function
fn f_zeta(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("zeta", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::zeta(n(a, 0)?, &eps)?))
}

// Random integer
fn f_rand(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rand", a, 0)?;
    let r = number::rand(&mut it.rng_seed);
    Ok(Value::Number(Num::from_integer(BigInt::from(r))))
}

// Random float [0, 1)
fn f_random(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("random", a, 0)?;
    let r = number::random(&mut it.rng_seed);
    Num::from_float(r).ok_or_else(|| "random: non-finite result".to_string()).map(Value::Number)
}

// Random bit
fn f_randbit(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("randbit", a, 0)?;
    let r = number::randbit(&mut it.rng_seed);
    Ok(Value::Number(Num::from_integer(BigInt::from(r))))
}

// Set random seed
fn f_seed(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("seed", a, 1)?;
    let s = int(a, 0)?.to_u64().ok_or("seed: value out of range")?;
    it.rng_seed = s;
    Ok(Value::Number(Num::from_integer(BigInt::from(s as i64))))
}

// Set random seed (alias)
fn f_srand(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    f_seed(it, a)
}

// Set random seed (alias)
fn f_srandom(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    f_seed(it, a)
}

// Random integer in range
fn f_randint(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("randint", a, 2)?;
    let a_val = int(a, 0)?.to_i64().ok_or("randint: a out of range")?;
    let b_val = int(a, 1)?.to_i64().ok_or("randint: b out of range")?;
    let r = number::randint(a_val, b_val, &mut it.rng_seed)?;
    Ok(Value::Number(Num::from_integer(BigInt::from(r))))
}

// Random permutation
fn f_randperm(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("randperm", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("randperm: n out of range")?;
    let perm = number::randperm(n, &mut it.rng_seed)?;
    let result_list: Vec<Value> = perm.iter().map(|x| Value::Number(Num::from_integer(x.clone()))).collect();
    Ok(Value::List(result_list))
}

// Phase 4.6: Environment & System Functions

fn f_time(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("time", a, 0)?;
    let timestamp = number::time()?;
    Ok(Value::Number(Num::from_integer(BigInt::from(timestamp))))
}

fn f_systime(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("systime", a, 0)?;
    let timestamp = number::systime()?;
    Ok(Value::Number(Num::from_integer(BigInt::from(timestamp))))
}

fn f_ctime(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ctime", a, 1)?;
    let timestamp = int(a, 0)?.to_i64().ok_or("ctime: timestamp out of range")?;
    let result = number::ctime(timestamp)?;
    Ok(Value::Str(result))
}

fn f_sleep(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sleep", a, 1)?;
    let seconds = n(a, 0)?.to_f64().ok_or("sleep: seconds must be convertible to float")?;
    number::sleep_fn(seconds)?;
    Ok(Value::Number(Num::from_integer(BigInt::from(0))))
}

fn f_getenv(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("getenv", a, 1)?;
    let name = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("getenv: argument must be a string".to_string()),
    };
    let value = number::getenv(&name)?;
    Ok(Value::Str(value))
}

fn f_putenv(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("putenv", a, 2)?;
    let name = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("putenv: first argument must be a string".to_string()),
    };
    let value = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("putenv: second argument must be a string".to_string()),
    };
    number::putenv(&name, &value)?;
    Ok(Value::Str(value))
}

fn f_system(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("system", a, 1)?;
    let cmd = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("system: argument must be a string".to_string()),
    };
    let exit_code = number::system(&cmd)?;
    Ok(Value::Number(Num::from_integer(BigInt::from(exit_code))))
}

fn f_usertime(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("usertime", a, 0)?;
    let elapsed = number::usertime()?;
    Ok(Value::Number(Num::from_float(elapsed).ok_or("usertime: overflow")?))
}

// Phase 5.1: Character Classification Functions

fn f_isalnum(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isalnum", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isalnum: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::isalnum(&s)))))
}

fn f_isupper(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isupper", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isupper: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::isupper(&s)))))
}

fn f_islower(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("islower", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("islower: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::islower(&s)))))
}

fn f_isprint(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isprint", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isprint: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::isprint(&s)))))
}

fn f_isgraph(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isgraph", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isgraph: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::isgraph(&s)))))
}

fn f_iscntrl(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("iscntrl", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("iscntrl: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::iscntrl(&s)))))
}

fn f_ispunct(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ispunct", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("ispunct: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::ispunct(&s)))))
}

fn f_isxdigit(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isxdigit", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isxdigit: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::isxdigit(&s)))))
}

fn f_isascii(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isascii", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isascii: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(number::isascii(&s)))))
}

fn f_toupper(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("toupper", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("toupper: argument must be a string".to_string()),
    };
    Ok(Value::Str(number::toupper(&s)))
}

fn f_tolower(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("tolower", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("tolower: argument must be a string".to_string()),
    };
    Ok(Value::Str(number::tolower(&s)))
}

fn f_strrev(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strrev", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("strrev: argument must be a string".to_string()),
    };
    Ok(Value::Str(number::strrev(&s)))
}

// Phase 5.2: Advanced Modular Arithmetic Functions

fn f_pmod(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("pmod", a, 2)?;
    let x = n(a, 0)?;
    let y = n(a, 1)?;
    let result = number::pmod(x, y)?;
    Ok(Value::Number(result))
}

fn f_quomod(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("quomod", a, 2)?;
    let x = n(a, 0)?;
    let y = n(a, 1)?;
    let (quotient, remainder) = number::quomod(x, y)?;
    Ok(Value::List(vec![Value::Number(quotient), Value::Number(remainder)]))
}

fn f_quo(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("quo", a, 2)?;
    let x = n(a, 0)?;
    let y = n(a, 1)?;
    let result = number::quo(x, y)?;
    Ok(Value::Number(result))
}

fn f_rem(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rem", a, 2)?;
    let x = n(a, 0)?;
    let y = n(a, 1)?;
    let result = number::rem(x, y)?;
    Ok(Value::Number(result))
}

fn f_hnrmod(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hnrmod", a, 2)?;
    let x = n(a, 0)?;
    let y = n(a, 1)?;
    let result = number::hnrmod(x, y)?;
    Ok(Value::Number(result))
}

// Phase 5.3: Rational Approximations

fn f_appr(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() || a.len() > 2 {
        return Err("appr: expects 1 or 2 arguments".to_string());
    }
    let x = n(a, 0)?;
    let epsilon = if a.len() == 2 {
        n(a, 1)?
    } else {
        &_it.epsilon()
    };
    let result = number::appr(x, epsilon)?;
    Ok(Value::Number(result))
}

fn f_cfappr(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() || a.len() > 2 {
        return Err("cfappr: expects 1 or 2 arguments".to_string());
    }
    let x = n(a, 0)?;
    let maxd = if a.len() == 2 {
        int(a, 1)?.to_i64().ok_or("cfappr: maxd out of range")?
    } else {
        1000000
    };
    let result = number::cfappr(x, maxd)?;
    Ok(Value::Number(result))
}

fn f_cfsim(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() || a.len() > 2 {
        return Err("cfsim: expects 1 or 2 arguments".to_string());
    }
    let x = n(a, 0)?;
    let maxd = if a.len() == 2 {
        int(a, 1)?.to_i64().ok_or("cfsim: maxd out of range")?
    } else {
        1000000
    };
    let result = number::cfsim(x, maxd)?;
    Ok(Value::Number(result))
}

fn f_scale(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() || a.len() > 2 {
        return Err("scale: expects 1 or 2 arguments".to_string());
    }
    let x = n(a, 0)?;
    let places = if a.len() == 2 {
        int(a, 1)?.to_i64().ok_or("scale: places out of range")?
    } else {
        0
    };
    let result = number::scale(x, places)?;
    Ok(Value::Number(result))
}

// Phase 5.4: Matrix Operations

fn f_matdim(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("matdim", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            if rows.is_empty() {
                return Ok(Value::List(vec![Value::Number(Num::from_integer(BigInt::from(0))), Value::Number(Num::from_integer(BigInt::from(0)))]));
            }
            let m = rows.len() as i64;
            let n = match &rows[0] {
                Value::List(row) => row.len() as i64,
                _ => return Err("matdim: matrix rows must be lists".to_string()),
            };
            Ok(Value::List(vec![Value::Number(Num::from_integer(BigInt::from(m))), Value::Number(Num::from_integer(BigInt::from(n)))]))
        }
        _ => Err("matdim: argument must be a list (matrix)".to_string()),
    }
}

fn f_mattrans(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("mattrans", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            let mut matrix: Vec<Vec<Num>> = vec![];
            for row in rows {
                match row {
                    Value::List(cols) => {
                        let mut row_nums = vec![];
                        for col in cols {
                            match col {
                                Value::Number(n) => row_nums.push(n.clone()),
                                _ => return Err("mattrans: matrix elements must be numbers".to_string()),
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("mattrans: matrix rows must be lists".to_string()),
                }
            }
            let transposed = number::mattrans(&matrix)?;
            let result = transposed.into_iter().map(|row| {
                Value::List(row.into_iter().map(|n| Value::Number(n)).collect())
            }).collect();
            Ok(Value::List(result))
        }
        _ => Err("mattrans: argument must be a list (matrix)".to_string()),
    }
}

fn f_mattrace(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("mattrace", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            let mut matrix: Vec<Vec<Num>> = vec![];
            for row in rows {
                match row {
                    Value::List(cols) => {
                        let mut row_nums = vec![];
                        for col in cols {
                            match col {
                                Value::Number(n) => row_nums.push(n.clone()),
                                _ => return Err("mattrace: matrix elements must be numbers".to_string()),
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("mattrace: matrix rows must be lists".to_string()),
                }
            }
            let trace = number::mattrace(&matrix)?;
            Ok(Value::Number(trace))
        }
        _ => Err("mattrace: argument must be a list (matrix)".to_string()),
    }
}

fn f_det(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("det", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            let mut matrix: Vec<Vec<Num>> = vec![];
            for row in rows {
                match row {
                    Value::List(cols) => {
                        let mut row_nums = vec![];
                        for col in cols {
                            match col {
                                Value::Number(n) => row_nums.push(n.clone()),
                                _ => return Err("det: matrix elements must be numbers".to_string()),
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("det: matrix rows must be lists".to_string()),
                }
            }
            let determinant = number::det(&matrix)?;
            Ok(Value::Number(determinant))
        }
        _ => Err("det: argument must be a list (matrix)".to_string()),
    }
}

fn f_inverse(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("inverse", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            let mut matrix: Vec<Vec<Num>> = vec![];
            for row in rows {
                match row {
                    Value::List(cols) => {
                        let mut row_nums = vec![];
                        for col in cols {
                            match col {
                                Value::Number(n) => row_nums.push(n.clone()),
                                _ => return Err("inverse: matrix elements must be numbers".to_string()),
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("inverse: matrix rows must be lists".to_string()),
                }
            }
            let inv = number::inverse(&matrix)?;
            let result = inv.into_iter().map(|row| {
                Value::List(row.into_iter().map(|n| Value::Number(n)).collect())
            }).collect();
            Ok(Value::List(result))
        }
        _ => Err("inverse: argument must be a list (matrix)".to_string()),
    }
}

fn f_matsum(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("matsum", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            let mut matrix: Vec<Vec<Num>> = vec![];
            for row in rows {
                match row {
                    Value::List(cols) => {
                        let mut row_nums = vec![];
                        for col in cols {
                            match col {
                                Value::Number(n) => row_nums.push(n.clone()),
                                _ => return Err("matsum: matrix elements must be numbers".to_string()),
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("matsum: matrix rows must be lists".to_string()),
                }
            }
            let sum = number::matsum(&matrix)?;
            Ok(Value::Number(sum))
        }
        _ => Err("matsum: argument must be a list (matrix)".to_string()),
    }
}

fn f_matmin(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("matmin", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            let mut matrix: Vec<Vec<Num>> = vec![];
            for row in rows {
                match row {
                    Value::List(cols) => {
                        let mut row_nums = vec![];
                        for col in cols {
                            match col {
                                Value::Number(n) => row_nums.push(n.clone()),
                                _ => return Err("matmin: matrix elements must be numbers".to_string()),
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("matmin: matrix rows must be lists".to_string()),
                }
            }
            let min = number::matmin(&matrix)?;
            Ok(Value::Number(min))
        }
        _ => Err("matmin: argument must be a list (matrix)".to_string()),
    }
}

fn f_matmax(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("matmax", a, 1)?;
    match &a[0] {
        Value::List(rows) => {
            let mut matrix: Vec<Vec<Num>> = vec![];
            for row in rows {
                match row {
                    Value::List(cols) => {
                        let mut row_nums = vec![];
                        for col in cols {
                            match col {
                                Value::Number(n) => row_nums.push(n.clone()),
                                _ => return Err("matmax: matrix elements must be numbers".to_string()),
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("matmax: matrix rows must be lists".to_string()),
                }
            }
            let max = number::matmax(&matrix)?;
            Ok(Value::Number(max))
        }
        _ => Err("matmax: argument must be a list (matrix)".to_string()),
    }
}

fn f_matfill(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.len() != 3 {
        return Err("matfill: expects 3 arguments (rows, cols, value)".to_string());
    }
    let rows = int(a, 0)?.to_i64().ok_or("matfill: rows out of range")?;
    let cols = int(a, 1)?.to_i64().ok_or("matfill: cols out of range")?;
    let val = n(a, 2)?;
    let matrix = number::matfill(rows, cols, val)?;
    let result = matrix.into_iter().map(|row| {
        Value::List(row.into_iter().map(|n| Value::Number(n)).collect())
    }).collect();
    Ok(Value::List(result))
}

// Phase 5.5: Hash & Associative Arrays

// Create an associative array from key-value pairs
fn f_assoc(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() {
        return Ok(Value::Hash(std::collections::HashMap::new()));
    }
    if a.len() % 2 != 0 {
        return Err("assoc: expects even number of arguments (key-value pairs)".to_string());
    }
    let mut map = std::collections::HashMap::new();
    for i in (0..a.len()).step_by(2) {
        let key = match &a[i] {
            Value::Str(s) => s.clone(),
            _ => return Err("assoc: keys must be strings".to_string()),
        };
        let val = a[i + 1].clone();
        map.insert(key, val);
    }
    Ok(Value::Hash(map))
}

// Get all keys from a hash as a list
fn f_indices(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("indices", a, 1)?;
    match &a[0] {
        Value::Hash(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let items = keys.into_iter()
                .map(|k| Value::Str(k))
                .collect();
            Ok(Value::List(items))
        }
        _ => Err("indices: argument must be a hash".to_string()),
    }
}

// Insert or update a key-value pair in a hash
fn f_insert(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("insert", a, 3)?;
    match &a[0] {
        Value::Hash(map) => {
            let mut new_map = map.clone();
            let key = match &a[1] {
                Value::Str(s) => s.clone(),
                _ => return Err("insert: key must be a string".to_string()),
            };
            let val = a[2].clone();
            new_map.insert(key, val);
            Ok(Value::Hash(new_map))
        }
        _ => Err("insert: first argument must be a hash".to_string()),
    }
}

// Delete a key from a hash
fn f_delete(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("delete", a, 2)?;
    match &a[0] {
        Value::Hash(map) => {
            let mut new_map = map.clone();
            let key = match &a[1] {
                Value::Str(s) => s.clone(),
                _ => return Err("delete: key must be a string".to_string()),
            };
            new_map.remove(&key);
            Ok(Value::Hash(new_map))
        }
        _ => Err("delete: first argument must be a hash".to_string()),
    }
}

// Count the number of key-value pairs in a hash
fn f_count(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("count", a, 1)?;
    match &a[0] {
        Value::Hash(map) => {
            Ok(Value::Number(Num::from_integer(BigInt::from(map.len() as i64))))
        }
        _ => Err("count: argument must be a hash".to_string()),
    }
}

// Join all values in a hash with a separator
fn f_join(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("join", a, 2)?;
    let sep = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("join: separator must be a string".to_string()),
    };
    match &a[0] {
        Value::Hash(map) => {
            let mut values: Vec<String> = vec![];
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                if let Some(val) = map.get(&key) {
                    values.push(match val {
                        Value::Str(s) => s.clone(),
                        _ => val.render(&crate::config::Config::default()),
                    });
                }
            }
            Ok(Value::Str(values.join(&sep)))
        }
        _ => Err("join: first argument must be a hash".to_string()),
    }
}

// Phase 6.3: Error & Exception Handling

// Get error count
fn f_errcount(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("errcount", a, 0)?;
    Ok(Value::Number(Num::from_integer(BigInt::from(it.error_count))))
}

// Set maximum number of errors before stopping
fn f_errmax(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("errmax", a, 1)?;
    let max = int(a, 0)?.to_i64().ok_or("errmax: value out of range")?;
    let old_max = it.error_max;
    it.error_max = max;
    Ok(Value::Number(Num::from_integer(BigInt::from(old_max))))
}

// Get last error code
fn f_errno(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("errno", a, 0)?;
    Ok(Value::Number(Num::from_integer(BigInt::from(it.last_errno))))
}

// Get error message for error code
fn f_errsym(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("errsym", a, 1)?;
    let code = int(a, 0)?.to_i64().ok_or("errsym: code out of range")?;
    match it.error_messages.get(&code) {
        Some(msg) => Ok(Value::Str(msg.clone())),
        None => {
            // Return a default error message
            let default_msg = match code {
                1 => "syntax error",
                2 => "undefined variable",
                3 => "division by zero",
                4 => "type error",
                5 => "range error",
                _ => "unknown error",
            };
            Ok(Value::Str(default_msg.to_string()))
        }
    }
}

// Raise an error
fn f_error(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("error", a, 1)?;
    let msg = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("error: argument must be a string".to_string()),
    };
    it.error_count += 1;
    it.last_errno = 1; // Generic error code
    if it.error_max > 0 && it.error_count >= it.error_max {
        Err(format!("error limit exceeded: {}", msg))
    } else {
        Err(msg)
    }
}

// Register a new error type
fn f_newerror(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("newerror", a, 2)?;
    let code = int(a, 0)?.to_i64().ok_or("newerror: code out of range")?;
    let msg = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("newerror: message must be a string".to_string()),
    };
    it.error_messages.insert(code, msg);
    Ok(Value::Number(Num::from_integer(BigInt::from(code))))
}

// Issue a warning (doesn't count as error)
fn f_warn(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("warn", a, 1)?;
    let msg = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("warn: argument must be a string".to_string()),
    };
    // Print warning to stderr
    eprintln!("warning: {}", msg);
    Ok(Value::Null)
}

// Catalan number
fn f_catalan(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("catalan", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("catalan: index out of range")?;
    let result = number::catalan_num(n)?;
    Ok(Value::Number(Num::from_integer(result)))
}

// String length
fn f_strlen(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strlen", a, 1)?;
    let len = match &a[0] {
        Value::Str(s) => s.len() as i64,
        _ => return Err("strlen: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(len))))
}

// Find substring index
fn f_index(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("index", a, 2)?;
    let haystack = match &a[0] {
        Value::Str(s) => s,
        _ => return Err("index: first argument must be a string".to_string()),
    };
    let needle = match &a[1] {
        Value::Str(s) => s,
        _ => return Err("index: second argument must be a string".to_string()),
    };

    match haystack.find(needle.as_str()) {
        Some(idx) => Ok(Value::Number(Num::from_integer(BigInt::from(idx as i64)))),
        None => Ok(Value::Number(Num::from_integer(BigInt::from(-1)))),
    }
}

// Check if all characters are alphabetic
fn f_isalpha(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isalpha", a, 1)?;
    let result = match &a[0] {
        Value::Str(s) => !s.is_empty() && s.chars().all(|c| c.is_alphabetic()),
        _ => return Err("isalpha: argument must be a string".to_string()),
    };
    Ok(Value::boolean(result))
}

// Check if all characters are digits
fn f_isdigit(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isdigit", a, 1)?;
    let result = match &a[0] {
        Value::Str(s) => !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()),
        _ => return Err("isdigit: argument must be a string".to_string()),
    };
    Ok(Value::boolean(result))
}

// Check if all characters are whitespace
fn f_isspace(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isspace", a, 1)?;
    let result = match &a[0] {
        Value::Str(s) => !s.is_empty() && s.chars().all(|c| c.is_whitespace()),
        _ => return Err("isspace: argument must be a string".to_string()),
    };
    Ok(Value::boolean(result))
}

// Get type of value
fn f_typeof(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("typeof", a, 1)?;
    let type_str = match &a[0] {
        Value::Number(_) => "number",
        Value::Complex(_, _) => "complex",
        Value::Str(_) => "string",
        Value::List(_) => "list",
        Value::Function(_, _) => "function",
        Value::Hash(_) => "hash",
        Value::Null => "null",
    };
    Ok(Value::Str(type_str.to_string()))
}

// Check for NaN (not applicable for rationals, but return 0)
fn f_isnan(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isnan", a, 1)?;
    match &a[0] {
        Value::Number(_) | Value::Complex(_, _) => Ok(Value::Number(Num::zero())),
        _ => Err("isnan: argument must be a number".to_string()),
    }
}

// Check for infinity (not applicable for rationals, but return 0)
fn f_isinf(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isinf", a, 1)?;
    match &a[0] {
        Value::Number(_) | Value::Complex(_, _) => Ok(Value::Number(Num::zero())),
        _ => Err("isinf: argument must be a number".to_string()),
    }
}

// Degrees to radians
fn f_d2r(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("d2r", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::d2r(n(a, 0)?, &eps)?))
}

// Radians to degrees
fn f_r2d(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("r2d", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::r2d(n(a, 0)?, &eps)?))
}

// Degrees to gradians
fn f_d2g(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("d2g", a, 1)?;
    Ok(Value::Number(number::d2g(n(a, 0)?)))
}

// Gradians to radians
fn f_g2r(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("g2r", a, 1)?;
    let eps = it.epsilon();
    Ok(Value::Number(number::g2r(n(a, 0)?, &eps)?))
}

// Gradians to degrees
fn f_g2d(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("g2d", a, 1)?;
    Ok(Value::Number(number::g2d(n(a, 0)?)))
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
    builtins.insert("root".to_string(), f_root as BuiltinFn);
    builtins.insert("cbrt".to_string(), f_cbrt as BuiltinFn);
    builtins.insert("isqrt".to_string(), f_isqrt as BuiltinFn);
    builtins.insert("iroot".to_string(), f_iroot as BuiltinFn);
    builtins.insert("re".to_string(), f_re as BuiltinFn);
    builtins.insert("im".to_string(), f_im as BuiltinFn);
    builtins.insert("arg".to_string(), f_arg as BuiltinFn);
    builtins.insert("fact".to_string(), f_fact as BuiltinFn);
    builtins.insert("comb".to_string(), f_comb as BuiltinFn);
    builtins.insert("perm".to_string(), f_perm as BuiltinFn);
    builtins.insert("fib".to_string(), f_fib as BuiltinFn);
    builtins.insert("isprime".to_string(), f_isprime as BuiltinFn);
    builtins.insert("nextprime".to_string(), f_nextprime as BuiltinFn);
    builtins.insert("prevprime".to_string(), f_prevprime as BuiltinFn);
    builtins.insert("factor".to_string(), f_factor as BuiltinFn);
    builtins.insert("lfactor".to_string(), f_lfactor as BuiltinFn);
    builtins.insert("ptest".to_string(), f_ptest as BuiltinFn);
    builtins.insert("euler".to_string(), f_euler as BuiltinFn);
    builtins.insert("bernoulli".to_string(), f_bernoulli as BuiltinFn);
    builtins.insert("jacobi".to_string(), f_jacobi as BuiltinFn);
    builtins.insert("num".to_string(), f_num as BuiltinFn);
    builtins.insert("den".to_string(), f_den as BuiltinFn);
    builtins.insert("pi".to_string(), f_pi as BuiltinFn);
    builtins.insert("e".to_string(), f_e as BuiltinFn);
    builtins.insert("base".to_string(), f_base as BuiltinFn);
    builtins.insert("exp".to_string(), f_exp as BuiltinFn);
    builtins.insert("ln".to_string(), f_ln as BuiltinFn);
    builtins.insert("log".to_string(), f_log as BuiltinFn);
    builtins.insert("log2".to_string(), f_log2 as BuiltinFn);
    builtins.insert("logn".to_string(), f_logn as BuiltinFn);
    builtins.insert("ilog10".to_string(), f_ilog10 as BuiltinFn);
    builtins.insert("ilog2".to_string(), f_ilog2 as BuiltinFn);
    builtins.insert("ilog".to_string(), f_ilog as BuiltinFn);
    builtins.insert("ilogn".to_string(), f_ilogn as BuiltinFn);
    builtins.insert("sin".to_string(), f_sin as BuiltinFn);
    builtins.insert("cos".to_string(), f_cos as BuiltinFn);
    builtins.insert("tan".to_string(), f_tan as BuiltinFn);
    builtins.insert("cot".to_string(), f_cot as BuiltinFn);
    builtins.insert("sec".to_string(), f_sec as BuiltinFn);
    builtins.insert("csc".to_string(), f_csc as BuiltinFn);
    builtins.insert("asin".to_string(), f_asin as BuiltinFn);
    builtins.insert("acos".to_string(), f_acos as BuiltinFn);
    builtins.insert("atan".to_string(), f_atan as BuiltinFn);
    builtins.insert("atan2".to_string(), f_atan2 as BuiltinFn);
    builtins.insert("acot".to_string(), f_acot as BuiltinFn);
    builtins.insert("asec".to_string(), f_asec as BuiltinFn);
    builtins.insert("acsc".to_string(), f_acsc as BuiltinFn);
    builtins.insert("sinh".to_string(), f_sinh as BuiltinFn);
    builtins.insert("cosh".to_string(), f_cosh as BuiltinFn);
    builtins.insert("tanh".to_string(), f_tanh as BuiltinFn);
    builtins.insert("coth".to_string(), f_coth as BuiltinFn);
    builtins.insert("sech".to_string(), f_sech as BuiltinFn);
    builtins.insert("csch".to_string(), f_csch as BuiltinFn);
    builtins.insert("asinh".to_string(), f_asinh as BuiltinFn);
    builtins.insert("acosh".to_string(), f_acosh as BuiltinFn);
    builtins.insert("atanh".to_string(), f_atanh as BuiltinFn);
    builtins.insert("acoth".to_string(), f_acoth as BuiltinFn);
    builtins.insert("asech".to_string(), f_asech as BuiltinFn);
    builtins.insert("acsch".to_string(), f_acsch as BuiltinFn);
    builtins.insert("cas".to_string(), f_cas as BuiltinFn);
    builtins.insert("cis".to_string(), f_cis as BuiltinFn);
    builtins.insert("conj".to_string(), f_conj as BuiltinFn);
    builtins.insert("round".to_string(), f_round as BuiltinFn);
    builtins.insert("hypot".to_string(), f_hypot as BuiltinFn);
    builtins.insert("erf".to_string(), f_erf as BuiltinFn);
    builtins.insert("erfc".to_string(), f_erfc as BuiltinFn);
    builtins.insert("gd".to_string(), f_gd as BuiltinFn);
    builtins.insert("agd".to_string(), f_agd as BuiltinFn);
    builtins.insert("j0".to_string(), f_j0 as BuiltinFn);
    builtins.insert("j1".to_string(), f_j1 as BuiltinFn);
    builtins.insert("y0".to_string(), f_y0 as BuiltinFn);
    builtins.insert("y1".to_string(), f_y1 as BuiltinFn);
    builtins.insert("gamma".to_string(), f_gamma as BuiltinFn);
    builtins.insert("lgamma".to_string(), f_lgamma as BuiltinFn);
    builtins.insert("polygamma".to_string(), f_polygamma as BuiltinFn);
    builtins.insert("zeta".to_string(), f_zeta as BuiltinFn);
    // Random number functions
    builtins.insert("rand".to_string(), f_rand as BuiltinFn);
    builtins.insert("random".to_string(), f_random as BuiltinFn);
    builtins.insert("randbit".to_string(), f_randbit as BuiltinFn);
    builtins.insert("seed".to_string(), f_seed as BuiltinFn);
    builtins.insert("srand".to_string(), f_srand as BuiltinFn);
    builtins.insert("srandom".to_string(), f_srandom as BuiltinFn);
    builtins.insert("randint".to_string(), f_randint as BuiltinFn);
    builtins.insert("randperm".to_string(), f_randperm as BuiltinFn);
    // Environment & system functions
    builtins.insert("time".to_string(), f_time as BuiltinFn);
    builtins.insert("systime".to_string(), f_systime as BuiltinFn);
    builtins.insert("ctime".to_string(), f_ctime as BuiltinFn);
    builtins.insert("sleep".to_string(), f_sleep as BuiltinFn);
    builtins.insert("getenv".to_string(), f_getenv as BuiltinFn);
    builtins.insert("putenv".to_string(), f_putenv as BuiltinFn);
    builtins.insert("system".to_string(), f_system as BuiltinFn);
    builtins.insert("usertime".to_string(), f_usertime as BuiltinFn);
    // Character classification functions
    builtins.insert("isalnum".to_string(), f_isalnum as BuiltinFn);
    builtins.insert("isupper".to_string(), f_isupper as BuiltinFn);
    builtins.insert("islower".to_string(), f_islower as BuiltinFn);
    builtins.insert("isprint".to_string(), f_isprint as BuiltinFn);
    builtins.insert("isgraph".to_string(), f_isgraph as BuiltinFn);
    builtins.insert("iscntrl".to_string(), f_iscntrl as BuiltinFn);
    builtins.insert("ispunct".to_string(), f_ispunct as BuiltinFn);
    builtins.insert("isxdigit".to_string(), f_isxdigit as BuiltinFn);
    builtins.insert("isascii".to_string(), f_isascii as BuiltinFn);
    builtins.insert("toupper".to_string(), f_toupper as BuiltinFn);
    builtins.insert("tolower".to_string(), f_tolower as BuiltinFn);
    builtins.insert("strrev".to_string(), f_strrev as BuiltinFn);
    // Advanced modular arithmetic functions
    builtins.insert("pmod".to_string(), f_pmod as BuiltinFn);
    builtins.insert("quomod".to_string(), f_quomod as BuiltinFn);
    builtins.insert("quo".to_string(), f_quo as BuiltinFn);
    builtins.insert("rem".to_string(), f_rem as BuiltinFn);
    builtins.insert("hnrmod".to_string(), f_hnrmod as BuiltinFn);
    // Rational approximations
    builtins.insert("appr".to_string(), f_appr as BuiltinFn);
    builtins.insert("cfappr".to_string(), f_cfappr as BuiltinFn);
    builtins.insert("cfsim".to_string(), f_cfsim as BuiltinFn);
    builtins.insert("scale".to_string(), f_scale as BuiltinFn);
    // Matrix operations
    builtins.insert("matdim".to_string(), f_matdim as BuiltinFn);
    builtins.insert("mattrans".to_string(), f_mattrans as BuiltinFn);
    builtins.insert("mattrace".to_string(), f_mattrace as BuiltinFn);
    builtins.insert("det".to_string(), f_det as BuiltinFn);
    builtins.insert("inverse".to_string(), f_inverse as BuiltinFn);
    builtins.insert("matsum".to_string(), f_matsum as BuiltinFn);
    builtins.insert("matmin".to_string(), f_matmin as BuiltinFn);
    builtins.insert("matmax".to_string(), f_matmax as BuiltinFn);
    builtins.insert("matfill".to_string(), f_matfill as BuiltinFn);
    builtins.insert("catalan".to_string(), f_catalan as BuiltinFn);
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
    // String operations
    builtins.insert("strlen".to_string(), f_strlen as BuiltinFn);
    builtins.insert("index".to_string(), f_index as BuiltinFn);
    builtins.insert("isalpha".to_string(), f_isalpha as BuiltinFn);
    builtins.insert("isdigit".to_string(), f_isdigit as BuiltinFn);
    builtins.insert("isspace".to_string(), f_isspace as BuiltinFn);
    // Type operations
    builtins.insert("typeof".to_string(), f_typeof as BuiltinFn);
    builtins.insert("isnan".to_string(), f_isnan as BuiltinFn);
    builtins.insert("isinf".to_string(), f_isinf as BuiltinFn);
    // Angle conversions
    builtins.insert("d2r".to_string(), f_d2r as BuiltinFn);
    builtins.insert("r2d".to_string(), f_r2d as BuiltinFn);
    builtins.insert("d2g".to_string(), f_d2g as BuiltinFn);
    builtins.insert("g2r".to_string(), f_g2r as BuiltinFn);
    builtins.insert("g2d".to_string(), f_g2d as BuiltinFn);
    // Hash & associative arrays (Phase 5.5)
    builtins.insert("assoc".to_string(), f_assoc as BuiltinFn);
    builtins.insert("indices".to_string(), f_indices as BuiltinFn);
    builtins.insert("insert".to_string(), f_insert as BuiltinFn);
    builtins.insert("delete".to_string(), f_delete as BuiltinFn);
    builtins.insert("count".to_string(), f_count as BuiltinFn);
    builtins.insert("join".to_string(), f_join as BuiltinFn);
    // Error & exception handling (Phase 6.3)
    builtins.insert("errcount".to_string(), f_errcount as BuiltinFn);
    builtins.insert("errmax".to_string(), f_errmax as BuiltinFn);
    builtins.insert("errno".to_string(), f_errno as BuiltinFn);
    builtins.insert("errsym".to_string(), f_errsym as BuiltinFn);
    builtins.insert("error".to_string(), f_error as BuiltinFn);
    builtins.insert("newerror".to_string(), f_newerror as BuiltinFn);
    builtins.insert("warn".to_string(), f_warn as BuiltinFn);
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
        ("root", "root(x,n)", "nth root"),
        ("cbrt", "cbrt(x)", "cube root"),
        ("isqrt", "isqrt(x)", "integer square root"),
        ("iroot", "iroot(x,n)", "integer nth root"),
        ("re", "re(z)", "real part of complex number"),
        ("im", "im(z)", "imaginary part of complex number"),
        ("arg", "arg(z)", "argument (phase angle) of complex number"),
        ("fact", "fact(n)", "factorial"),
        ("comb", "comb(n,k)", "combinations"),
        ("perm", "perm(n,k)", "permutations"),
        ("fib", "fib(n)", "nth Fibonacci number"),
        ("isprime", "isprime(n)", "is n prime? (1 or 0)"),
        ("nextprime", "nextprime(n)", "next prime after n"),
        ("prevprime", "prevprime(n)", "previous prime before n"),
        ("factor", "factor(n)", "prime factorization (returns list)"),
        ("lfactor", "lfactor(n)", "largest prime factor"),
        ("ptest", "ptest(n,k)", "probabilistic primality test"),
        ("euler", "euler(n)", "Euler number E_n"),
        ("bernoulli", "bernoulli(n)", "Bernoulli number B_n"),
        ("jacobi", "jacobi(a,n)", "Jacobi symbol (a|n)"),
        ("num", "num(x)", "numerator"),
        ("den", "den(x)", "denominator"),
        ("pi", "pi()", "π constant (60 digits)"),
        ("e", "e()", "e constant (60 digits)"),
        ("base", "base([ibase[,obase]])", "get/set input and output base (2-36)"),
        ("exp", "exp(x)", "e^x"),
        ("ln", "ln(x)", "natural logarithm"),
        ("log", "log(x)", "base-10 logarithm"),
        ("log2", "log2(x)", "base-2 logarithm"),
        ("logn", "logn(x,n)", "logarithm base n"),
        ("ilog10", "ilog10(x)", "integer log base 10"),
        ("ilog2", "ilog2(x)", "integer log base 2"),
        ("ilog", "ilog(x)", "integer log base e"),
        ("ilogn", "ilogn(x,n)", "integer log base n"),
        ("sin", "sin(x)", "sine (radians)"),
        ("cos", "cos(x)", "cosine (radians)"),
        ("tan", "tan(x)", "tangent (radians)"),
        ("cot", "cot(x)", "cotangent (radians)"),
        ("sec", "sec(x)", "secant (radians)"),
        ("csc", "csc(x)", "cosecant (radians)"),
        ("asin", "asin(x)", "inverse sine"),
        ("acos", "acos(x)", "inverse cosine"),
        ("atan", "atan(x)", "inverse tangent"),
        ("atan2", "atan2(y,x)", "two-argument inverse tangent"),
        ("acot", "acot(x)", "inverse cotangent"),
        ("asec", "asec(x)", "inverse secant"),
        ("acsc", "acsc(x)", "inverse cosecant"),
        ("sinh", "sinh(x)", "hyperbolic sine"),
        ("cosh", "cosh(x)", "hyperbolic cosine"),
        ("tanh", "tanh(x)", "hyperbolic tangent"),
        ("coth", "coth(x)", "hyperbolic cotangent"),
        ("sech", "sech(x)", "hyperbolic secant"),
        ("csch", "csch(x)", "hyperbolic cosecant"),
        ("asinh", "asinh(x)", "inverse hyperbolic sine"),
        ("acosh", "acosh(x)", "inverse hyperbolic cosine"),
        ("atanh", "atanh(x)", "inverse hyperbolic tangent"),
        ("acoth", "acoth(x)", "inverse hyperbolic cotangent"),
        ("asech", "asech(x)", "inverse hyperbolic secant"),
        ("acsch", "acsch(x)", "inverse hyperbolic cosecant"),
        ("cas", "cas(x)", "cosine + sine"),
        ("cis", "cis(x)", "cos(x) + i*sin(x) (returns complex)"),
        ("conj", "conj(x)", "complex conjugate"),
        ("round", "round(x[,places])", "round to decimal places"),
        ("hypot", "hypot(x,y)", "sqrt(x^2 + y^2)"),
        ("erf", "erf(x)", "error function"),
        ("erfc", "erfc(x)", "complementary error function"),
        ("gd", "gd(x)", "Gudermannian function"),
        ("agd", "agd(x)", "inverse Gudermannian function"),
        ("j0", "j0(x)", "Bessel function J0"),
        ("j1", "j1(x)", "Bessel function J1"),
        ("y0", "y0(x)", "Bessel function Y0 (second kind)"),
        ("y1", "y1(x)", "Bessel function Y1 (second kind)"),
        ("gamma", "gamma(x)", "gamma function (generalized factorial)"),
        ("lgamma", "lgamma(x)", "log-gamma function"),
        ("polygamma", "polygamma(n,x)", "polygamma function (nth derivative of log-gamma)"),
        ("zeta", "zeta(s)", "Riemann zeta function"),
        ("rand", "rand()", "random 32-bit integer"),
        ("random", "random()", "random float [0,1)"),
        ("randbit", "randbit()", "random bit (0 or 1)"),
        ("seed", "seed(s)", "set random seed"),
        ("srand", "srand(s)", "set random seed (alias)"),
        ("srandom", "srandom(s)", "set random seed (alias)"),
        ("randint", "randint(a,b)", "random integer in [a,b]"),
        ("randperm", "randperm(n)", "random permutation of 0..n-1 (returns list)"),
        ("time", "time()", "current Unix timestamp (seconds since epoch)"),
        ("systime", "systime()", "system time (alias for time)"),
        ("ctime", "ctime(t)", "convert Unix timestamp to string"),
        ("sleep", "sleep(s)", "sleep for s seconds"),
        ("getenv", "getenv(name)", "get environment variable"),
        ("putenv", "putenv(name,value)", "set environment variable"),
        ("system", "system(cmd)", "execute shell command (returns exit code)"),
        ("usertime", "usertime()", "user/system time in seconds"),
        ("isalnum", "isalnum(s)", "is alphanumeric (1 or 0)"),
        ("isupper", "isupper(s)", "is uppercase letter (1 or 0)"),
        ("islower", "islower(s)", "is lowercase letter (1 or 0)"),
        ("isprint", "isprint(s)", "is printable (1 or 0)"),
        ("isgraph", "isgraph(s)", "is visible character (1 or 0)"),
        ("iscntrl", "iscntrl(s)", "is control character (1 or 0)"),
        ("ispunct", "ispunct(s)", "is punctuation (1 or 0)"),
        ("isxdigit", "isxdigit(s)", "is hex digit (1 or 0)"),
        ("isascii", "isascii(s)", "is ASCII-only (1 or 0)"),
        ("toupper", "toupper(s)", "convert to uppercase"),
        ("tolower", "tolower(s)", "convert to lowercase"),
        ("strrev", "strrev(s)", "reverse string"),
        ("pmod", "pmod(x,y)", "positive modulus (result in [0,y))"),
        ("quomod", "quomod(x,y)", "quotient and modulus (returns [q,r])"),
        ("quo", "quo(x,y)", "quotient (floor(x/y))"),
        ("rem", "rem(x,y)", "remainder (x - y*floor(x/y))"),
        ("hnrmod", "hnrmod(x,y)", "Hensel modular"),
        ("appr", "appr(x[,eps])", "rational approximation within epsilon"),
        ("cfappr", "cfappr(x[,maxd])", "continued fraction approximation"),
        ("cfsim", "cfsim(x[,maxd])", "continued fraction simplification"),
        ("scale", "scale(x[,places])", "scale to decimal places"),
        ("matdim", "matdim(m)", "matrix dimensions [rows, cols]"),
        ("mattrans", "mattrans(m)", "matrix transpose"),
        ("mattrace", "mattrace(m)", "matrix trace (sum of diagonal)"),
        ("det", "det(m)", "matrix determinant (2x2, 3x3)"),
        ("inverse", "inverse(m)", "matrix inverse (2x2)"),
        ("matsum", "matsum(m)", "sum of all matrix elements"),
        ("matmin", "matmin(m)", "minimum matrix element"),
        ("matmax", "matmax(m)", "maximum matrix element"),
        ("matfill", "matfill(r,c,v)", "create matrix filled with value"),
        ("catalan", "catalan(n)", "Catalan number"),
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
        ("strlen", "strlen(s)", "length of string"),
        ("index", "index(haystack,needle)", "find substring position (-1 if not found)"),
        ("isalpha", "isalpha(s)", "is string all alphabetic? (1 or 0)"),
        ("isdigit", "isdigit(s)", "is string all digits? (1 or 0)"),
        ("isspace", "isspace(s)", "is string all whitespace? (1 or 0)"),
        ("typeof", "typeof(x)", "get type of value (number, complex, string, list, function, null)"),
        ("isnan", "isnan(x)", "is NaN? (always 0 for rationals)"),
        ("isinf", "isinf(x)", "is infinite? (always 0 for rationals)"),
        ("d2r", "d2r(x)", "degrees to radians"),
        ("r2d", "r2d(x)", "radians to degrees"),
        ("d2g", "d2g(x)", "degrees to gradians"),
        ("g2r", "g2r(x)", "gradians to radians"),
        ("g2d", "g2d(x)", "gradians to degrees"),
        // Hash & associative arrays (Phase 5.5)
        ("assoc", "assoc(k1,v1,...)", "create associative array from key-value pairs"),
        ("indices", "indices(h)", "get all keys from hash as list"),
        ("insert", "insert(h,key,val)", "insert/update key-value pair in hash"),
        ("delete", "delete(h,key)", "delete key from hash"),
        ("count", "count(h)", "count key-value pairs in hash"),
        ("join", "join(h,sep)", "join hash values with separator"),
        // Error & exception handling (Phase 6.3)
        ("errcount", "errcount()", "number of errors occurred"),
        ("errmax", "errmax(n)", "set max errors before stopping (0=unlimited)"),
        ("errno", "errno()", "last error code"),
        ("errsym", "errsym(code)", "error message for error code"),
        ("error", "error(msg)", "raise an error with message"),
        ("newerror", "newerror(code,msg)", "register a new error type"),
        ("warn", "warn(msg)", "issue a warning (not counted as error)"),
    ]
}
