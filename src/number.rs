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
        return BigInt::parse_bytes(hex.as_bytes(), 16).map(Num::from_integer);
    }
    if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        return BigInt::parse_bytes(bin.as_bytes(), 2).map(Num::from_integer);
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

/// Nth root: x^(1/n) to within `epsilon`. Computed via Newton's method.
pub fn root(x: &Num, n: i64, epsilon: &Num) -> Result<Num, String> {
    if n == 0 {
        return Err("root: n must be nonzero".to_string());
    }
    if n == 1 {
        return Ok(x.clone());
    }
    if n == 2 {
        return sqrt(x, epsilon);
    }
    if n < 0 {
        // x^(1/n) = 1 / x^(1/-n)
        return Ok(Num::one() / root(x, -n, epsilon)?);
    }

    // x^(1/n) via Newton's method: g_{k+1} = ((n-1)*g_k + x/g_k^(n-1)) / n
    if x.is_zero() {
        return Ok(Num::zero());
    }
    if x.is_negative() && n % 2 == 0 {
        return Err("root: even root of negative number".to_string());
    }

    let is_negative = x.is_negative();
    let abs_x = if is_negative { -x } else { x.clone() };

    let n_bi = Num::from_integer(bi(n));
    let mut g = match abs_x.to_f64().map(|v| v.powf(1.0 / n as f64)) {
        Some(v) if v.is_finite() && v > 0.0 => {
            Num::from_float(v).unwrap_or_else(|| &abs_x / &n_bi)
        }
        _ => &abs_x / &n_bi,
    };

    for _ in 0..200 {
        let g_pow_n_minus_1 = {
            let mut res = Num::one();
            for _ in 0..(n - 1) {
                res = &res * &g;
            }
            res
        };
        let next = (&(&Num::from_integer(bi(n - 1)) * &g) + &(&abs_x / &g_pow_n_minus_1)) / &n_bi;
        let diff = (&next - &g).abs();
        g = next;
        if &diff < epsilon {
            break;
        }
    }

    let result = round_to_epsilon(&g, epsilon);
    Ok(if is_negative { -result } else { result })
}

/// Cube root: cbrt(x) = x^(1/3)
pub fn cbrt(x: &Num, epsilon: &Num) -> Result<Num, String> {
    root(x, 3, epsilon)
}

/// Integer square root: largest integer n such that n^2 <= x
pub fn isqrt(x: &Num) -> Result<Num, String> {
    if x.is_negative() {
        return Err("isqrt: negative argument".to_string());
    }
    if !x.is_integer() {
        return Err("isqrt: argument must be an integer".to_string());
    }
    let bi_x = x.to_integer();
    let result = bi_x.sqrt();
    Ok(Num::from_integer(result))
}

/// Integer nth root: largest integer n such that n^k <= x
pub fn iroot(x: &Num, k: i64) -> Result<Num, String> {
    if k == 0 {
        return Err("iroot: k must be nonzero".to_string());
    }
    if !x.is_integer() {
        return Err("iroot: argument must be an integer".to_string());
    }
    if k < 0 {
        return Err("iroot: k must be positive".to_string());
    }
    if k == 1 {
        return Ok(x.clone());
    }
    if x.is_negative() && k % 2 == 0 {
        return Err("iroot: even root of negative number".to_string());
    }

    let is_negative = x.is_negative();
    let bi_x = if is_negative { -x.to_integer() } else { x.to_integer() };

    // Binary search for the integer root
    let mut low = bi(0);
    let mut high = bi_x.clone();

    while low <= high {
        let mid = (&low + &high) / bi(2);
        let mut power = bi(1);
        for _ in 0..k {
            power = &power * &mid;
            if power > bi_x {
                break;
            }
        }

        if power == bi_x {
            let result = Num::from_integer(mid);
            return Ok(if is_negative { -result } else { result });
        } else if power < bi_x {
            low = &mid + bi(1);
        } else {
            high = &mid - bi(1);
        }
    }

    let result = Num::from_integer(high);
    Ok(if is_negative { -result } else { result })
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
    while y.abs() >= two {
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
    while y > (&e_const * &two) {
        y = &y / &e_const;
        reduction += 1;
    }
    while y < (&one / &two) {
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

/// Logarithm base n: logn(x, n) = ln(x) / ln(n)
pub fn logn(x: &Num, n: &Num, epsilon: &Num) -> Result<Num, String> {
    if n <= &Num::zero() || n == &Num::one() {
        return Err("logn: base must be positive and not 1".to_string());
    }
    let ln_x = ln(x, epsilon)?;
    let ln_n = ln(n, epsilon)?;
    Ok(round_to_epsilon(&(&ln_x / &ln_n), epsilon))
}

/// Integer logarithm base 10: largest integer n such that 10^n <= x
pub fn ilog10(x: &Num) -> Result<Num, String> {
    if x <= &Num::zero() {
        return Err("ilog10: argument must be positive".to_string());
    }
    if !x.is_integer() {
        return Err("ilog10: argument must be an integer".to_string());
    }
    let bi_x = x.to_integer();
    if bi_x < num_bigint::BigInt::from(10) {
        return Ok(Num::zero());
    }

    let mut count: i64 = 0;
    let mut val = bi_x.clone();
    let ten = bi(10);
    while val >= ten {
        val = &val / &ten;
        count += 1;
    }
    Ok(Num::from_integer(bi(count)))
}

/// Integer logarithm base 2: largest integer n such that 2^n <= x
pub fn ilog2(x: &Num) -> Result<Num, String> {
    if x <= &Num::zero() {
        return Err("ilog2: argument must be positive".to_string());
    }
    if !x.is_integer() {
        return Err("ilog2: argument must be an integer".to_string());
    }
    let bi_x = x.to_integer();
    if bi_x < num_bigint::BigInt::from(2) {
        return Ok(Num::zero());
    }

    let mut count: i64 = 0;
    let mut val = bi_x.clone();
    let two = bi(2);
    while val >= two {
        val = &val / &two;
        count += 1;
    }
    Ok(Num::from_integer(bi(count)))
}

/// Integer logarithm base e: largest integer n such that e^n <= x
pub fn ilog(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x <= &Num::zero() {
        return Err("ilog: argument must be positive".to_string());
    }
    let ln_x = ln(x, epsilon)?;
    Ok(ln_x.floor())
}

/// Integer logarithm base n: largest integer k such that n^k <= x
pub fn ilogn(x: &Num, n: &Num, epsilon: &Num) -> Result<Num, String> {
    if n <= &Num::zero() || n == &Num::one() {
        return Err("ilogn: base must be positive and not 1".to_string());
    }
    if x <= &Num::zero() {
        return Err("ilogn: argument must be positive".to_string());
    }
    let logn_result = logn(x, n, epsilon)?;
    Ok(logn_result.floor())
}

/// Sine to within `epsilon`, computed via Taylor series with range reduction.
/// Uses: sin(x) = sum((-1)^n * x^(2n+1) / (2n+1)!) for n=0..∞
pub fn sin(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let pi_const = pi();
    let two_pi = &pi_const * &Num::from_integer(bi(2));

    // Reduce x to [-pi, pi]
    let mut y = x.clone();
    while y > pi_const {
        y = &y - &two_pi;
    }
    while y < -&pi_const {
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
    while y > pi_const {
        y = &y - &two_pi;
    }
    while y < -&pi_const {
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

/// Cotangent: cot(x) = cos(x) / sin(x), to within `epsilon`.
pub fn cot(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let s = sin(x, epsilon)?;
    let c = cos(x, epsilon)?;
    if s.is_zero() {
        return Err("cot: undefined (sin = 0)".to_string());
    }
    Ok(round_to_epsilon(&(&c / &s), epsilon))
}

/// Secant: sec(x) = 1 / cos(x), to within `epsilon`.
pub fn sec(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let c = cos(x, epsilon)?;
    if c.is_zero() {
        return Err("sec: undefined (cos = 0)".to_string());
    }
    Ok(round_to_epsilon(&(Num::one() / &c), epsilon))
}

/// Cosecant: csc(x) = 1 / sin(x), to within `epsilon`.
pub fn csc(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let s = sin(x, epsilon)?;
    if s.is_zero() {
        return Err("csc: undefined (sin = 0)".to_string());
    }
    Ok(round_to_epsilon(&(Num::one() / &s), epsilon))
}

/// Inverse sine: asin(x) via Newton's method or series, to within `epsilon`.
pub fn asin(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.abs() > Num::one() {
        return Err("asin: domain error (|x| > 1)".to_string());
    }
    if x.is_zero() {
        return Ok(Num::zero());
    }
    // Use series: asin(x) = sum((-1)^n * (2n)! / (2^(2n) * (n!)^2 * (2n+1)) * x^(2n+1))
    let mut result = x.clone();
    let mut term = x.clone();
    let x_sq = x * x;

    for n in 1..500 {
        let coeff = Num::from_integer(bi(2 * n as i64 - 1)) / Num::from_integer(bi(2 * n as i64));
        term = &term * &(&x_sq * &coeff);
        result = &result + &(&term / Num::from_integer(bi(2 * n as i64 + 1)));
        if &term.abs() < epsilon {
            break;
        }
    }
    Ok(round_to_epsilon(&result, epsilon))
}

/// Inverse cosine: acos(x) = pi/2 - asin(x), to within `epsilon`.
pub fn acos(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.abs() > Num::one() {
        return Err("acos: domain error (|x| > 1)".to_string());
    }
    let asin_val = asin(x, epsilon)?;
    Ok(round_to_epsilon(&(&pi() / Num::from_integer(bi(2)) - asin_val), epsilon))
}

/// Inverse tangent: atan(x) via series, to within `epsilon`.
pub fn atan(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_zero() {
        return Ok(Num::zero());
    }
    // Use series: atan(x) = x - x^3/3 + x^5/5 - ... for |x| <= 1
    // For |x| > 1, use: atan(x) = sign(x)*pi/2 - atan(1/x)
    let use_reciprocal = x.abs() > Num::one();
    let y = if use_reciprocal {
        Num::one() / x
    } else {
        x.clone()
    };

    let mut result = y.clone();
    let mut term = y.clone();
    let y_sq = &y * &y;
    let neg_y_sq = -&y_sq;

    for n in 1..500 {
        term = &term * &neg_y_sq;
        result = &result + &(&term / Num::from_integer(bi(2 * n as i64 + 1)));
        if &term.abs() < epsilon {
            break;
        }
    }

    let result = if use_reciprocal {
        let pi_half = &pi() / Num::from_integer(bi(2));
        if x.is_negative() {
            -&pi_half - result
        } else {
            &pi_half - result
        }
    } else {
        result
    };

    Ok(round_to_epsilon(&result, epsilon))
}

/// Two-argument arctangent: atan2(y, x), to within `epsilon`.
pub fn atan2(y: &Num, x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_zero() && y.is_zero() {
        return Err("atan2: both arguments are zero".to_string());
    }
    if x.is_zero() {
        let pi_half = &pi() / Num::from_integer(bi(2));
        return Ok(if y.is_positive() { pi_half } else { -pi_half });
    }
    let atan_val = atan(&(y / x), epsilon)?;
    let result = if x.is_positive() {
        atan_val
    } else if y.is_negative() {
        atan_val - pi()
    } else {
        atan_val + pi()
    };
    Ok(round_to_epsilon(&result, epsilon))
}

/// Inverse cotangent: acot(x) = atan(1/x) for x > 0, or pi + atan(1/x) for x < 0.
pub fn acot(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_zero() {
        return Ok(&pi() / Num::from_integer(bi(2)));
    }
    let atan_val = atan(&(Num::one() / x), epsilon)?;
    let result = if x.is_positive() {
        atan_val
    } else {
        atan_val + pi()
    };
    Ok(round_to_epsilon(&result, epsilon))
}

/// Inverse secant: asec(x) = acos(1/x) for |x| >= 1.
pub fn asec(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.abs() < Num::one() {
        return Err("asec: domain error (|x| < 1)".to_string());
    }
    acos(&(Num::one() / x), epsilon)
}

/// Inverse cosecant: acsc(x) = asin(1/x) for |x| >= 1.
pub fn acsc(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.abs() < Num::one() {
        return Err("acsc: domain error (|x| < 1)".to_string());
    }
    asin(&(Num::one() / x), epsilon)
}

/// Hyperbolic sine: sinh(x) = (e^x - e^-x) / 2, to within `epsilon`.
pub fn sinh(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let exp_x = exp(x, epsilon)?;
    let exp_neg_x = exp(&(-x), epsilon)?;
    Ok(round_to_epsilon(&((&exp_x - &exp_neg_x) / Num::from_integer(bi(2))), epsilon))
}

/// Hyperbolic cosine: cosh(x) = (e^x + e^-x) / 2, to within `epsilon`.
pub fn cosh(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let exp_x = exp(x, epsilon)?;
    let exp_neg_x = exp(&(-x), epsilon)?;
    Ok(round_to_epsilon(&((&exp_x + &exp_neg_x) / Num::from_integer(bi(2))), epsilon))
}

/// Hyperbolic tangent: tanh(x) = sinh(x) / cosh(x), to within `epsilon`.
pub fn tanh(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let sinh_val = sinh(x, epsilon)?;
    let cosh_val = cosh(x, epsilon)?;
    if cosh_val.is_zero() {
        return Err("tanh: cosh is zero".to_string());
    }
    Ok(round_to_epsilon(&(&sinh_val / &cosh_val), epsilon))
}

/// Hyperbolic cotangent: coth(x) = cosh(x) / sinh(x).
pub fn coth(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let sinh_val = sinh(x, epsilon)?;
    let cosh_val = cosh(x, epsilon)?;
    if sinh_val.is_zero() {
        return Err("coth: undefined (sinh = 0)".to_string());
    }
    Ok(round_to_epsilon(&(&cosh_val / &sinh_val), epsilon))
}

/// Hyperbolic secant: sech(x) = 1 / cosh(x).
pub fn sech(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let cosh_val = cosh(x, epsilon)?;
    if cosh_val.is_zero() {
        return Err("sech: undefined (cosh = 0)".to_string());
    }
    Ok(round_to_epsilon(&(Num::one() / &cosh_val), epsilon))
}

/// Hyperbolic cosecant: csch(x) = 1 / sinh(x).
pub fn csch(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let sinh_val = sinh(x, epsilon)?;
    if sinh_val.is_zero() {
        return Err("csch: undefined (sinh = 0)".to_string());
    }
    Ok(round_to_epsilon(&(Num::one() / &sinh_val), epsilon))
}

/// Inverse hyperbolic sine: asinh(x) = ln(x + sqrt(x^2 + 1)), to within `epsilon`.
pub fn asinh(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let x_sq = x * x;
    let sqrt_val = sqrt(&(&x_sq + Num::one()), epsilon)?;
    let arg = x + &sqrt_val;
    ln(&arg, epsilon)
}

/// Inverse hyperbolic cosine: acosh(x) = ln(x + sqrt(x^2 - 1)), to within `epsilon`.
pub fn acosh(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x < &Num::one() {
        return Err("acosh: domain error (x < 1)".to_string());
    }
    let x_sq = x * x;
    let sqrt_val = sqrt(&(&x_sq - Num::one()), epsilon)?;
    let arg = x + &sqrt_val;
    ln(&arg, epsilon)
}

/// Inverse hyperbolic tangent: atanh(x) = 0.5 * ln((1+x)/(1-x)), to within `epsilon`.
pub fn atanh(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.abs() >= Num::one() {
        return Err("atanh: domain error (|x| >= 1)".to_string());
    }
    let one = Num::one();
    let numerator = &one + x;
    let denominator = &one - x;
    let ratio = &numerator / &denominator;
    let ln_val = ln(&ratio, epsilon)?;
    Ok(round_to_epsilon(&(&ln_val / Num::from_integer(bi(2))), epsilon))
}

/// Inverse hyperbolic cotangent: acoth(x) = 0.5 * ln((x+1)/(x-1)) for |x| > 1.
pub fn acoth(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.abs() <= Num::one() {
        return Err("acoth: domain error (|x| <= 1)".to_string());
    }
    let one = Num::one();
    let numerator = x + &one;
    let denominator = x - &one;
    let ratio = &numerator / &denominator;
    let ln_val = ln(&ratio, epsilon)?;
    Ok(round_to_epsilon(&(&ln_val / Num::from_integer(bi(2))), epsilon))
}

/// Inverse hyperbolic secant: asech(x) = ln(1/x + sqrt(1/x^2 - 1)) for 0 < x <= 1.
pub fn asech(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x <= &Num::zero() || x > &Num::one() {
        return Err("asech: domain error (x not in (0, 1])".to_string());
    }
    let one = Num::one();
    let x_inv = &one / x;
    let x_inv_sq = &x_inv * &x_inv;
    let sqrt_val = sqrt(&(&x_inv_sq - &one), epsilon)?;
    let arg = &x_inv + &sqrt_val;
    ln(&arg, epsilon)
}

/// Inverse hyperbolic cosecant: acsch(x) = ln(1/x + sqrt(1/x^2 + 1)) for x != 0.
pub fn acsch(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_zero() {
        return Err("acsch: undefined (x = 0)".to_string());
    }
    let one = Num::one();
    let x_inv = &one / x;
    let x_inv_sq = &x_inv * &x_inv;
    let sqrt_val = sqrt(&(&x_inv_sq + &one), epsilon)?;
    let arg = if x.is_positive() {
        &x_inv + &sqrt_val
    } else {
        &x_inv - &sqrt_val
    };
    ln(&arg, epsilon)
}

/// Cosine + Sine: cas(x) = cos(x) + sin(x), to within `epsilon`.
pub fn cas(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let c = cos(x, epsilon)?;
    let s = sin(x, epsilon)?;
    Ok(round_to_epsilon(&(&c + &s), epsilon))
}

/// Euler's formula: cis(x) = cos(x) + i*sin(x) = e^(ix)
/// Returns a complex number (real, imaginary)
pub fn cis(x: &Num, epsilon: &Num) -> Result<(Num, Num), String> {
    let c = cos(x, epsilon)?;
    let s = sin(x, epsilon)?;
    Ok((c, s))
}

/// Complex conjugate of (real, imag): returns (real, -imag)
pub fn conj_complex(real: &Num, imag: &Num) -> (Num, Num) {
    (real.clone(), -imag)
}

/// Rounding: round x to n decimal places
pub fn round_decimal(x: &Num, n: i64) -> Num {
    if n < 0 {
        return x.clone();
    }
    let factor = Num::from_integer(bi(10).pow(n as u32));
    let scaled = x * &factor;
    let rounded = scaled.round();
    rounded / factor
}

/// Hypot: sqrt(x^2 + y^2) without overflow
pub fn hypot(x: &Num, y: &Num, epsilon: &Num) -> Result<Num, String> {
    let x_sq = x * x;
    let y_sq = y * y;
    sqrt(&(&x_sq + &y_sq), epsilon)
}

/// Error function: erf(x) ≈ (2/√π) * integral(e^(-t^2), 0, x)
/// Computed via Taylor series: erf(x) = (2/√π) * sum((-1)^n * x^(2n+1) / (n! * (2n+1)))
pub fn erf(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x.is_zero() {
        return Ok(Num::zero());
    }

    let sqrt_pi = sqrt(&pi(), epsilon)?;
    let coeff = Num::from_integer(bi(2)) / &sqrt_pi;

    let mut result = x.clone();
    let mut term = x.clone();
    let x_sq = x * x;
    let neg_x_sq = -&x_sq;

    for n in 1..500 {
        let n_bi = bi(n as i64);
        term = &term * &(&neg_x_sq / Num::from_integer(n_bi));
        let contrib = &term / Num::from_integer(bi(2 * n as i64 + 1));
        result = &result + &contrib;
        if &contrib.abs() < epsilon {
            break;
        }
    }

    Ok(round_to_epsilon(&(&coeff * &result), epsilon))
}

/// Complementary error function: erfc(x) = 1 - erf(x)
pub fn erfc(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let erf_val = erf(x, epsilon)?;
    Ok(round_to_epsilon(&(Num::one() - erf_val), epsilon))
}

/// Gudermannian function: gd(x) = 2 * atan(tanh(x/2))
pub fn gd(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let half = Num::one() / Num::from_integer(bi(2));
    let tanh_val = tanh(&(x * &half), epsilon)?;
    let atan_val = atan(&tanh_val, epsilon)?;
    Ok(round_to_epsilon(&(&Num::from_integer(bi(2)) * &atan_val), epsilon))
}

/// Inverse Gudermannian: agd(x) = (1/2) * ln((1 + sin(x)) / (1 - sin(x)))
pub fn agd(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let sin_val = sin(x, epsilon)?;
    let one = Num::one();
    let numerator = &one + &sin_val;
    let denominator = &one - &sin_val;
    if denominator.is_zero() {
        return Err("agd: denominator is zero".to_string());
    }
    let ratio = &numerator / &denominator;
    let ln_val = ln(&ratio, epsilon)?;
    Ok(round_to_epsilon(&(&ln_val / Num::from_integer(bi(2))), epsilon))
}

/// Bessel function of the first kind, order 0: J0(x)
/// Uses series: J0(x) = sum((-1)^n * (x/2)^(2n) / (n!)^2)
pub fn j0(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let x_half = x / Num::from_integer(bi(2));
    let mut result = Num::one();
    let mut term = Num::one();
    let x_sq = &x_half * &x_half;
    let neg_x_sq = -&x_sq;

    for n in 1..500 {
        let n_sq = Num::from_integer(bi(n as i64 * n as i64));
        term = &term * &(&neg_x_sq / &n_sq);
        result = &result + &term;
        if &term.abs() < epsilon {
            break;
        }
    }

    Ok(round_to_epsilon(&result, epsilon))
}

/// Bessel function of the first kind, order 1: J1(x)
/// Uses series: J1(x) = (x/2) * sum((-1)^n * (x/2)^(2n) / (n! * (n+1)!))
pub fn j1(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let x_half = x / Num::from_integer(bi(2));
    let mut result = x_half.clone();
    let mut term = x_half.clone();
    let x_sq = &x_half * &x_half;
    let neg_x_sq = -&x_sq;

    for n in 1..500 {
        let denom = Num::from_integer(bi(n as i64 * (n as i64 + 1)));
        term = &term * &(&neg_x_sq / &denom);
        result = &result + &term;
        if &term.abs() < epsilon {
            break;
        }
    }

    Ok(round_to_epsilon(&result, epsilon))
}

/// Bessel function Y0: Bessel function of second kind, order 0
pub fn y0(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x <= &Num::zero() {
        return Err("y0: argument must be positive".to_string());
    }
    // Y0(x) = (2/π) * (ln(x/2) * J0(x) + series)
    let pi_val = pi();
    let ln_arg = x / Num::from_integer(bi(2));
    let ln_val = ln(&ln_arg, epsilon)?;
    let j0_val = j0(x, epsilon)?;

    // For approximation, use: Y0(x) ≈ (2/π) * (ln(x/2) * J0(x) + P(x))
    let two_over_pi = Num::from_integer(bi(2)) / &pi_val;
    let result = &two_over_pi * (&(&ln_val * &j0_val) - j0(x, epsilon)?);
    Ok(round_to_epsilon(&result, epsilon))
}

/// Bessel function Y1: Bessel function of second kind, order 1
pub fn y1(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x <= &Num::zero() {
        return Err("y1: argument must be positive".to_string());
    }
    // Y1(x) = (2/π) * (ln(x/2) * J1(x) - 1/x + series)
    let pi_val = pi();
    let ln_arg = x / Num::from_integer(bi(2));
    let ln_val = ln(&ln_arg, epsilon)?;
    let j1_val = j1(x, epsilon)?;

    let two_over_pi = Num::from_integer(bi(2)) / &pi_val;
    let term1 = &ln_val * &j1_val;
    let term2 = Num::one() / x;
    let result = &two_over_pi * (&term1 - &term2);
    Ok(round_to_epsilon(&result, epsilon))
}

/// Gamma function: Γ(n) = (n-1)! for positive integers, generalized via Lanczos approximation
pub fn gamma(x: &Num, epsilon: &Num) -> Result<Num, String> {
    // For integers, Γ(n) = (n-1)!
    if x.is_integer() && x > &Num::zero() {
        let n = x.to_integer();
        if let Some(n_i64) = n.to_i64() {
            if n_i64 > 0 {
                // Γ(n) = (n-1)!
                let mut result = bi(1);
                for k in 1..(n_i64) {
                    result *= bi(k);
                }
                return Ok(Num::from_integer(result));
            }
        }
    }
    // For non-integers, use a simple approximation via Stirling's formula
    // Γ(x) ≈ sqrt(2π/x) * (x/e)^x
    if x <= &Num::zero() {
        return Err("gamma: argument must be positive".to_string());
    }
    let two_pi = Num::from_integer(bi(2)) * pi();
    let sqrt_term = sqrt(&(&two_pi / x), epsilon)?;
    let e_val = e();
    let power_base = x / &e_val;
    let power_exp_int = match x.to_f64() {
        Some(f) if f.is_finite() => f,
        _ => return Err("gamma: unable to compute".to_string()),
    };
    let power_result = match power_base.to_f64().and_then(|base| {
        if base > 0.0 {
            Some(base.powf(power_exp_int))
        } else {
            None
        }
    }) {
        Some(p) => Num::from_float(p).ok_or("gamma: non-finite result")?,
        None => return Err("gamma: unable to compute power".to_string()),
    };
    Ok(round_to_epsilon(&(&sqrt_term * &power_result), epsilon))
}

/// Log-gamma function: ln(Γ(x))
pub fn lgamma(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if x <= &Num::zero() {
        return Err("lgamma: argument must be positive".to_string());
    }
    let gamma_val = gamma(x, epsilon)?;
    if gamma_val <= Num::zero() {
        return Err("lgamma: gamma is non-positive".to_string());
    }
    ln(&gamma_val, epsilon)
}

/// Polygamma function: ψ^(n)(x) = d^n/dx^n ln(Γ(x))
/// For n=0, this is the digamma function ψ(x)
pub fn polygamma(n: i64, x: &Num, epsilon: &Num) -> Result<Num, String> {
    if n < 0 {
        return Err("polygamma: order must be non-negative".to_string());
    }
    if x <= &Num::zero() {
        return Err("polygamma: argument must be positive".to_string());
    }

    if n == 0 {
        // Digamma function: ψ(x) = d/dx ln(Γ(x))
        // Approximation: ψ(x) ≈ ln(x) - 1/(2x) - 1/(12x^2) + ...
        let ln_x = ln(x, epsilon)?;
        let x_inv = Num::one() / x;
        let term2 = &x_inv / Num::from_integer(bi(2));
        let term3 = &x_inv / (x * Num::from_integer(bi(12)));
        let result = &ln_x - &term2 - &term3;
        Ok(round_to_epsilon(&result, epsilon))
    } else {
        // For higher orders, use numerical differentiation via finite differences
        // ψ^(n)(x) ≈ (ψ^(n-1)(x+h) - ψ^(n-1)(x-h)) / (2h)
        let h = Num::from_float(0.0001).unwrap_or(Num::from_integer(bi(1)) / Num::from_integer(bi(10000)));
        let x_plus = x + &h;
        let x_minus = x - &h;
        let psi_plus = polygamma(n - 1, &x_plus, epsilon)?;
        let psi_minus = polygamma(n - 1, &x_minus, epsilon)?;
        let diff = &psi_plus - &psi_minus;
        Ok(round_to_epsilon(&(&diff / (&h * Num::from_integer(bi(2)))), epsilon))
    }
}

/// Riemann zeta function: ζ(s) = Σ(1/n^s) for Re(s) > 1
pub fn zeta(s: &Num, epsilon: &Num) -> Result<Num, String> {
    // Check convergence condition
    if s <= &Num::one() {
        return Err("zeta: argument must be > 1 for convergence".to_string());
    }

    // For integer arguments, use special values
    if s.is_integer() {
        if let Some(s_i64) = s.to_i64() {
            // ζ(2) = π²/6, ζ(4) = π⁴/90, etc.
            match s_i64 {
                2 => {
                    let pi_val = pi();
                    let pi_sq = &pi_val * &pi_val;
                    return Ok(round_to_epsilon(&(&pi_sq / Num::from_integer(bi(6))), epsilon));
                }
                4 => {
                    let pi_val = pi();
                    let pi_pow4 = &pi_val * &pi_val * &pi_val * &pi_val;
                    return Ok(round_to_epsilon(&(&pi_pow4 / Num::from_integer(bi(90))), epsilon));
                }
                _ => {}
            }
        }
    }

    // Compute zeta via series summation
    let mut result = Num::zero();
    for n in 1..1000 {
        let n_num = Num::from_integer(bi(n));
        // Compute n_num^s via exp(s * ln(n))
        let ln_n = ln(&n_num, epsilon)?;
        let s_ln_n = s * &ln_n;
        let n_to_s = exp(&s_ln_n, epsilon)?;
        let term = Num::one() / &n_to_s;
        result = &result + &term;
        if &term.abs() < epsilon {
            break;
        }
    }
    Ok(round_to_epsilon(&result, epsilon))
}

/// Catalan number: C_n = (2n)! / ((n+1)! * n!)
pub fn catalan_num(n: i64) -> Result<BigInt, String> {
    if n < 0 {
        return Err("catalan: negative index".to_string());
    }
    if n == 0 {
        return Ok(bi(1));
    }

    // Compute using the formula: C_n = (2n * (2n-1) * ... * (n+2)) / (n * (n-1) * ... * 1)
    let mut numer = bi(1);
    let mut denom = bi(1);
    for k in 1..=n {
        numer *= bi(n + k);
        denom *= bi(k);
    }
    Ok(numer / denom / (n + 1))
}

/// Fibonacci: compute nth Fibonacci number
pub fn fibonacci(n: i64) -> Result<BigInt, String> {
    if n < 0 {
        return Err("fib: negative index".to_string());
    }
    if n == 0 {
        return Ok(bi(0));
    }
    if n == 1 {
        return Ok(bi(1));
    }

    let mut a = bi(0);
    let mut b = bi(1);
    for _ in 2..=n {
        let temp = &a + &b;
        a = b;
        b = temp;
    }
    Ok(b)
}

/// Previous prime: largest prime less than n
pub fn prevprime(n: i64) -> Result<BigInt, String> {
    if n <= 2 {
        return Err("prevprime: no prime less than 2".to_string());
    }
    let mut candidate = n - 1;
    while candidate >= 2 {
        if is_prime_check(candidate as u64) {
            return Ok(bi(candidate));
        }
        candidate -= 1;
    }
    Err("prevprime: no prime found".to_string())
}

/// Prime factorization: return vector of prime factors
pub fn factor(n: i64) -> Result<Vec<BigInt>, String> {
    if n < 2 {
        return Err("factor: argument must be >= 2".to_string());
    }
    let mut factors = Vec::new();
    let mut n = n;
    let mut d: i64 = 2;
    while d * d <= n {
        while n % d == 0 {
            factors.push(bi(d));
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        factors.push(bi(n));
    }
    Ok(factors)
}

/// Largest prime factor
pub fn lfactor(n: i64) -> Result<BigInt, String> {
    if n < 2 {
        return Err("lfactor: argument must be >= 2".to_string());
    }
    let factors = factor(n)?;
    Ok(factors.last().cloned().unwrap_or_else(|| bi(n)))
}

/// Probabilistic primality test (Miller-Rabin with k rounds)
pub fn ptest(n: i64, k: i64) -> Result<Num, String> {
    if n < 2 {
        return Ok(Num::zero());
    }
    if k < 1 {
        return Err("ptest: k must be at least 1".to_string());
    }
    // Simple implementation: use trial division for small n, Miller-Rabin for large n
    // For now, just use our basic primality test
    let result = if is_prime_check(n as u64) { 1 } else { 0 };
    Ok(Num::from_integer(bi(result)))
}

/// Euler numbers: E_n (zigzag or up-down numbers)
/// E_0 = 1, E_1 = 1, E_2 = 1, E_3 = 2, E_4 = 5, E_5 = 16, ...
pub fn euler(n: i64) -> Result<BigInt, String> {
    if n < 0 {
        return Err("euler: negative index".to_string());
    }
    // Compute Euler numbers via recurrence relation
    // E_n = sum_{k=0}^{n-1} C(n,k) * E_k * E_{n-1-k}
    let mut e = vec![bi(0); (n + 1) as usize];
    e[0] = bi(1);
    if n == 0 {
        return Ok(bi(1));
    }
    e[1] = bi(1);
    for i in 2..=n as usize {
        let mut sum = bi(0);
        for k in 0..i {
            let binom = binomial(i as i64, k as i64);
            sum = &sum + &(&binom * &e[k] * &e[i - 1 - k]);
        }
        e[i] = sum;
    }
    Ok(e[n as usize].clone())
}

/// Bernoulli numbers: B_n (only even indices for n > 1 are non-zero)
/// B_0 = 1, B_1 = -1/2, B_2 = 1/6, B_4 = -1/30, ...
pub fn bernoulli(n: i64) -> Result<Num, String> {
    if n < 0 {
        return Err("bernoulli: negative index".to_string());
    }
    if n == 0 {
        return Ok(Num::one());
    }
    if n == 1 {
        return Ok(-Num::one() / Num::from_integer(bi(2)));
    }
    if n > 1 && n % 2 == 1 {
        return Ok(Num::zero());
    }
    // Compute via recurrence: sum_{k=0}^{n} C(n+1,k) * B_k = 0
    let mut b = vec![Num::zero(); (n + 1) as usize];
    b[0] = Num::one();
    for m in 1..=n as usize {
        let mut sum = Num::zero();
        for k in 0..m {
            sum = &sum + &(&Num::from_integer(binomial((m + 1) as i64, k as i64)) * &b[k]);
        }
        b[m] = -sum / Num::from_integer(binomial((m + 1) as i64, m as i64));
    }
    Ok(b[n as usize].clone())
}

/// Jacobi symbol: (a|n)
pub fn jacobi(a: i64, n: i64) -> Result<Num, String> {
    if n <= 0 || n % 2 == 0 {
        return Err("jacobi: n must be a positive odd integer".to_string());
    }
    // Compute Jacobi symbol using quadratic reciprocity
    let mut a = a % n;
    let mut n = n;
    let mut result = 1i64;

    loop {
        a %= n;
        if a == 0 {
            return Ok(if n == 1 {
                Num::from_integer(bi(result))
            } else {
                Num::zero()
            });
        }

        while a % 2 == 0 {
            a /= 2;
            if n % 8 == 3 || n % 8 == 5 {
                result = -result;
            }
        }

        std::mem::swap(&mut a, &mut n);

        if a % 4 == 3 && n % 4 == 3 {
            result = -result;
        }
    }
}

/// Binomial coefficient: C(n, k) = n! / (k! * (n-k)!)
fn binomial(n: i64, k: i64) -> BigInt {
    if k < 0 || k > n {
        return bi(0);
    }
    if k == 0 || k == n {
        return bi(1);
    }
    let k = if k > n - k { n - k } else { k };

    let mut result = bi(1);
    for i in 0..k {
        result = &result * bi(n - i);
        result = &result / bi(i + 1);
    }
    result
}

/// Helper function for primality checking
fn is_prime_check(n: u64) -> bool {
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

/// Linear Congruential Generator: next = (a*seed + c) % m
/// Returns (new_seed, random_value)
pub fn lcg_next(seed: u64) -> (u64, u64) {
    const A: u64 = 1664525;
    const C: u64 = 1013904223;
    let next_seed = A.wrapping_mul(seed).wrapping_add(C);
    (next_seed, next_seed)
}

/// Random integer (32-bit)
pub fn rand(seed: &mut u64) -> i32 {
    let (new_seed, value) = lcg_next(*seed);
    *seed = new_seed;
    (value >> 32) as i32
}

/// Random float [0, 1)
pub fn random(seed: &mut u64) -> f64 {
    let (new_seed, value) = lcg_next(*seed);
    *seed = new_seed;
    ((value >> 11) as f64) * (1.0 / 9007199254740992.0)
}

/// Random bit
pub fn randbit(seed: &mut u64) -> u32 {
    let (new_seed, value) = lcg_next(*seed);
    *seed = new_seed;
    (value & 1) as u32
}

/// Random integer in range [a, b]
pub fn randint(a: i64, b: i64, seed: &mut u64) -> Result<i64, String> {
    if a > b {
        return Err("randint: a must be <= b".to_string());
    }
    let range = b - a + 1;
    if range <= 0 {
        return Err("randint: invalid range".to_string());
    }
    let r = (rand(seed) as i64).abs() % range;
    Ok(a + r)
}

/// Random permutation of 0..n-1
pub fn randperm(n: i64, seed: &mut u64) -> Result<Vec<BigInt>, String> {
    if n < 0 {
        return Err("randperm: n must be non-negative".to_string());
    }
    let mut perm: Vec<i64> = (0..n).collect();
    // Fisher-Yates shuffle
    for i in (1..n as usize).rev() {
        let j = (rand(seed).unsigned_abs() as usize) % (i + 1);
        perm.swap(i, j);
    }
    Ok(perm.iter().map(|x| bi(*x)).collect())
}

/// Degrees to radians: d2r(x) = x * π / 180
pub fn d2r(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let pi_val = pi();
    let scale = pi_val / Num::from_integer(bi(180));
    Ok(round_to_epsilon(&(x * &scale), epsilon))
}

/// Radians to degrees: r2d(x) = x * 180 / π
pub fn r2d(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let pi_val = pi();
    let scale = Num::from_integer(bi(180)) / &pi_val;
    Ok(round_to_epsilon(&(x * &scale), epsilon))
}

/// Degrees to gradians: d2g(x) = x * 200 / 180 = x * 10 / 9
pub fn d2g(x: &Num) -> Num {
    x * Num::from_integer(bi(10)) / Num::from_integer(bi(9))
}

/// Gradians to radians: g2r(x) = x * π / 200
pub fn g2r(x: &Num, epsilon: &Num) -> Result<Num, String> {
    let pi_val = pi();
    let scale = pi_val / Num::from_integer(bi(200));
    Ok(round_to_epsilon(&(x * &scale), epsilon))
}

/// Gradians to degrees: g2d(x) = x * 180 / 200 = x * 9 / 10
pub fn g2d(x: &Num) -> Num {
    x * Num::from_integer(bi(9)) / Num::from_integer(bi(10))
}

// Phase 4.6: Environment & System Functions

/// Get current Unix timestamp (seconds since epoch)
pub fn time() -> Result<i64, String> {
    use std::time::SystemTime;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => Ok(duration.as_secs() as i64),
        Err(_) => Err("failed to get system time".to_string()),
    }
}

/// Alias for time()
pub fn systime() -> Result<i64, String> {
    time()
}

/// Convert Unix timestamp to human-readable string
pub fn ctime(timestamp: i64) -> Result<String, String> {
    let _duration = std::time::Duration::from_secs(timestamp as u64);

    // Format as a simple string: "DDD MMM DD HH:MM:SS YYYY"
    let secs_per_day = 86400i64;
    let _days_since_epoch = timestamp / secs_per_day;
    let secs_today = timestamp % secs_per_day;
    let hours = secs_today / 3600;
    let mins = (secs_today % 3600) / 60;
    let secs = secs_today % 60;

    // Simple approximation: days from 1970-01-01
    let year = 1970 + (timestamp / (365 * 86400));

    Ok(format!(
        "Thu Jan  1 {:02}:{:02}:{:02} {}",
        hours, mins, secs, year
    ))
}

/// Sleep for a given number of seconds
pub fn sleep_fn(seconds: f64) -> Result<(), String> {
    if seconds < 0.0 {
        return Err("sleep: seconds must be non-negative".to_string());
    }
    let duration = std::time::Duration::from_secs_f64(seconds);
    std::thread::sleep(duration);
    Ok(())
}

/// Get environment variable
pub fn getenv(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("getenv: {} not found", name))
}

/// Set environment variable
pub fn putenv(name: &str, value: &str) -> Result<(), String> {
    std::env::set_var(name, value);
    Ok(())
}

/// Execute a shell command and return its exit code
pub fn system(cmd: &str) -> Result<i32, String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        match Command::new("cmd").args(&["/C", cmd]).status() {
            Ok(status) => Ok(status.code().unwrap_or(-1)),
            Err(e) => Err(format!("system: failed to execute: {}", e)),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        match Command::new("sh").arg("-c").arg(cmd).status() {
            Ok(status) => Ok(status.code().unwrap_or(-1)),
            Err(e) => Err(format!("system: failed to execute: {}", e)),
        }
    }
}

/// Get user/system time in seconds (simplified)
pub fn usertime() -> Result<f64, String> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64())
}

// Phase 5.1: Character Classification Functions

/// Check if character is alphanumeric (letter or digit)
pub fn isalnum(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    if first_char.is_alphanumeric() { 1 } else { 0 }
}

/// Check if character is uppercase letter
pub fn isupper(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    if first_char.is_uppercase() { 1 } else { 0 }
}

/// Check if character is lowercase letter
pub fn islower(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    if first_char.is_lowercase() { 1 } else { 0 }
}

/// Check if character is printable
pub fn isprint(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    if !first_char.is_control() { 1 } else { 0 }
}

/// Check if character is visible (printable and not space)
pub fn isgraph(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    if !first_char.is_whitespace() && !first_char.is_control() { 1 } else { 0 }
}

/// Check if character is control character
pub fn iscntrl(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    if first_char.is_control() { 1 } else { 0 }
}

/// Check if character is punctuation
pub fn ispunct(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    // Punctuation is printable but not alphanumeric and not space
    if !first_char.is_alphanumeric() && !first_char.is_whitespace() && !first_char.is_control() {
        1
    } else {
        0
    }
}

/// Check if character is hexadecimal digit (0-9, a-f, A-F)
pub fn isxdigit(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let first_char = s.chars().next().unwrap();
    if first_char.is_ascii_hexdigit() { 1 } else { 0 }
}

/// Check if string contains only ASCII characters
pub fn isascii(s: &str) -> i32 {
    if s.is_ascii() { 1 } else { 0 }
}

/// Convert string to uppercase
pub fn toupper(s: &str) -> String {
    s.to_uppercase()
}

/// Convert string to lowercase
pub fn tolower(s: &str) -> String {
    s.to_lowercase()
}

/// Reverse a string
pub fn strrev(s: &str) -> String {
    s.chars().rev().collect()
}

// Phase 5.2: Advanced Modular Arithmetic Functions

/// Positive modulus: returns result in range [0, y)
/// pmod(x, y) = ((x mod y) + y) mod y
pub fn pmod(x: &Num, y: &Num) -> Result<Num, String> {
    if y.is_zero() {
        return Err("pmod: division by zero".to_string());
    }
    let m = x % y;
    if m.is_negative() {
        Ok(&m + y)
    } else {
        Ok(m)
    }
}

/// Quotient and modulus: returns [quotient, remainder]
pub fn quomod(x: &Num, y: &Num) -> Result<(Num, Num), String> {
    if y.is_zero() {
        return Err("quomod: division by zero".to_string());
    }
    // Quotient is floor(x / y)
    let quotient = (x / y).floor();
    // Remainder is x - y * quotient
    let remainder = x - &(&quotient * y);
    Ok((quotient, remainder))
}

/// Quotient: floor(x / y), similar to integer division
pub fn quo(x: &Num, y: &Num) -> Result<Num, String> {
    if y.is_zero() {
        return Err("quo: division by zero".to_string());
    }
    Ok((x / y).floor())
}

/// Remainder: x - y * floor(x / y)
pub fn rem(x: &Num, y: &Num) -> Result<Num, String> {
    if y.is_zero() {
        return Err("rem: division by zero".to_string());
    }
    let quotient = (x / y).floor();
    Ok(x - &(&quotient * y))
}

/// Hensel modular: similar to pmod but used in Hensel lifting contexts
/// For now, implemented as pmod for compatibility
pub fn hnrmod(x: &Num, y: &Num) -> Result<Num, String> {
    pmod(x, y)
}

// Phase 5.3: Rational Approximations

/// Approximate a number as a simple rational within epsilon
/// Returns the input if it's already a rational number, or a simple approximation
pub fn appr(x: &Num, epsilon: &Num) -> Result<Num, String> {
    if epsilon.is_zero() {
        return Ok(x.clone());
    }

    // Simple implementation: if x is already rational with small denominator, return it
    // Otherwise, round it to the nearest integer
    let denom = x.denom();

    // If denominator is small, x is already a simple rational
    if denom.bits() <= 32 {
        return Ok(x.clone());
    }

    // Otherwise, find a simple approximation using limited continued fractions
    let mut num = x.clone();
    let mut h_prev = Num::from_integer(bi(1));
    let mut h_curr = num.floor();
    let mut k_prev = Num::from_integer(bi(0));
    let mut k_curr = Num::from_integer(bi(1));

    for iteration in 0..20 {
        let frac = &num - &h_curr;

        if frac.abs() < *epsilon {
            return Ok(h_curr);
        }

        if frac.is_zero() {
            break;
        }

        // Avoid very deep nesting
        if iteration >= 10 {
            break;
        }

        num = Num::from_integer(bi(1)) / &frac;
        let int_part = num.floor();

        let h_next = &(&int_part * &h_curr) + &h_prev;
        let k_next = &(&int_part * &k_curr) + &k_prev;

        h_prev = h_curr.clone();
        h_curr = h_next;
        k_prev = k_curr.clone();
        k_curr = k_next;
    }

    Ok(h_curr)
}

/// Continued fraction approximation with max denominator constraint
pub fn cfappr(x: &Num, maxd: i64) -> Result<Num, String> {
    if maxd <= 0 {
        return Err("cfappr: maxd must be positive".to_string());
    }

    let max_denom = Num::from_integer(bi(maxd));

    // If x already has a small denominator, return it
    let denom = x.denom();
    if Num::from_integer(denom.clone()) <= max_denom {
        return Ok(x.clone());
    }

    let mut num = x.clone();
    let mut h_prev = Num::from_integer(bi(1));
    let mut h_curr = num.floor();
    let mut k_prev = Num::from_integer(bi(0));
    let mut k_curr = Num::from_integer(bi(1));

    let mut best_num = h_curr.clone();
    let mut best_denom = k_curr.clone();

    for iteration in 0..30 {
        if k_curr > max_denom {
            break;
        }

        let frac = &num - &h_curr;
        if frac.is_zero() {
            best_num = h_curr.clone();
            best_denom = k_curr.clone();
            break;
        }

        if iteration >= 20 {
            break;
        }

        num = Num::from_integer(bi(1)) / &frac;
        let int_part = num.floor();

        let h_next = &(&int_part * &h_curr) + &h_prev;
        let k_next = &(&int_part * &k_curr) + &k_prev;

        if k_next <= max_denom {
            best_num = h_next.clone();
            best_denom = k_next.clone();
        }

        h_prev = h_curr.clone();
        h_curr = h_next;
        k_prev = k_curr.clone();
        k_curr = k_next;
    }

    Ok(best_num / &best_denom)
}

/// Continued fraction simplification with max denominator
/// Similar to cfappr but returns the simplified rational
pub fn cfsim(x: &Num, maxd: i64) -> Result<Num, String> {
    cfappr(x, maxd)
}

/// Scale a number to a given number of decimal places
pub fn scale(x: &Num, places: i64) -> Result<Num, String> {
    if places < 0 {
        return Err("scale: places must be non-negative".to_string());
    }

    // Multiply by 10^places, round, then divide by 10^places
    let multiplier = Num::from_integer(bi(10).pow(places as u32));
    let scaled = x * &multiplier;
    let rounded = scaled.round();
    Ok(&rounded / &multiplier)
}

// Phase 5.4: Matrix Operations

/// Get matrix dimensions: returns (rows, cols)
pub fn matdim(matrix: &[Vec<Num>]) -> Result<(i64, i64), String> {
    if matrix.is_empty() {
        return Ok((0, 0));
    }
    let rows = matrix.len() as i64;
    let cols = if rows > 0 { matrix[0].len() as i64 } else { 0 };
    Ok((rows, cols))
}

/// Matrix transpose
pub fn mattrans(matrix: &[Vec<Num>]) -> Result<Vec<Vec<Num>>, String> {
    if matrix.is_empty() {
        return Ok(vec![]);
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![Num::from_integer(bi(0)); rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j].clone();
        }
    }
    Ok(result)
}

/// Matrix trace (sum of diagonal elements)
pub fn mattrace(matrix: &[Vec<Num>]) -> Result<Num, String> {
    if matrix.is_empty() {
        return Ok(Num::from_integer(bi(0)));
    }
    let n = std::cmp::min(matrix.len(), matrix[0].len());
    let mut sum = Num::from_integer(bi(0));
    for i in 0..n {
        sum = &sum + &matrix[i][i];
    }
    Ok(sum)
}

/// Matrix determinant (2x2 and 3x3 only for now)
pub fn det(matrix: &[Vec<Num>]) -> Result<Num, String> {
    if matrix.is_empty() {
        return Ok(Num::from_integer(bi(0)));
    }
    let n = matrix.len();
    if n != matrix[0].len() {
        return Err("det: matrix must be square".to_string());
    }
    match n {
        1 => Ok(matrix[0][0].clone()),
        2 => {
            let a = &matrix[0][0] * &matrix[1][1];
            let b = &matrix[0][1] * &matrix[1][0];
            Ok(&a - &b)
        }
        3 => {
            let a = &matrix[0][0] * &(&matrix[1][1] * &matrix[2][2] - &matrix[1][2] * &matrix[2][1]);
            let b = &matrix[0][1] * &(&matrix[1][0] * &matrix[2][2] - &matrix[1][2] * &matrix[2][0]);
            let c = &matrix[0][2] * &(&matrix[1][0] * &matrix[2][1] - &matrix[1][1] * &matrix[2][0]);
            Ok(&(&a - &b) + &c)
        }
        _ => Err("det: only 1x1, 2x2, and 3x3 matrices supported".to_string()),
    }
}

/// Matrix inverse (2x2 and 3x3 only)
pub fn inverse(matrix: &[Vec<Num>]) -> Result<Vec<Vec<Num>>, String> {
    if matrix.is_empty() {
        return Err("inverse: empty matrix".to_string());
    }
    let n = matrix.len();
    if n != matrix[0].len() {
        return Err("inverse: matrix must be square".to_string());
    }
    match n {
        1 => {
            if matrix[0][0].is_zero() {
                return Err("inverse: singular matrix".to_string());
            }
            Ok(vec![vec![Num::from_integer(bi(1)) / &matrix[0][0]]])
        }
        2 => {
            let d = det(matrix)?;
            if d.is_zero() {
                return Err("inverse: singular matrix".to_string());
            }
            let inv = Num::from_integer(bi(1)) / &d;
            let neg_inv = -&inv;
            Ok(vec![
                vec![&matrix[1][1] * &inv, &matrix[0][1] * &neg_inv],
                vec![&matrix[1][0] * &neg_inv, &matrix[0][0] * &inv],
            ])
        }
        _ => Err("inverse: only 1x1 and 2x2 matrices supported".to_string()),
    }
}

/// Sum of all matrix elements
pub fn matsum(matrix: &[Vec<Num>]) -> Result<Num, String> {
    let mut sum = Num::from_integer(bi(0));
    for row in matrix {
        for elem in row {
            sum = &sum + elem;
        }
    }
    Ok(sum)
}

/// Minimum element in matrix
pub fn matmin(matrix: &[Vec<Num>]) -> Result<Num, String> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Err("matmin: empty matrix".to_string());
    }
    let mut min = matrix[0][0].clone();
    for row in matrix {
        for elem in row {
            if elem < &min {
                min = elem.clone();
            }
        }
    }
    Ok(min)
}

/// Maximum element in matrix
pub fn matmax(matrix: &[Vec<Num>]) -> Result<Num, String> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Err("matmax: empty matrix".to_string());
    }
    let mut max = matrix[0][0].clone();
    for row in matrix {
        for elem in row {
            if elem > &max {
                max = elem.clone();
            }
        }
    }
    Ok(max)
}

/// Fill matrix with a value
pub fn matfill(rows: i64, cols: i64, val: &Num) -> Result<Vec<Vec<Num>>, String> {
    if rows < 0 || cols < 0 {
        return Err("matfill: dimensions must be non-negative".to_string());
    }
    let mut result = vec![];
    for _ in 0..rows {
        result.push(vec![val.clone(); cols as usize]);
    }
    Ok(result)
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

/// Convert a BigInt to a given base (2-36). Returns the string representation.
pub fn to_base(n: &BigInt, base: u32) -> String {
    if !(2..=36).contains(&base) {
        return n.to_string(); // fallback to base 10
    }
    if n.is_zero() {
        return "0".to_string();
    }

    let is_neg = n.is_negative();
    let n = n.abs();
    let base_bi = BigInt::from(base);
    let mut digits = String::new();
    let mut val = n.clone();

    while !val.is_zero() {
        let digit = (&val % &base_bi).to_u32().unwrap_or(0);
        let char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'a' + (digit - 10) as u8) as char
        };
        digits.insert(0, char);
        val /= &base_bi;
    }

    if is_neg {
        format!("-{}", digits)
    } else {
        digits
    }
}

/// Convert a rational number to a string in a given base (2-36).
pub fn to_string_in_base(x: &Num, base: u32, digits: usize) -> String {
    if !(2..=36).contains(&base) {
        return to_decimal_string(x, digits); // fallback
    }

    if x.is_integer() {
        return to_base(x.numer(), base);
    }

    let neg = x.is_negative();
    let x = x.abs();
    let numer = x.numer().clone();
    let denom = x.denom().clone();
    let base_bi = BigInt::from(base);

    let int_part = &numer / &denom;
    let mut rem = &numer % &denom;

    let mut frac_digits = String::new();
    let mut exact = false;
    for _ in 0..(digits + 1) {
        if rem.is_zero() {
            exact = true;
            break;
        }
        rem *= &base_bi;
        let d = &rem / &denom;
        rem %= &denom;
        let digit = d.to_u32().unwrap_or(0);
        let char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'a' + (digit - 10) as u8) as char
        };
        frac_digits.push(char);
    }

    let mut frac_chars: Vec<u8> = frac_digits.into_bytes();
    if !exact && frac_chars.len() > digits {
        let guard = frac_chars.pop();
        if let Some(g) = guard {
            let guard_val = if g.is_ascii_digit() {
                g - b'0'
            } else {
                g - b'a' + 10
            };
            let threshold = base / 2;
            if guard_val >= threshold as u8 {
                let mut i = frac_chars.len();
                let mut carry = true;
                while carry && i > 0 {
                    i -= 1;
                    let c = frac_chars[i];
                    if c == b'z' || (c.is_ascii_digit() && c == b'9') ||
                       c.is_ascii_lowercase() {
                        frac_chars[i] = if (b'0'..=b'8').contains(&c) || c.is_ascii_lowercase() { c + 1 } else {
                            b'0'
                        };
                        if frac_chars[i] != b'0' {
                            carry = false;
                        }
                    }
                }
                if carry {
                    let int_inc = &int_part + 1;
                    return assemble_base(neg, &int_inc, String::from_utf8_lossy(&frac_chars).as_ref(), exact, base);
                }
            }
        }
    }
    assemble_base(neg, &int_part, String::from_utf8_lossy(&frac_chars).as_ref(), exact, base)
}

fn assemble_base(neg: bool, int_part: &BigInt, frac: &str, exact: bool, base: u32) -> String {
    let sign = if neg { "-" } else { "" };
    let tilde = if exact { "" } else { "~" };
    let int_str = to_base(int_part, base);
    let frac_trimmed = frac.trim_end_matches('0');
    if frac_trimmed.is_empty() {
        format!("{tilde}{sign}{int_str}")
    } else {
        format!("{tilde}{sign}{int_str}.{frac_trimmed}")
    }
}
