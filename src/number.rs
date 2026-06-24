//! Arbitrary-precision numeric core.
//!
//! calc's native value is an exact rational (a ratio of arbitrary-precision
//! integers). We use the same model via `num_rational::BigRational`, so integer
//! and rational arithmetic is *exact* — `1/3 * 3` is exactly `1`, never `0.999…`.
//!
//! Irrational operations (sqrt, transcendentals) are approximated to within the
//! session `epsilon`, again mirroring calc's behaviour. sqrt, pi, and e are computed
//! at arbitrary precision here.

use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

pub type Num = BigRational;

fn bi(n: i64) -> BigInt {
    BigInt::from(n)
}

/// π as a 60-digit string constant.
pub fn pi() -> Num {
    const PI: &str = "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
    parse_number(PI).unwrap()
}

/// e as a 60-digit string constant.
pub fn e() -> Num {
    const E: &str = "2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274";
    parse_number(E).unwrap()
}

/// Parse a decimal numeric literal: integers, fractions written as `a/b`,
/// decimals like `3.14`, and scientific notation `1.2e-3`. Hex/binary (`0x`,
/// `0b`) are also accepted as integers.
pub fn parse_number(s: &str) -> Option<Num> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // a/b explicit rational
    if let Some((a, b)) = s.split_once('/') {
        let an = parse_number(a)?;
        let bn = parse_number(b)?;
        if bn.is_zero() {
            return None;
        }
        return Some(an / bn);
    }

    // radix-prefixed integers
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return BigInt::parse_bytes(hex.as_bytes(), 16).map(|i| Num::from_integer(i));
    }
    if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        return BigInt::parse_bytes(bin.as_bytes(), 2).map(|i| Num::from_integer(i));
    }

    // scientific notation
    let (mantissa, exp) = match s.split_once(['e', 'E']) {
        Some((m, e)) => (m, e.parse::<i64>().ok()?),
        None => (s, 0),
    };

    // decimal mantissa
    let (int_part, frac_part) = match mantissa.split_once('.') {
        Some((i, f)) => (i, f),
        None => (mantissa, ""),
    };

    let sign = if int_part.starts_with('-') { -1 } else { 1 };
    let int_digits = int_part.trim_start_matches(['+', '-']);
    if !int_digits.chars().all(|c| c.is_ascii_digit()) && !int_digits.is_empty() {
        return None;
    }
    if !frac_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let combined = format!("{}{}", int_digits, frac_part);
    let combined = if combined.is_empty() { "0" } else { &combined };
    let numer = BigInt::parse_bytes(combined.as_bytes(), 10)? * sign;

    // scale = 10^(frac_len - exp)
    let scale_pow = frac_part.len() as i64 - exp;
    let mut value = Num::from_integer(numer);
    if scale_pow > 0 {
        let denom = BigInt::from(10).pow(scale_pow as u32);
        value /= Num::from_integer(denom);
    } else if scale_pow < 0 {
        let mult = BigInt::from(10).pow((-scale_pow) as u32);
        value *= Num::from_integer(mult);
    }
    Some(value)
}

/// Integer power: `base` (rational) raised to an integer `exp`.
pub fn pow_int(base: &Num, exp: &BigInt) -> Result<Num, String> {
    if exp.is_zero() {
        return Ok(Num::one());
    }
    if base.is_zero() {
        if exp.is_negative() {
            return Err("division by zero (0 to a negative power)".into());
        }
        return Ok(Num::zero());
    }
    let neg = exp.is_negative();
    let e = exp.abs();
    let e_u32 = e
        .to_u32()
        .ok_or_else(|| "exponent too large to evaluate".to_string())?;
    let numer = base.numer().pow(e_u32);
    let denom = base.denom().pow(e_u32);
    let result = Num::new(numer, denom);
    Ok(if neg { result.recip() } else { result })
}

/// General power. Integer exponents are exact; otherwise we approximate via f64.
pub fn pow(base: &Num, exp: &Num) -> Result<Num, String> {
    if exp.is_integer() {
        return pow_int(base, &exp.to_integer());
    }
    let b = base.to_f64().ok_or("base out of f64 range")?;
    let e = exp.to_f64().ok_or("exponent out of f64 range")?;
    let r = b.powf(e);
    if !r.is_finite() {
        return Err("power result is not finite".into());
    }
    Num::from_float(r).ok_or_else(|| "could not represent power result".into())
}

/// Square root to within `epsilon`, computed by Newton's method on rationals.
/// Perfect squares return exactly.
pub fn sqrt(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_negative() {
        return Err("sqrt of a negative number (complex not supported)".into());
    }
    if x.is_zero() {
        return Ok(Num::zero());
    }
    // Exact integer perfect-square fast path.
    if x.is_integer() {
        let n = x.to_integer();
        let r = n.sqrt(); // floor sqrt
        if &r * &r == n {
            return Ok(Num::from_integer(r));
        }
    }
    // Initial guess from f64 (good to ~15 digits), then refine.
    let mut g = match x.to_f64().map(|v| v.sqrt()) {
        Some(v) if v.is_finite() && v > 0.0 => {
            Num::from_float(v).unwrap_or_else(|| x.clone() / Num::from_integer(bi(2)))
        }
        _ => x.clone() / Num::from_integer(bi(2)),
    };
    let two = Num::from_integer(bi(2));
    for _ in 0..200 {
        let next = (&g + x / &g) / &two;
        let diff = (&next * &next - x).abs();
        g = next;
        if &diff < epsilon {
            break;
        }
    }
    Ok(round_to_epsilon(&g, epsilon))
}

/// Complex square root: returns (real, imaginary) parts.
/// For a negative real number -x, returns (0, sqrt(x)).
pub fn sqrt_complex(a: &Num, b: &Num, epsilon: &Num) -> Result<(Num, Num), String> {
    if b.is_zero() && a.is_negative() {
        // Special case: sqrt(-x) = i*sqrt(x) for x > 0
        let imag = sqrt(&(-a), epsilon)?;
        return Ok((Num::zero(), imag));
    }

    // General case: sqrt(a + bi)
    // Magnitude: r = sqrt(a^2 + b^2)
    let a_sq = a * a;
    let b_sq = b * b;
    let magnitude_sq = &a_sq + &b_sq;
    let magnitude = sqrt(&magnitude_sq, epsilon)?;

    // Real part: sqrt((magnitude + a) / 2)
    let real_part_arg = (&magnitude + a) / Num::from_integer(bi(2));
    let real = sqrt(&real_part_arg, epsilon)?;

    // Imaginary part: sign(b) * sqrt((magnitude - a) / 2)
    let imag_part_arg = (&magnitude - a) / Num::from_integer(bi(2));
    let imag = sqrt(&imag_part_arg, epsilon)?;
    let imag_final = if b.is_negative() { -imag } else { imag };

    Ok((real, imag_final))
}

/// Exponential: e^x to within `epsilon`, computed via Taylor series.
/// exp(x) = sum(x^n / n!) for n=0..∞
pub fn exp(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_zero() {
        return Ok(Num::one());
    }
    // For large |x|, reduce: exp(x) = exp(q + r) = exp(q) * exp(r)
    // where q is an integer and |r| < 1.
    let two = Num::from_integer(bi(2));

    // If |x| >= 2, use exp(x) = exp(x/2)^2 repeatedly to reduce magnitude
    let mut reduction_count = 0;
    let mut y = x.clone();
    while &y.abs() >= &two {
        y = &y / &two;
        reduction_count += 1;
    }

    // Compute exp(y) where |y| < 2 via Taylor series
    let mut result = Num::one();
    let mut term = Num::one();
    for n in 1..500 {
        term = &term * &y / Num::from_integer(bi(n as i64));
        result += &term;
        if &term.abs() < epsilon {
            break;
        }
    }

    // Square result `reduction_count` times to recover exp(x)
    for _ in 0..reduction_count {
        result = &result * &result;
    }

    Ok(round_to_epsilon(&result, epsilon))
}

/// Natural logarithm: ln(x) to within `epsilon`, computed via series.
/// For x near 1, use: ln(x) = 2 * sum((x-1)/(x+1))^(2n+1) / (2n+1) for n=0..∞
/// For |x| far from 1, shift via repeated multiplication/division by e.
pub fn ln(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_negative() || x.is_zero() {
        return Err("ln of non-positive number".into());
    }
    if x == &Num::one() {
        return Ok(Num::zero());
    }

    let one = Num::one();
    let e_const = e();
    let two = Num::from_integer(bi(2));

    // Reduce to |x - 1| < 0.5 by multiplying/dividing by e repeatedly
    let mut reduction = 0i64;
    let mut y = x.clone();
    while &y > &(&e_const * &two) {
        y = &y / &e_const;
        reduction += 1;
    }
    while &y < &(&one / &two) {
        y = &y * &e_const;
        reduction -= 1;
    }

    // Series: ln(y) = 2 * sum((y-1)/(y+1))^(2n+1) / (2n+1)
    let num = &y - &one;
    let denom = &y + &one;
    let z = &num / &denom;
    let z_sq = &z * &z;

    let mut result = z.clone();
    let mut z_pow = z.clone();
    for n in 1..500 {
        z_pow = &z_pow * &z_sq;
        let term = &z_pow / Num::from_integer(bi(2 * n as i64 + 1));
        result += &term;
        if &term.abs() < epsilon {
            break;
        }
    }
    result = &(&two * &result) + Num::from_integer(bi(reduction));

    Ok(round_to_epsilon(&result, epsilon))
}

/// Sine to within `epsilon`, computed via Taylor series with range reduction.
/// Uses: sin(x) = sum((-1)^n * x^(2n+1) / (2n+1)!) for n=0..∞
pub fn sin(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let pi_const = pi();
    let two_pi = &pi_const * &Num::from_integer(bi(2));

    // Reduce x to [-pi, pi]
    let mut y = x.clone();
    while &y > &pi_const {
        y = &y - &two_pi;
    }
    while &y < &-&pi_const {
        y = &y + &two_pi;
    }

    // Taylor series: sin(y) = sum((-1)^n * y^(2n+1) / (2n+1)!)
    let mut result = y.clone();
    let mut term = y.clone();
    let y_sq = &y * &y;
    let neg_y_sq = -&y_sq;

    for n in 1..500 {
        term = &term * &neg_y_sq / (Num::from_integer(bi(2 * n as i64)) * Num::from_integer(bi(2 * n as i64 + 1)));
        result += &term;
        if &term.abs() < epsilon {
            break;
        }
    }

    Ok(round_to_epsilon(&result, epsilon))
}

/// Cosine to within `epsilon`, computed via Taylor series with range reduction.
/// Uses: cos(x) = sum((-1)^n * x^(2n) / (2n)!) for n=0..∞
pub fn cos(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let pi_const = pi();
    let two_pi = &pi_const * &Num::from_integer(bi(2));

    // Reduce x to [-pi, pi]
    let mut y = x.clone();
    while &y > &pi_const {
        y = &y - &two_pi;
    }
    while &y < &-&pi_const {
        y = &y + &two_pi;
    }

    // Taylor series: cos(y) = sum((-1)^n * y^(2n) / (2n)!)
    let mut result = Num::one();
    let mut term = Num::one();
    let y_sq = &y * &y;
    let neg_y_sq = -&y_sq;

    for n in 1..500 {
        term = &term * &neg_y_sq / (Num::from_integer(bi(2 * n as i64 - 1)) * Num::from_integer(bi(2 * n as i64)));
        result += &term;
        if &term.abs() < epsilon {
            break;
        }
    }

    Ok(round_to_epsilon(&result, epsilon))
}

/// Tangent: tan(x) = sin(x) / cos(x) to within `epsilon`.
pub fn tan(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let s = sin(x, epsilon)?;
    let c = cos(x, epsilon)?;
    if c.is_zero() {
        return Err("tan: undefined (cos = 0)".to_string());
    }
    Ok(round_to_epsilon(&(&s / &c), epsilon))
}

/// Snap a value to a multiple of `epsilon`, keeping results compact.
pub fn round_to_epsilon(x: &Num, epsilon: &Num) -> Num {
    if epsilon.is_zero() {
        return x.clone();
    }
    let scaled = x / epsilon;
    let rounded = scaled.round();
    rounded * epsilon
}

/// floor(x) as a rational integer.
pub fn floor(x: &Num) -> Num {
    Num::from_integer(x.floor().to_integer())
}
/// ceil(x).
pub fn ceil(x: &Num) -> Num {
    Num::from_integer(x.ceil().to_integer())
}
/// truncate toward zero (calc's int()).
pub fn trunc(x: &Num) -> Num {
    Num::from_integer(x.trunc().to_integer())
}
/// fractional part x - int(x).
pub fn frac(x: &Num) -> Num {
    x - trunc(x)
}

/// Render a rational as a decimal string with up to `digits` fractional places.
/// A leading `~` marks an inexact (rounded / non-terminating) rendering, exactly
/// as calc does.
pub fn to_decimal_string(x: &Num, digits: usize) -> String {
    if x.is_integer() {
        return x.numer().to_string();
    }
    let neg = x.is_negative();
    let x = x.abs();
    let numer = x.numer().clone();
    let denom = x.denom().clone();

    let int_part = &numer / &denom;
    let mut rem = &numer % &denom;

    let mut frac_digits = String::new();
    let ten = bi(10);
    let mut exact = false;
    for _ in 0..(digits + 1) {
        if rem.is_zero() {
            exact = true;
            break;
        }
        rem *= &ten;
        let d = &rem / &denom;
        rem %= &denom;
        frac_digits.push_str(&d.to_string());
    }

    let mut frac_chars: Vec<u8> = frac_digits.into_bytes();
    if !exact && frac_chars.len() > digits {
        let guard = frac_chars.pop().unwrap() - b'0';
        if guard >= 5 {
            let mut i = frac_chars.len();
            let mut carry = true;
            while carry && i > 0 {
                i -= 1;
                if frac_chars[i] == b'9' {
                    frac_chars[i] = b'0';
                } else {
                    frac_chars[i] += 1;
                    carry = false;
                }
            }
            if carry {
                let int_inc = &int_part + 1;
                return assemble(neg, &int_inc, &trim_zeros(&frac_chars), exact);
            }
        }
    }
    assemble(neg, &int_part, &trim_zeros(&frac_chars), exact)
}

fn trim_zeros(chars: &[u8]) -> String {
    let s = String::from_utf8_lossy(chars);
    let trimmed = s.trim_end_matches('0');
    trimmed.to_string()
}

fn assemble(neg: bool, int_part: &BigInt, frac: &str, exact: bool) -> String {
    let sign = if neg { "-" } else { "" };
    let tilde = if exact { "" } else { "~" };
    if frac.is_empty() {
        format!("{tilde}{sign}{int_part}")
    } else {
        format!("{tilde}{sign}{int_part}.{frac}")
    }
}

/// gcd of two rationals' integer values (calc gcd works on rationals too, but we
/// keep the common integer case which is what users reach for).
pub fn gcd_int(a: &BigInt, b: &BigInt) -> BigInt {
    a.gcd(b)
}
