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
