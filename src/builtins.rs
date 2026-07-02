//! Builtin functions (~40 of them).

use crate::eval::BuiltinFn;
use crate::eval::Interp;
use crate::number::{self, Num};
use crate::value::Value;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};

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
    Ok(Value::Number(
        sum / Num::from_integer(BigInt::from(a.len())),
    ))
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
    let angle_f64 = i
        .to_f64()
        .and_then(|im| r.to_f64().map(|re| im.atan2(re)))
        .ok_or("overflow in arg")?;
    let angle = Num::from_float(angle_f64).ok_or("non-finite result in arg")?;
    Ok(Value::Number(angle))
}

// Factorial
fn f_fact(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fact", a, 1)?;
    let n_val = int(a, 0)?;
    if n_val.is_negative() {
        return Err("factorial of negative number".to_string());
    }
    let n_u32 = n_val.to_u32().ok_or("factorial argument too large")?;
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
        Ok(Value::Number(Num::from_integer(BigInt::from(if is_p {
            1
        } else {
            0
        }))))
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

// Next candidate prime: nextcand(n [, count [, skip [, residue [, modulus]]]])
fn f_nextcand(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("nextcand", a, 1, 5)?;
    let n = int(a, 0)?.to_i64().ok_or("nextcand: n too large")?;
    let count = if a.len() > 1 {
        int(a, 1)?.to_i64().ok_or("nextcand: count too large")?
    } else {
        1
    };
    let skip = if a.len() > 2 {
        int(a, 2)?.to_i64().ok_or("nextcand: skip too large")?
    } else {
        1
    };
    let residue = if a.len() > 3 {
        int(a, 3)?.to_i64().ok_or("nextcand: residue too large")?
    } else {
        0
    };
    let modulus = if a.len() > 4 {
        int(a, 4)?.to_i64().ok_or("nextcand: modulus too large")?
    } else {
        0
    };
    Ok(Value::Number(Num::from_integer(number::nextcand(
        n, count, skip, residue, modulus,
    )?)))
}

// Previous candidate prime: prevcand(n [, count [, skip [, residue [, modulus]]]])
fn f_prevcand(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("prevcand", a, 1, 5)?;
    let n = int(a, 0)?.to_i64().ok_or("prevcand: n too large")?;
    let count = if a.len() > 1 {
        int(a, 1)?.to_i64().ok_or("prevcand: count too large")?
    } else {
        1
    };
    let skip = if a.len() > 2 {
        int(a, 2)?.to_i64().ok_or("prevcand: skip too large")?
    } else {
        1
    };
    let residue = if a.len() > 3 {
        int(a, 3)?.to_i64().ok_or("prevcand: residue too large")?
    } else {
        0
    };
    let modulus = if a.len() > 4 {
        int(a, 4)?.to_i64().ok_or("prevcand: modulus too large")?
    } else {
        0
    };
    Ok(Value::Number(Num::from_integer(number::prevcand(
        n, count, skip, residue, modulus,
    )?)))
}

// gcdrem(x, y): remove factors common with y from x
fn f_gcdrem(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("gcdrem", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;
    Ok(Value::Number(Num::from_integer(number::gcdrem(&x, &y))))
}

// bround(x, places): round to binary places
fn f_bround(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("bround", a, 1, 2)?;
    let x = n(a, 0)?;
    let places = if a.len() == 2 {
        int(a, 1)?.to_i64().ok_or("bround: places out of range")?
    } else {
        0
    };
    Ok(Value::Number(number::bround(x, places)))
}

// btrunc(x, places): truncate to binary places
fn f_btrunc(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("btrunc", a, 1, 2)?;
    let x = n(a, 0)?;
    let places = if a.len() == 2 {
        int(a, 1)?.to_i64().ok_or("btrunc: places out of range")?
    } else {
        0
    };
    Ok(Value::Number(number::btrunc(x, places)))
}

// Prime factorization
fn f_factor(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("factor", a, 1)?;
    let n = int(a, 0)?.to_i64().ok_or("factor: number too large")?;
    let factors = number::factor(n)?;
    let result_list: Vec<Value> = factors
        .iter()
        .map(|f| Value::Number(Num::from_integer(f.clone())))
        .collect();
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
        if !(2..=36).contains(&base_u32) {
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
        if !(2..=36).contains(&ibase_u32) {
            return Err("ibase must be between 2 and 36".to_string());
        }
        if !(2..=36).contains(&obase_u32) {
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
fn f_log(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("log", a, 1)?;
    let x = n(a, 0)?;
    if x.is_negative() || x.is_zero() {
        return Err("log of non-positive number".to_string());
    }
    let eps = it.epsilon();
    Ok(Value::Number(number::logn(
        x,
        &Num::from_integer(BigInt::from(10)),
        &eps,
    )?))
}

// Log base 2
fn f_log2(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("log2", a, 1)?;
    let x = n(a, 0)?;
    if x.is_negative() || x.is_zero() {
        return Err("log2 of non-positive number".to_string());
    }
    let eps = it.epsilon();
    Ok(Value::Number(number::logn(
        x,
        &Num::from_integer(BigInt::from(2)),
        &eps,
    )?))
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
    Num::from_float(r)
        .ok_or_else(|| "random: non-finite result".to_string())
        .map(Value::Number)
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
    let result_list: Vec<Value> = perm
        .iter()
        .map(|x| Value::Number(Num::from_integer(x.clone())))
        .collect();
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
    let seconds = n(a, 0)?
        .to_f64()
        .ok_or("sleep: seconds must be convertible to float")?;
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
    Ok(Value::Number(
        Num::from_float(elapsed).ok_or("usertime: overflow")?,
    ))
}

// Phase 5.1: Character Classification Functions

fn f_isalnum(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isalnum", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isalnum: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::isalnum(&s),
    ))))
}

fn f_isupper(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isupper", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isupper: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::isupper(&s),
    ))))
}

fn f_islower(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("islower", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("islower: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::islower(&s),
    ))))
}

fn f_isprint(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isprint", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isprint: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::isprint(&s),
    ))))
}

fn f_isgraph(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isgraph", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isgraph: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::isgraph(&s),
    ))))
}

fn f_iscntrl(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("iscntrl", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("iscntrl: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::iscntrl(&s),
    ))))
}

fn f_ispunct(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ispunct", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("ispunct: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::ispunct(&s),
    ))))
}

fn f_isxdigit(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isxdigit", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isxdigit: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::isxdigit(&s),
    ))))
}

fn f_isascii(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isascii", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isascii: argument must be a string".to_string()),
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(
        number::isascii(&s),
    ))))
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
    Ok(Value::List(vec![
        Value::Number(quotient),
        Value::Number(remainder),
    ]))
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
                return Ok(Value::List(vec![
                    Value::Number(Num::from_integer(BigInt::from(0))),
                    Value::Number(Num::from_integer(BigInt::from(0))),
                ]));
            }
            let m = rows.len() as i64;
            let n = match &rows[0] {
                Value::List(row) => row.len() as i64,
                _ => return Err("matdim: matrix rows must be lists".to_string()),
            };
            Ok(Value::List(vec![
                Value::Number(Num::from_integer(BigInt::from(m))),
                Value::Number(Num::from_integer(BigInt::from(n))),
            ]))
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
                                _ => {
                                    return Err(
                                        "mattrans: matrix elements must be numbers".to_string()
                                    )
                                }
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("mattrans: matrix rows must be lists".to_string()),
                }
            }
            let transposed = number::mattrans(&matrix)?;
            let result = transposed
                .into_iter()
                .map(|row| Value::List(row.into_iter().map(Value::Number).collect()))
                .collect();
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
                                _ => {
                                    return Err(
                                        "mattrace: matrix elements must be numbers".to_string()
                                    )
                                }
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
                                _ => {
                                    return Err(
                                        "inverse: matrix elements must be numbers".to_string()
                                    )
                                }
                            }
                        }
                        matrix.push(row_nums);
                    }
                    _ => return Err("inverse: matrix rows must be lists".to_string()),
                }
            }
            let inv = number::inverse(&matrix)?;
            let result = inv
                .into_iter()
                .map(|row| Value::List(row.into_iter().map(Value::Number).collect()))
                .collect();
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
                                _ => {
                                    return Err(
                                        "matsum: matrix elements must be numbers".to_string()
                                    )
                                }
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
                                _ => {
                                    return Err(
                                        "matmin: matrix elements must be numbers".to_string()
                                    )
                                }
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
                                _ => {
                                    return Err(
                                        "matmax: matrix elements must be numbers".to_string()
                                    )
                                }
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
    let result = matrix
        .into_iter()
        .map(|row| Value::List(row.into_iter().map(Value::Number).collect()))
        .collect();
    Ok(Value::List(result))
}

// Phase 5.5: Hash & Associative Arrays

// Create an associative array from key-value pairs
fn f_assoc(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() {
        return Ok(Value::Hash(std::collections::HashMap::new()));
    }
    if !a.len().is_multiple_of(2) {
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
            let items = keys.into_iter().map(Value::Str).collect();
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
    if a.len() == 1 {
        // Hash count: count(hash) - counts key-value pairs
        match &a[0] {
            Value::Hash(map) => Ok(Value::Number(Num::from_integer(BigInt::from(
                map.len() as i64
            )))),
            _ => Err("count: argument must be a hash or list".to_string()),
        }
    } else if a.len() == 2 {
        // List count: count(list, value) - counts occurrences of value
        let items = match &a[0] {
            Value::List(items) => items.clone(),
            _ => return Err("count: first argument must be list".to_string()),
        };
        let search_val = &a[1];

        let count = items.iter().filter(|item| *item == search_val).count();
        Ok(Value::Number(Num::from_integer(BigInt::from(count as i64))))
    } else {
        Err("count: expects 1 or 2 arguments".to_string())
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
    Ok(Value::Number(Num::from_integer(BigInt::from(
        it.error_count,
    ))))
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
    Ok(Value::Number(Num::from_integer(BigInt::from(
        it.last_errno,
    ))))
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
fn f_warn(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("warn", a, 1)?;
    let msg = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("warn: argument must be a string".to_string()),
    };
    // Print warning to stderr
    eprintln!("warning: {}", msg);
    Ok(Value::Null)
}

// Phase 6.1: File I/O

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

// Open a file
fn f_fopen(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fopen", a, 2)?;
    let filename = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("fopen: filename must be a string".to_string()),
    };
    let mode = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("fopen: mode must be a string".to_string()),
    };

    // Validate the file can be opened
    let _f = match mode.as_str() {
        "r" => {
            File::open(&filename).map_err(|e| format!("fopen: cannot open {}: {}", filename, e))?
        }
        "w" => OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&filename)
            .map_err(|e| format!("fopen: cannot create {}: {}", filename, e))?,
        "a" => OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)
            .map_err(|e| format!("fopen: cannot open {}: {}", filename, e))?,
        _ => return Err("fopen: mode must be 'r', 'w', or 'a'".to_string()),
    };

    // Store file info
    let fd = it.next_fd;
    it.open_files.push((filename, 0)); // path, position
    it.next_fd += 1;

    Ok(Value::Number(Num::from_integer(BigInt::from(fd))))
}

// Close a file
fn f_fclose(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fclose", a, 1)?;
    let fd = int(a, 0)?.to_i64().ok_or("fclose: fd out of range")?;

    if fd < 3 || fd as usize >= it.next_fd as usize || fd as usize - 3 >= it.open_files.len() {
        return Err("fclose: invalid file descriptor".to_string());
    }

    // Remove from open files
    let idx = (fd - 3) as usize;
    if idx < it.open_files.len() {
        it.open_files.remove(idx);
    }

    Ok(Value::Number(Num::zero()))
}

// Read a line from a file
fn f_fgets(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fgets", a, 1)?;
    let fd = int(a, 0)?.to_i64().ok_or("fgets: fd out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fgets: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fgets: file not open".to_string());
    }

    let (path, _pos) = &it.open_files[idx];
    let mut file = File::open(path).map_err(|e| format!("fgets: cannot read {}: {}", path, e))?;

    // Simple approach: read entire file into memory (not ideal for large files)
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("fgets: read error: {}", e))?;

    // For now, return first line
    match contents.lines().next() {
        Some(line) => Ok(Value::Str(line.to_string())),
        None => Ok(Value::Str(String::new())),
    }
}

// Read a character from a file
fn f_fgetc(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fgetc", a, 1)?;
    let fd = int(a, 0)?.to_i64().ok_or("fgetc: fd out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fgetc: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fgetc: file not open".to_string());
    }

    let (path, pos) = it.open_files[idx].clone();
    let mut file = File::open(&path).map_err(|e| format!("fgetc: cannot read {}: {}", path, e))?;

    file.seek(SeekFrom::Start(pos))
        .map_err(|e| format!("fgetc: seek error: {}", e))?;

    let mut buf = [0; 1];
    match file.read_exact(&mut buf) {
        Ok(_) => {
            it.open_files[idx].1 = pos + 1;
            Ok(Value::Number(Num::from_integer(BigInt::from(
                buf[0] as i64,
            ))))
        }
        Err(_) => Ok(Value::Number(Num::from_integer(BigInt::from(-1)))),
    }
}

// Write string to a file
fn f_fputs(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fputs", a, 2)?;
    let fd = int(a, 0)?.to_i64().ok_or("fputs: fd out of range")?;
    let text = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("fputs: text must be a string".to_string()),
    };

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fputs: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fputs: file not open".to_string());
    }

    let path = it.open_files[idx].0.clone();
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .map_err(|e| format!("fputs: cannot write to {}: {}", path, e))?;

    file.write_all(text.as_bytes())
        .map_err(|e| format!("fputs: write error: {}", e))?;

    Ok(Value::Number(Num::from_integer(BigInt::from(
        text.len() as i64
    ))))
}

// Write character to a file
fn f_fputc(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fputc", a, 2)?;
    let fd = int(a, 0)?.to_i64().ok_or("fputc: fd out of range")?;
    let ch = int(a, 1)?.to_i64().ok_or("fputc: character out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fputc: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fputc: file not open".to_string());
    }

    let path = it.open_files[idx].0.clone();
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .map_err(|e| format!("fputc: cannot write to {}: {}", path, e))?;

    let byte = (ch & 0xFF) as u8;
    file.write_all(&[byte])
        .map_err(|e| format!("fputc: write error: {}", e))?;

    Ok(Value::Number(Num::from_integer(BigInt::from(byte as i64))))
}

// Seek to position in file
fn f_seek(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("seek", a, 2)?;
    let fd = int(a, 0)?.to_i64().ok_or("seek: fd out of range")?;
    let offset = int(a, 1)?.to_i64().ok_or("seek: offset out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("seek: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("seek: file not open".to_string());
    }

    if offset < 0 {
        return Err("seek: offset cannot be negative".to_string());
    }

    it.open_files[idx].1 = offset as u64;
    Ok(Value::Number(Num::from_integer(BigInt::from(offset))))
}

// Get current position in file
fn f_tell(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("tell", a, 1)?;
    let fd = int(a, 0)?.to_i64().ok_or("tell: fd out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("tell: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("tell: file not open".to_string());
    }

    let pos = it.open_files[idx].1;
    Ok(Value::Number(Num::from_integer(BigInt::from(pos as i64))))
}

// Check end-of-file
fn f_eof(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("eof", a, 1)?;
    let fd = int(a, 0)?.to_i64().ok_or("eof: fd out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("eof: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("eof: file not open".to_string());
    }

    let (path, pos) = &it.open_files[idx];

    let metadata =
        std::fs::metadata(path).map_err(|e| format!("eof: cannot stat {}: {}", path, e))?;

    let at_eof = *pos >= metadata.len();
    Ok(Value::Number(Num::from_integer(BigInt::from(if at_eof {
        1
    } else {
        0
    }))))
}

// Remove (delete) a file
fn f_remove(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("remove", a, 1)?;
    let filename = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("remove: filename must be a string".to_string()),
    };

    std::fs::remove_file(&filename)
        .map_err(|e| format!("remove: cannot delete {}: {}", filename, e))?;

    Ok(Value::Number(Num::zero()))
}

// Rename a file
fn f_rename(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rename", a, 2)?;
    let old_name = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("rename: old name must be a string".to_string()),
    };
    let new_name = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("rename: new name must be a string".to_string()),
    };

    std::fs::rename(&old_name, &new_name)
        .map_err(|e| format!("rename: cannot rename {} to {}: {}", old_name, new_name, e))?;

    Ok(Value::Number(Num::zero()))
}

// Flush file buffer (no-op in our implementation, just return success)
fn f_fflush(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fflush", a, 1)?;
    let fd = int(a, 0)?.to_i64().ok_or("fflush: fd out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fflush: invalid file descriptor".to_string());
    }

    // In our implementation, we don't maintain actual file buffers, so this is a no-op
    Ok(Value::Number(Num::zero()))
}

// Rewind file to beginning
fn f_rewind(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rewind", a, 1)?;
    let fd = int(a, 0)?.to_i64().ok_or("rewind: fd out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("rewind: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("rewind: file not open".to_string());
    }

    it.open_files[idx].1 = 0; // Reset position to 0
    Ok(Value::Number(Num::zero()))
}

// Get file descriptor number (just returns the fd itself)
fn f_fileno(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fileno", a, 1)?;
    let fd = int(a, 0)?;
    Ok(Value::Number(Num::from_integer(fd)))
}

// Read bytes from file
fn f_fread(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fread", a, 2)?;
    let fd = int(a, 0)?.to_i64().ok_or("fread: fd out of range")?;
    let size = int(a, 1)?.to_usize().ok_or("fread: size out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fread: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fread: file not open".to_string());
    }

    let (path, pos) = it.open_files[idx].clone();
    let mut file = File::open(&path).map_err(|e| format!("fread: cannot read {}: {}", path, e))?;

    file.seek(SeekFrom::Start(pos))
        .map_err(|e| format!("fread: seek error: {}", e))?;

    let mut buf = vec![0; size];
    match file.read(&mut buf) {
        Ok(n) => {
            it.open_files[idx].1 = pos + n as u64;
            // Return the bytes as a string
            let data_str = String::from_utf8_lossy(&buf[..n]).to_string();
            Ok(Value::Str(data_str))
        }
        Err(e) => Err(format!("fread: read error: {}", e)),
    }
}

// Write bytes to file
fn f_fwrite(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fwrite", a, 2)?;
    let fd = int(a, 0)?.to_i64().ok_or("fwrite: fd out of range")?;
    let data = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("fwrite: data must be a string".to_string()),
    };

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fwrite: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fwrite: file not open".to_string());
    }

    let path = it.open_files[idx].0.clone();
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .map_err(|e| format!("fwrite: cannot write to {}: {}", path, e))?;

    match file.write_all(data.as_bytes()) {
        Ok(_) => Ok(Value::Number(Num::from_integer(BigInt::from(
            data.len() as i64
        )))),
        Err(e) => Err(format!("fwrite: write error: {}", e)),
    }
}

// Seek with whence parameter (0=SEEK_SET, 1=SEEK_CUR, 2=SEEK_END)
fn f_fseek(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fseek", a, 3)?;
    let fd = int(a, 0)?.to_i64().ok_or("fseek: fd out of range")?;
    let offset = int(a, 1)?.to_i64().ok_or("fseek: offset out of range")?;
    let whence = int(a, 2)?.to_i64().ok_or("fseek: whence out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fseek: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fseek: file not open".to_string());
    }

    let (path, _) = &it.open_files[idx];
    let _file = File::open(path).map_err(|e| format!("fseek: cannot open {}: {}", path, e))?;

    let new_pos = match whence {
        0 => {
            // SEEK_SET
            if offset < 0 {
                return Err("fseek: offset cannot be negative with SEEK_SET".to_string());
            }
            offset as u64
        }
        1 => {
            // SEEK_CUR
            let current = it.open_files[idx].1 as i64;
            let new = current + offset;
            if new < 0 {
                return Err("fseek: resulting position would be negative".to_string());
            }
            new as u64
        }
        2 => {
            // SEEK_END
            let metadata = std::fs::metadata(path)
                .map_err(|e| format!("fseek: cannot stat {}: {}", path, e))?;
            let end = metadata.len() as i64;
            let new = end + offset;
            if new < 0 {
                return Err("fseek: resulting position would be negative".to_string());
            }
            new as u64
        }
        _ => {
            return Err(
                "fseek: whence must be 0 (SEEK_SET), 1 (SEEK_CUR), or 2 (SEEK_END)".to_string(),
            )
        }
    };

    it.open_files[idx].1 = new_pos;
    Ok(Value::Number(Num::from_integer(BigInt::from(
        new_pos as i64,
    ))))
}

// Write formatted string to file (simplified - just writes the value as string)
fn f_fprintf(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.len() < 2 {
        return Err("fprintf: expects at least 2 arguments (fd, value, ...)".to_string());
    }

    let fd = int(a, 0)?.to_i64().ok_or("fprintf: fd out of range")?;

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fprintf: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fprintf: file not open".to_string());
    }

    // Render all arguments as strings and concatenate
    let mut output = String::new();
    for item in a.iter().skip(1) {
        match item {
            Value::Str(s) => output.push_str(s),
            Value::Number(n) => output.push_str(&number::to_decimal_string(n, 15)),
            _ => output.push_str(&item.render(&it.cfg)),
        }
    }

    let path = it.open_files[idx].0.clone();
    let mut file = OpenOptions::new()
        .append(true)
        .open(&path)
        .map_err(|e| format!("fprintf: cannot write to {}: {}", path, e))?;

    file.write_all(output.as_bytes())
        .map_err(|e| format!("fprintf: write error: {}", e))?;

    Ok(Value::Number(Num::from_integer(BigInt::from(
        output.len() as i64
    ))))
}

// Read formatted data from file (simplified scanf)
fn f_fscan(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fscan", a, 2)?;
    let fd = int(a, 0)?.to_i64().ok_or("fscan: fd out of range")?;
    let fmt = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("fscan: format must be a string".to_string()),
    };

    if fd < 3 || (fd as usize) >= it.next_fd as usize {
        return Err("fscan: invalid file descriptor".to_string());
    }

    let idx = (fd - 3) as usize;
    if idx >= it.open_files.len() {
        return Err("fscan: file not open".to_string());
    }

    let (path, pos) = it.open_files[idx].clone();
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("fscan: cannot read {}: {}", path, e))?;

    // Skip to current position
    let remaining = if (pos as usize) < content.len() {
        &content[(pos as usize)..]
    } else {
        ""
    };

    let mut results = Vec::new();
    let mut input_idx = 0;
    let mut fmt_idx = 0;
    let fmt_bytes = fmt.as_bytes();
    let remaining_bytes = remaining.as_bytes();

    while fmt_idx < fmt_bytes.len() && input_idx < remaining_bytes.len() {
        if fmt_bytes[fmt_idx] == b'%' && fmt_idx + 1 < fmt_bytes.len() {
            fmt_idx += 1;
            let spec = fmt_bytes[fmt_idx] as char;

            // Skip whitespace in input
            while input_idx < remaining_bytes.len()
                && (remaining_bytes[input_idx] as char).is_whitespace()
            {
                input_idx += 1;
            }

            match spec {
                'd' | 'i' => {
                    // Read integer
                    let start = input_idx;
                    if input_idx < remaining_bytes.len() && (remaining_bytes[input_idx] as char) == '-' {
                        input_idx += 1;
                    }
                    while input_idx < remaining_bytes.len() && (remaining_bytes[input_idx] as char).is_ascii_digit() {
                        input_idx += 1;
                    }
                    if input_idx > start {
                        let num_str = String::from_utf8_lossy(&remaining_bytes[start..input_idx]);
                        if let Ok(n) = num_str.parse::<i64>() {
                            results.push(Value::Number(Num::from_integer(BigInt::from(n))));
                        }
                    }
                }
                'x' => {
                    // Read hex integer
                    let start = input_idx;
                    while input_idx < remaining_bytes.len() && (remaining_bytes[input_idx] as char).is_ascii_hexdigit() {
                        input_idx += 1;
                    }
                    if input_idx > start {
                        let num_str = String::from_utf8_lossy(&remaining_bytes[start..input_idx]);
                        if let Ok(n) = i64::from_str_radix(&num_str, 16) {
                            results.push(Value::Number(Num::from_integer(BigInt::from(n))));
                        }
                    }
                }
                'o' => {
                    // Read octal integer
                    let start = input_idx;
                    while input_idx < remaining_bytes.len() && (remaining_bytes[input_idx] as char).is_ascii_digit() && remaining_bytes[input_idx] < b'8' {
                        input_idx += 1;
                    }
                    if input_idx > start {
                        let num_str = String::from_utf8_lossy(&remaining_bytes[start..input_idx]);
                        if let Ok(n) = i64::from_str_radix(&num_str, 8) {
                            results.push(Value::Number(Num::from_integer(BigInt::from(n))));
                        }
                    }
                }
                'f' => {
                    // Read float
                    let start = input_idx;
                    if input_idx < remaining_bytes.len() && (remaining_bytes[input_idx] as char) == '-' {
                        input_idx += 1;
                    }
                    while input_idx < remaining_bytes.len() && ((remaining_bytes[input_idx] as char).is_ascii_digit() || (remaining_bytes[input_idx] as char) == '.') {
                        input_idx += 1;
                    }
                    if input_idx > start {
                        let num_str = String::from_utf8_lossy(&remaining_bytes[start..input_idx]);
                        if let Ok(n) = num_str.parse::<f64>() {
                            if let Some(num) = Num::from_float(n) {
                                results.push(Value::Number(num));
                            }
                        }
                    }
                }
                's' => {
                    // Read string (until whitespace)
                    let start = input_idx;
                    while input_idx < remaining_bytes.len() && !(remaining_bytes[input_idx] as char).is_whitespace() {
                        input_idx += 1;
                    }
                    if input_idx > start {
                        let str_val = String::from_utf8_lossy(&remaining_bytes[start..input_idx]).to_string();
                        results.push(Value::Str(str_val));
                    }
                }
                'c'
                    // Read single character
                    if input_idx < remaining_bytes.len() => {
                        let ch = (remaining_bytes[input_idx] as char).to_string();
                        results.push(Value::Str(ch));
                        input_idx += 1;
                    }
                _ => {}
            }
            fmt_idx += 1;
        } else if (fmt_bytes[fmt_idx] as char).is_whitespace() {
            // Skip whitespace in format string
            while input_idx < remaining_bytes.len()
                && (remaining_bytes[input_idx] as char).is_whitespace()
            {
                input_idx += 1;
            }
            while fmt_idx < fmt_bytes.len() && (fmt_bytes[fmt_idx] as char).is_whitespace() {
                fmt_idx += 1;
            }
        } else {
            // Match literal character
            if remaining_bytes[input_idx] as char == fmt_bytes[fmt_idx] as char {
                input_idx += 1;
            }
            fmt_idx += 1;
        }
    }

    // Update file position
    it.open_files[idx].1 = pos + input_idx as u64;

    // Return results as list
    Ok(Value::List(results))
}

// Formatted read with arguments (reads multiple values according to format string)
fn f_fscanf(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.len() < 2 {
        return Err("fscanf: expects at least 2 arguments (fd, format, ...)".to_string());
    }

    // First call fscan to get the parsed values
    let fscan_result = f_fscan(it, &[a[0].clone(), a[1].clone()])?;

    // If there are additional arguments, we can assign the results to them
    // For now, just return the list from fscan
    Ok(fscan_result)
}

// Additional File I/O functions (Phase 6.1 final - file system operations)

// Get file size in bytes
fn f_fsize(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("fsize", a, 1)?;
    let filename = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("fsize: filename must be a string".to_string()),
    };

    match std::fs::metadata(&filename) {
        Ok(metadata) => Ok(Value::Number(Num::from_integer(BigInt::from(
            metadata.len() as i64,
        )))),
        Err(e) => Err(format!("fsize: cannot stat {}: {}", filename, e)),
    }
}

// Check if file exists
fn f_exists(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("exists", a, 1)?;
    let filename = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("exists: filename must be a string".to_string()),
    };

    let exists = std::path::Path::new(&filename).exists();
    Ok(Value::Number(Num::from_integer(BigInt::from(if exists {
        1
    } else {
        0
    }))))
}

// Check if path is a directory
fn f_isdir(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isdir", a, 1)?;
    let path = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("isdir: path must be a string".to_string()),
    };

    match std::fs::metadata(&path) {
        Ok(metadata) => Ok(Value::Number(Num::from_integer(BigInt::from(
            if metadata.is_dir() { 1 } else { 0 },
        )))),
        Err(_) => Ok(Value::Number(Num::zero())), // Path doesn't exist or can't be read
    }
}

// Create directory
fn f_mkdir(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("mkdir", a, 1)?;
    let path = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("mkdir: path must be a string".to_string()),
    };

    match std::fs::create_dir(&path) {
        Ok(_) => Ok(Value::Number(Num::zero())),
        Err(e) => {
            // Return 0 on success, negative on error (for compatibility)
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                // Directory already exists, return success
                Ok(Value::Number(Num::zero()))
            } else {
                Err(format!("mkdir: cannot create directory {}: {}", path, e))
            }
        }
    }
}

// Phase 6.2: Memory & Stack Management

// Allocate memory block
fn f_blk(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("blk", a, 1)?;
    let size = int(a, 0)?.to_usize().ok_or("blk: size out of range")?;

    let block_id = it.next_block_id;
    it.memory_blocks.insert(block_id, vec![0; size]);
    it.next_block_id += 1;

    Ok(Value::Number(Num::from_integer(BigInt::from(block_id))))
}

// Copy memory block
fn f_blkcpy(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("blkcpy", a, 3)?;
    let dest_id = int(a, 0)?.to_i64().ok_or("blkcpy: dest out of range")?;
    let src_id = int(a, 1)?.to_i64().ok_or("blkcpy: src out of range")?;
    let size = int(a, 2)?.to_usize().ok_or("blkcpy: size out of range")?;

    let src = it
        .memory_blocks
        .get(&src_id)
        .ok_or("blkcpy: source block not found")?
        .clone();

    if size > src.len() {
        return Err("blkcpy: size exceeds source block".to_string());
    }

    let dest = it
        .memory_blocks
        .get_mut(&dest_id)
        .ok_or("blkcpy: destination block not found")?;

    if size > dest.len() {
        return Err("blkcpy: size exceeds destination block".to_string());
    }

    dest[..size].copy_from_slice(&src[..size]);

    Ok(Value::Number(Num::from_integer(BigInt::from(size as i64))))
}

// Free memory block
fn f_blkfree(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("blkfree", a, 1)?;
    let block_id = int(a, 0)?
        .to_i64()
        .ok_or("blkfree: block_id out of range")?;

    match it.memory_blocks.remove(&block_id) {
        Some(_) => Ok(Value::Number(Num::zero())),
        None => Err("blkfree: block not found".to_string()),
    }
}

// Get number of allocated blocks
fn f_blocks(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("blocks", a, 0)?;
    Ok(Value::Number(Num::from_integer(BigInt::from(
        it.memory_blocks.len() as i64,
    ))))
}

// Free all memory (clears all blocks)
fn f_free(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("free", a, 0)?;
    let count = it.memory_blocks.len();
    it.memory_blocks.clear();
    Ok(Value::Number(Num::from_integer(BigInt::from(count as i64))))
}

// Free all global variables
fn f_freeglobals(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("freeglobals", a, 0)?;
    let count = it.global_vars.len();
    it.global_vars.clear();
    Ok(Value::Number(Num::from_integer(BigInt::from(count as i64))))
}

// Push value onto evaluation stack
fn f_push(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("push", a, 1)?;
    it.eval_stack.push(a[0].clone());
    Ok(Value::Number(Num::from_integer(BigInt::from(
        it.eval_stack.len() as i64,
    ))))
}

// Pop value from evaluation stack
fn f_pop(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("pop", a, 0)?;
    match it.eval_stack.pop() {
        Some(val) => Ok(val),
        None => Err("pop: stack is empty".to_string()),
    }
}

// Get evaluation stack depth
fn f_depth(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("depth", a, 0)?;
    Ok(Value::Number(Num::from_integer(BigInt::from(
        it.eval_stack.len() as i64,
    ))))
}

// Additional Memory Management Functions (Phase 6.2 extended - memory address functions)

// Get size of allocated memory block
fn f_blksize(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("blksize", a, 1)?;
    let block_id = int(a, 0)?
        .to_i64()
        .ok_or("blksize: block id out of range")?;

    if let Some(block) = it.memory_blocks.get(&block_id) {
        Ok(Value::Number(Num::from_integer(BigInt::from(
            block.len() as i64
        ))))
    } else {
        Err(format!("blksize: block {} not allocated", block_id))
    }
}

// Read byte from memory block at offset
fn f_peek(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("peek", a, 2)?;
    let block_id = int(a, 0)?.to_i64().ok_or("peek: block id out of range")?;
    let offset = int(a, 1)?.to_usize().ok_or("peek: offset out of range")?;

    if let Some(block) = it.memory_blocks.get(&block_id) {
        if offset < block.len() {
            Ok(Value::Number(Num::from_integer(BigInt::from(
                block[offset] as i64,
            ))))
        } else {
            Err(format!(
                "peek: offset {} out of bounds for block {}",
                offset, block_id
            ))
        }
    } else {
        Err(format!("peek: block {} not allocated", block_id))
    }
}

// Write byte to memory block at offset
fn f_poke(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("poke", a, 3)?;
    let block_id = int(a, 0)?.to_i64().ok_or("poke: block id out of range")?;
    let offset = int(a, 1)?.to_usize().ok_or("poke: offset out of range")?;
    let value = int(a, 2)?.to_u8().ok_or("poke: value must be 0-255")?;

    if let Some(block) = it.memory_blocks.get_mut(&block_id) {
        if offset < block.len() {
            block[offset] = value;
            Ok(Value::Number(Num::zero()))
        } else {
            Err(format!(
                "poke: offset {} out of bounds for block {}",
                offset, block_id
            ))
        }
    } else {
        Err(format!("poke: block {} not allocated", block_id))
    }
}

// Read multiple bytes from memory block (returns as string)
fn f_memread(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("memread", a, 3)?;
    let block_id = int(a, 0)?
        .to_i64()
        .ok_or("memread: block id out of range")?;
    let offset = int(a, 1)?
        .to_usize()
        .ok_or("memread: offset out of range")?;
    let size = int(a, 2)?.to_usize().ok_or("memread: size out of range")?;

    if let Some(block) = it.memory_blocks.get(&block_id) {
        if offset + size <= block.len() {
            let data = &block[offset..offset + size];
            let result = String::from_utf8_lossy(data).to_string();
            Ok(Value::Str(result))
        } else {
            Err(format!(
                "memread: read extends beyond block {} bounds",
                block_id
            ))
        }
    } else {
        Err(format!("memread: block {} not allocated", block_id))
    }
}

// Phase 6.4: Command & Script Functions

// Get command-line argument
fn f_argv(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("argv", a, 1)?;
    let n = int(a, 0)?.to_usize().ok_or("argv: index out of range")?;

    if n < it.argv_vec.len() {
        Ok(Value::Str(it.argv_vec[n].clone()))
    } else {
        Ok(Value::Null)
    }
}

// Get current command buffer
fn f_cmdbuf(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cmdbuf", a, 0)?;
    Ok(Value::Str(it.current_cmd.clone()))
}

// Execute shell command
fn f_command(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("command", a, 1)?;
    let cmd = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("command: argument must be a string".to_string()),
    };

    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .map_err(|e| format!("command: cannot execute: {}", e))?;

    let exit_code = output.status.code().unwrap_or(-1) as i64;
    Ok(Value::Number(Num::from_integer(BigInt::from(exit_code))))
}

// Evaluate string expression
fn f_eval(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("eval", a, 1)?;
    let expr_str = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("eval: argument must be a string".to_string()),
    };

    // Parse the string as an expression
    let exprs = crate::parser::parse(&expr_str).map_err(|e| format!("eval: parse error: {}", e))?;

    // Evaluate all expressions and return the last result
    let mut result = Value::Null;
    for expr in exprs {
        result = it.eval(&expr)?;
    }
    Ok(result)
}

// Phase 6.5: Obscure Trigonometric Variants

// Haversine: (1 - cos(x)) / 2
fn f_haversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("haversin", a, 1)?;
    let x = n(a, 0)?;
    let cos_x = number::cos(x, &it.cfg.epsilon)?;
    let result =
        (&Num::from_integer(BigInt::from(1)) - &cos_x) / &Num::from_integer(BigInt::from(2));
    Ok(Value::Number(result))
}

// Versine: 1 - cos(x)
fn f_versin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("versin", a, 1)?;
    let x = n(a, 0)?;
    let cos_x = number::cos(x, &it.cfg.epsilon)?;
    let result = Num::from_integer(BigInt::from(1)) - &cos_x;
    Ok(Value::Number(result))
}

// Coversine: 1 - sin(x)
fn f_coversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("coversin", a, 1)?;
    let x = n(a, 0)?;
    let sin_x = number::sin(x, &it.cfg.epsilon)?;
    let result = Num::from_integer(BigInt::from(1)) - &sin_x;
    Ok(Value::Number(result))
}

// Exsecant: sec(x) - 1
fn f_exsecant(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("exsecant", a, 1)?;
    let x = n(a, 0)?;
    let cos_x = number::cos(x, &it.cfg.epsilon)?;
    if cos_x.is_zero() {
        return Err("exsecant: division by zero".to_string());
    }
    let sec_x = Num::from_integer(BigInt::from(1)) / &cos_x;
    let result = sec_x - &Num::from_integer(BigInt::from(1));
    Ok(Value::Number(result))
}

// Chord: 2 * sin(x/2)
fn f_chord(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("chord", a, 1)?;
    let x = n(a, 0)?;
    let half_x = x / &Num::from_integer(BigInt::from(2));
    let sin_half = number::sin(&half_x, &it.cfg.epsilon)?;
    let result = &Num::from_integer(BigInt::from(2)) * &sin_half;
    Ok(Value::Number(result))
}

// Semiversine: haversine(x)
fn f_semiversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for haversine
    f_haversin(it, a)
}

// Hacoversine: (1 - sin(x)) / 2 (upstream calc definition; was wrongly (1+cos)/2)
fn f_hacoversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hacoversin", a, 1)?;
    let x = n(a, 0)?;
    let sin_x = number::sin(x, &it.cfg.epsilon)?;
    let result =
        (&Num::from_integer(BigInt::from(1)) - &sin_x) / &Num::from_integer(BigInt::from(2));
    Ok(Value::Number(result))
}

// Versed sine (alternative name)
fn f_vers(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for versin
    f_versin(it, a)
}

// Exsecant alternative spelling
fn f_exsec(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for exsecant
    f_exsecant(it, a)
}

// Vercosine: 1 + cos(x)
fn f_vercosin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("vercosin", a, 1)?;
    let x = n(a, 0)?;
    let cos_x = number::cos(x, &it.cfg.epsilon)?;
    let result = Num::from_integer(BigInt::from(1)) + &cos_x;
    Ok(Value::Number(result))
}

// Vercosine alternative spelling
fn f_vercos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for vercosin
    f_vercosin(it, a)
}

// Covercosine: 1 + sin(x)
fn f_covercosin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("covercosin", a, 1)?;
    let x = n(a, 0)?;
    let sin_x = number::sin(x, &it.cfg.epsilon)?;
    let result = Num::from_integer(BigInt::from(1)) + &sin_x;
    Ok(Value::Number(result))
}

// Covercosine alternative spelling
fn f_covercos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for covercosin
    f_covercosin(it, a)
}

// Cohaversine (half-coversine): (1 - sin(x)) / 2
fn f_cohaversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cohaversin", a, 1)?;
    let x = n(a, 0)?;
    let sin_x = number::sin(x, &it.cfg.epsilon)?;
    let result =
        (&Num::from_integer(BigInt::from(1)) - &sin_x) / &Num::from_integer(BigInt::from(2));
    Ok(Value::Number(result))
}

// Hacovercosine (half-covercosine): (1 + sin(x)) / 2
fn f_hacovercosin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hacovercosin", a, 1)?;
    let x = n(a, 0)?;
    let sin_x = number::sin(x, &it.cfg.epsilon)?;
    let result =
        (&Num::from_integer(BigInt::from(1)) + &sin_x) / &Num::from_integer(BigInt::from(2));
    Ok(Value::Number(result))
}

// ---- Inverse rare-trig variants (upstream-parity batch B3) ----
// Each inverts its forward variant via a closed form over asin/acos/asec/acsc.

// aversin: inverse of versin(x) = 1 - cos(x)  =>  acos(1 - x)
fn f_aversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("aversin", a, 1)?;
    let eps = it.epsilon();
    let y = Num::one() - n(a, 0)?;
    Ok(Value::Number(number::acos(&y, &eps)?))
}

// avercos: inverse of vercos(x) = 1 + cos(x)  =>  acos(x - 1)
fn f_avercos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("avercos", a, 1)?;
    let eps = it.epsilon();
    let y = n(a, 0)? - Num::one();
    Ok(Value::Number(number::acos(&y, &eps)?))
}

// acoversin: inverse of coversin(x) = 1 - sin(x)  =>  asin(1 - x)
fn f_acoversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acoversin", a, 1)?;
    let eps = it.epsilon();
    let y = Num::one() - n(a, 0)?;
    Ok(Value::Number(number::asin(&y, &eps)?))
}

// acovercos: inverse of covercos(x) = 1 + sin(x)  =>  asin(x - 1)
fn f_acovercos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acovercos", a, 1)?;
    let eps = it.epsilon();
    let y = n(a, 0)? - Num::one();
    Ok(Value::Number(number::asin(&y, &eps)?))
}

// ahaversin: inverse of haversin(x) = (1 - cos(x))/2  =>  acos(1 - 2x)
fn f_ahaversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ahaversin", a, 1)?;
    let eps = it.epsilon();
    let two = Num::from_integer(BigInt::from(2));
    let y = Num::one() - &(n(a, 0)? * &two);
    Ok(Value::Number(number::acos(&y, &eps)?))
}

// ahavercos: inverse of havercos(x) = (1 + cos(x))/2  =>  acos(2x - 1)
fn f_ahavercos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ahavercos", a, 1)?;
    let eps = it.epsilon();
    let two = Num::from_integer(BigInt::from(2));
    let y = (n(a, 0)? * &two) - Num::one();
    Ok(Value::Number(number::acos(&y, &eps)?))
}

// ahacoversin: inverse of hacoversin(x) = (1 - sin(x))/2  =>  asin(1 - 2x)
fn f_ahacoversin(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ahacoversin", a, 1)?;
    let eps = it.epsilon();
    let two = Num::from_integer(BigInt::from(2));
    let y = Num::one() - &(n(a, 0)? * &two);
    Ok(Value::Number(number::asin(&y, &eps)?))
}

// ahacovercos: inverse of hacovercos(x) = (1 + sin(x))/2  =>  asin(2x - 1)
fn f_ahacovercos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ahacovercos", a, 1)?;
    let eps = it.epsilon();
    let two = Num::from_integer(BigInt::from(2));
    let y = (n(a, 0)? * &two) - Num::one();
    Ok(Value::Number(number::asin(&y, &eps)?))
}

// aexsec: inverse of exsec(x) = sec(x) - 1  =>  asec(x + 1)
fn f_aexsec(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("aexsec", a, 1)?;
    let eps = it.epsilon();
    let y = n(a, 0)? + Num::one();
    Ok(Value::Number(number::asec(&y, &eps)?))
}

// aexcsc: inverse of excsc(x) = csc(x) - 1  =>  acsc(x + 1)
fn f_aexcsc(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("aexcsc", a, 1)?;
    let eps = it.epsilon();
    let y = n(a, 0)? + Num::one();
    Ok(Value::Number(number::acsc(&y, &eps)?))
}

// acrd: inverse of chord(x) = 2 sin(x/2)  =>  2 asin(x/2)
fn f_acrd(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("acrd", a, 1)?;
    let eps = it.epsilon();
    let two = Num::from_integer(BigInt::from(2));
    let half = n(a, 0)? / &two;
    Ok(Value::Number(number::asin(&half, &eps)? * &two))
}

// Excosecant: csc(x) - 1
fn f_excosec(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("excosec", a, 1)?;
    let x = n(a, 0)?;
    let sin_x = number::sin(x, &it.cfg.epsilon)?;
    if sin_x.is_zero() {
        return Err("excosec: division by zero".to_string());
    }
    let csc_x = Num::from_integer(BigInt::from(1)) / &sin_x;
    let result = csc_x - &Num::from_integer(BigInt::from(1));
    Ok(Value::Number(result))
}

// Excosecant alternative spelling
fn f_excsc(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for excosec
    f_excosec(it, a)
}

// Haversine short name
fn f_hav(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for haversin
    f_haversin(it, a)
}

// Chord short name
fn f_crd(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for chord
    f_chord(it, a)
}

// Coversine short name
fn f_cvs(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    // Alias for coversin
    f_coversin(it, a)
}

// Havercosine proper name: (1 + cos(x)) / 2
// Havercosine: (1 + cos(x)) / 2 (was wrongly aliased to hacoversin)
fn f_havercos(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("havercos", a, 1)?;
    let x = n(a, 0)?;
    let cos_x = number::cos(x, &it.cfg.epsilon)?;
    let result =
        (&Num::from_integer(BigInt::from(1)) + &cos_x) / &Num::from_integer(BigInt::from(2));
    Ok(Value::Number(result))
}

// Phase 6.6: Cryptographic & Hashing

use crc::{Crc, CRC_32_CKSUM};
use md5;
use sha1::{Digest, Sha1};

// SHA-1 hash
fn f_sha1(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sha1", a, 1)?;
    let data = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("sha1: argument must be a string".to_string()),
    };

    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();

    // Convert to hex string
    let hex_string = format!("{:x}", result);
    Ok(Value::Str(hex_string))
}

// MD5 hash
fn f_md5(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("md5", a, 1)?;
    let data = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("md5: argument must be a string".to_string()),
    };

    let digest = md5::compute(data.as_bytes());

    // Convert to hex string
    let hex_string = format!("{:x}", digest);
    Ok(Value::Str(hex_string))
}

// CRC32 checksum
fn f_crc32(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("crc32", a, 1)?;
    let data = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("crc32: argument must be a string".to_string()),
    };

    let crc = Crc::<u32>::new(&CRC_32_CKSUM);
    let mut digest = crc.digest();
    digest.update(data.as_bytes());
    let checksum = digest.finalize();

    Ok(Value::Number(Num::from_integer(BigInt::from(
        checksum as i64,
    ))))
}

// Phase 6.7: Residue Class & Modular Operations

// Helper: normalize modulo result to [0, m)
fn normalize_mod(val: &BigInt, m: &BigInt) -> BigInt {
    let r = val % m;
    if r.is_negative() {
        &r + m
    } else {
        r
    }
}

// Create residue class: reduce n modulo m
fn f_rc(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rc", a, 2)?;
    let n = int(a, 0)?;
    let m = int(a, 1)?;

    if m.is_zero() {
        return Err("rc: modulus cannot be zero".to_string());
    }

    let result = normalize_mod(&n, &m);
    Ok(Value::Number(Num::from_integer(result)))
}

// Add in residue class: (a + b) mod m
fn f_rcadd(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rcadd", a, 3)?;
    let a_val = int(a, 0)?;
    let b_val = int(a, 1)?;
    let m = int(a, 2)?;

    if m.is_zero() {
        return Err("rcadd: modulus cannot be zero".to_string());
    }

    let sum = &a_val + &b_val;
    let result = normalize_mod(&sum, &m);
    Ok(Value::Number(Num::from_integer(result)))
}

// Subtract in residue class: (a - b) mod m
fn f_rcsub(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rcsub", a, 3)?;
    let a_val = int(a, 0)?;
    let b_val = int(a, 1)?;
    let m = int(a, 2)?;

    if m.is_zero() {
        return Err("rcsub: modulus cannot be zero".to_string());
    }

    let diff = &a_val - &b_val;
    let result = normalize_mod(&diff, &m);
    Ok(Value::Number(Num::from_integer(result)))
}

// Multiply in residue class: (a * b) mod m
fn f_rcmul(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rcmul", a, 3)?;
    let a_val = int(a, 0)?;
    let b_val = int(a, 1)?;
    let m = int(a, 2)?;

    if m.is_zero() {
        return Err("rcmul: modulus cannot be zero".to_string());
    }

    let prod = &a_val * &b_val;
    let result = normalize_mod(&prod, &m);
    Ok(Value::Number(Num::from_integer(result)))
}

// Modular inverse using extended Euclidean algorithm
fn f_rcinv(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rcinv", a, 2)?;
    let a_val = int(a, 0)?;
    let m = int(a, 1)?;

    if m.is_zero() {
        return Err("rcinv: modulus cannot be zero".to_string());
    }

    // Extended Euclidean algorithm
    let mut old_r = a_val.clone();
    let mut r = m.clone();
    let mut old_s = BigInt::from(1);
    let mut s = BigInt::from(0);

    while !r.is_zero() {
        let quotient = &old_r / &r;
        let temp_r = &old_r - &quotient * &r;
        old_r = r;
        r = temp_r;

        let temp_s = &old_s - &quotient * &s;
        old_s = s;
        s = temp_s;
    }

    if old_r != BigInt::from(1) {
        return Err("rcinv: modular inverse does not exist".to_string());
    }

    let result = normalize_mod(&old_s, &m);
    Ok(Value::Number(Num::from_integer(result)))
}

// Check equality in residue class: a ≡ b (mod m)
fn f_rceq(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rceq", a, 3)?;
    let a_val = int(a, 0)?;
    let b_val = int(a, 1)?;
    let m = int(a, 2)?;

    if m.is_zero() {
        return Err("rceq: modulus cannot be zero".to_string());
    }

    let a_norm = normalize_mod(&a_val, &m);
    let b_norm = normalize_mod(&b_val, &m);
    let result = a_norm == b_norm;
    Ok(Value::Number(Num::from_integer(BigInt::from(if result {
        1
    } else {
        0
    }))))
}

// Negate in residue class: (-a) mod m
fn f_rcneg(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rcneg", a, 2)?;
    let a_val = int(a, 0)?;
    let m = int(a, 1)?;

    if m.is_zero() {
        return Err("rcneg: modulus cannot be zero".to_string());
    }

    let neg = &m - &a_val;
    let result = normalize_mod(&neg, &m);
    Ok(Value::Number(Num::from_integer(result)))
}

// Divide in residue class: (a / b) mod m = a * (b^-1) mod m
fn f_rcdiv(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rcdiv", a, 3)?;
    let a_val = int(a, 0)?;
    let b_val = int(a, 1)?;
    let m = int(a, 2)?;

    if m.is_zero() {
        return Err("rcdiv: modulus cannot be zero".to_string());
    }

    // Find modular inverse of b
    let mut old_r = b_val.clone();
    let mut r = m.clone();
    let mut old_s = BigInt::from(1);
    let mut s = BigInt::from(0);

    while !r.is_zero() {
        let quotient = &old_r / &r;
        let temp_r = &old_r - &quotient * &r;
        old_r = r;
        r = temp_r;

        let temp_s = &old_s - &quotient * &s;
        old_s = s;
        s = temp_s;
    }

    if old_r != BigInt::from(1) {
        return Err("rcdiv: divisor has no modular inverse".to_string());
    }

    let b_inv = normalize_mod(&old_s, &m);
    let prod = &a_val * &b_inv;
    let result = normalize_mod(&prod, &m);
    Ok(Value::Number(Num::from_integer(result)))
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

// ---- String ops (upstream-parity batch B2) ----

fn str_arg<'a>(name: &str, a: &'a [Value], i: usize) -> Result<&'a str, String> {
    match &a[i] {
        Value::Str(s) => Ok(s.as_str()),
        _ => Err(format!("{}: argument {} must be a string", name, i + 1)),
    }
}

// Concatenate all string arguments
fn f_strcat(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("strcat", a, 1, 100)?;
    let mut out = String::new();
    for (i, _) in a.iter().enumerate() {
        out.push_str(str_arg("strcat", a, i)?);
    }
    Ok(Value::Str(out))
}

// String compare: -1 / 0 / 1
fn f_strcmp(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strcmp", a, 2)?;
    let x = str_arg("strcmp", a, 0)?;
    let y = str_arg("strcmp", a, 1)?;
    let ord = x.cmp(y) as i64;
    Ok(Value::Number(Num::from_integer(BigInt::from(ord))))
}

// Case-insensitive compare
fn f_strcasecmp(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strcasecmp", a, 2)?;
    let x = str_arg("strcasecmp", a, 0)?.to_lowercase();
    let y = str_arg("strcasecmp", a, 1)?.to_lowercase();
    let ord = x.cmp(&y) as i64;
    Ok(Value::Number(Num::from_integer(BigInt::from(ord))))
}

// Compare first n characters
fn f_strncmp(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strncmp", a, 3)?;
    let x = str_arg("strncmp", a, 0)?;
    let y = str_arg("strncmp", a, 1)?;
    let n_chars = int(a, 2)?.to_usize().ok_or("strncmp: n out of range")?;
    let xs: String = x.chars().take(n_chars).collect();
    let ys: String = y.chars().take(n_chars).collect();
    let ord = xs.cmp(&ys) as i64;
    Ok(Value::Number(Num::from_integer(BigInt::from(ord))))
}

// Case-insensitive compare of first n characters
fn f_strncasecmp(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strncasecmp", a, 3)?;
    let x = str_arg("strncasecmp", a, 0)?.to_lowercase();
    let y = str_arg("strncasecmp", a, 1)?.to_lowercase();
    let n_chars = int(a, 2)?.to_usize().ok_or("strncasecmp: n out of range")?;
    let xs: String = x.chars().take(n_chars).collect();
    let ys: String = y.chars().take(n_chars).collect();
    let ord = xs.cmp(&ys) as i64;
    Ok(Value::Number(Num::from_integer(BigInt::from(ord))))
}

// Copy of src (calc copies into dst; values are immutable here, so we return the copy)
fn f_strcpy(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strcpy", a, 2)?;
    let _dst = str_arg("strcpy", a, 0)?;
    let src = str_arg("strcpy", a, 1)?;
    Ok(Value::Str(src.to_string()))
}

// Copy of first n characters of src
fn f_strncpy(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strncpy", a, 3)?;
    let _dst = str_arg("strncpy", a, 0)?;
    let src = str_arg("strncpy", a, 1)?;
    let n_chars = int(a, 2)?.to_usize().ok_or("strncpy: n out of range")?;
    Ok(Value::Str(src.chars().take(n_chars).collect()))
}

// Position of needle in haystack, 1-based; 0 if absent (calc semantics)
fn f_strpos(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strpos", a, 2)?;
    let haystack = str_arg("strpos", a, 0)?;
    let needle = str_arg("strpos", a, 1)?;
    let pos = match haystack.find(needle) {
        Some(byte_idx) => haystack[..byte_idx].chars().count() as i64 + 1,
        None => 0,
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(pos))))
}

// Message for an error code (defaults to the last error)
fn f_strerror(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("strerror", a, 0, 1)?;
    let code = if a.is_empty() {
        it.last_errno
    } else {
        int(a, 0)?.to_i64().ok_or("strerror: code out of range")?
    };
    f_errsym(it, &[Value::Number(Num::from_integer(BigInt::from(code)))])
}

// Character for a code, or first character of a string
fn f_char(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("char", a, 1)?;
    match &a[0] {
        Value::Number(x) => {
            if !x.is_integer() {
                return Err("char: non-integer code".to_string());
            }
            let code = x.numer().to_u32().ok_or("char: code out of range")?;
            let ch = char::from_u32(code).ok_or("char: invalid character code")?;
            Ok(Value::Str(ch.to_string()))
        }
        Value::Str(s) => Ok(Value::Str(
            s.chars().next().map(String::from).unwrap_or_default(),
        )),
        _ => Err("char: argument must be a number or string".to_string()),
    }
}

// digit(x, n [, base]): coefficient of base^n in |x| (n may be negative)
fn f_digit(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("digit", a, 2, 3)?;
    let x = n(a, 0)?.abs();
    let pos = int(a, 1)?.to_i64().ok_or("digit: position out of range")?;
    let base = if a.len() == 3 {
        int(a, 2)?
    } else {
        BigInt::from(10)
    };
    if base < BigInt::from(2) {
        return Err("digit: base must be at least 2".to_string());
    }
    let base_r = Num::from_integer(base.clone());
    // shift x so the wanted digit lands in the units place, then floor & mod
    let mut shifted = x;
    if pos >= 0 {
        for _ in 0..pos {
            shifted = &shifted / &base_r;
        }
    } else {
        for _ in 0..(-pos) {
            shifted = &shifted * &base_r;
        }
    }
    let floored = number::floor(&shifted);
    let digit = floored.numer() % &base;
    Ok(Value::Number(Num::from_integer(digit)))
}

// strscan(s, fmt): scan values out of a string per a simplified scanf format
// (%d %i %f %s %c %x %o), returning a list of the scanned values.
fn f_strscan(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("strscan", a, 2)?;
    let input = str_arg("strscan", a, 0)?;
    let fmt = str_arg("strscan", a, 1)?;
    Ok(Value::List(scan_str(input, fmt)?))
}

fn scan_str(input: &str, fmt: &str) -> Result<Vec<Value>, String> {
    let mut results = Vec::new();
    let bytes = input.as_bytes();
    let fmt_bytes = fmt.as_bytes();
    let mut i = 0usize; // input index
    let mut f = 0usize; // format index
    while f < fmt_bytes.len() && i <= bytes.len() {
        if fmt_bytes[f] == b'%' && f + 1 < fmt_bytes.len() {
            f += 1;
            let spec = fmt_bytes[f] as char;
            // skip leading whitespace in input for all specs except %c
            if spec != 'c' {
                while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                    i += 1;
                }
            }
            match spec {
                'd' | 'i' | 'f' => {
                    let start = i;
                    if i < bytes.len() && (bytes[i] == b'-' || bytes[i] == b'+') {
                        i += 1;
                    }
                    while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                        i += 1;
                    }
                    if spec == 'f' && i < bytes.len() && bytes[i] == b'.' {
                        i += 1;
                        while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
                            i += 1;
                        }
                    }
                    if i > start {
                        let tok = &input[start..i];
                        match number::parse_number(tok) {
                            Some(v) => results.push(Value::Number(v)),
                            None => return Err(format!("strscan: bad number {}", tok)),
                        }
                    }
                }
                'x' | 'o' => {
                    let radix = if spec == 'x' { 16 } else { 8 };
                    let start = i;
                    while i < bytes.len() && (bytes[i] as char).is_digit(radix) {
                        i += 1;
                    }
                    if i > start {
                        match BigInt::parse_bytes(&bytes[start..i], radix) {
                            Some(v) => results.push(Value::Number(Num::from_integer(v))),
                            None => return Err("strscan: bad radix literal".to_string()),
                        }
                    }
                }
                's' => {
                    let start = i;
                    while i < bytes.len() && !(bytes[i] as char).is_whitespace() {
                        i += 1;
                    }
                    results.push(Value::Str(input[start..i].to_string()));
                }
                'c' => {
                    if i < bytes.len() {
                        let ch = input[i..].chars().next().unwrap();
                        results.push(Value::Str(ch.to_string()));
                        i += ch.len_utf8();
                    }
                }
                '%' => {
                    if i < bytes.len() && bytes[i] == b'%' {
                        i += 1;
                    }
                }
                _ => return Err(format!("strscan: unsupported format %{}", spec)),
            }
            f += 1;
        } else if (fmt_bytes[f] as char).is_whitespace() {
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            f += 1;
        } else {
            // literal character must match
            if i < bytes.len() && bytes[i] == fmt_bytes[f] {
                i += 1;
            }
            f += 1;
        }
    }
    Ok(results)
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

// ---- Type predicates (upstream-parity batch B1) ----

// Even integer test: 1 if x is an even integer, 0 otherwise (incl. non-integers)
fn f_iseven(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("iseven", a, 1)?;
    Ok(Value::boolean(match &a[0] {
        Value::Number(x) => x.is_integer() && (x.numer() % BigInt::from(2)).is_zero(),
        _ => false,
    }))
}

// Odd integer test
fn f_isodd(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isodd", a, 1)?;
    Ok(Value::boolean(match &a[0] {
        Value::Number(x) => x.is_integer() && !(x.numer() % BigInt::from(2)).is_zero(),
        _ => false,
    }))
}

// Integer test
fn f_isint(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isint", a, 1)?;
    Ok(Value::boolean(
        matches!(&a[0], Value::Number(x) if x.is_integer()),
    ))
}

// Number test (real or complex)
fn f_isnum(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isnum", a, 1)?;
    Ok(Value::boolean(matches!(
        &a[0],
        Value::Number(_) | Value::Complex(_, _)
    )))
}

// Real number test
fn f_isreal(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isreal", a, 1)?;
    Ok(Value::boolean(matches!(&a[0], Value::Number(_))))
}

// String test
fn f_isstr(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isstr", a, 1)?;
    Ok(Value::boolean(matches!(&a[0], Value::Str(_))))
}

// List test
fn f_islist(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("islist", a, 1)?;
    Ok(Value::boolean(matches!(&a[0], Value::List(_))))
}

// Null test
fn f_isnull(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isnull", a, 1)?;
    Ok(Value::boolean(matches!(&a[0], Value::Null)))
}

// Associative array test (our Hash type)
fn f_isassoc(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isassoc", a, 1)?;
    Ok(Value::boolean(matches!(&a[0], Value::Hash(_))))
}

// Hash test (alias semantics of isassoc for our value model)
fn f_ishash(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ishash", a, 1)?;
    Ok(Value::boolean(matches!(&a[0], Value::Hash(_))))
}

// Matrix test: list of equal-length lists
fn f_ismat(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ismat", a, 1)?;
    Ok(Value::boolean(is_matrix(&a[0])))
}

fn is_matrix(v: &Value) -> bool {
    match v {
        Value::List(rows) if !rows.is_empty() => {
            let width = match &rows[0] {
                Value::List(r) => r.len(),
                _ => return false,
            };
            rows.iter()
                .all(|r| matches!(r, Value::List(x) if x.len() == width))
        }
        _ => false,
    }
}

// Identity matrix test
fn f_isident(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isident", a, 1)?;
    let rows = match &a[0] {
        Value::List(rows) if is_matrix(&a[0]) => rows,
        _ => return Ok(Value::boolean(false)),
    };
    let n_rows = rows.len();
    for (i, row) in rows.iter().enumerate() {
        let cells = match row {
            Value::List(c) => c,
            _ => return Ok(Value::boolean(false)),
        };
        if cells.len() != n_rows {
            return Ok(Value::boolean(false));
        }
        for (j, cell) in cells.iter().enumerate() {
            let want = if i == j { Num::one() } else { Num::zero() };
            match cell {
                Value::Number(x) if *x == want => {}
                _ => return Ok(Value::boolean(false)),
            }
        }
    }
    Ok(Value::boolean(true))
}

// Error value test: we have no error values, so always 0
fn f_iserror(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("iserror", a, 1)?;
    Ok(Value::boolean(false))
}

// Multiple test: 1 if x is an integer multiple of y
fn f_ismult(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ismult", a, 2)?;
    let x = n(a, 0)?;
    let y = n(a, 1)?;
    if y.is_zero() {
        return Err("ismult: division by zero".to_string());
    }
    Ok(Value::boolean((x / y).is_integer()))
}

// Relatively-prime test: gcd(x, y) == 1 for integers
fn f_isrel(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isrel", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;
    Ok(Value::boolean(number::gcd_int(&x, &y) == BigInt::from(1)))
}

// Perfect-square test for rationals: numerator and denominator both squares
fn f_issq(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("issq", a, 1)?;
    let x = n(a, 0)?;
    if x.is_negative() {
        return Ok(Value::boolean(false));
    }
    let is_square = |v: &BigInt| -> bool {
        let r = v.sqrt();
        &(&r * &r) == v
    };
    Ok(Value::boolean(is_square(x.numer()) && is_square(x.denom())))
}

// Simple value test: number, string, or null
fn f_issimple(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("issimple", a, 1)?;
    Ok(Value::boolean(matches!(
        &a[0],
        Value::Number(_) | Value::Complex(_, _) | Value::Str(_) | Value::Null
    )))
}

// Same-type test
fn f_istype(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("istype", a, 2)?;
    Ok(Value::boolean(
        std::mem::discriminant(&a[0]) == std::mem::discriminant(&a[1]),
    ))
}

// Open-file-descriptor test
fn f_isfile(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isfile", a, 1)?;
    Ok(Value::boolean(match &a[0] {
        Value::Number(x) if x.is_integer() => match x.numer().to_i64() {
            Some(fd) => fd >= 3 && ((fd - 3) as usize) < it.open_files.len(),
            None => false,
        },
        _ => false,
    }))
}

// Type predicates for types this port does not model as distinct values:
// rand/random state, config, obj, pointer, block, octet. A value can never be
// one of those types here, so these correctly return 0.
fn f_isrand(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isrand", a, 1)?;
    Ok(Value::boolean(false))
}
fn f_israndom(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("israndom", a, 1)?;
    Ok(Value::boolean(false))
}
fn f_isconfig(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isconfig", a, 1)?;
    Ok(Value::boolean(false))
}
fn f_isobj(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isobj", a, 1)?;
    Ok(Value::boolean(false))
}
fn f_isobjtype(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isobjtype", a, 1)?;
    Ok(Value::boolean(false))
}
fn f_isptr(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isptr", a, 1)?;
    Ok(Value::boolean(false))
}
fn f_isblk(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isblk", a, 1)?;
    Ok(Value::boolean(false))
}
fn f_isoctet(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("isoctet", a, 1)?;
    Ok(Value::boolean(false))
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
    if n.is_multiple_of(2) {
        return false;
    }
    for i in (3..=(n as f64).sqrt() as u64 + 1).step_by(2) {
        if n.is_multiple_of(i) {
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
    Ok(Value::Number(Num::from_integer(BigInt::from(if is_set {
        1
    } else {
        0
    }))))
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
    Ok(Value::Number(Num::from_integer(BigInt::from(
        bits as i64 - 1,
    ))))
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
        val >>= 1;
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
        val >>= 1;
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
    if !(2..=36).contains(&base) {
        return Err("base must be between 2 and 36".to_string());
    }
    if x.is_zero() {
        return Ok(Value::Number(Num::from_integer(BigInt::from(1))));
    }
    let mut count = 0u32;
    let mut val = x.abs();
    let base_bi = BigInt::from(base);
    while val.is_positive() {
        val /= &base_bi;
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
        Value::List(items) => Ok(Value::Number(Num::from_integer(BigInt::from(items.len())))),
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
        Value::List(items) => items.first().cloned().ok_or("list is empty".to_string()),
        _ => Err("first() requires a list".to_string()),
    }
}

// Get the last item of a list
fn f_last(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("last", a, 1)?;
    match &a[0] {
        Value::List(items) => items.last().cloned().ok_or("list is empty".to_string()),
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
                (len + start.to_i64().unwrap_or(0)) as usize
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
                    (len + end.to_i64().unwrap_or(0)) as usize
                } else {
                    end.to_usize().unwrap_or(items.len())
                }
            } else {
                items.len()
            };

            let result: Vec<Value> = items
                .iter()
                .skip(start_idx)
                .take(end_idx.saturating_sub(start_idx))
                .cloned()
                .collect();
            Ok(Value::List(result))
        }
        _ => Err("slice() requires a list".to_string()),
    }
}

// Phase 7: String Operations

// Extract substring from string
fn f_substr(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("substr", a, 2, 3)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("substr: first argument must be string".to_string()),
    };
    let start = int(a, 1)?.to_usize().ok_or("substr: start out of range")?;
    let len = if a.len() > 2 {
        int(a, 2)?.to_usize().ok_or("substr: length out of range")?
    } else {
        s.len()
    };

    if start > s.len() {
        Ok(Value::Str(String::new()))
    } else {
        let end = std::cmp::min(start + len, s.len());
        Ok(Value::Str(s[start..end].to_string()))
    }
}

// Convert value to string
fn f_str(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("str", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        Value::Number(n) => number::to_decimal_string(n, 15),
        Value::Complex(r, i) => format!(
            "{}+{}i",
            number::to_decimal_string(r, 15),
            number::to_decimal_string(i, 15)
        ),
        Value::List(_) => format!("{:?}", a[0]),
        Value::Null => "null".to_string(),
        _ => format!("{:?}", a[0]),
    };
    Ok(Value::Str(s))
}

// Replace all occurrences of substring
fn f_replace(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("replace", a, 3)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("replace: first argument must be string".to_string()),
    };
    let old = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("replace: second argument must be string".to_string()),
    };
    let new = match &a[2] {
        Value::Str(s) => s.clone(),
        _ => return Err("replace: third argument must be string".to_string()),
    };

    Ok(Value::Str(s.replace(&old, &new)))
}

// Split string by separator into list
fn f_split(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("split", a, 2)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("split: first argument must be string".to_string()),
    };
    let sep = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("split: separator must be string".to_string()),
    };

    let parts: Vec<Value> = if sep.is_empty() {
        s.chars().map(|c| Value::Str(c.to_string())).collect()
    } else {
        s.split(&sep).map(|p| Value::Str(p.to_string())).collect()
    };
    Ok(Value::List(parts))
}

// Trim whitespace from left
fn f_ltrim(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ltrim", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("ltrim: argument must be string".to_string()),
    };
    Ok(Value::Str(s.trim_start().to_string()))
}

// Trim whitespace from right
fn f_rtrim(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rtrim", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("rtrim: argument must be string".to_string()),
    };
    Ok(Value::Str(s.trim_end().to_string()))
}

// Trim whitespace from both sides
fn f_trim(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("trim", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("trim: argument must be string".to_string()),
    };
    Ok(Value::Str(s.trim().to_string()))
}

// Repeat string n times
fn f_repeat(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("repeat", a, 2)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("repeat: first argument must be string".to_string()),
    };
    let n = int(a, 1)?.to_usize().ok_or("repeat: count out of range")?;

    Ok(Value::Str(s.repeat(n)))
}

// Check if string starts with prefix
fn f_startswith(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("startswith", a, 2)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("startswith: first argument must be string".to_string()),
    };
    let prefix = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("startswith: prefix must be string".to_string()),
    };

    Ok(Value::Number(Num::from_integer(BigInt::from(
        if s.starts_with(&prefix) { 1 } else { 0 },
    ))))
}

// Check if string ends with suffix
fn f_endswith(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("endswith", a, 2)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("endswith: first argument must be string".to_string()),
    };
    let suffix = match &a[1] {
        Value::Str(s) => s.clone(),
        _ => return Err("endswith: suffix must be string".to_string()),
    };

    Ok(Value::Number(Num::from_integer(BigInt::from(
        if s.ends_with(&suffix) { 1 } else { 0 },
    ))))
}

// Left pad string to width
fn f_lpad(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("lpad", a, 2, 3)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("lpad: first argument must be string".to_string()),
    };
    let width = int(a, 1)?.to_usize().ok_or("lpad: width out of range")?;
    let fill = if a.len() > 2 {
        match &a[2] {
            Value::Str(s) => {
                if s.is_empty() {
                    ' '
                } else {
                    s.chars().next().unwrap()
                }
            }
            _ => return Err("lpad: fill must be string".to_string()),
        }
    } else {
        ' '
    };

    if s.len() >= width {
        Ok(Value::Str(s))
    } else {
        let padding = fill.to_string().repeat(width - s.len());
        Ok(Value::Str(format!("{}{}", padding, s)))
    }
}

// Right pad string to width
fn f_rpad(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("rpad", a, 2, 3)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("rpad: first argument must be string".to_string()),
    };
    let width = int(a, 1)?.to_usize().ok_or("rpad: width out of range")?;
    let fill = if a.len() > 2 {
        match &a[2] {
            Value::Str(s) => {
                if s.is_empty() {
                    ' '
                } else {
                    s.chars().next().unwrap()
                }
            }
            _ => return Err("rpad: fill must be string".to_string()),
        }
    } else {
        ' '
    };

    if s.len() >= width {
        Ok(Value::Str(s))
    } else {
        let padding = fill.to_string().repeat(width - s.len());
        Ok(Value::Str(format!("{}{}", s, padding)))
    }
}

// Get ASCII code of character
fn f_ord(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ord", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("ord: argument must be string".to_string()),
    };

    if s.is_empty() {
        Err("ord: string cannot be empty".to_string())
    } else {
        let code = s.chars().next().unwrap() as u32 as i64;
        Ok(Value::Number(Num::from_integer(BigInt::from(code))))
    }
}

// Get character from ASCII code
fn f_chr(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("chr", a, 1)?;
    let code = int(a, 0)?.to_u32().ok_or("chr: code out of range")?;

    match char::from_u32(code) {
        Some(c) => Ok(Value::Str(c.to_string())),
        None => Err(format!("chr: invalid Unicode code point: {}", code)),
    }
}

// Swap case of string
fn f_swapcase(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("swapcase", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("swapcase: argument must be string".to_string()),
    };

    let swapped = s
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect::<String>();
    Ok(Value::Str(swapped))
}

// Title case string
fn f_title(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("title", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("title: argument must be string".to_string()),
    };

    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push_str(&c.to_uppercase().to_string());
            capitalize_next = false;
        } else {
            result.push_str(&c.to_lowercase().to_string());
        }
    }
    Ok(Value::Str(result))
}

// Phase 8: List Operations

// Sort list in ascending order
fn f_sort(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sort", a, 1)?;
    let mut items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("sort: argument must be list".to_string()),
    };

    items.sort_by(|a, b| match (a, b) {
        (Value::Number(n1), Value::Number(n2)) => n1.cmp(n2),
        (Value::Str(s1), Value::Str(s2)) => s1.cmp(s2),
        _ => std::cmp::Ordering::Equal,
    });
    Ok(Value::List(items))
}

// Sort list in descending order
fn f_rsort(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rsort", a, 1)?;
    let mut items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("rsort: argument must be list".to_string()),
    };

    items.sort_by(|a, b| match (a, b) {
        (Value::Number(n1), Value::Number(n2)) => n2.cmp(n1),
        (Value::Str(s1), Value::Str(s2)) => s2.cmp(s1),
        _ => std::cmp::Ordering::Equal,
    });
    Ok(Value::List(items))
}

// Reverse list order
fn f_reverse_list(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("reverse", a, 1)?;
    let mut items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("reverse: argument must be list".to_string()),
    };

    items.reverse();
    Ok(Value::List(items))
}

// Remove duplicates from list
fn f_unique(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("unique", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("unique: argument must be list".to_string()),
    };

    let mut unique_items = Vec::new();
    for item in items {
        if !unique_items.iter().any(|x| x == &item) {
            unique_items.push(item);
        }
    }
    Ok(Value::List(unique_items))
}

// Find minimum value in list
fn f_min_list(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("min", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("min: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut min_val = items[0].clone();
    for item in &items[1..] {
        if let (Value::Number(n1), Value::Number(n2)) = (&min_val, item) {
            if n2 < n1 {
                min_val = item.clone();
            }
        }
    }
    Ok(min_val)
}

// Find maximum value in list
fn f_max_list(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("max", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("max: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut max_val = items[0].clone();
    for item in &items[1..] {
        if let (Value::Number(n1), Value::Number(n2)) = (&max_val, item) {
            if n2 > n1 {
                max_val = item.clone();
            }
        }
    }
    Ok(max_val)
}

// Sum all numeric elements in list
fn f_sum_list(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sum", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("sum: argument must be list".to_string()),
    };

    let mut total = Num::zero();
    for item in items {
        if let Value::Number(n) = item {
            total = &total + &n;
        }
    }
    Ok(Value::Number(total))
}

// Multiply all numeric elements in list
fn f_product(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("product", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("product: argument must be list".to_string()),
    };

    let mut total = Num::from_integer(BigInt::from(1));
    for item in items {
        if let Value::Number(n) = item {
            total = &total * &n;
        }
    }
    Ok(Value::Number(total))
}

// Find index of value in list
fn f_find(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("find", a, 2)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("find: first argument must be list".to_string()),
    };
    let search_val = &a[1];

    for (i, item) in items.iter().enumerate() {
        if item == search_val {
            return Ok(Value::Number(Num::from_integer(BigInt::from(i as i64))));
        }
    }
    Ok(Value::Number(Num::from_integer(BigInt::from(-1))))
}

// Check if list contains value
fn f_contains_list(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("contains", a, 2)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("contains: first argument must be list".to_string()),
    };
    let search_val = &a[1];

    let found = items.iter().any(|item| item == search_val);
    Ok(Value::Number(Num::from_integer(BigInt::from(if found {
        1
    } else {
        0
    }))))
}

// Count occurrences of value in list

// Flatten nested list
fn f_flatten(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("flatten", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("flatten: argument must be list".to_string()),
    };

    let mut result = Vec::new();
    for item in items {
        match item {
            Value::List(inner) => result.extend(inner),
            other => result.push(other),
        }
    }
    Ok(Value::List(result))
}

// Combine two lists (zip)
fn f_zip(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("zip", a, 2)?;
    let list1 = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("zip: first argument must be list".to_string()),
    };
    let list2 = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("zip: second argument must be list".to_string()),
    };

    let mut result = Vec::new();
    let len = std::cmp::min(list1.len(), list2.len());
    for i in 0..len {
        result.push(Value::List(vec![list1[i].clone(), list2[i].clone()]));
    }
    Ok(Value::List(result))
}

// Create range of numbers
fn f_range(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc_range("range", a, 2, 3)?;
    let start = int(a, 0)?.to_i64().ok_or("range: start out of range")?;
    let end = int(a, 1)?.to_i64().ok_or("range: end out of range")?;
    let step = if a.len() > 2 {
        int(a, 2)?.to_i64().ok_or("range: step out of range")?
    } else {
        1
    };

    if step == 0 {
        return Err("range: step cannot be zero".to_string());
    }

    let mut result = Vec::new();
    if step > 0 {
        let mut current = start;
        while current <= end {
            result.push(Value::Number(Num::from_integer(BigInt::from(current))));
            current += step;
        }
    } else {
        let mut current = start;
        while current >= end {
            result.push(Value::Number(Num::from_integer(BigInt::from(current))));
            current += step;
        }
    }
    Ok(Value::List(result))
}

// Phase 9: Variable/Scope Management

// List all global variables
fn f_vars(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("vars", a, 0)?;
    let mut names = Vec::new();
    for key in it.global_vars.keys() {
        names.push(Value::Str(key.clone()));
    }
    names.sort_by(|a, b| match (a, b) {
        (Value::Str(s1), Value::Str(s2)) => s1.cmp(s2),
        _ => std::cmp::Ordering::Equal,
    });
    Ok(Value::List(names))
}

// Check if variable is defined
fn f_defined(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("defined", a, 1)?;
    let name = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("defined: argument must be string".to_string()),
    };

    let exists = it.global_vars.contains_key(&name)
        || it
            .scope_stack
            .iter()
            .rev()
            .any(|scope| scope.contains_key(&name));
    Ok(Value::Number(Num::from_integer(BigInt::from(if exists {
        1
    } else {
        0
    }))))
}

// Delete/undefine a variable
fn f_undefine(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("undefine", a, 1)?;
    let name = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("undefine: argument must be string".to_string()),
    };

    it.global_vars.remove(&name);
    Ok(Value::Number(Num::zero()))
}

// Alias for undefine
fn f_del(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    f_undefine(it, a)
}

// Get type name of value
fn f_type_name(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("type", a, 1)?;
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

// Get size representation of value
fn f_sizeof(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("sizeof", a, 1)?;
    let size = match &a[0] {
        Value::Number(n) => {
            // Rough estimate of bigint size in bytes
            let (numer, denom) = (n.numer(), n.denom());
            let numer_bits = numer.bits();
            let denom_bits = denom.bits();
            ((numer_bits + denom_bits) / 8 + 16) as i64 // Add overhead
        }
        Value::Complex(r, i) => {
            let rb = r.numer().bits() + r.denom().bits();
            let ib = i.numer().bits() + i.denom().bits();
            ((rb + ib) / 8 + 32) as i64
        }
        Value::Str(s) => s.len() as i64,
        Value::List(items) => (items.len() * 8) as i64,
        Value::Hash(map) => (map.len() * 16) as i64,
        Value::Function(_, _) => 32,
        Value::Null => 1,
    };
    Ok(Value::Number(Num::from_integer(BigInt::from(size))))
}

// List environment variables
fn f_env(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("env", a, 0)?;
    let mut env_list = Vec::new();
    for (key, val) in std::env::vars() {
        let entry = Value::List(vec![Value::Str(key), Value::Str(val)]);
        env_list.push(entry);
    }
    Ok(Value::List(env_list))
}

// Dump all state (variables, functions, etc.)
fn f_dump(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("dump", a, 0)?;

    let mut output = String::new();
    output.push_str("=== Global Variables ===\n");
    for (key, val) in &it.global_vars {
        output.push_str(&format!("{} = {}\n", key, val.render(&it.cfg)));
    }

    output.push_str("\n=== Configuration ===\n");
    output.push_str(&format!("epsilon: {}\n", it.cfg.epsilon));
    output.push_str(&format!("display: {}\n", it.cfg.display));
    output.push_str(&format!("ibase: {}\n", it.cfg.ibase));
    output.push_str(&format!("obase: {}\n", it.cfg.obase));

    output.push_str("\n=== Statistics ===\n");
    output.push_str(&format!("Global vars: {}\n", it.global_vars.len()));
    output.push_str(&format!("Open files: {}\n", it.open_files.len()));
    output.push_str(&format!("Memory blocks: {}\n", it.memory_blocks.len()));
    output.push_str(&format!("Stack depth: {}\n", it.eval_stack.len()));

    Ok(Value::Str(output))
}

// Phase 10: I/O & Formatting

// Print with newline
fn f_println(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    let mut output = Vec::new();
    for arg in a {
        match arg {
            Value::Str(s) => output.push(s.clone()),
            Value::Number(n) => output.push(number::to_decimal_string(n, it.cfg.display)),
            Value::Null => {}
            _ => output.push(format!("{:?}", arg)),
        }
    }
    let result = output.join(" ");
    println!("{}", result);
    Ok(Value::Str(result))
}

// Put string with newline
fn f_puts(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("puts", a, 1)?;
    let s = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("puts: argument must be string".to_string()),
    };
    println!("{}", s);
    Ok(Value::Number(Num::from_integer(BigInt::from(
        s.len() as i64
    ))))
}

// Read line from stdin
fn f_getline(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("getline", a, 0)?;
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {
            // Remove trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            Ok(Value::Str(line))
        }
        Err(e) => Err(format!("getline: read error: {}", e)),
    }
}

// Read input with prompt
fn f_input(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("input", a, 1)?;
    let prompt = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("input: prompt must be string".to_string()),
    };

    print!("{}", prompt);
    use std::io::Write;
    std::io::stdout().flush().ok();

    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {
            // Remove trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            Ok(Value::Str(line))
        }
        Err(e) => Err(format!("input: read error: {}", e)),
    }
}

// Formatted print (basic version without format parsing)
fn f_printf(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() {
        return Err("printf: expects at least 1 argument".to_string());
    }

    let format_str = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("printf: format must be string".to_string()),
    };

    // Simple implementation: just concatenate all arguments
    let mut output = format_str.clone();
    for item in a.iter().skip(1) {
        let replacement = match item {
            Value::Str(s) => s.clone(),
            Value::Number(n) => number::to_decimal_string(n, it.cfg.display),
            _ => format!("{:?}", item),
        };
        // Replace first %s or %d with the argument
        if let Some(pos) = output.find("%s").or_else(|| output.find("%d")) {
            output.replace_range(pos..pos + 2, &replacement);
        }
    }

    print!("{}", output);
    use std::io::Write;
    std::io::stdout().flush().ok();
    Ok(Value::Number(Num::from_integer(BigInt::from(
        output.len() as i64
    ))))
}

// Formatted string (basic version)
fn f_sprintf(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() {
        return Err("sprintf: expects at least 1 argument".to_string());
    }

    let format_str = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("sprintf: format must be string".to_string()),
    };

    let mut output = format_str.clone();
    for item in a.iter().skip(1) {
        let replacement = match item {
            Value::Str(s) => s.clone(),
            Value::Number(n) => number::to_decimal_string(n, it.cfg.display),
            _ => format!("{:?}", item),
        };
        // Replace first %s or %d with the argument
        if let Some(pos) = output.find("%s").or_else(|| output.find("%d")) {
            output.replace_range(pos..pos + 2, &replacement);
        }
    }

    Ok(Value::Str(output))
}

// Generic formatting function
fn f_format(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    if a.is_empty() {
        return Err("format: expects at least 1 argument".to_string());
    }

    let format_str = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("format: format must be string".to_string()),
    };

    let mut output = format_str.clone();
    for item in a.iter().skip(1) {
        let replacement = match item {
            Value::Str(s) => s.clone(),
            Value::Number(n) => number::to_decimal_string(n, it.cfg.display),
            _ => format!("{:?}", item),
        };
        // Replace next placeholder with the argument
        if let Some(pos) = output.find("{}") {
            output.replace_range(pos..pos + 2, &replacement);
        }
    }

    Ok(Value::Str(output))
}

// Debug output
fn f_debug(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("debug", a, 1)?;
    let debug_str = format!("[DEBUG] {:?}", a[0]);
    eprintln!("{}", debug_str);
    Ok(Value::Str(debug_str))
}

// Format as hexadecimal
fn f_hex(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hex", a, 1)?;
    let n = int(a, 0)?;
    let hex_str = format!("{:x}", n);
    Ok(Value::Str(hex_str))
}

// Format as octal
fn f_oct(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("oct", a, 1)?;
    let n = int(a, 0)?;
    let oct_str = format!("{:o}", n);
    Ok(Value::Str(oct_str))
}

// Format as binary
fn f_bin(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("bin", a, 1)?;
    let n = int(a, 0)?;
    let bin_str = format!("{:b}", n);
    Ok(Value::Str(bin_str))
}

// Phase 11: Math Extensions

// Calculate mean (average) of list
fn f_mean(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("mean", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("mean: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut sum = Num::zero();
    let mut count = 0;
    for item in items {
        if let Value::Number(n) = item {
            sum = &sum + &n;
            count += 1;
        }
    }

    if count == 0 {
        return Ok(Value::Null);
    }

    Ok(Value::Number(
        &sum / &Num::from_integer(BigInt::from(count)),
    ))
}

// Calculate median of list
fn f_median(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("median", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("median: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    // Filter to only numeric items and sort
    let mut numbers = Vec::new();
    for item in items {
        if let Value::Number(n) = item {
            numbers.push(n);
        }
    }

    if numbers.is_empty() {
        return Ok(Value::Null);
    }

    numbers.sort();
    let len = numbers.len();

    if len % 2 == 1 {
        Ok(Value::Number(numbers[len / 2].clone()))
    } else {
        let mid1 = &numbers[len / 2 - 1];
        let mid2 = &numbers[len / 2];
        Ok(Value::Number(
            (mid1 + mid2) / &Num::from_integer(BigInt::from(2)),
        ))
    }
}

// Calculate variance of list
fn f_variance(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("variance", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("variance: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut sum = Num::zero();
    let mut count = 0;
    let mut numbers = Vec::new();

    for item in items {
        if let Value::Number(n) = item {
            sum = &sum + &n;
            numbers.push(n);
            count += 1;
        }
    }

    if count == 0 {
        return Ok(Value::Null);
    }

    let mean = &sum / &Num::from_integer(BigInt::from(count));
    let mut var_sum = Num::zero();

    for num in numbers {
        let diff = &num - &mean;
        var_sum = &var_sum + &(&diff * &diff);
    }

    Ok(Value::Number(
        &var_sum / &Num::from_integer(BigInt::from(count)),
    ))
}

// Calculate standard deviation of list
fn f_stdev(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("stdev", a, 1)?;
    let variance = f_variance(_it, a)?;

    match variance {
        Value::Number(v) => {
            let result = number::sqrt(&v, &Num::from_float(1e-15).unwrap())?;
            Ok(Value::Number(result))
        }
        _ => Ok(variance),
    }
}

// Count leading zeros in binary representation
fn f_clz(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("clz", a, 1)?;
    let n = int(a, 0)?;

    if n <= BigInt::from(0) {
        return Ok(Value::Number(Num::from_integer(BigInt::from(64))));
    }

    let bits = n.bits();
    let leading_zeros = 64 - bits;

    Ok(Value::Number(Num::from_integer(BigInt::from(
        leading_zeros as i64,
    ))))
}

// Count trailing zeros in binary representation
fn f_ctz(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ctz", a, 1)?;
    let n = int(a, 0)?;

    if n == BigInt::from(0) {
        return Ok(Value::Number(Num::from_integer(BigInt::from(0))));
    }

    let mut count = 0;
    let mut val = n.clone();
    while &val & &BigInt::from(1) == BigInt::from(0) {
        count += 1;
        val = &val >> 1;
    }

    Ok(Value::Number(Num::from_integer(BigInt::from(count))))
}

// Find next power of 2
fn f_nextpow2(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("nextpow2", a, 1)?;
    let n = int(a, 0)?;

    if n <= BigInt::from(1) {
        return Ok(Value::Number(Num::from_integer(BigInt::from(1))));
    }

    let bits = n.bits();
    let next = BigInt::from(1) << bits;

    Ok(Value::Number(Num::from_integer(next)))
}

// Find previous power of 2
fn f_prevpow2(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("prevpow2", a, 1)?;
    let n = int(a, 0)?;

    if n <= BigInt::from(1) {
        return Ok(Value::Number(Num::from_integer(BigInt::from(0))));
    }

    let bits = n.bits();
    let prev = BigInt::from(1) << (bits - 1);
    Ok(Value::Number(Num::from_integer(prev)))
}

// Check if power of 2
fn f_ispow2(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("ispow2", a, 1)?;
    let n = int(a, 0)?;

    if n <= BigInt::from(0) {
        return Ok(Value::Number(Num::from_integer(BigInt::from(0))));
    }

    // A number is a power of 2 if it has exactly one bit set
    // This is true if n & (n-1) == 0
    let n_minus_1 = &n - &BigInt::from(1);
    let is_pow2 = (&n & &n_minus_1) == BigInt::from(0);

    Ok(Value::Number(Num::from_integer(BigInt::from(if is_pow2 {
        1
    } else {
        0
    }))))
}

// Hamming distance between two numbers (count differing bits)
fn f_hammingdist(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hammingdist", a, 2)?;
    let x = int(a, 0)?;
    let y = int(a, 1)?;

    let xor = &x ^ &y;
    let mut count = 0;
    let mut val = xor;

    while val > BigInt::from(0) {
        if &val & &BigInt::from(1) == BigInt::from(1) {
            count += 1;
        }
        val = &val >> 1;
    }

    Ok(Value::Number(Num::from_integer(BigInt::from(count))))
}

// Convert to Gray code
fn f_gray(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("gray", a, 1)?;
    let n = int(a, 0)?;

    let gray = &n ^ &(&n >> 1);
    Ok(Value::Number(Num::from_integer(gray)))
}

// Convert from Gray code
fn f_igray(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("igray", a, 1)?;
    let mut gray = int(a, 0)?;
    let mut result = gray.clone();

    while gray > BigInt::from(0) {
        gray = &gray >> 1;
        result = &result ^ &gray;
    }

    Ok(Value::Number(Num::from_integer(result)))
}

// Population count (number of set bits)
fn f_popcount(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("popcount", a, 1)?;
    let n = int(a, 0)?;

    let mut count = 0;
    let mut val = n;
    while val > BigInt::from(0) {
        if &val & &BigInt::from(1) == BigInt::from(1) {
            count += 1;
        }
        val = &val >> 1;
    }

    Ok(Value::Number(Num::from_integer(BigInt::from(count))))
}

// Root mean square
fn f_rms(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("rms", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("rms: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut sum = Num::zero();
    let mut count = 0;

    for item in items {
        if let Value::Number(n) = item {
            sum = &sum + &(&n * &n);
            count += 1;
        }
    }

    if count == 0 {
        return Ok(Value::Null);
    }

    let mean_sq = &sum / &Num::from_integer(BigInt::from(count));
    let result = number::sqrt(&mean_sq, &Num::from_float(1e-15).unwrap())?;
    Ok(Value::Number(result))
}

// Geometric mean
fn f_gmean(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("gmean", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("gmean: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut product = Num::from_integer(BigInt::from(1));
    let mut count = 0;

    for item in items {
        if let Value::Number(n) = item {
            if n < Num::zero() {
                return Err("gmean: all values must be non-negative".to_string());
            }
            product = &product * &n;
            count += 1;
        }
    }

    if count == 0 {
        return Ok(Value::Null);
    }

    // nth root using root function
    let n_inv = Num::from_integer(BigInt::from(1)) / &Num::from_integer(BigInt::from(count));
    let result = number::pow(&product, &n_inv)?;
    Ok(Value::Number(result))
}

// Harmonic mean
fn f_hmean(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hmean", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("hmean: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut sum_inv = Num::zero();
    let mut count = 0;

    for item in items {
        if let Value::Number(n) = item {
            if n == Num::zero() {
                return Err("hmean: division by zero".to_string());
            }
            sum_inv = &sum_inv + &(Num::from_integer(BigInt::from(1)) / &n);
            count += 1;
        }
    }

    if count == 0 {
        return Ok(Value::Null);
    }

    let n_over_sum = Num::from_integer(BigInt::from(count)) / &sum_inv;
    Ok(Value::Number(n_over_sum))
}

// Phase 12: System & Utility Functions

// Get version string
fn f_version(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("version", a, 0)?;
    Ok(Value::Str("toRustCalcMCP 1.0.0".to_string()))
}

// Get platform/OS name
fn f_platform(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("platform", a, 0)?;
    let os = std::env::consts::OS;
    Ok(Value::Str(os.to_string()))
}

// Get hostname
fn f_hostname(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("hostname", a, 0)?;
    match std::env::var("HOSTNAME") {
        Ok(host) => Ok(Value::Str(host)),
        Err(_) => match std::env::var("COMPUTERNAME") {
            Ok(host) => Ok(Value::Str(host)),
            Err(_) => Ok(Value::Str("unknown".to_string())),
        },
    }
}

// Get process ID
fn f_pid(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("pid", a, 0)?;
    let pid = std::process::id();
    Ok(Value::Number(Num::from_integer(BigInt::from(pid as i64))))
}

// Get username
fn f_username(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("username", a, 0)?;
    match std::env::var("USER") {
        Ok(user) => Ok(Value::Str(user)),
        Err(_) => match std::env::var("USERNAME") {
            Ok(user) => Ok(Value::Str(user)),
            Err(_) => Ok(Value::Str("unknown".to_string())),
        },
    }
}

// Get home directory
fn f_homedir(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("homedir", a, 0)?;
    match std::env::var("HOME") {
        Ok(home) => Ok(Value::Str(home)),
        Err(_) => {
            // Try USERPROFILE on Windows
            match std::env::var("USERPROFILE") {
                Ok(home) => Ok(Value::Str(home)),
                Err(_) => Ok(Value::Str("/root".to_string())),
            }
        }
    }
}

// Get temp directory
fn f_tmpdir(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("tmpdir", a, 0)?;
    let tmp = std::env::temp_dir();
    match tmp.to_str() {
        Some(s) => Ok(Value::Str(s.to_string())),
        None => Ok(Value::Str("/tmp".to_string())),
    }
}

// Get current working directory
fn f_pwd(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("pwd", a, 0)?;
    match std::env::current_dir() {
        Ok(dir) => match dir.to_str() {
            Some(s) => Ok(Value::Str(s.to_string())),
            None => Err("pwd: path contains invalid UTF-8".to_string()),
        },
        Err(e) => Err(format!("pwd: {}", e)),
    }
}

// Change directory
fn f_cd(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cd", a, 1)?;
    let path = match &a[0] {
        Value::Str(s) => s.clone(),
        _ => return Err("cd: argument must be string".to_string()),
    };

    match std::env::set_current_dir(&path) {
        Ok(_) => Ok(Value::Str(path)),
        Err(e) => Err(format!("cd: {}", e)),
    }
}

// Get user ID (returns string representation)
fn f_getuid(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("getuid", a, 0)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        match std::fs::metadata(".") {
            Ok(meta) => {
                let uid = meta.uid();
                Ok(Value::Number(Num::from_integer(BigInt::from(uid as i64))))
            }
            Err(_) => Ok(Value::Number(Num::from_integer(BigInt::from(0)))),
        }
    }
    #[cfg(not(unix))]
    {
        Ok(Value::Number(Num::from_integer(BigInt::from(0))))
    }
}

// Get architecture
fn f_arch(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("arch", a, 0)?;
    let arch = std::env::consts::ARCH;
    Ok(Value::Str(arch.to_string()))
}

// Get machine info (os-arch)
fn f_uname(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("uname", a, 0)?;
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let info = format!("{}-{}", os, arch);
    Ok(Value::Str(info))
}

// Phase 13: Advanced Operations

// Matrix multiplication
fn f_matmul(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("matmul", a, 2)?;
    let m1 = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("matmul: first argument must be matrix (list of lists)".to_string()),
    };
    let m2 = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("matmul: second argument must be matrix (list of lists)".to_string()),
    };

    if m1.is_empty() || m2.is_empty() {
        return Err("matmul: matrices cannot be empty".to_string());
    }

    // Get dimensions
    let rows1 = m1.len();
    let cols1 = match &m1[0] {
        Value::List(row) => row.len(),
        _ => return Err("matmul: first matrix rows must be lists".to_string()),
    };
    let rows2 = m2.len();
    let cols2 = match &m2[0] {
        Value::List(row) => row.len(),
        _ => return Err("matmul: second matrix rows must be lists".to_string()),
    };

    if cols1 != rows2 {
        return Err("matmul: incompatible dimensions for multiplication".to_string());
    }

    let mut result = Vec::new();
    for (i, _) in m1.iter().take(rows1).enumerate() {
        let mut row = Vec::new();
        for j in 0..cols2 {
            let mut sum = Num::zero();
            for k in 0..cols1 {
                let val1 = match &m1[i] {
                    Value::List(r) => match &r[k] {
                        Value::Number(n) => n.clone(),
                        _ => return Err("matmul: non-numeric element".to_string()),
                    },
                    _ => return Err("matmul: row must be list".to_string()),
                };
                let val2 = match &m2[k] {
                    Value::List(r) => match &r[j] {
                        Value::Number(n) => n.clone(),
                        _ => return Err("matmul: non-numeric element".to_string()),
                    },
                    _ => return Err("matmul: row must be list".to_string()),
                };
                sum = &sum + &(&val1 * &val2);
            }
            row.push(Value::Number(sum));
        }
        result.push(Value::List(row));
    }

    Ok(Value::List(result))
}

// Polynomial evaluation (Horner's method)
fn f_polyval(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("polyval", a, 2)?;
    let coeffs = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("polyval: first argument must be coefficient list".to_string()),
    };
    let x = match &a[1] {
        Value::Number(n) => n.clone(),
        _ => return Err("polyval: second argument must be number".to_string()),
    };

    if coeffs.is_empty() {
        return Ok(Value::Number(Num::zero()));
    }

    // Extract numeric coefficients
    let mut nums = Vec::new();
    for c in coeffs {
        match c {
            Value::Number(n) => nums.push(n),
            _ => return Err("polyval: non-numeric coefficient".to_string()),
        }
    }

    // Horner's method: p(x) = a0 + x(a1 + x(a2 + ...))
    let mut result = nums[nums.len() - 1].clone();
    for i in (0..nums.len() - 1).rev() {
        result = &(&result * &x) + &nums[i];
    }

    Ok(Value::Number(result))
}

// Dot product of two vectors
fn f_dot(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("dot", a, 2)?;
    let v1 = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("dot: first argument must be vector (list)".to_string()),
    };
    let v2 = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("dot: second argument must be vector (list)".to_string()),
    };

    if v1.len() != v2.len() {
        return Err("dot: vectors must have same length".to_string());
    }

    let mut sum = Num::zero();
    for i in 0..v1.len() {
        let n1 = match &v1[i] {
            Value::Number(n) => n.clone(),
            _ => return Err("dot: non-numeric element".to_string()),
        };
        let n2 = match &v2[i] {
            Value::Number(n) => n.clone(),
            _ => return Err("dot: non-numeric element".to_string()),
        };
        sum = &sum + &(&n1 * &n2);
    }

    Ok(Value::Number(sum))
}

// Vector norm (magnitude)
fn f_norm(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("norm", a, 1)?;
    let v = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("norm: argument must be vector (list)".to_string()),
    };

    let mut sum = Num::zero();
    for item in v {
        match item {
            Value::Number(n) => {
                sum = &sum + &(&n * &n);
            }
            _ => return Err("norm: non-numeric element".to_string()),
        }
    }

    let result = number::sqrt(&sum, &Num::from_float(1e-15).unwrap())?;
    Ok(Value::Number(result))
}

// Polynomial derivative
fn f_polyderiv(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("polyderiv", a, 1)?;
    let coeffs = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("polyderiv: argument must be coefficient list".to_string()),
    };

    if coeffs.len() <= 1 {
        return Ok(Value::List(vec![Value::Number(Num::zero())]));
    }

    let mut deriv = Vec::new();
    for (i, coeff) in coeffs.iter().enumerate().skip(1) {
        match coeff {
            Value::Number(n) => {
                let multiplier = Num::from_integer(BigInt::from(i as i64));
                deriv.push(Value::Number(n * &multiplier));
            }
            _ => return Err("polyderiv: non-numeric coefficient".to_string()),
        }
    }

    if deriv.is_empty() {
        deriv.push(Value::Number(Num::zero()));
    }

    Ok(Value::List(deriv))
}

// Union of two sets (lists with duplicates removed)
fn f_union(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("union", a, 2)?;
    let set1 = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("union: first argument must be set (list)".to_string()),
    };
    let set2 = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("union: second argument must be set (list)".to_string()),
    };

    let mut result = set1.clone();
    for item in set2 {
        if !result.iter().any(|x| x == &item) {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

// Intersection of two sets
fn f_intersection(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("intersection", a, 2)?;
    let set1 = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("intersection: first argument must be set (list)".to_string()),
    };
    let set2 = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("intersection: second argument must be set (list)".to_string()),
    };

    let mut result = Vec::new();
    for item in set1 {
        if set2.iter().any(|x| x == &item) && !result.iter().any(|x| x == &item) {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

// Difference of two sets (set1 - set2)
fn f_difference(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("difference", a, 2)?;
    let set1 = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("difference: first argument must be set (list)".to_string()),
    };
    let set2 = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("difference: second argument must be set (list)".to_string()),
    };

    let mut result = Vec::new();
    for item in set1 {
        if !set2.iter().any(|x| x == &item) && !result.iter().any(|x| x == &item) {
            result.push(item);
        }
    }

    Ok(Value::List(result))
}

// Check if set1 is subset of set2
fn f_subset(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("subset", a, 2)?;
    let set1 = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("subset: first argument must be set (list)".to_string()),
    };
    let set2 = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("subset: second argument must be set (list)".to_string()),
    };

    let is_subset = set1.iter().all(|item| set2.iter().any(|x| x == item));
    Ok(Value::Number(Num::from_integer(BigInt::from(
        if is_subset { 1 } else { 0 },
    ))))
}

// Linear interpolation
fn f_interp(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("interp", a, 3)?;
    let x_vals = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("interp: x values must be list".to_string()),
    };
    let y_vals = match &a[1] {
        Value::List(items) => items.clone(),
        _ => return Err("interp: y values must be list".to_string()),
    };
    let x = match &a[2] {
        Value::Number(n) => n.clone(),
        _ => return Err("interp: x must be number".to_string()),
    };

    if x_vals.len() != y_vals.len() || x_vals.is_empty() {
        return Err("interp: x and y must have same non-empty length".to_string());
    }

    // Extract numeric values
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (xv, yv) in x_vals.iter().zip(y_vals.iter()) {
        match (xv, yv) {
            (Value::Number(xn), Value::Number(yn)) => {
                xs.push(xn.clone());
                ys.push(yn.clone());
            }
            _ => return Err("interp: non-numeric values".to_string()),
        }
    }

    // Find interval containing x
    for i in 0..xs.len() - 1 {
        if x >= xs[i] && x <= xs[i + 1] {
            let x0 = &xs[i];
            let x1 = &xs[i + 1];
            let y0 = &ys[i];
            let y1 = &ys[i + 1];

            // Linear interpolation: y = y0 + (x - x0) * (y1 - y0) / (x1 - x0)
            let dx = x1 - x0;
            let dy = y1 - y0;
            let t = &(&x - x0) / &dx;
            let result = y0 + &(&t * &dy);
            return Ok(Value::Number(result));
        }
    }

    // Out of range - extrapolate from endpoints
    let x0 = &xs[0];
    let x1 = &xs[xs.len() - 1];
    let y0 = &ys[0];
    let y1 = &ys[ys.len() - 1];

    let dx = x1 - x0;
    let dy = y1 - y0;
    let t = &(&x - x0) / &dx;
    let result = y0 + &(&t * &dy);
    Ok(Value::Number(result))
}

// Cumulative sum
fn f_cumsum(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("cumsum", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("cumsum: argument must be list".to_string()),
    };

    let mut result = Vec::new();
    let mut sum = Num::zero();
    for item in items {
        match item {
            Value::Number(n) => {
                sum = &sum + &n;
                result.push(Value::Number(sum.clone()));
            }
            _ => return Err("cumsum: non-numeric element".to_string()),
        }
    }

    Ok(Value::List(result))
}

// Differences between consecutive elements
fn f_diff(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("diff", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("diff: argument must be list".to_string()),
    };

    if items.len() < 2 {
        return Ok(Value::List(Vec::new()));
    }

    let mut result = Vec::new();
    for i in 1..items.len() {
        match (&items[i - 1], &items[i]) {
            (Value::Number(n1), Value::Number(n2)) => {
                result.push(Value::Number(n2 - n1));
            }
            _ => return Err("diff: non-numeric element".to_string()),
        }
    }

    Ok(Value::List(result))
}

// Statistical mode (most common value)
fn f_mode(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("mode", a, 1)?;
    let items = match &a[0] {
        Value::List(items) => items.clone(),
        _ => return Err("mode: argument must be list".to_string()),
    };

    if items.is_empty() {
        return Ok(Value::Null);
    }

    let mut max_count = 0;
    let mut mode_val = items[0].clone();

    for item in items.iter() {
        let count = items.iter().filter(|x| *x == item).count();
        if count > max_count {
            max_count = count;
            mode_val = item.clone();
        }
    }

    Ok(mode_val)
}

// Final 5 Functions to reach 100% coverage

// Truncate to integer (remove fractional part)
fn f_trunc(_it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("trunc", a, 1)?;
    let n = match &a[0] {
        Value::Number(num) => num.clone(),
        _ => return Err("trunc: argument must be number".to_string()),
    };

    // Truncate: convert to integer and back
    let numer = n.numer().clone();
    let denom = n.denom().clone();
    let int_part = &numer / &denom;
    Ok(Value::Number(Num::from_integer(int_part)))
}

// 2^x (exponential base 2)
fn f_exp2(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("exp2", a, 1)?;
    let x = match &a[0] {
        Value::Number(n) => n.clone(),
        _ => return Err("exp2: argument must be number".to_string()),
    };

    // exp2(x) = 2^x, computed via exp(x * ln(2))
    let ln2 = number::ln(&Num::from_integer(BigInt::from(2)), &it.cfg.epsilon)?;
    let x_ln2 = &x * &ln2;
    let result = number::exp(&x_ln2, &it.cfg.epsilon)?;
    Ok(Value::Number(result))
}

// 10^x (exponential base 10)
fn f_exp10(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("exp10", a, 1)?;
    let x = match &a[0] {
        Value::Number(n) => n.clone(),
        _ => return Err("exp10: argument must be number".to_string()),
    };

    // exp10(x) = 10^x, computed via exp(x * ln(10))
    let ln10 = number::ln(&Num::from_integer(BigInt::from(10)), &it.cfg.epsilon)?;
    let x_ln10 = &x * &ln10;
    let result = number::exp(&x_ln10, &it.cfg.epsilon)?;
    Ok(Value::Number(result))
}

// pow10 is alias for exp10
fn f_pow10(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    f_exp10(it, a)
}

// exp(x) - 1, accurate for small x
fn f_expm1(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("expm1", a, 1)?;
    let x = match &a[0] {
        Value::Number(n) => n.clone(),
        _ => return Err("expm1: argument must be number".to_string()),
    };

    // For small x, use Taylor series: expm1(x) = x + x^2/2 + x^3/6 + ...
    // For larger x, compute exp(x) - 1 directly
    let threshold = Num::from_float(0.1)
        .unwrap_or(Num::from_integer(BigInt::from(1)) / &Num::from_integer(BigInt::from(10)));

    if x.abs() < threshold {
        // Use Taylor series for small x
        let mut result = x.clone();
        let mut term = x.clone();
        let mut n = 1;

        for _ in 0..50 {
            n += 1;
            term = &term * &x;
            term = &term / &Num::from_integer(BigInt::from(n));
            if term.abs() < it.cfg.epsilon {
                break;
            }
            result = &result + &term;
        }
        Ok(Value::Number(result))
    } else {
        // For larger x, use exp(x) - 1
        let exp_x = number::exp(&x, &it.cfg.epsilon)?;
        Ok(Value::Number(&exp_x - &Num::from_integer(BigInt::from(1))))
    }
}

// log(1 + x), accurate for small x
fn f_log1p(it: &mut Interp, a: &[Value]) -> Result<Value, String> {
    argc("log1p", a, 1)?;
    let x = match &a[0] {
        Value::Number(n) => n.clone(),
        _ => return Err("log1p: argument must be number".to_string()),
    };

    // For small x, use Taylor series: log1p(x) = x - x^2/2 + x^3/3 - x^4/4 + ...
    // For larger x, compute log(1 + x) directly
    let threshold = Num::from_float(0.1)
        .unwrap_or(Num::from_integer(BigInt::from(1)) / &Num::from_integer(BigInt::from(10)));

    if x.abs() < threshold {
        // Use Taylor series for small x
        let mut result = x.clone();
        let mut term = x.clone();
        let mut n = 1;

        for _ in 0..50 {
            n += 1;
            term = &term * &(&x * &Num::from_integer(BigInt::from(-1)));
            term = &term / &Num::from_integer(BigInt::from(n));
            if term.abs() < it.cfg.epsilon {
                break;
            }
            result = &result + &term;
        }
        Ok(Value::Number(result))
    } else {
        // For larger x, use log(1 + x)
        let one_plus_x = &Num::from_integer(BigInt::from(1)) + &x;
        let result = number::ln(&one_plus_x, &it.cfg.epsilon)?;
        Ok(Value::Number(result))
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
    builtins.insert("aversin".to_string(), f_aversin as BuiltinFn);
    builtins.insert("avercos".to_string(), f_avercos as BuiltinFn);
    builtins.insert("acoversin".to_string(), f_acoversin as BuiltinFn);
    builtins.insert("acovercos".to_string(), f_acovercos as BuiltinFn);
    builtins.insert("ahaversin".to_string(), f_ahaversin as BuiltinFn);
    builtins.insert("ahavercos".to_string(), f_ahavercos as BuiltinFn);
    builtins.insert("ahacoversin".to_string(), f_ahacoversin as BuiltinFn);
    builtins.insert("ahacovercos".to_string(), f_ahacovercos as BuiltinFn);
    builtins.insert("aexsec".to_string(), f_aexsec as BuiltinFn);
    builtins.insert("aexcsc".to_string(), f_aexcsc as BuiltinFn);
    builtins.insert("acrd".to_string(), f_acrd as BuiltinFn);
    builtins.insert("hacovercos".to_string(), f_hacovercosin as BuiltinFn);
    builtins.insert("strcat".to_string(), f_strcat as BuiltinFn);
    builtins.insert("strcmp".to_string(), f_strcmp as BuiltinFn);
    builtins.insert("strcasecmp".to_string(), f_strcasecmp as BuiltinFn);
    builtins.insert("strncmp".to_string(), f_strncmp as BuiltinFn);
    builtins.insert("strncasecmp".to_string(), f_strncasecmp as BuiltinFn);
    builtins.insert("strcpy".to_string(), f_strcpy as BuiltinFn);
    builtins.insert("strncpy".to_string(), f_strncpy as BuiltinFn);
    builtins.insert("strpos".to_string(), f_strpos as BuiltinFn);
    builtins.insert("strerror".to_string(), f_strerror as BuiltinFn);
    builtins.insert("char".to_string(), f_char as BuiltinFn);
    builtins.insert("digit".to_string(), f_digit as BuiltinFn);
    builtins.insert("strscan".to_string(), f_strscan as BuiltinFn);
    builtins.insert("strscanf".to_string(), f_strscan as BuiltinFn);
    builtins.insert("strtolower".to_string(), f_tolower as BuiltinFn);
    builtins.insert("strtoupper".to_string(), f_toupper as BuiltinFn);
    builtins.insert("strprintf".to_string(), f_sprintf as BuiltinFn);
    builtins.insert("iseven".to_string(), f_iseven as BuiltinFn);
    builtins.insert("isodd".to_string(), f_isodd as BuiltinFn);
    builtins.insert("isint".to_string(), f_isint as BuiltinFn);
    builtins.insert("isnum".to_string(), f_isnum as BuiltinFn);
    builtins.insert("isreal".to_string(), f_isreal as BuiltinFn);
    builtins.insert("isstr".to_string(), f_isstr as BuiltinFn);
    builtins.insert("islist".to_string(), f_islist as BuiltinFn);
    builtins.insert("isnull".to_string(), f_isnull as BuiltinFn);
    builtins.insert("isassoc".to_string(), f_isassoc as BuiltinFn);
    builtins.insert("ishash".to_string(), f_ishash as BuiltinFn);
    builtins.insert("ismat".to_string(), f_ismat as BuiltinFn);
    builtins.insert("isident".to_string(), f_isident as BuiltinFn);
    builtins.insert("iserror".to_string(), f_iserror as BuiltinFn);
    builtins.insert("ismult".to_string(), f_ismult as BuiltinFn);
    builtins.insert("isrel".to_string(), f_isrel as BuiltinFn);
    builtins.insert("issq".to_string(), f_issq as BuiltinFn);
    builtins.insert("issimple".to_string(), f_issimple as BuiltinFn);
    builtins.insert("istype".to_string(), f_istype as BuiltinFn);
    builtins.insert("isfile".to_string(), f_isfile as BuiltinFn);
    builtins.insert("isdefined".to_string(), f_defined as BuiltinFn);
    builtins.insert("isrand".to_string(), f_isrand as BuiltinFn);
    builtins.insert("israndom".to_string(), f_israndom as BuiltinFn);
    builtins.insert("isconfig".to_string(), f_isconfig as BuiltinFn);
    builtins.insert("isobj".to_string(), f_isobj as BuiltinFn);
    builtins.insert("isobjtype".to_string(), f_isobjtype as BuiltinFn);
    builtins.insert("isptr".to_string(), f_isptr as BuiltinFn);
    builtins.insert("isblk".to_string(), f_isblk as BuiltinFn);
    builtins.insert("isoctet".to_string(), f_isoctet as BuiltinFn);
    builtins.insert("nextcand".to_string(), f_nextcand as BuiltinFn);
    builtins.insert("prevcand".to_string(), f_prevcand as BuiltinFn);
    builtins.insert("gcdrem".to_string(), f_gcdrem as BuiltinFn);
    builtins.insert("bround".to_string(), f_bround as BuiltinFn);
    builtins.insert("btrunc".to_string(), f_btrunc as BuiltinFn);
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
    // File I/O (Phase 6.1)
    builtins.insert("fopen".to_string(), f_fopen as BuiltinFn);
    builtins.insert("fclose".to_string(), f_fclose as BuiltinFn);
    builtins.insert("fgets".to_string(), f_fgets as BuiltinFn);
    builtins.insert("fgetc".to_string(), f_fgetc as BuiltinFn);
    builtins.insert("fputs".to_string(), f_fputs as BuiltinFn);
    builtins.insert("fputc".to_string(), f_fputc as BuiltinFn);
    builtins.insert("seek".to_string(), f_seek as BuiltinFn);
    builtins.insert("tell".to_string(), f_tell as BuiltinFn);
    builtins.insert("eof".to_string(), f_eof as BuiltinFn);
    builtins.insert("remove".to_string(), f_remove as BuiltinFn);
    builtins.insert("rename".to_string(), f_rename as BuiltinFn);
    // Additional File I/O functions (Phase 6.1 extended)
    builtins.insert("fflush".to_string(), f_fflush as BuiltinFn);
    builtins.insert("rewind".to_string(), f_rewind as BuiltinFn);
    builtins.insert("fileno".to_string(), f_fileno as BuiltinFn);
    builtins.insert("fread".to_string(), f_fread as BuiltinFn);
    builtins.insert("fwrite".to_string(), f_fwrite as BuiltinFn);
    builtins.insert("fseek".to_string(), f_fseek as BuiltinFn);
    builtins.insert("fprintf".to_string(), f_fprintf as BuiltinFn);
    builtins.insert("fscan".to_string(), f_fscan as BuiltinFn);
    builtins.insert("fscanf".to_string(), f_fscanf as BuiltinFn);
    // File system operations (Phase 6.1 final)
    builtins.insert("fsize".to_string(), f_fsize as BuiltinFn);
    builtins.insert("exists".to_string(), f_exists as BuiltinFn);
    builtins.insert("isdir".to_string(), f_isdir as BuiltinFn);
    builtins.insert("mkdir".to_string(), f_mkdir as BuiltinFn);
    // Memory & stack management (Phase 6.2)
    builtins.insert("blk".to_string(), f_blk as BuiltinFn);
    builtins.insert("blkcpy".to_string(), f_blkcpy as BuiltinFn);
    builtins.insert("blkfree".to_string(), f_blkfree as BuiltinFn);
    builtins.insert("blocks".to_string(), f_blocks as BuiltinFn);
    builtins.insert("free".to_string(), f_free as BuiltinFn);
    builtins.insert("freeglobals".to_string(), f_freeglobals as BuiltinFn);
    builtins.insert("push".to_string(), f_push as BuiltinFn);
    builtins.insert("pop".to_string(), f_pop as BuiltinFn);
    builtins.insert("depth".to_string(), f_depth as BuiltinFn);
    // Memory address functions (Phase 6.2 extended)
    builtins.insert("blksize".to_string(), f_blksize as BuiltinFn);
    builtins.insert("peek".to_string(), f_peek as BuiltinFn);
    builtins.insert("poke".to_string(), f_poke as BuiltinFn);
    builtins.insert("memread".to_string(), f_memread as BuiltinFn);
    // Command & script functions (Phase 6.4)
    builtins.insert("argv".to_string(), f_argv as BuiltinFn);
    builtins.insert("cmdbuf".to_string(), f_cmdbuf as BuiltinFn);
    builtins.insert("command".to_string(), f_command as BuiltinFn);
    builtins.insert("eval".to_string(), f_eval as BuiltinFn);
    // Obscure trigonometric variants (Phase 6.5)
    builtins.insert("haversin".to_string(), f_haversin as BuiltinFn);
    builtins.insert("versin".to_string(), f_versin as BuiltinFn);
    builtins.insert("coversin".to_string(), f_coversin as BuiltinFn);
    builtins.insert("exsecant".to_string(), f_exsecant as BuiltinFn);
    builtins.insert("chord".to_string(), f_chord as BuiltinFn);
    builtins.insert("semiversin".to_string(), f_semiversin as BuiltinFn);
    builtins.insert("hacoversin".to_string(), f_hacoversin as BuiltinFn);
    builtins.insert("vers".to_string(), f_vers as BuiltinFn);
    builtins.insert("exsec".to_string(), f_exsec as BuiltinFn);
    builtins.insert("vercosin".to_string(), f_vercosin as BuiltinFn);
    builtins.insert("vercos".to_string(), f_vercos as BuiltinFn);
    builtins.insert("covercosin".to_string(), f_covercosin as BuiltinFn);
    builtins.insert("covercos".to_string(), f_covercos as BuiltinFn);
    builtins.insert("cohaversin".to_string(), f_cohaversin as BuiltinFn);
    builtins.insert("hacovercosin".to_string(), f_hacovercosin as BuiltinFn);
    builtins.insert("excosec".to_string(), f_excosec as BuiltinFn);
    builtins.insert("excsc".to_string(), f_excsc as BuiltinFn);
    builtins.insert("hav".to_string(), f_hav as BuiltinFn);
    builtins.insert("crd".to_string(), f_crd as BuiltinFn);
    builtins.insert("cvs".to_string(), f_cvs as BuiltinFn);
    builtins.insert("havercos".to_string(), f_havercos as BuiltinFn);
    // Cryptographic & hashing (Phase 6.6)
    builtins.insert("sha1".to_string(), f_sha1 as BuiltinFn);
    builtins.insert("md5".to_string(), f_md5 as BuiltinFn);
    builtins.insert("crc32".to_string(), f_crc32 as BuiltinFn);

    // String operations (Phase 7)
    builtins.insert("substr".to_string(), f_substr as BuiltinFn);
    builtins.insert("str".to_string(), f_str as BuiltinFn);
    builtins.insert("replace".to_string(), f_replace as BuiltinFn);
    builtins.insert("split".to_string(), f_split as BuiltinFn);
    builtins.insert("ltrim".to_string(), f_ltrim as BuiltinFn);
    builtins.insert("rtrim".to_string(), f_rtrim as BuiltinFn);
    builtins.insert("trim".to_string(), f_trim as BuiltinFn);
    builtins.insert("repeat".to_string(), f_repeat as BuiltinFn);
    builtins.insert("startswith".to_string(), f_startswith as BuiltinFn);
    builtins.insert("endswith".to_string(), f_endswith as BuiltinFn);
    builtins.insert("lpad".to_string(), f_lpad as BuiltinFn);
    builtins.insert("rpad".to_string(), f_rpad as BuiltinFn);
    builtins.insert("ord".to_string(), f_ord as BuiltinFn);
    builtins.insert("chr".to_string(), f_chr as BuiltinFn);
    builtins.insert("swapcase".to_string(), f_swapcase as BuiltinFn);
    builtins.insert("title".to_string(), f_title as BuiltinFn);

    // List operations (Phase 8)
    builtins.insert("sort".to_string(), f_sort as BuiltinFn);
    builtins.insert("rsort".to_string(), f_rsort as BuiltinFn);
    builtins.insert("reverse".to_string(), f_reverse_list as BuiltinFn);
    builtins.insert("unique".to_string(), f_unique as BuiltinFn);
    builtins.insert("min".to_string(), f_min_list as BuiltinFn);
    builtins.insert("max".to_string(), f_max_list as BuiltinFn);
    builtins.insert("sum".to_string(), f_sum_list as BuiltinFn);
    builtins.insert("product".to_string(), f_product as BuiltinFn);
    builtins.insert("find".to_string(), f_find as BuiltinFn);
    builtins.insert("contains".to_string(), f_contains_list as BuiltinFn);
    builtins.insert("flatten".to_string(), f_flatten as BuiltinFn);
    builtins.insert("zip".to_string(), f_zip as BuiltinFn);
    builtins.insert("range".to_string(), f_range as BuiltinFn);

    // Variable/scope management (Phase 9)
    builtins.insert("vars".to_string(), f_vars as BuiltinFn);
    builtins.insert("defined".to_string(), f_defined as BuiltinFn);
    builtins.insert("undefine".to_string(), f_undefine as BuiltinFn);
    builtins.insert("del".to_string(), f_del as BuiltinFn);
    builtins.insert("type".to_string(), f_type_name as BuiltinFn);
    builtins.insert("sizeof".to_string(), f_sizeof as BuiltinFn);
    builtins.insert("env".to_string(), f_env as BuiltinFn);
    builtins.insert("dump".to_string(), f_dump as BuiltinFn);

    // Residue class & modular operations (Phase 6.7)
    builtins.insert("rc".to_string(), f_rc as BuiltinFn);
    builtins.insert("rcadd".to_string(), f_rcadd as BuiltinFn);
    builtins.insert("rcsub".to_string(), f_rcsub as BuiltinFn);
    builtins.insert("rcmul".to_string(), f_rcmul as BuiltinFn);
    builtins.insert("rcinv".to_string(), f_rcinv as BuiltinFn);
    builtins.insert("rceq".to_string(), f_rceq as BuiltinFn);
    builtins.insert("rcneg".to_string(), f_rcneg as BuiltinFn);
    builtins.insert("rcdiv".to_string(), f_rcdiv as BuiltinFn);
    // I/O & Formatting functions (Phase 10)
    builtins.insert("println".to_string(), f_println as BuiltinFn);
    builtins.insert("puts".to_string(), f_puts as BuiltinFn);
    builtins.insert("getline".to_string(), f_getline as BuiltinFn);
    builtins.insert("input".to_string(), f_input as BuiltinFn);
    builtins.insert("printf".to_string(), f_printf as BuiltinFn);
    builtins.insert("sprintf".to_string(), f_sprintf as BuiltinFn);
    builtins.insert("format".to_string(), f_format as BuiltinFn);
    builtins.insert("debug".to_string(), f_debug as BuiltinFn);
    builtins.insert("hex".to_string(), f_hex as BuiltinFn);
    builtins.insert("oct".to_string(), f_oct as BuiltinFn);
    builtins.insert("bin".to_string(), f_bin as BuiltinFn);
    // Math Extensions (Phase 11)
    builtins.insert("mean".to_string(), f_mean as BuiltinFn);
    builtins.insert("median".to_string(), f_median as BuiltinFn);
    builtins.insert("variance".to_string(), f_variance as BuiltinFn);
    builtins.insert("stdev".to_string(), f_stdev as BuiltinFn);
    builtins.insert("clz".to_string(), f_clz as BuiltinFn);
    builtins.insert("ctz".to_string(), f_ctz as BuiltinFn);
    builtins.insert("nextpow2".to_string(), f_nextpow2 as BuiltinFn);
    builtins.insert("prevpow2".to_string(), f_prevpow2 as BuiltinFn);
    builtins.insert("ispow2".to_string(), f_ispow2 as BuiltinFn);
    builtins.insert("hammingdist".to_string(), f_hammingdist as BuiltinFn);
    builtins.insert("gray".to_string(), f_gray as BuiltinFn);
    builtins.insert("igray".to_string(), f_igray as BuiltinFn);
    builtins.insert("popcount".to_string(), f_popcount as BuiltinFn);
    builtins.insert("rms".to_string(), f_rms as BuiltinFn);
    builtins.insert("gmean".to_string(), f_gmean as BuiltinFn);
    builtins.insert("hmean".to_string(), f_hmean as BuiltinFn);
    // System & Utility functions (Phase 12)
    builtins.insert("version".to_string(), f_version as BuiltinFn);
    builtins.insert("platform".to_string(), f_platform as BuiltinFn);
    builtins.insert("hostname".to_string(), f_hostname as BuiltinFn);
    builtins.insert("pid".to_string(), f_pid as BuiltinFn);
    builtins.insert("username".to_string(), f_username as BuiltinFn);
    builtins.insert("homedir".to_string(), f_homedir as BuiltinFn);
    builtins.insert("tmpdir".to_string(), f_tmpdir as BuiltinFn);
    builtins.insert("pwd".to_string(), f_pwd as BuiltinFn);
    builtins.insert("cd".to_string(), f_cd as BuiltinFn);
    builtins.insert("getuid".to_string(), f_getuid as BuiltinFn);
    builtins.insert("arch".to_string(), f_arch as BuiltinFn);
    builtins.insert("uname".to_string(), f_uname as BuiltinFn);
    // Advanced Operations (Phase 13)
    builtins.insert("matmul".to_string(), f_matmul as BuiltinFn);
    builtins.insert("polyval".to_string(), f_polyval as BuiltinFn);
    builtins.insert("dot".to_string(), f_dot as BuiltinFn);
    builtins.insert("norm".to_string(), f_norm as BuiltinFn);
    builtins.insert("polyderiv".to_string(), f_polyderiv as BuiltinFn);
    builtins.insert("union".to_string(), f_union as BuiltinFn);
    builtins.insert("intersection".to_string(), f_intersection as BuiltinFn);
    builtins.insert("difference".to_string(), f_difference as BuiltinFn);
    builtins.insert("subset".to_string(), f_subset as BuiltinFn);
    builtins.insert("interp".to_string(), f_interp as BuiltinFn);
    builtins.insert("cumsum".to_string(), f_cumsum as BuiltinFn);
    builtins.insert("diff".to_string(), f_diff as BuiltinFn);
    builtins.insert("mode".to_string(), f_mode as BuiltinFn);
    // Final functions to reach 100% coverage
    builtins.insert("trunc".to_string(), f_trunc as BuiltinFn);
    builtins.insert("exp2".to_string(), f_exp2 as BuiltinFn);
    builtins.insert("exp10".to_string(), f_exp10 as BuiltinFn);
    builtins.insert("pow10".to_string(), f_pow10 as BuiltinFn);
    builtins.insert("expm1".to_string(), f_expm1 as BuiltinFn);
    builtins.insert("log1p".to_string(), f_log1p as BuiltinFn);
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
        (
            "sqrt",
            "sqrt(x)",
            "square root (returns complex for negative x)",
        ),
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
        ("aversin", "aversin(x)", "inverse versine: acos(1-x)"),
        ("avercos", "avercos(x)", "inverse vercosine: acos(x-1)"),
        ("acoversin", "acoversin(x)", "inverse coversine: asin(1-x)"),
        (
            "acovercos",
            "acovercos(x)",
            "inverse covercosine: asin(x-1)",
        ),
        ("ahaversin", "ahaversin(x)", "inverse haversine: acos(1-2x)"),
        (
            "ahavercos",
            "ahavercos(x)",
            "inverse havercosine: acos(2x-1)",
        ),
        (
            "ahacoversin",
            "ahacoversin(x)",
            "inverse hacoversine: asin(1-2x)",
        ),
        (
            "ahacovercos",
            "ahacovercos(x)",
            "inverse hacovercosine: asin(2x-1)",
        ),
        ("aexsec", "aexsec(x)", "inverse exsecant: asec(x+1)"),
        ("aexcsc", "aexcsc(x)", "inverse excosecant: acsc(x+1)"),
        ("acrd", "acrd(x)", "inverse chord: 2*asin(x/2)"),
        (
            "hacovercos",
            "hacovercos(x)",
            "hacovercosine: (1 + sin(x)) / 2",
        ),
        ("strcat", "strcat(s1,s2,...)", "concatenate strings"),
        ("strcmp", "strcmp(s1,s2)", "compare strings (-1/0/1)"),
        (
            "strcasecmp",
            "strcasecmp(s1,s2)",
            "case-insensitive compare",
        ),
        ("strncmp", "strncmp(s1,s2,n)", "compare first n characters"),
        (
            "strncasecmp",
            "strncasecmp(s1,s2,n)",
            "case-insensitive compare of first n chars",
        ),
        ("strcpy", "strcpy(dst,src)", "copy of src"),
        (
            "strncpy",
            "strncpy(dst,src,n)",
            "copy of first n chars of src",
        ),
        (
            "strpos",
            "strpos(haystack,needle)",
            "1-based position of needle, 0 if absent",
        ),
        ("strerror", "strerror([code])", "message for an error code"),
        (
            "char",
            "char(x)",
            "character for a code, or first char of string",
        ),
        ("digit", "digit(x,n[,base])", "digit of x at base^n place"),
        (
            "strscan",
            "strscan(s,fmt)",
            "scan values from string per scanf format (returns list)",
        ),
        ("strscanf", "strscanf(s,fmt)", "alias for strscan"),
        (
            "strtolower",
            "strtolower(s)",
            "lowercase (alias for tolower)",
        ),
        (
            "strtoupper",
            "strtoupper(s)",
            "uppercase (alias for toupper)",
        ),
        (
            "strprintf",
            "strprintf(fmt,...)",
            "formatted string (alias for sprintf)",
        ),
        ("iseven", "iseven(x)", "1 if x is an even integer"),
        ("isodd", "isodd(x)", "1 if x is an odd integer"),
        ("isint", "isint(x)", "1 if x is an integer"),
        ("isnum", "isnum(x)", "1 if x is a number (real or complex)"),
        ("isreal", "isreal(x)", "1 if x is a real number"),
        ("isstr", "isstr(x)", "1 if x is a string"),
        ("islist", "islist(x)", "1 if x is a list"),
        ("isnull", "isnull(x)", "1 if x is null"),
        ("isassoc", "isassoc(x)", "1 if x is an associative array"),
        ("ishash", "ishash(x)", "1 if x is a hash/associative array"),
        (
            "ismat",
            "ismat(x)",
            "1 if x is a matrix (list of equal-length lists)",
        ),
        ("isident", "isident(m)", "1 if m is an identity matrix"),
        (
            "iserror",
            "iserror(x)",
            "1 if x is an error value (always 0 here)",
        ),
        (
            "ismult",
            "ismult(x,y)",
            "1 if x is an integer multiple of y",
        ),
        ("isrel", "isrel(x,y)", "1 if x and y are relatively prime"),
        ("issq", "issq(x)", "1 if x is a perfect square (rational)"),
        (
            "issimple",
            "issimple(x)",
            "1 if x is a simple value (number/string/null)",
        ),
        ("istype", "istype(x,y)", "1 if x and y have the same type"),
        ("isfile", "isfile(x)", "1 if x is an open file descriptor"),
        (
            "isdefined",
            "isdefined(name)",
            "1 if name is a defined variable",
        ),
        (
            "isrand",
            "isrand(x)",
            "1 if x is a rand state (always 0 here)",
        ),
        (
            "israndom",
            "israndom(x)",
            "1 if x is a random state (always 0 here)",
        ),
        (
            "isconfig",
            "isconfig(x)",
            "1 if x is a config value (always 0 here)",
        ),
        ("isobj", "isobj(x)", "1 if x is an object (always 0 here)"),
        (
            "isobjtype",
            "isobjtype(x)",
            "1 if x is an object type (always 0 here)",
        ),
        ("isptr", "isptr(x)", "1 if x is a pointer (always 0 here)"),
        (
            "isblk",
            "isblk(x)",
            "1 if x is a block value (always 0 here)",
        ),
        (
            "isoctet",
            "isoctet(x)",
            "1 if x is an octet (always 0 here)",
        ),
        (
            "nextcand",
            "nextcand(n[,count[,skip[,residue[,modulus]]]])",
            "next probable prime after n (optional residue mod modulus)",
        ),
        (
            "prevcand",
            "prevcand(n[,count[,skip[,residue[,modulus]]]])",
            "previous probable prime before n (optional residue mod modulus)",
        ),
        (
            "gcdrem",
            "gcdrem(x,y)",
            "remove from x all prime factors shared with y",
        ),
        (
            "bround",
            "bround(x[,places])",
            "round x to given number of binary places",
        ),
        (
            "btrunc",
            "btrunc(x[,places])",
            "truncate x to given number of binary places",
        ),
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
        (
            "base",
            "base([ibase[,obase]])",
            "get/set input and output base (2-36)",
        ),
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
        (
            "gamma",
            "gamma(x)",
            "gamma function (generalized factorial)",
        ),
        ("lgamma", "lgamma(x)", "log-gamma function"),
        (
            "polygamma",
            "polygamma(n,x)",
            "polygamma function (nth derivative of log-gamma)",
        ),
        ("zeta", "zeta(s)", "Riemann zeta function"),
        ("rand", "rand()", "random 32-bit integer"),
        ("random", "random()", "random float [0,1)"),
        ("randbit", "randbit()", "random bit (0 or 1)"),
        ("seed", "seed(s)", "set random seed"),
        ("srand", "srand(s)", "set random seed (alias)"),
        ("srandom", "srandom(s)", "set random seed (alias)"),
        ("randint", "randint(a,b)", "random integer in [a,b]"),
        (
            "randperm",
            "randperm(n)",
            "random permutation of 0..n-1 (returns list)",
        ),
        (
            "time",
            "time()",
            "current Unix timestamp (seconds since epoch)",
        ),
        ("systime", "systime()", "system time (alias for time)"),
        ("ctime", "ctime(t)", "convert Unix timestamp to string"),
        ("sleep", "sleep(s)", "sleep for s seconds"),
        ("getenv", "getenv(name)", "get environment variable"),
        ("putenv", "putenv(name,value)", "set environment variable"),
        (
            "system",
            "system(cmd)",
            "execute shell command (returns exit code)",
        ),
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
        (
            "quomod",
            "quomod(x,y)",
            "quotient and modulus (returns [q,r])",
        ),
        ("quo", "quo(x,y)", "quotient (floor(x/y))"),
        ("rem", "rem(x,y)", "remainder (x - y*floor(x/y))"),
        ("hnrmod", "hnrmod(x,y)", "Hensel modular"),
        (
            "appr",
            "appr(x[,eps])",
            "rational approximation within epsilon",
        ),
        (
            "cfappr",
            "cfappr(x[,maxd])",
            "continued fraction approximation",
        ),
        (
            "cfsim",
            "cfsim(x[,maxd])",
            "continued fraction simplification",
        ),
        ("scale", "scale(x[,places])", "scale to decimal places"),
        ("matdim", "matdim(m)", "matrix dimensions [rows, cols]"),
        ("mattrans", "mattrans(m)", "matrix transpose"),
        ("mattrace", "mattrace(m)", "matrix trace (sum of diagonal)"),
        ("det", "det(m)", "matrix determinant (2x2, 3x3)"),
        ("inverse", "inverse(m)", "matrix inverse (2x2)"),
        ("matsum", "matsum(m)", "sum of all matrix elements"),
        ("matmin", "matmin(m)", "minimum matrix element"),
        ("matmax", "matmax(m)", "maximum matrix element"),
        (
            "matfill",
            "matfill(r,c,v)",
            "create matrix filled with value",
        ),
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
        (
            "digits",
            "digits(x[,base])",
            "number of digits (base 10 or specified)",
        ),
        ("list", "list(x,...)", "create a list from items"),
        ("size", "size(list)", "number of items in list"),
        ("append", "append(list,x,...)", "append items to list"),
        ("first", "first(list)", "get first item"),
        ("last", "last(list)", "get last item"),
        (
            "slice",
            "slice(list,start[,end])",
            "get sublist from start to end",
        ),
        ("strlen", "strlen(s)", "length of string"),
        (
            "index",
            "index(haystack,needle)",
            "find substring position (-1 if not found)",
        ),
        (
            "isalpha",
            "isalpha(s)",
            "is string all alphabetic? (1 or 0)",
        ),
        ("isdigit", "isdigit(s)", "is string all digits? (1 or 0)"),
        (
            "isspace",
            "isspace(s)",
            "is string all whitespace? (1 or 0)",
        ),
        (
            "typeof",
            "typeof(x)",
            "get type of value (number, complex, string, list, function, null)",
        ),
        ("isnan", "isnan(x)", "is NaN? (always 0 for rationals)"),
        ("isinf", "isinf(x)", "is infinite? (always 0 for rationals)"),
        ("d2r", "d2r(x)", "degrees to radians"),
        ("r2d", "r2d(x)", "radians to degrees"),
        ("d2g", "d2g(x)", "degrees to gradians"),
        ("g2r", "g2r(x)", "gradians to radians"),
        ("g2d", "g2d(x)", "gradians to degrees"),
        // Hash & associative arrays (Phase 5.5)
        (
            "assoc",
            "assoc(k1,v1,...)",
            "create associative array from key-value pairs",
        ),
        ("indices", "indices(h)", "get all keys from hash as list"),
        (
            "insert",
            "insert(h,key,val)",
            "insert/update key-value pair in hash",
        ),
        ("delete", "delete(h,key)", "delete key from hash"),
        ("count", "count(h)", "count key-value pairs in hash"),
        ("join", "join(h,sep)", "join hash values with separator"),
        // Error & exception handling (Phase 6.3)
        ("errcount", "errcount()", "number of errors occurred"),
        (
            "errmax",
            "errmax(n)",
            "set max errors before stopping (0=unlimited)",
        ),
        ("errno", "errno()", "last error code"),
        ("errsym", "errsym(code)", "error message for error code"),
        ("error", "error(msg)", "raise an error with message"),
        (
            "newerror",
            "newerror(code,msg)",
            "register a new error type",
        ),
        (
            "warn",
            "warn(msg)",
            "issue a warning (not counted as error)",
        ),
        // File I/O (Phase 6.1)
        (
            "fopen",
            "fopen(filename,mode)",
            "open file (mode: 'r', 'w', 'a')",
        ),
        ("fclose", "fclose(fd)", "close file"),
        ("fgets", "fgets(fd)", "read line from file"),
        ("fgetc", "fgetc(fd)", "read character from file"),
        ("fputs", "fputs(fd,str)", "write string to file"),
        ("fputc", "fputc(fd,ch)", "write character to file"),
        ("seek", "seek(fd,offset)", "seek to position in file"),
        ("tell", "tell(fd)", "get current position in file"),
        ("eof", "eof(fd)", "check if at end-of-file"),
        ("remove", "remove(filename)", "delete file"),
        ("rename", "rename(old,new)", "rename file"),
        ("fflush", "fflush(fd)", "flush file buffer"),
        ("rewind", "rewind(fd)", "rewind file to beginning"),
        ("fileno", "fileno(fd)", "get file descriptor number"),
        ("fread", "fread(fd,size)", "read bytes from file"),
        ("fwrite", "fwrite(fd,data)", "write bytes to file"),
        (
            "fseek",
            "fseek(fd,offset,whence)",
            "seek with whence (0=SET, 1=CUR, 2=END)",
        ),
        ("fprintf", "fprintf(fd,...)", "formatted write to file"),
        (
            "fscan",
            "fscan(fd,fmt)",
            "read formatted data from file (returns list)",
        ),
        (
            "fscanf",
            "fscanf(fd,fmt,...)",
            "read formatted data with arguments (returns list)",
        ),
        ("fsize", "fsize(filename)", "get file size in bytes"),
        (
            "exists",
            "exists(filename)",
            "check if file exists (returns 1 or 0)",
        ),
        (
            "isdir",
            "isdir(path)",
            "check if path is directory (returns 1 or 0)",
        ),
        (
            "mkdir",
            "mkdir(path)",
            "create directory (returns 0 on success)",
        ),
        // Memory & stack management (Phase 6.2)
        ("blk", "blk(size)", "allocate memory block"),
        ("blkcpy", "blkcpy(dest,src,size)", "copy memory block"),
        ("blkfree", "blkfree(id)", "free memory block"),
        ("blocks", "blocks()", "get number of allocated blocks"),
        ("free", "free()", "free all allocated memory"),
        ("freeglobals", "freeglobals()", "free all global variables"),
        ("push", "push(val)", "push value onto evaluation stack"),
        ("pop", "pop()", "pop value from evaluation stack"),
        ("depth", "depth()", "get evaluation stack depth"),
        ("blksize", "blksize(id)", "get size of memory block"),
        (
            "peek",
            "peek(id,offset)",
            "read byte from memory block at offset",
        ),
        (
            "poke",
            "poke(id,offset,val)",
            "write byte to memory block at offset",
        ),
        (
            "memread",
            "memread(id,offset,size)",
            "read bytes from block as string",
        ),
        // Command & script functions (Phase 6.4)
        ("argv", "argv(n)", "get nth command-line argument"),
        ("cmdbuf", "cmdbuf()", "get current command buffer"),
        ("command", "command(str)", "execute shell command"),
        ("eval", "eval(str)", "evaluate string expression"),
        // Obscure trigonometric variants (Phase 6.5)
        ("haversin", "haversin(x)", "haversine: (1 - cos(x)) / 2"),
        ("versin", "versin(x)", "versine: 1 - cos(x)"),
        ("coversin", "coversin(x)", "coversine: 1 - sin(x)"),
        ("exsecant", "exsecant(x)", "exsecant: sec(x) - 1"),
        ("chord", "chord(x)", "chord: 2 * sin(x/2)"),
        (
            "semiversin",
            "semiversin(x)",
            "semiversine: alias for haversin",
        ),
        (
            "hacoversin",
            "hacoversin(x)",
            "hacoversine: (1 - sin(x)) / 2",
        ),
        ("vers", "vers(x)", "versed sine: alias for versin"),
        ("exsec", "exsec(x)", "exsecant: alias for exsecant"),
        ("vercosin", "vercosin(x)", "vercosine: 1 + cos(x)"),
        ("vercos", "vercos(x)", "vercosine: alias for vercosin"),
        ("covercosin", "covercosin(x)", "covercosine: 1 + sin(x)"),
        (
            "covercos",
            "covercos(x)",
            "covercosine: alias for covercosin",
        ),
        (
            "cohaversin",
            "cohaversin(x)",
            "cohaversine: (1 - sin(x)) / 2",
        ),
        (
            "hacovercosin",
            "hacovercosin(x)",
            "hacovercosine: (1 + sin(x)) / 2",
        ),
        ("excosec", "excosec(x)", "excosecant: csc(x) - 1"),
        ("excsc", "excsc(x)", "excosecant: alias for excosec"),
        ("hav", "hav(x)", "haversine: alias for haversin"),
        ("crd", "crd(x)", "chord: alias for chord"),
        ("cvs", "cvs(x)", "coversine: alias for coversin"),
        ("havercos", "havercos(x)", "havercosine: (1 + cos(x)) / 2"),
        // Cryptographic & hashing (Phase 6.6)
        ("sha1", "sha1(str)", "SHA-1 hash (returns hex string)"),
        ("md5", "md5(str)", "MD5 hash (returns hex string)"),
        ("crc32", "crc32(str)", "CRC32 checksum (returns integer)"),
        // Residue class & modular operations (Phase 6.7)
        ("rc", "rc(n,m)", "residue class: reduce n modulo m"),
        ("rcadd", "rcadd(a,b,m)", "residue addition: (a+b) mod m"),
        ("rcsub", "rcsub(a,b,m)", "residue subtraction: (a-b) mod m"),
        (
            "rcmul",
            "rcmul(a,b,m)",
            "residue multiplication: (a*b) mod m",
        ),
        ("rcdiv", "rcdiv(a,b,m)", "residue division: (a/b) mod m"),
        ("rcinv", "rcinv(a,m)", "modular inverse of a mod m"),
        (
            "rceq",
            "rceq(a,b,m)",
            "residue equality: check if a≡b (mod m)",
        ),
        ("rcneg", "rcneg(a,m)", "residue negation: (-a) mod m"),
        // String operations (Phase 7)
        (
            "substr",
            "substr(s,start[,len])",
            "extract substring from position",
        ),
        ("str", "str(x)", "convert value to string"),
        (
            "replace",
            "replace(s,old,new)",
            "replace all occurrences in string",
        ),
        (
            "split",
            "split(s,sep)",
            "split string by separator into list",
        ),
        ("ltrim", "ltrim(s)", "trim whitespace from left"),
        ("rtrim", "rtrim(s)", "trim whitespace from right"),
        ("trim", "trim(s)", "trim whitespace from both sides"),
        ("repeat", "repeat(s,n)", "repeat string n times"),
        (
            "startswith",
            "startswith(s,prefix)",
            "check if string starts with prefix",
        ),
        (
            "endswith",
            "endswith(s,suffix)",
            "check if string ends with suffix",
        ),
        ("lpad", "lpad(s,width[,fill])", "left pad string to width"),
        ("rpad", "rpad(s,width[,fill])", "right pad string to width"),
        ("ord", "ord(c)", "get ASCII code of character"),
        ("chr", "chr(code)", "get character from ASCII code"),
        ("swapcase", "swapcase(s)", "swap case of all characters"),
        ("title", "title(s)", "convert string to title case"),
        // List operations (Phase 8)
        ("sort", "sort(list)", "sort list in ascending order"),
        ("rsort", "rsort(list)", "sort list in descending order"),
        ("reverse", "reverse(list)", "reverse list order"),
        ("unique", "unique(list)", "remove duplicates from list"),
        ("min", "min(list)", "find minimum value in list"),
        ("max", "max(list)", "find maximum value in list"),
        ("sum", "sum(list)", "sum all numeric elements"),
        ("product", "product(list)", "multiply all numeric elements"),
        (
            "find",
            "find(list,value)",
            "find index of value (-1 if not found)",
        ),
        (
            "contains",
            "contains(list,value)",
            "check if list contains value",
        ),
        ("count", "count(list,value)", "count occurrences of value"),
        ("flatten", "flatten(list)", "flatten nested lists"),
        ("zip", "zip(list1,list2)", "combine two lists into pairs"),
        ("range", "range(start,end[,step])", "create list of numbers"),
        // Variable/scope management (Phase 9)
        ("vars", "vars()", "list all global variables"),
        ("defined", "defined(name)", "check if variable exists"),
        ("undefine", "undefine(name)", "delete variable"),
        ("del", "del(name)", "alias for undefine"),
        ("type", "type(x)", "get type name of value"),
        ("sizeof", "sizeof(x)", "get approximate size in bytes"),
        (
            "env",
            "env()",
            "list environment variables as [name,value] pairs",
        ),
        (
            "dump",
            "dump()",
            "dump all state (variables, config, stats)",
        ),
        // Phase 10: I/O & Formatting
        ("println", "println(x,...)", "print with newline"),
        ("puts", "puts(s)", "put string with newline"),
        ("getline", "getline()", "read line from stdin"),
        ("input", "input(prompt)", "read input with prompt"),
        ("printf", "printf(fmt,...)", "formatted print"),
        ("sprintf", "sprintf(fmt,...)", "formatted string"),
        ("format", "format(fmt,...)", "generic formatting"),
        ("debug", "debug(x)", "debug output to stderr"),
        ("hex", "hex(x)", "format as hexadecimal"),
        ("oct", "oct(x)", "format as octal"),
        ("bin", "bin(x)", "format as binary"),
        // Phase 11: Math Extensions
        ("mean", "mean(list)", "arithmetic mean (average)"),
        ("median", "median(list)", "median value"),
        ("variance", "variance(list)", "variance"),
        ("stdev", "stdev(list)", "standard deviation"),
        ("clz", "clz(x)", "count leading zeros"),
        ("ctz", "ctz(x)", "count trailing zeros"),
        ("nextpow2", "nextpow2(x)", "next power of 2"),
        ("prevpow2", "prevpow2(x)", "previous power of 2"),
        ("ispow2", "ispow2(x)", "check if power of 2"),
        (
            "hammingdist",
            "hammingdist(x,y)",
            "Hamming distance between two numbers",
        ),
        ("gray", "gray(x)", "convert to Gray code"),
        ("igray", "igray(x)", "convert from Gray code"),
        ("popcount", "popcount(x)", "population count (set bits)"),
        ("rms", "rms(list)", "root mean square"),
        ("gmean", "gmean(list)", "geometric mean"),
        ("hmean", "hmean(list)", "harmonic mean"),
        // Phase 12: System & Utility
        ("version", "version()", "get version string"),
        ("platform", "platform()", "get OS platform name"),
        ("hostname", "hostname()", "get system hostname"),
        ("pid", "pid()", "get process ID"),
        ("username", "username()", "get current username"),
        ("homedir", "homedir()", "get home directory path"),
        ("tmpdir", "tmpdir()", "get temp directory path"),
        ("pwd", "pwd()", "get current working directory"),
        ("cd", "cd(path)", "change directory"),
        ("getuid", "getuid()", "get user ID"),
        ("arch", "arch()", "get CPU architecture"),
        ("uname", "uname()", "get system info (os-arch)"),
        // Phase 13: Advanced Operations
        ("matmul", "matmul(m1,m2)", "matrix multiplication"),
        ("polyval", "polyval(coeffs,x)", "polynomial evaluation"),
        ("dot", "dot(v1,v2)", "dot product of vectors"),
        ("norm", "norm(v)", "vector norm (magnitude)"),
        ("polyderiv", "polyderiv(coeffs)", "polynomial derivative"),
        ("union", "union(set1,set2)", "set union"),
        (
            "intersection",
            "intersection(set1,set2)",
            "set intersection",
        ),
        ("difference", "difference(set1,set2)", "set difference"),
        ("subset", "subset(set1,set2)", "check if subset"),
        ("interp", "interp(xs,ys,x)", "linear interpolation"),
        ("cumsum", "cumsum(list)", "cumulative sum"),
        ("diff", "diff(list)", "consecutive differences"),
        ("mode", "mode(list)", "most common value"),
        // Final functions to 100%
        ("trunc", "trunc(x)", "truncate to integer"),
        ("exp2", "exp2(x)", "exponential base 2 (2^x)"),
        ("exp10", "exp10(x)", "exponential base 10 (10^x)"),
        ("pow10", "pow10(x)", "alias for exp10"),
        ("expm1", "expm1(x)", "exp(x) - 1, accurate for small x"),
        ("log1p", "log1p(x)", "log(1 + x), accurate for small x"),
    ]
}
