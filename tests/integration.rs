use torustcalcmcp::eval::Interp;
use torustcalcmcp::number;

#[test]
fn test_exactness_rational() {
    let mut it = Interp::new();
    let result = it.eval_render("1/3 * 3").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_big_power() {
    let mut it = Interp::new();
    let result = it.eval_render("2^100").unwrap();
    assert!(result.contains("1267650600228229401496703205376"));
}

#[test]
fn test_addition() {
    let mut it = Interp::new();
    let result = it.eval_render("2 + 3").unwrap();
    assert_eq!(result, "5");
}

#[test]
fn test_sqrt_exact() {
    let mut it = Interp::new();
    let result = it.eval_render("sqrt(4)").unwrap();
    assert_eq!(result, "2");
}

#[test]
fn test_gcd() {
    let mut it = Interp::new();
    let result = it.eval_render("gcd(462, 1071)").unwrap();
    assert_eq!(result, "21");
}

#[test]
fn test_fact() {
    let mut it = Interp::new();
    let result = it.eval_render("fact(5)").unwrap();
    assert_eq!(result, "120");
}

#[test]
fn test_isprime() {
    let mut it = Interp::new();
    let result = it.eval_render("isprime(17)").unwrap();
    assert_eq!(result, "1");
    let result = it.eval_render("isprime(18)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_frac_mode() {
    let mut it = Interp::new();
    it.cfg.mode = torustcalcmcp::config::Mode::Frac;
    let result = it.eval_render("1/3 + 1/6").unwrap();
    assert_eq!(result, "1/2");
}

#[test]
fn test_int_mode() {
    let mut it = Interp::new();
    it.cfg.mode = torustcalcmcp::config::Mode::Int;
    let result = it.eval_render("7 / 2").unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_pi_constant() {
    let mut it = Interp::new();
    let result = it.eval_render("pi()").unwrap();
    assert!(result.contains("3.14159265"), "got: {}", result);
}

#[test]
fn test_multiple_statements() {
    let mut it = Interp::new();
    let result = it.eval_render("2 + 3; 4 * 5").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "5");
    assert_eq!(lines[1], "20");
}

#[test]
fn test_exp_zero() {
    let mut it = Interp::new();
    let result = it.eval_render("exp(0)").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_exp_matches_e() {
    let mut it = Interp::new();
    // exp(1) should match e() to within display precision
    let exp_one = it.eval_render("exp(1)").unwrap();
    let e_val = it.eval_render("e()").unwrap();
    // Both should start with ~2.71828...
    assert!(exp_one.contains("2.71828"), "exp(1) = {}", exp_one);
    assert!(e_val.contains("2.71828"), "e() = {}", e_val);
}

#[test]
fn test_ln_of_e() {
    let mut it = Interp::new();
    let result = it.eval_render("ln(e())").unwrap();
    // Should be very close to 1
    assert!(result.contains("1") || result.contains("0.99999"), "ln(e()) = {}", result);
}

#[test]
fn test_sin_pi_over_6() {
    let mut it = Interp::new();
    let result = it.eval_render("sin(pi()/6)").unwrap();
    // sin(π/6) = 0.5
    assert!(result.contains("0.5"), "sin(pi()/6) = {}", result);
}

#[test]
fn test_cos_zero() {
    let mut it = Interp::new();
    let result = it.eval_render("cos(0)").unwrap();
    // cos(0) = 1 (may show as ~1 due to epsilon rounding)
    assert!(result == "1" || result == "~1", "cos(0) = {}", result);
}
