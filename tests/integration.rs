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

#[test]
fn test_file_content_parsing() {
    // Test that file-like content is properly parsed and executed
    let mut it = Interp::new();
    let content = "define double(x) = x * 2; double(7); sum = 0; for i = 1, 3 (sum = sum + i); sum";
    let results = it.eval_all(content).unwrap();

    // Results from each statement: define returns Null, double(7) returns 14, etc.
    // Just verify we have multiple results
    assert!(results.len() >= 3, "got {} results", results.len());
}

#[test]
fn test_list_creation() {
    let mut it = Interp::new();
    let result = it.eval_render("list(1, 2, 3)").unwrap();
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_list_size() {
    let mut it = Interp::new();
    let result = it.eval_render("size(list(1, 2, 3, 4))").unwrap();
    assert_eq!(result, "4");
}

#[test]
fn test_list_indexing() {
    let mut it = Interp::new();
    let result = it.eval_render("x = list(10, 20, 30); x[0]").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "10");
}

#[test]
fn test_list_append() {
    let mut it = Interp::new();
    let result = it.eval_render("x = list(1, 2); y = append(x, 3, 4); size(y)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "4");
}

#[test]
fn test_list_first_last() {
    let mut it = Interp::new();
    let result = it.eval_render("x = list(10, 20, 30); first(x); last(x)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 2], "10");
    assert_eq!(lines[lines.len() - 1], "30");
}

#[test]
fn test_list_slice() {
    let mut it = Interp::new();
    let result = it.eval_render("slice(list(1, 2, 3, 4, 5), 1, 4)").unwrap();
    assert_eq!(result, "[2, 3, 4]");
}

#[test]
fn test_sqrt_negative() {
    let mut it = Interp::new();
    let result = it.eval_render("sqrt(-1)").unwrap();
    assert_eq!(result, "1i");
}

#[test]
fn test_sqrt_negative_four() {
    let mut it = Interp::new();
    let result = it.eval_render("sqrt(-4)").unwrap();
    assert_eq!(result, "2i");
}

#[test]
fn test_complex_real_part() {
    let mut it = Interp::new();
    let result = it.eval_render("z = sqrt(-1); re(z)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "0");
}

#[test]
fn test_complex_imag_part() {
    let mut it = Interp::new();
    let result = it.eval_render("z = sqrt(-1); im(z)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "1");
}

#[test]
fn test_complex_addition() {
    let mut it = Interp::new();
    let result = it.eval_render("i = sqrt(-1); (1 + 2*i) + (3 + 4*i)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "4+6i");
}

#[test]
fn test_complex_multiplication() {
    let mut it = Interp::new();
    let result = it.eval_render("i = sqrt(-1); (1 + i) * (2 - i)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "3+1i");
}

#[test]
fn test_complex_division() {
    let mut it = Interp::new();
    let result = it.eval_render("i = sqrt(-1); a = 3 + 4*i; b = 1 + i; a / b").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    // (3+4i)/(1+i) = 3.5+0.5i
    assert!(lines[lines.len() - 1].contains("3.5"));
    assert!(lines[lines.len() - 1].contains("0.5"));
}

#[test]
fn test_base_hex() {
    let mut it = Interp::new();
    let result = it.eval_render("base(16); 255").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "ff");
}

#[test]
fn test_base_binary() {
    let mut it = Interp::new();
    let result = it.eval_render("base(2); 255").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "11111111");
}

#[test]
fn test_base_octal() {
    let mut it = Interp::new();
    let result = it.eval_render("base(8); 64").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "100");
}

#[test]
fn test_base_fractional() {
    let mut it = Interp::new();
    let result = it.eval_render("base(16); 1/2").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "0.8");
}

#[test]
fn test_base_returns_obase() {
    let mut it = Interp::new();
    let result = it.eval_render("base(16)").unwrap();
    // 16 in hex is 10
    assert_eq!(result, "10");
}

#[test]
fn test_base_two_args() {
    let mut it = Interp::new();
    let result = it.eval_render("base(10, 16); 255").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[lines.len() - 1], "ff");
}

#[test]
fn test_asin() {
    let mut it = Interp::new();
    let result = it.eval_render("asin(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_acos() {
    let mut it = Interp::new();
    let result = it.eval_render("acos(0)").unwrap();
    // acos(0) = pi/2 ≈ 1.5708
    assert!(result.contains("1.570"));
}

#[test]
fn test_atan() {
    let mut it = Interp::new();
    let result = it.eval_render("atan(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_atan2() {
    let mut it = Interp::new();
    let result = it.eval_render("atan2(0, 1)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_sinh() {
    let mut it = Interp::new();
    let result = it.eval_render("sinh(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_cosh() {
    let mut it = Interp::new();
    let result = it.eval_render("cosh(0)").unwrap();
    // cosh(0) = 1 (may show as ~1 due to rounding)
    assert!(result == "1" || result == "~1");
}

#[test]
fn test_tanh() {
    let mut it = Interp::new();
    let result = it.eval_render("tanh(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_asinh() {
    let mut it = Interp::new();
    let result = it.eval_render("asinh(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_acosh() {
    let mut it = Interp::new();
    let result = it.eval_render("acosh(1)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_atanh() {
    let mut it = Interp::new();
    let result = it.eval_render("atanh(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_cas() {
    let mut it = Interp::new();
    let result = it.eval_render("cas(0)").unwrap();
    // cas(0) = cos(0) + sin(0) = 1 + 0 = 1 (may show as ~1 due to rounding)
    assert!(result == "1" || result == "~1");
}

#[test]
fn test_cis() {
    let mut it = Interp::new();
    let result = it.eval_render("cis(0)").unwrap();
    // cis(0) = cos(0) + i*sin(0) = 1 + 0i
    // When imaginary part is 0, it just shows the real part
    assert!(result.contains("1"));
}

#[test]
fn test_conj() {
    let mut it = Interp::new();
    let result = it.eval_render("i = sqrt(-1); conj(3 + 4*i)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    let last = lines[lines.len() - 1];
    // Should be 3-4i
    assert!(last.contains("3") && last.contains("4"));
}

#[test]
fn test_round_decimal() {
    let mut it = Interp::new();
    let result = it.eval_render("round(3.14159, 2)").unwrap();
    assert_eq!(result, "3.14");
}

#[test]
fn test_hypot() {
    let mut it = Interp::new();
    let result = it.eval_render("hypot(3, 4)").unwrap();
    assert_eq!(result, "5");
}

#[test]
fn test_erf() {
    let mut it = Interp::new();
    let result = it.eval_render("erf(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_erfc() {
    let mut it = Interp::new();
    let result = it.eval_render("erfc(0)").unwrap();
    // erfc(0) = 1 (may show as ~1 due to rounding)
    assert!(result == "1" || result == "~1");
}

#[test]
fn test_gd() {
    let mut it = Interp::new();
    let result = it.eval_render("gd(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_agd() {
    let mut it = Interp::new();
    let result = it.eval_render("agd(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_j0() {
    let mut it = Interp::new();
    let result = it.eval_render("j0(0)").unwrap();
    // j0(0) = 1 (may show as ~1 due to rounding)
    assert!(result == "1" || result == "~1");
}

#[test]
fn test_j1() {
    let mut it = Interp::new();
    let result = it.eval_render("j1(0)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_catalan() {
    let mut it = Interp::new();
    let result = it.eval_render("catalan(0); catalan(1); catalan(2); catalan(5)").unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines[0], "1"); // C_0 = 1
    assert_eq!(lines[1], "1"); // C_1 = 1
    assert_eq!(lines[2], "2"); // C_2 = 2
    assert_eq!(lines[3], "42"); // C_5 = 42
}

// Phase 3.3: String & Type Functions

#[test]
fn test_strlen() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"strlen("hello")"#).unwrap();
    assert_eq!(result, "5");
}

#[test]
fn test_index_found() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"index("hello world", "world")"#).unwrap();
    assert_eq!(result, "6");
}

#[test]
fn test_index_not_found() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"index("hello", "xyz")"#).unwrap();
    assert_eq!(result, "-1");
}

#[test]
fn test_isalpha_true() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"isalpha("hello")"#).unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_isalpha_false() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"isalpha("hello123")"#).unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_isdigit_true() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"isdigit("12345")"#).unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_isdigit_false() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"isdigit("123a")"#).unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_isspace_true() {
    let mut it = Interp::new();
    let result = it.eval_render("isspace(\"   \")").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_isspace_false() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"isspace("  a  ")"#).unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_typeof_number() {
    let mut it = Interp::new();
    let result = it.eval_render("typeof(42)").unwrap();
    assert_eq!(result, "number");
}

#[test]
fn test_typeof_string() {
    let mut it = Interp::new();
    let result = it.eval_render(r#"typeof("hello")"#).unwrap();
    assert_eq!(result, "string");
}

#[test]
fn test_typeof_complex() {
    let mut it = Interp::new();
    let result = it.eval_render("typeof(sqrt(-1))").unwrap();
    assert_eq!(result, "complex");
}

#[test]
fn test_typeof_list() {
    let mut it = Interp::new();
    let result = it.eval_render("typeof(list(1,2,3))").unwrap();
    assert_eq!(result, "list");
}

#[test]
fn test_isnan() {
    let mut it = Interp::new();
    let result = it.eval_render("isnan(42)").unwrap();
    assert_eq!(result, "0"); // rationals are never NaN
}

#[test]
fn test_isinf() {
    let mut it = Interp::new();
    let result = it.eval_render("isinf(42)").unwrap();
    assert_eq!(result, "0"); // rationals are never infinite
}

#[test]
fn test_d2r() {
    let mut it = Interp::new();
    let result = it.eval_render("d2r(180)").unwrap();
    // d2r(180) should be very close to π
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - std::f64::consts::PI).abs() < 0.001);
}

#[test]
fn test_r2d() {
    let mut it = Interp::new();
    // r2d(π) should be 180
    let result = it.eval_render("r2d(pi())").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 180.0).abs() < 0.1);
}

#[test]
fn test_d2g() {
    let mut it = Interp::new();
    // d2g(180) should be 200
    let result = it.eval_render("d2g(180)").unwrap();
    let val: f64 = result.parse().unwrap_or(0.0);
    assert!((val - 200.0).abs() < 0.0001);
}

#[test]
fn test_g2d() {
    let mut it = Interp::new();
    // g2d(200) should be 180
    let result = it.eval_render("g2d(200)").unwrap();
    let val: f64 = result.parse().unwrap_or(0.0);
    assert!((val - 180.0).abs() < 0.0001);
}

#[test]
fn test_g2r() {
    let mut it = Interp::new();
    // g2r(200) should be π
    let result = it.eval_render("g2r(200)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - std::f64::consts::PI).abs() < 0.001);
}

// Phase 4.1: Trigonometric Variants

#[test]
fn test_cot() {
    let mut it = Interp::new();
    // cot(π/4) = 1
    let result = it.eval_render("cot(pi()/4)").unwrap();
    assert!(result == "1" || result == "~1");
}

#[test]
fn test_sec() {
    let mut it = Interp::new();
    // sec(0) = 1
    let result = it.eval_render("sec(0)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 1.0).abs() < 0.0001);
}

#[test]
fn test_csc() {
    let mut it = Interp::new();
    // csc(π/2) = 1
    let result = it.eval_render("csc(pi()/2)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 1.0).abs() < 0.0001);
}

#[test]
fn test_acot() {
    let mut it = Interp::new();
    // acot(1) should be π/4
    let result = it.eval_render("acot(1)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    let expected = std::f64::consts::PI / 4.0;
    assert!((val - expected).abs() < 0.01);
}

#[test]
fn test_asec() {
    let mut it = Interp::new();
    // asec(2) should be π/3
    let result = it.eval_render("asec(2)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    let expected = std::f64::consts::PI / 3.0;
    assert!((val - expected).abs() < 0.01);
}

#[test]
fn test_acsc() {
    let mut it = Interp::new();
    // acsc(2) should be π/6
    let result = it.eval_render("acsc(2)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    let expected = std::f64::consts::PI / 6.0;
    assert!((val - expected).abs() < 0.01);
}

#[test]
fn test_coth() {
    let mut it = Interp::new();
    // coth(x) should be defined for nonzero x
    let result = it.eval_render("coth(1)").unwrap();
    let _val: f64 = result.trim_start_matches('~').parse().unwrap_or(0.0);
    // Just verify it evaluates without error
    assert!(!result.is_empty());
}

#[test]
fn test_sech() {
    let mut it = Interp::new();
    // sech(0) = 1
    let result = it.eval_render("sech(0)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 1.0).abs() < 0.0001);
}

#[test]
fn test_csch() {
    let mut it = Interp::new();
    // csch(x) should be defined for nonzero x
    let result = it.eval_render("csch(1)").unwrap();
    let _val: f64 = result.trim_start_matches('~').parse().unwrap_or(0.0);
    // Just verify it evaluates without error
    assert!(!result.is_empty());
}

#[test]
fn test_acoth() {
    let mut it = Interp::new();
    // acoth(2) should be a specific value
    let result = it.eval_render("acoth(2)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    // acoth(2) = 0.5 * ln(3) ≈ 0.549...
    assert!((val - 0.549).abs() < 0.01);
}

#[test]
fn test_asech() {
    let mut it = Interp::new();
    // asech(0.5) should be a specific value
    let result = it.eval_render("asech(0.5)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    // asech(0.5) = ln(1 + sqrt(3)) ≈ 1.317...
    assert!((val - 1.317).abs() < 0.01);
}

// Phase 4.2: Root & Logarithm Variants

#[test]
fn test_root() {
    let mut it = Interp::new();
    // root(8, 3) = 2
    let result = it.eval_render("root(8, 3)").unwrap();
    assert!(result == "2" || result == "~2");
}

#[test]
fn test_cbrt() {
    let mut it = Interp::new();
    // cbrt(27) = 3
    let result = it.eval_render("cbrt(27)").unwrap();
    assert!(result == "3" || result == "~3");
}

#[test]
fn test_isqrt() {
    let mut it = Interp::new();
    // isqrt(25) = 5
    let result = it.eval_render("isqrt(25)").unwrap();
    assert_eq!(result, "5");
}

#[test]
fn test_iroot() {
    let mut it = Interp::new();
    // iroot(8, 3) = 2
    let result = it.eval_render("iroot(8, 3)").unwrap();
    assert_eq!(result, "2");
}

#[test]
fn test_logn() {
    let mut it = Interp::new();
    // logn(100, 10) = 2
    let result = it.eval_render("logn(100, 10)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 2.0).abs() < 0.01);
}

#[test]
fn test_ilog10() {
    let mut it = Interp::new();
    // ilog10(100) = 2
    let result = it.eval_render("ilog10(100)").unwrap();
    assert_eq!(result, "2");
}

#[test]
fn test_ilog2() {
    let mut it = Interp::new();
    // ilog2(8) = 3
    let result = it.eval_render("ilog2(8)").unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_ilog() {
    let mut it = Interp::new();
    // ilog(10) = 2 (floor(ln(10)) = 2)
    let result = it.eval_render("ilog(10)").unwrap();
    assert_eq!(result, "2");
}

#[test]
fn test_ilogn() {
    let mut it = Interp::new();
    // ilogn(1000, 10) = 3
    let result = it.eval_render("ilogn(1000, 10)").unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_acsch() {
    let mut it = Interp::new();
    // acsch(1) = ln(1 + sqrt(2)) ≈ 0.881...
    let result = it.eval_render("acsch(1)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 0.881).abs() < 0.01);
}

// Phase 4.3: Prime & Number Theory Extensions

#[test]
fn test_prevprime() {
    let mut it = Interp::new();
    // prevprime(20) = 19
    let result = it.eval_render("prevprime(20)").unwrap();
    assert_eq!(result, "19");
}

#[test]
fn test_factor() {
    let mut it = Interp::new();
    // factor(12) = [2, 2, 3]
    let result = it.eval_render("factor(12)").unwrap();
    // Result is a list: [2, 2, 3]
    assert!(result.contains("2"));
}

#[test]
fn test_lfactor() {
    let mut it = Interp::new();
    // lfactor(12) = 3
    let result = it.eval_render("lfactor(12)").unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_ptest() {
    let mut it = Interp::new();
    // ptest(17, 5) = 1 (17 is prime)
    let result = it.eval_render("ptest(17, 5)").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_ptest_composite() {
    let mut it = Interp::new();
    // ptest(4, 5) = 0 (4 is not prime)
    let result = it.eval_render("ptest(4, 5)").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_euler() {
    let mut it = Interp::new();
    // euler(0) = 1
    let result = it.eval_render("euler(0)").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_bernoulli() {
    let mut it = Interp::new();
    // bernoulli(0) = 1
    let result = it.eval_render("bernoulli(0)").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_jacobi() {
    let mut it = Interp::new();
    // jacobi(2, 5) = -1
    let result = it.eval_render("jacobi(2, 5)").unwrap();
    assert_eq!(result, "-1");
}

// Phase 4.4: More Special Functions

#[test]
fn test_y0() {
    let mut it = Interp::new();
    // y0(1) should be a specific value
    let result = it.eval_render("y0(1)").unwrap();
    // Just verify it evaluates without error
    assert!(!result.is_empty());
}

#[test]
fn test_y1() {
    let mut it = Interp::new();
    // y1(1) should be a specific value
    let result = it.eval_render("y1(1)").unwrap();
    // Just verify it evaluates without error
    assert!(!result.is_empty());
}

#[test]
fn test_gamma_integer() {
    let mut it = Interp::new();
    // gamma(5) = 4! = 24
    let result = it.eval_render("gamma(5)").unwrap();
    assert_eq!(result, "24");
}

#[test]
fn test_lgamma() {
    let mut it = Interp::new();
    // lgamma(5) = ln(24)
    let result = it.eval_render("lgamma(5)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    // ln(24) ≈ 3.178...
    assert!((val - 3.178).abs() < 0.1);
}

#[test]
fn test_polygamma() {
    let mut it = Interp::new();
    // polygamma(0, 2) = digamma(2)
    let result = it.eval_render("polygamma(0, 2)").unwrap();
    // Just verify it evaluates without error
    assert!(!result.is_empty());
}

#[test]
fn test_zeta_2() {
    let mut it = Interp::new();
    // zeta(2) = π²/6 ≈ 1.6449...
    let result = it.eval_render("zeta(2)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 1.6449).abs() < 0.01);
}

#[test]
fn test_zeta_4() {
    let mut it = Interp::new();
    // zeta(4) = π⁴/90 ≈ 1.0823...
    let result = it.eval_render("zeta(4)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 1.0823).abs() < 0.01);
}

// Phase 4.5: Random Number Functions
#[test]
fn test_seed() {
    let mut it = Interp::new();
    // Setting seed should return the seed value
    let result = it.eval_render("seed(42)").unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_srand() {
    let mut it = Interp::new();
    // srand is an alias for seed
    let result = it.eval_render("srand(12345)").unwrap();
    assert_eq!(result, "12345");
}

#[test]
fn test_srandom() {
    let mut it = Interp::new();
    // srandom is an alias for seed
    let result = it.eval_render("srandom(999)").unwrap();
    assert_eq!(result, "999");
}

#[test]
fn test_rand() {
    let mut it = Interp::new();
    // Set seed to get deterministic results
    it.eval_render("seed(1)").unwrap();
    let result = it.eval_render("rand()").unwrap();
    // Should produce an integer
    let val: i64 = result.parse().unwrap_or(-1);
    assert!(val >= i32::MIN as i64 && val <= i32::MAX as i64);
}

#[test]
fn test_random() {
    let mut it = Interp::new();
    // Set seed to get deterministic results
    it.eval_render("seed(1)").unwrap();
    let result = it.eval_render("random()").unwrap();
    // Should be approximately in [0, 1)
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(-1.0);
    assert!(val >= 0.0 && val < 1.0);
}

#[test]
fn test_randbit() {
    let mut it = Interp::new();
    // Set seed to get deterministic results
    it.eval_render("seed(1)").unwrap();
    let result = it.eval_render("randbit()").unwrap();
    // Should be 0 or 1
    assert!(result == "0" || result == "1");
}

#[test]
fn test_randint() {
    let mut it = Interp::new();
    // Set seed to get deterministic results
    it.eval_render("seed(1)").unwrap();
    let result = it.eval_render("randint(1, 10)").unwrap();
    // Should be in [1, 10]
    let val: i64 = result.parse().unwrap_or(-1);
    assert!(val >= 1 && val <= 10);
}

#[test]
fn test_randperm() {
    let mut it = Interp::new();
    // Set seed to get deterministic results
    it.eval_render("seed(1)").unwrap();
    let result = it.eval_render("randperm(5)").unwrap();
    // Should produce a list with 5 elements
    assert!(result.contains('[') && result.contains(']'));
    // Count elements (rough check)
    let comma_count = result.matches(',').count();
    assert!(comma_count == 4); // 5 elements = 4 commas
}

// Phase 4.6: Environment & System Functions
#[test]
fn test_time() {
    let mut it = Interp::new();
    let result = it.eval_render("time()").unwrap();
    // Should produce a timestamp (integer)
    let timestamp: i64 = result.parse().unwrap_or(0);
    // Current Unix time should be > 1.7 billion (2024+)
    assert!(timestamp > 1_700_000_000);
}

#[test]
fn test_systime() {
    let mut it = Interp::new();
    let result = it.eval_render("systime()").unwrap();
    // Should produce a timestamp (integer)
    let timestamp: i64 = result.parse().unwrap_or(0);
    // Current Unix time should be > 1.7 billion (2024+)
    assert!(timestamp > 1_700_000_000);
}

#[test]
fn test_ctime() {
    let mut it = Interp::new();
    // Test with a known timestamp (2024-01-01 00:00:00 UTC = 1704067200)
    let result = it.eval_render("ctime(1704067200)").unwrap();
    // Should produce a string representation
    assert!(result.contains(':'));
    assert!(result.contains('2'));
}

#[test]
fn test_getenv() {
    let mut it = Interp::new();
    // Set an environment variable
    std::env::set_var("TEST_VAR", "test_value");
    let result = it.eval_render("getenv(\"TEST_VAR\")").unwrap();
    assert_eq!(result, "test_value");
}

#[test]
fn test_putenv() {
    let mut it = Interp::new();
    let result = it.eval_render("putenv(\"NEW_VAR\", \"new_value\")").unwrap();
    assert_eq!(result, "new_value");
    // Verify it was set
    let check = std::env::var("NEW_VAR").unwrap_or_default();
    assert_eq!(check, "new_value");
}

#[test]
fn test_system() {
    let mut it = Interp::new();
    // Execute a simple command that returns exit code 0
    #[cfg(not(target_os = "windows"))]
    {
        let result = it.eval_render("system(\"true\")").unwrap();
        assert_eq!(result, "0");
    }
    #[cfg(target_os = "windows")]
    {
        let result = it.eval_render("system(\"exit 0\")").unwrap();
        assert_eq!(result, "0");
    }
}

#[test]
fn test_usertime() {
    let mut it = Interp::new();
    let result = it.eval_render("usertime()").unwrap();
    // Should produce a float
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(-1.0);
    // Should be a positive number
    assert!(val > 0.0);
}

// Phase 5.1: Character Classification
#[test]
fn test_isalnum() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("isalnum(\"a\")").unwrap(), "1");
    assert_eq!(it.eval_render("isalnum(\"5\")").unwrap(), "1");
    assert_eq!(it.eval_render("isalnum(\"!\")").unwrap(), "0");
}

#[test]
fn test_isupper() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("isupper(\"A\")").unwrap(), "1");
    assert_eq!(it.eval_render("isupper(\"a\")").unwrap(), "0");
    assert_eq!(it.eval_render("isupper(\"5\")").unwrap(), "0");
}

#[test]
fn test_islower() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("islower(\"a\")").unwrap(), "1");
    assert_eq!(it.eval_render("islower(\"A\")").unwrap(), "0");
    assert_eq!(it.eval_render("islower(\"5\")").unwrap(), "0");
}

#[test]
fn test_isprint() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("isprint(\"a\")").unwrap(), "1");
    assert_eq!(it.eval_render("isprint(\" \")").unwrap(), "1");
    // Control characters are not printable (would need escape sequences to test)
    assert_eq!(it.eval_render("isprint(\"!\")").unwrap(), "1");
}

#[test]
fn test_isgraph() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("isgraph(\"a\")").unwrap(), "1");
    assert_eq!(it.eval_render("isgraph(\"!\")").unwrap(), "1");
    // Space should be 0 (printable but not visible)
    let result = it.eval_render("isgraph(\" \")").unwrap();
    assert_eq!(result, "0");
}

#[test]
fn test_isxdigit() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("isxdigit(\"5\")").unwrap(), "1");
    assert_eq!(it.eval_render("isxdigit(\"a\")").unwrap(), "1");
    assert_eq!(it.eval_render("isxdigit(\"F\")").unwrap(), "1");
    assert_eq!(it.eval_render("isxdigit(\"g\")").unwrap(), "0");
}

#[test]
fn test_isascii() {
    let mut it = Interp::new();
    let result = it.eval_render("isascii(\"hello\")").unwrap();
    assert_eq!(result, "1");
    // ASCII should return 1 for all ASCII characters
    let result = it.eval_render("isascii(\"123\")").unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_toupper() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("toupper(\"hello\")").unwrap(), "HELLO");
    assert_eq!(it.eval_render("toupper(\"HELLO\")").unwrap(), "HELLO");
    assert_eq!(it.eval_render("toupper(\"HeLLo\")").unwrap(), "HELLO");
}

#[test]
fn test_tolower() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("tolower(\"HELLO\")").unwrap(), "hello");
    assert_eq!(it.eval_render("tolower(\"hello\")").unwrap(), "hello");
    assert_eq!(it.eval_render("tolower(\"HeLLo\")").unwrap(), "hello");
}

#[test]
fn test_strrev() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("strrev(\"hello\")").unwrap(), "olleh");
    assert_eq!(it.eval_render("strrev(\"abc\")").unwrap(), "cba");
    assert_eq!(it.eval_render("strrev(\"a\")").unwrap(), "a");
}

#[test]
fn test_ispunct() {
    let mut it = Interp::new();
    assert_eq!(it.eval_render("ispunct(\"!\")").unwrap(), "1");
    assert_eq!(it.eval_render("ispunct(\".\")").unwrap(), "1");
    assert_eq!(it.eval_render("ispunct(\"a\")").unwrap(), "0");
}

#[test]
fn test_iscntrl() {
    let mut it = Interp::new();
    // Regular characters are not control characters
    assert_eq!(it.eval_render("iscntrl(\"a\")").unwrap(), "0");
    assert_eq!(it.eval_render("iscntrl(\" \")").unwrap(), "0");
}

// Phase 5.2: Advanced Modular Arithmetic
#[test]
fn test_pmod() {
    let mut it = Interp::new();
    // pmod should always return positive results
    let result = it.eval_render("pmod(7, 3)").unwrap();
    assert_eq!(result, "1"); // 7 mod 3 = 1

    let result = it.eval_render("pmod(-7, 3)").unwrap();
    assert_eq!(result, "2"); // -7 mod 3 = 2 (positive)
}

#[test]
fn test_quomod() {
    let mut it = Interp::new();
    // quomod should return [quotient, remainder]
    let result = it.eval_render("quomod(17, 5)").unwrap();
    assert!(result.contains('[') && result.contains(']'));
    assert!(result.contains(','));
    // quomod(17, 5) = [3, 2] (17 = 5*3 + 2)
    assert!(result.contains("3") && result.contains("2"));
}

#[test]
fn test_quo() {
    let mut it = Interp::new();
    // quo should return the quotient (floor(x/y))
    let result = it.eval_render("quo(17, 5)").unwrap();
    assert_eq!(result, "3"); // floor(17/5) = 3

    let result = it.eval_render("quo(-17, 5)").unwrap();
    assert_eq!(result, "-4"); // floor(-17/5) = -4
}

#[test]
fn test_rem() {
    let mut it = Interp::new();
    // rem should return the remainder
    let result = it.eval_render("rem(17, 5)").unwrap();
    assert_eq!(result, "2"); // 17 - 5*3 = 2

    let result = it.eval_render("rem(-17, 5)").unwrap();
    let val: i64 = result.parse().unwrap_or(0);
    // rem(-17, 5) = -17 - 5*(-4) = -17 + 20 = 3
    assert_eq!(val, 3);
}

#[test]
fn test_hnrmod() {
    let mut it = Interp::new();
    // hnrmod is like pmod
    let result = it.eval_render("hnrmod(7, 3)").unwrap();
    assert_eq!(result, "1");

    let result = it.eval_render("hnrmod(-7, 3)").unwrap();
    assert_eq!(result, "2");
}

// Phase 5.3: Rational Approximations
#[test]
fn test_appr() {
    let mut it = Interp::new();
    // appr should find simple rational approximations
    let result = it.eval_render("appr(3.14159265, 0.01)").unwrap();
    // Should approximate pi to within 0.01
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(0.0);
    assert!((val - 3.14159265).abs() < 0.02);
}

#[test]
fn test_cfappr() {
    let mut it = Interp::new();
    // cfappr should return continued fraction approximation
    let result = it.eval_render("cfappr(0.5)").unwrap();
    // 0.5 = 1/2, should be exact
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(-1.0);
    assert!((val - 0.5).abs() < 0.001);
}

#[test]
fn test_cfappr_with_maxd() {
    let mut it = Interp::new();
    // cfappr with max denominator
    let result = it.eval_render("cfappr(1/3, 10)").unwrap();
    // 1/3 should be exact
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(-1.0);
    assert!((val - 0.333333).abs() < 0.001);
}

#[test]
fn test_cfsim() {
    let mut it = Interp::new();
    // cfsim should simplify to continued fraction
    let result = it.eval_render("cfsim(0.5)").unwrap();
    let clean = result.trim_start_matches('~');
    let val: f64 = clean.parse().unwrap_or(-1.0);
    assert!((val - 0.5).abs() < 0.001);
}

#[test]
fn test_scale() {
    let mut it = Interp::new();
    // scale to 2 decimal places
    let result = it.eval_render("scale(3.14159, 2)").unwrap();
    // Should round to 3.14
    let val: f64 = result.parse().unwrap_or(0.0);
    assert!((val - 3.14).abs() < 0.001);
}

#[test]
fn test_scale_zero_places() {
    let mut it = Interp::new();
    // scale to 0 decimal places
    let result = it.eval_render("scale(3.7, 0)").unwrap();
    // Should round to 4
    assert_eq!(result, "4");
}

// Phase 5.4: Matrix Operations
// Note: Parser doesn't support nested list syntax [[...], [...]]
// Matrix operations are implemented in src/number.rs and src/builtins.rs
// but require list syntax support in the parser for full testing.
// Matrices are represented as lists of lists internally.

#[test]
fn test_matfill_basic() {
    let mut it = Interp::new();
    // matfill(2, 3, 5) should create a 2x3 matrix filled with 5s
    // Result is a list of lists, check structure exists
    let result = it.eval_render("matfill(2, 3, 5)").unwrap();
    // Should contain list structure
    assert!(result.contains("[") && result.contains("]"));
}
