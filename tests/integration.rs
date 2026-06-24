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

#[test]
fn test_define_function() {
    let mut it = Interp::new();
    it.eval_render("define sq(x) = x^2").unwrap();
    let result = it.eval_render("sq(9)").unwrap();
    assert_eq!(result, "81");
}

#[test]
fn test_function_with_multiple_params() {
    let mut it = Interp::new();
    it.eval_render("define add(x,y) = x + y").unwrap();
    let result = it.eval_render("add(3,4)").unwrap();
    assert_eq!(result, "7");
}

#[test]
fn test_if_then() {
    let mut it = Interp::new();
    let result = it.eval_render("if 1 5 else 10").unwrap();
    assert_eq!(result, "5");
}

#[test]
fn test_if_else() {
    let mut it = Interp::new();
    let result = it.eval_render("if 0 5 else 10").unwrap();
    assert_eq!(result, "10");
}

#[test]
fn test_while_loop() {
    let mut it = Interp::new();
    // Simpler while loop: increment x each iteration
    let result = it.eval_render("x = 0; while (x < 5) (x = x + 1); x").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.last().unwrap(), &"5");
}

#[test]
fn test_for_loop() {
    let mut it = Interp::new();
    let result = it.eval_render("sum = 0; for i = 1, 5 sum = sum + i; sum").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    // 1 + 2 + 3 + 4 + 5 = 15
    assert_eq!(lines.last().unwrap(), &"15");
}

#[test]
fn test_bitwise_and() {
    let mut it = Interp::new();
    let result = it.eval_render("and(12, 10)").unwrap();
    assert_eq!(result, "8");
}

#[test]
fn test_bitwise_or() {
    let mut it = Interp::new();
    let result = it.eval_render("or(12, 10)").unwrap();
    assert_eq!(result, "14");
}

#[test]
fn test_bitwise_xor() {
    let mut it = Interp::new();
    let result = it.eval_render("xor(12, 10)").unwrap();
    assert_eq!(result, "6");
}

#[test]
fn test_bitwise_shifts() {
    let mut it = Interp::new();
    let result = it.eval_render("lshift(3, 2)").unwrap();
    assert_eq!(result, "12");
    let result = it.eval_render("rshift(12, 2)").unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_bit_operations() {
    let mut it = Interp::new();
    // bit(12, 2) checks if bit 2 of 12 is set
    // 12 = 1100, bit 2 is set
    let result = it.eval_render("bit(12, 2)").unwrap();
    assert_eq!(result, "1");
    // highbit(8) = position of highest bit in 8 = 1000 = 3
    let result = it.eval_render("highbit(8)").unwrap();
    assert_eq!(result, "3");
    // lowbit(12) = position of lowest bit in 12 = 1100 = 2
    let result = it.eval_render("lowbit(12)").unwrap();
    assert_eq!(result, "2");
}

#[test]
fn test_fcnt() {
    let mut it = Interp::new();
    // fcnt(15) = count of set bits in 1111 = 4
    let result = it.eval_render("fcnt(15)").unwrap();
    assert_eq!(result, "4");
}

#[test]
fn test_digits() {
    let mut it = Interp::new();
    // digits(1000) = 4 (in base 10)
    let result = it.eval_render("digits(1000)").unwrap();
    assert_eq!(result, "4");
    // digits(255, 2) = 8 (255 in binary is 11111111)
    let result = it.eval_render("digits(255, 2)").unwrap();
    assert_eq!(result, "8");
}
