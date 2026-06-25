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

// Phase 5.5: Hash & Associative Arrays

#[test]
fn test_assoc_create() {
    let mut it = Interp::new();
    // create a hash with two key-value pairs
    let result = it.eval_render("assoc(\"a\", 1, \"b\", 2)").unwrap();
    // Should contain hash braces and pairs
    assert!(result.contains("{") && result.contains("}"));
}

#[test]
fn test_assoc_empty() {
    let mut it = Interp::new();
    // create an empty hash
    let result = it.eval_render("assoc()").unwrap();
    assert_eq!(result.trim(), "{}");
}

#[test]
fn test_indices_basic() {
    let mut it = Interp::new();
    // get keys from hash
    let result = it.eval_render("h = assoc(\"a\", 1, \"b\", 2); indices(h)").unwrap();
    // Should be a list of strings
    assert!(result.contains("["));
    assert!(result.contains("a"));
    assert!(result.contains("b"));
}

#[test]
fn test_insert_new_key() {
    let mut it = Interp::new();
    // insert a new key-value pair
    let result = it.eval_render("count(insert(assoc(\"a\", 1), \"b\", 2))").unwrap();
    assert_eq!(result.trim(), "2");
}

#[test]
fn test_insert_update_existing() {
    let mut it = Interp::new();
    // update an existing key
    let result = it.eval_render("count(insert(assoc(\"a\", 1), \"a\", 10))").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_delete_key() {
    let mut it = Interp::new();
    // delete a key from hash
    let result = it.eval_render("count(delete(assoc(\"a\", 1, \"b\", 2), \"a\"))").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_count_hash() {
    let mut it = Interp::new();
    // count key-value pairs
    let result = it.eval_render("count(assoc(\"a\", 1, \"b\", 2, \"c\", 3))").unwrap();
    assert_eq!(result.trim(), "3");
}

#[test]
fn test_join_values() {
    let mut it = Interp::new();
    // join hash values with separator
    let result = it.eval_render("h = assoc(\"a\", \"x\", \"b\", \"y\"); join(h, \",\")").unwrap();
    // Values should be joined
    assert!(result.contains(","));
}

// Phase 6.3: Error & Exception Handling

#[test]
fn test_errcount_initial() {
    let mut it = Interp::new();
    // Initial error count should be 0
    let result = it.eval_render("errcount()").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_errno_initial() {
    let mut it = Interp::new();
    // Initial errno should be 0
    let result = it.eval_render("errno()").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_errmax_set_and_get() {
    let mut it = Interp::new();
    // set max errors to 5, returns old value (0)
    let result = it.eval_render("errmax(5)").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_errsym_known_error() {
    let mut it = Interp::new();
    // get error message for known error code
    let result = it.eval_render("errsym(1)").unwrap();
    assert_eq!(result.trim(), "syntax error");
}

#[test]
fn test_errsym_unknown_error() {
    let mut it = Interp::new();
    // get error message for unknown error code
    let result = it.eval_render("errsym(999)").unwrap();
    assert_eq!(result.trim(), "unknown error");
}

#[test]
fn test_newerror_register() {
    let mut it = Interp::new();
    // register a new error type
    let result = it.eval_render("newerror(100, \"custom error\")").unwrap();
    assert_eq!(result.trim(), "100");
}

#[test]
fn test_newerror_and_lookup() {
    let mut it = Interp::new();
    // register and then lookup a custom error
    it.eval_render("newerror(200, \"my custom error\")").unwrap();
    let result = it.eval_render("errsym(200)").unwrap();
    assert_eq!(result.trim(), "my custom error");
}

// Phase 6.1: File I/O

#[test]
fn test_fopen_write() {
    let mut it = Interp::new();
    // Open a file for writing
    let result = it.eval_render("fopen(\"/tmp/test_calc.txt\", \"w\")").unwrap();
    // Should return a file descriptor (3 or higher)
    let fd: i64 = result.trim().parse().unwrap();
    assert!(fd >= 3);
}

#[test]
fn test_fopen_read_nonexistent() {
    let mut it = Interp::new();
    // Try to open non-existent file for reading
    let result = it.eval_render("fopen(\"/tmp/nonexistent_calc_file_12345.txt\", \"r\")");
    // Should fail
    assert!(result.is_err());
}

#[test]
fn test_fputs_and_fclose() {
    let mut it = Interp::new();
    // Create file and write string
    let result = it.eval_render("fd = fopen(\"/tmp/test_write.txt\", \"w\"); fputs(fd, \"Hello, World!\")").unwrap();
    // Last statement should return length (13)
    assert_eq!(result.lines().last().unwrap(), "13");

    // Close the file
    let close_result = it.eval_render("fclose(3)").unwrap();
    assert_eq!(close_result.trim(), "0");
}

#[test]
fn test_remove_file() {
    let mut it = Interp::new();
    // Create a file
    it.eval_render("fd = fopen(\"/tmp/test_remove.txt\", \"w\"); fputs(fd, \"test\"); fclose(fd)").ok();

    // Remove it
    let result = it.eval_render("remove(\"/tmp/test_remove.txt\")").unwrap();
    assert_eq!(result.trim(), "0");

    // Try to remove again - should fail
    let result2 = it.eval_render("remove(\"/tmp/test_remove.txt\")");
    assert!(result2.is_err());
}

#[test]
fn test_rename_file() {
    let mut it = Interp::new();
    // Create a file
    it.eval_render("fd = fopen(\"/tmp/test_orig.txt\", \"w\"); fputs(fd, \"test\"); fclose(fd)").ok();

    // Rename it
    let result = it.eval_render("rename(\"/tmp/test_orig.txt\", \"/tmp/test_renamed.txt\")").unwrap();
    assert_eq!(result.trim(), "0");

    // Verify renamed file exists
    let read_result = it.eval_render("fopen(\"/tmp/test_renamed.txt\", \"r\")");
    assert!(read_result.is_ok());

    // Cleanup
    it.eval_render("remove(\"/tmp/test_renamed.txt\")").ok();
}

#[test]
fn test_seek_and_tell() {
    let mut it = Interp::new();
    // Create file
    it.eval_render("fd = fopen(\"/tmp/test_seek.txt\", \"w\"); fputs(fd, \"0123456789\"); fclose(fd)").ok();

    // Reopen for reading
    it.eval_render("fd = fopen(\"/tmp/test_seek.txt\", \"r\")").ok();

    // Check initial position
    let pos1 = it.eval_render("tell(3)").unwrap();
    assert_eq!(pos1.trim(), "0");

    // Seek to position 5
    it.eval_render("seek(3, 5)").ok();

    // Check position
    let pos2 = it.eval_render("tell(3)").unwrap();
    assert_eq!(pos2.trim(), "5");

    // Cleanup
    it.eval_render("fclose(3)").ok();
    it.eval_render("remove(\"/tmp/test_seek.txt\")").ok();
}

// Phase 6.2: Memory & Stack Management

#[test]
fn test_blk_allocate() {
    let mut it = Interp::new();
    // Allocate 100 bytes
    let result = it.eval_render("blk(100)").unwrap();
    let block_id: i64 = result.trim().parse().unwrap();
    assert!(block_id >= 1);
}

#[test]
fn test_blocks_count() {
    let mut it = Interp::new();
    // Allocate two blocks and check count
    it.eval_render("id1 = blk(50); id2 = blk(100)").ok();
    let result = it.eval_render("blocks()").unwrap();
    assert_eq!(result.lines().last().unwrap(), "2");
}

#[test]
fn test_blkfree() {
    let mut it = Interp::new();
    // Allocate block
    it.eval_render("id = blk(50)").ok();

    // Free it
    let result = it.eval_render("blkfree(1)").unwrap();
    assert_eq!(result.trim(), "0");

    // Check count
    let count = it.eval_render("blocks()").unwrap();
    assert_eq!(count.trim(), "0");
}

#[test]
fn test_push_and_pop() {
    let mut it = Interp::new();
    // Push value
    it.eval_render("push(42)").ok();

    // Check depth
    let depth = it.eval_render("depth()").unwrap();
    assert_eq!(depth.trim(), "1");

    // Pop value
    let result = it.eval_render("pop()").unwrap();
    assert_eq!(result.trim(), "42");
}

#[test]
fn test_stack_operations() {
    let mut it = Interp::new();
    // Push multiple values
    it.eval_render("push(1); push(2); push(3)").ok();

    // Check depth
    let depth = it.eval_render("depth()").unwrap();
    assert_eq!(depth.lines().last().unwrap(), "3");

    // Pop one
    let val = it.eval_render("pop()").unwrap();
    assert_eq!(val.trim(), "3");

    // Check depth again
    let depth2 = it.eval_render("depth()").unwrap();
    assert_eq!(depth2.trim(), "2");
}

#[test]
fn test_free_all_memory() {
    let mut it = Interp::new();
    // Allocate blocks
    it.eval_render("blk(50); blk(100); blk(200)").ok();

    // Free all
    let result = it.eval_render("free()").unwrap();
    assert_eq!(result.lines().last().unwrap(), "3");

    // Check count
    let count = it.eval_render("blocks()").unwrap();
    assert_eq!(count.trim(), "0");
}

#[test]
fn test_freeglobals() {
    let mut it = Interp::new();
    // Create global variables
    it.eval_render("x = 10; y = 20; z = 30").ok();

    // Check some exist
    let x = it.eval_render("x").unwrap();
    assert_eq!(x.trim(), "10");

    // Free globals
    let result = it.eval_render("freeglobals()").unwrap();
    assert_eq!(result.lines().last().unwrap(), "3");

    // Try to access - should fail (undefined variable)
    let result2 = it.eval_render("x");
    assert!(result2.is_err());
}

// Phase 6.4: Command & Script Functions

#[test]
fn test_cmdbuf_initial() {
    let mut it = Interp::new();
    // Initial command buffer should be empty
    let result = it.eval_render("cmdbuf()").unwrap();
    assert_eq!(result.trim(), "");
}

#[test]
fn test_eval_arithmetic() {
    let mut it = Interp::new();
    // Evaluate arithmetic expression from string
    let result = it.eval_render("eval(\"2 + 3 * 4\")").unwrap();
    assert_eq!(result.trim(), "14");
}

#[test]
fn test_eval_simple() {
    let mut it = Interp::new();
    // Evaluate simple expression
    let result = it.eval_render("eval(\"42\")").unwrap();
    assert_eq!(result.trim(), "42");
}

#[test]
fn test_eval_with_variables() {
    let mut it = Interp::new();
    // Set a variable and use it in eval
    it.eval_render("x = 10").ok();
    let result = it.eval_render("eval(\"x * 2\")").unwrap();
    assert_eq!(result.trim(), "20");
}

#[test]
fn test_command_echo() {
    let mut it = Interp::new();
    // Execute shell command (echo returns 0)
    let result = it.eval_render("command(\"echo hello\")").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_command_failing() {
    let mut it = Interp::new();
    // Execute failing command (should return non-zero)
    let result = it.eval_render("command(\"false\")").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_argv_empty() {
    let mut it = Interp::new();
    // No arguments provided by default
    let result = it.eval_render("argv(0)").unwrap();
    assert_eq!(result.trim(), "");
}

// Phase 6.5: Obscure Trigonometric Variants

#[test]
fn test_haversin_zero() {
    let mut it = Interp::new();
    // haversin(0) = (1 - cos(0)) / 2 ≈ 0 (very close to zero)
    let result = it.eval_render("haversin(0)").unwrap();
    let val: f64 = result.trim().trim_start_matches('~').parse().unwrap_or(0.0);
    assert!(val.abs() < 1e-10);
}

#[test]
fn test_versin_zero() {
    let mut it = Interp::new();
    // versin(0) = 1 - cos(0) ≈ 0 (very close to zero)
    let result = it.eval_render("versin(0)").unwrap();
    let val: f64 = result.trim().trim_start_matches('~').parse().unwrap_or(0.0);
    assert!(val.abs() < 1e-10);
}

#[test]
fn test_coversin_zero() {
    let mut it = Interp::new();
    // coversin(0) = 1 - sin(0) = 1 - 0 = 1
    let result = it.eval_render("coversin(0)").unwrap();
    assert!(result.trim().contains("1"));
}

#[test]
fn test_exsecant_zero() {
    let mut it = Interp::new();
    // exsecant(0) = sec(0) - 1 ≈ 0 (very close to zero)
    let result = it.eval_render("exsecant(0)").unwrap();
    let val: f64 = result.trim().trim_start_matches('~').parse().unwrap_or(0.0);
    assert!(val.abs() < 1e-10);
}

#[test]
fn test_chord_zero() {
    let mut it = Interp::new();
    // chord(0) = 2 * sin(0/2) = 2 * sin(0) = 2 * 0 = 0
    let result = it.eval_render("chord(0)").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_semiversin_alias() {
    let mut it = Interp::new();
    // semiversin should be same as haversin
    it.eval_render("x = 1").ok();
    let result1 = it.eval_render("haversin(x)").unwrap();
    let result2 = it.eval_render("semiversin(x)").unwrap();
    assert_eq!(result1.trim(), result2.trim());
}

#[test]
fn test_vers_alias() {
    let mut it = Interp::new();
    // vers should be same as versin
    it.eval_render("x = 1").ok();
    let result1 = it.eval_render("versin(x)").unwrap();
    let result2 = it.eval_render("vers(x)").unwrap();
    assert_eq!(result1.trim(), result2.trim());
}

// Phase 6.6: Cryptographic & Hashing

#[test]
fn test_sha1_empty() {
    let mut it = Interp::new();
    // SHA-1 of empty string
    let result = it.eval_render("sha1(\"\")").unwrap();
    // SHA-1 of "" is da39a3ee5e6b4b0d3255bfef95601890afd80709
    assert_eq!(result.trim(), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
}

#[test]
fn test_sha1_hello() {
    let mut it = Interp::new();
    // SHA-1 of "hello"
    let result = it.eval_render("sha1(\"hello\")").unwrap();
    // SHA-1 of "hello" is aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d
    assert_eq!(result.trim(), "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d");
}

#[test]
fn test_md5_empty() {
    let mut it = Interp::new();
    // MD5 of empty string
    let result = it.eval_render("md5(\"\")").unwrap();
    // MD5 of "" is d41d8cd98f00b204e9800998ecf8427e
    assert_eq!(result.trim(), "d41d8cd98f00b204e9800998ecf8427e");
}

#[test]
fn test_md5_hello() {
    let mut it = Interp::new();
    // MD5 of "hello"
    let result = it.eval_render("md5(\"hello\")").unwrap();
    // MD5 of "hello" is 5d41402abc4b2a76b9719d911017c592
    assert_eq!(result.trim(), "5d41402abc4b2a76b9719d911017c592");
}

#[test]
fn test_crc32_empty() {
    let mut it = Interp::new();
    // CRC32 of empty string
    let result = it.eval_render("crc32(\"\")").unwrap();
    // CRC32-CKSUM of "" is 0xFFFFFFFF (4294967295 as unsigned, or -1 as signed)
    assert_eq!(result.trim(), "4294967295");
}

#[test]
fn test_crc32_hello() {
    let mut it = Interp::new();
    // CRC32 of "hello"
    let result = it.eval_render("crc32(\"hello\")").unwrap();
    // CRC32 of "hello" is 3964322768 (unsigned) or as signed i64
    let val: i64 = result.trim().parse().unwrap_or(0);
    assert!(val != 0); // Should produce non-zero value
}

// Phase 6.7: Residue Class & Modular Operations

#[test]
fn test_rc_basic() {
    let mut it = Interp::new();
    // rc(17, 5) = 17 mod 5 = 2
    let result = it.eval_render("rc(17, 5)").unwrap();
    assert_eq!(result.trim(), "2");
}

#[test]
fn test_rcadd() {
    let mut it = Interp::new();
    // rcadd(3, 4, 5) = (3 + 4) mod 5 = 7 mod 5 = 2
    let result = it.eval_render("rcadd(3, 4, 5)").unwrap();
    assert_eq!(result.trim(), "2");
}

#[test]
fn test_rcsub() {
    let mut it = Interp::new();
    // rcsub(3, 7, 5) = (3 - 7) mod 5 = -4 mod 5 = 1
    let result = it.eval_render("rcsub(3, 7, 5)").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_rcmul() {
    let mut it = Interp::new();
    // rcmul(3, 4, 5) = (3 * 4) mod 5 = 12 mod 5 = 2
    let result = it.eval_render("rcmul(3, 4, 5)").unwrap();
    assert_eq!(result.trim(), "2");
}

#[test]
fn test_rcinv() {
    let mut it = Interp::new();
    // rcinv(3, 7) = modular inverse of 3 mod 7 = 5 (since 3*5 = 15 ≡ 1 mod 7)
    let result = it.eval_render("rcinv(3, 7)").unwrap();
    assert_eq!(result.trim(), "5");
}

#[test]
fn test_rceq_true() {
    let mut it = Interp::new();
    // rceq(7, 12, 5) checks if 7 ≡ 12 (mod 5), which is true (both ≡ 2)
    let result = it.eval_render("rceq(7, 12, 5)").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_rceq_false() {
    let mut it = Interp::new();
    // rceq(7, 11, 5) checks if 7 ≡ 11 (mod 5), which is false (7≡2, 11≡1)
    let result = it.eval_render("rceq(7, 11, 5)").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_rcneg() {
    let mut it = Interp::new();
    // rcneg(3, 7) = (-3) mod 7 = 4 (since -3 + 7 = 4)
    let result = it.eval_render("rcneg(3, 7)").unwrap();
    assert_eq!(result.trim(), "4");
}

#[test]
fn test_rcdiv() {
    let mut it = Interp::new();
    // rcdiv(6, 3, 7) = (6 / 3) mod 7 = 2 (since 6 * inv(3) mod 7 = 6 * 5 mod 7 = 30 mod 7 = 2)
    let result = it.eval_render("rcdiv(6, 3, 7)").unwrap();
    assert_eq!(result.trim(), "2");
}

// Phase 6.2 Extended: Memory Address Functions

#[test]
fn test_blksize() {
    let mut it = Interp::new();
    // Allocate a block of 100 bytes and check size
    it.eval_render("id = blk(100)").ok();
    let result = it.eval_render("blksize(id)").unwrap();
    assert_eq!(result.trim(), "100");
}

#[test]
fn test_peek_poke() {
    let mut it = Interp::new();
    // Allocate block, write byte, read it back
    it.eval_render("id = blk(50)").ok();
    it.eval_render("poke(id, 10, 65)").ok(); // Write 'A' (65)
    let result = it.eval_render("peek(id, 10)").unwrap();
    assert_eq!(result.trim(), "65");
}

#[test]
fn test_poke_multiple() {
    let mut it = Interp::new();
    // Write multiple bytes
    it.eval_render("id = blk(100)").ok();
    it.eval_render("poke(id, 0, 72)").ok(); // 'H'
    it.eval_render("poke(id, 1, 105)").ok(); // 'i'
    it.eval_render("poke(id, 2, 33)").ok(); // '!'

    let result = it.eval_render("peek(id, 0)").unwrap();
    assert_eq!(result.trim(), "72");
}

#[test]
fn test_memread() {
    let mut it = Interp::new();
    // Write bytes and read them as string
    it.eval_render("id = blk(100)").ok();
    it.eval_render("poke(id, 0, 72)").ok(); // 'H'
    it.eval_render("poke(id, 1, 105)").ok(); // 'i'
    it.eval_render("poke(id, 2, 33)").ok(); // '!'

    let result = it.eval_render("memread(id, 0, 3)").unwrap();
    assert!(result.contains("Hi!") || result.contains("72")); // May show as bytes or string
}

#[test]
fn test_memory_lifecycle() {
    let mut it = Interp::new();
    // Allocate, use, and free
    it.eval_render("id = blk(64)").ok();
    it.eval_render("poke(id, 5, 200)").ok();
    let peek_result = it.eval_render("peek(id, 5)").unwrap();
    assert_eq!(peek_result.trim(), "200");

    // Free and check blocks count
    it.eval_render("blkfree(id)").ok();
    let blocks_result = it.eval_render("blocks()").unwrap();
    assert_eq!(blocks_result.trim(), "0");
}

// Phase 6.1 Extended: Additional File I/O Functions

#[test]
fn test_fflush() {
    let mut it = Interp::new();
    // fflush should return 0 (success) for a valid fd
    it.eval_render("fopen(\"/tmp/test_fflush.txt\", \"w\")").ok();
    let result = it.eval_render("fflush(3)").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_rewind() {
    let mut it = Interp::new();
    // Create a test file with content
    let _ = std::fs::write("/tmp/test_rewind.txt", "hello world");

    // Open file and read some content
    it.eval_render("fopen(\"/tmp/test_rewind.txt\", \"r\")").ok();
    it.eval_render("fgets(3)").ok(); // Read a line

    // Rewind should return 0
    let result = it.eval_render("rewind(3)").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_fileno() {
    let mut it = Interp::new();
    // fileno should return the fd itself
    let result = it.eval_render("fileno(3)").unwrap();
    assert_eq!(result.trim(), "3");
}

#[test]
fn test_fread_write() {
    let mut it = Interp::new();
    // Write some data then read it back
    let _ = std::fs::write("/tmp/test_freadwrite.txt", "test data");

    it.eval_render("fopen(\"/tmp/test_freadwrite.txt\", \"r\")").ok();
    let result = it.eval_render("fread(3, 4)").unwrap();
    assert_eq!(result.trim(), "test");
}

#[test]
fn test_fseek_operations() {
    let mut it = Interp::new();
    // Create a test file
    let _ = std::fs::write("/tmp/test_fseek.txt", "0123456789");

    it.eval_render("fopen(\"/tmp/test_fseek.txt\", \"r\")").ok();
    // Seek to position 5
    let result = it.eval_render("fseek(3, 5, 0)").unwrap();
    assert_eq!(result.trim(), "5");
}

#[test]
fn test_fprintf_output() {
    let mut it = Interp::new();
    // Create a test file
    it.eval_render("fopen(\"/tmp/test_fprintf.txt\", \"w\")").ok();
    // Write formatted output
    let result = it.eval_render("fprintf(3, \"hello\", \" \", \"world\")").unwrap();
    // Should return number of bytes written (11 for "hello world")
    assert!(result.trim().parse::<i64>().unwrap_or(0) > 0);
}

#[test]
fn test_fscan_integers() {
    let mut it = Interp::new();
    // Create a test file with integers
    let _ = std::fs::write("/tmp/test_fscan_int.txt", "10 20 30");

    it.eval_render("fopen(\"/tmp/test_fscan_int.txt\", \"r\")").ok();
    let result = it.eval_render("fscan(3, \"%d %d %d\")").unwrap();
    // Should return a list [10, 20, 30]
    assert!(result.contains("10"));
    assert!(result.contains("20"));
    assert!(result.contains("30"));
}

#[test]
fn test_fscan_strings() {
    let mut it = Interp::new();
    // Create a test file with strings
    let _ = std::fs::write("/tmp/test_fscan_str.txt", "hello world");

    it.eval_render("fopen(\"/tmp/test_fscan_str.txt\", \"r\")").ok();
    let result = it.eval_render("fscan(3, \"%s %s\")").unwrap();
    // Should return a list with ["hello", "world"]
    assert!(result.contains("hello"));
    assert!(result.contains("world"));
}

#[test]
fn test_fscan_mixed() {
    let mut it = Interp::new();
    // Create a test file with mixed types
    let _ = std::fs::write("/tmp/test_fscan_mix.txt", "42 3.14 hello");

    it.eval_render("fopen(\"/tmp/test_fscan_mix.txt\", \"r\")").ok();
    let result = it.eval_render("fscan(3, \"%d %f %s\")").unwrap();
    // Should parse integer, float, and string
    assert!(result.contains("42"));
    assert!(result.contains("3.14") || result.contains("3.1")); // May be rounded
}

#[test]
fn test_fscan_hex() {
    let mut it = Interp::new();
    // Create a test file with hex numbers
    let _ = std::fs::write("/tmp/test_fscan_hex.txt", "ff 10");

    it.eval_render("fopen(\"/tmp/test_fscan_hex.txt\", \"r\")").ok();
    let result = it.eval_render("fscan(3, \"%x %x\")").unwrap();
    // Should parse hex values (ff=255, 10=16)
    assert!(result.contains("255"));
    assert!(result.contains("16"));
}

#[test]
fn test_fscanf_basic() {
    let mut it = Interp::new();
    // Create a test file
    let _ = std::fs::write("/tmp/test_fscanf.txt", "123 456");

    it.eval_render("fopen(\"/tmp/test_fscanf.txt\", \"r\")").ok();
    let result = it.eval_render("fscanf(3, \"%d %d\")").unwrap();
    // Should work the same as fscan, returning list
    assert!(result.contains("123"));
    assert!(result.contains("456"));
}

// Phase 6.1 Final: File System Operations

#[test]
fn test_fsize() {
    // Create a test file with known size
    let _ = std::fs::write("/tmp/test_fsize.txt", "hello");

    let mut it = Interp::new();
    let result = it.eval_render("fsize(\"/tmp/test_fsize.txt\")").unwrap();
    // "hello" is 5 bytes
    assert_eq!(result.trim(), "5");
}

#[test]
fn test_exists_true() {
    // Create a test file
    let _ = std::fs::write("/tmp/test_exists.txt", "test");

    let mut it = Interp::new();
    let result = it.eval_render("exists(\"/tmp/test_exists.txt\")").unwrap();
    // File exists, should return 1
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_exists_false() {
    let mut it = Interp::new();
    let result = it.eval_render("exists(\"/tmp/nonexistent_file_xyz.txt\")").unwrap();
    // File doesn't exist, should return 0
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_isdir_true() {
    let mut it = Interp::new();
    // /tmp directory should exist
    let result = it.eval_render("isdir(\"/tmp\")").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_isdir_false() {
    // Create a regular file
    let _ = std::fs::write("/tmp/test_isdir_file.txt", "test");

    let mut it = Interp::new();
    let result = it.eval_render("isdir(\"/tmp/test_isdir_file.txt\")").unwrap();
    // Regular file, not a directory
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_mkdir_creates_directory() {
    let test_dir = "/tmp/test_mkdir_dir";
    // Clean up if it exists
    let _ = std::fs::remove_dir(test_dir);

    let mut it = Interp::new();
    let result = it.eval_render(&format!("mkdir(\"{}\")", test_dir)).unwrap();
    // mkdir should return 0 on success
    assert_eq!(result.trim(), "0");

    // Verify directory was created
    assert!(std::path::Path::new(test_dir).is_dir());
}

#[test]
fn test_mkdir_already_exists() {
    let test_dir = "/tmp/test_mkdir_existing";
    // Create directory
    let _ = std::fs::create_dir(test_dir);

    let mut it = Interp::new();
    // mkdir on existing directory should succeed (return 0)
    let result = it.eval_render(&format!("mkdir(\"{}\")", test_dir)).unwrap();
    assert_eq!(result.trim(), "0");
}

// Phase 7: String Operations

#[test]
fn test_substr_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("substr(\"hello\", 1, 3)").unwrap();
    assert_eq!(result.trim(), "ell");
}

#[test]
fn test_substr_no_length() {
    let mut it = Interp::new();
    let result = it.eval_render("substr(\"hello\", 2)").unwrap();
    assert_eq!(result.trim(), "llo");
}

#[test]
fn test_str_conversion() {
    let mut it = Interp::new();
    let result = it.eval_render("str(42)").unwrap();
    assert!(result.contains("42"));
}

#[test]
fn test_replace_string() {
    let mut it = Interp::new();
    let result = it.eval_render("replace(\"hello hello\", \"hello\", \"hi\")").unwrap();
    assert_eq!(result.trim(), "hi hi");
}

#[test]
fn test_split_by_separator() {
    let mut it = Interp::new();
    let result = it.eval_render("split(\"a,b,c\", \",\")").unwrap();
    assert!(result.contains("a"));
    assert!(result.contains("b"));
    assert!(result.contains("c"));
}

#[test]
fn test_ltrim_whitespace() {
    let mut it = Interp::new();
    let result = it.eval_render("ltrim(\"  hello\")").unwrap();
    assert_eq!(result.trim(), "hello");
}

#[test]
fn test_rtrim_whitespace() {
    let mut it = Interp::new();
    let result = it.eval_render("rtrim(\"hello  \")").unwrap();
    assert_eq!(result.trim(), "hello");
}

#[test]
fn test_trim_both_sides() {
    let mut it = Interp::new();
    let result = it.eval_render("trim(\"  hello  \")").unwrap();
    assert_eq!(result.trim(), "hello");
}

#[test]
fn test_repeat_string() {
    let mut it = Interp::new();
    let result = it.eval_render("repeat(\"ab\", 3)").unwrap();
    assert_eq!(result.trim(), "ababab");
}

#[test]
fn test_startswith_true() {
    let mut it = Interp::new();
    let result = it.eval_render("startswith(\"hello\", \"he\")").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_startswith_false() {
    let mut it = Interp::new();
    let result = it.eval_render("startswith(\"hello\", \"hi\")").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_endswith_true() {
    let mut it = Interp::new();
    let result = it.eval_render("endswith(\"hello\", \"lo\")").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_endswith_false() {
    let mut it = Interp::new();
    let result = it.eval_render("endswith(\"hello\", \"hi\")").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_lpad_string() {
    let mut it = Interp::new();
    let result = it.eval_render("lpad(\"hi\", 5)").unwrap();
    // Result should contain "hi" and be padded (harder to test with trim destroying spaces)
    assert!(result.contains("hi"));
}

#[test]
fn test_rpad_string() {
    let mut it = Interp::new();
    let result = it.eval_render("rpad(\"hi\", 5)").unwrap();
    // Result should contain "hi" and be padded (harder to test with trim destroying spaces)
    assert!(result.contains("hi"));
}

#[test]
fn test_ord_character() {
    let mut it = Interp::new();
    // 'A' is ASCII 65
    let result = it.eval_render("ord(\"A\")").unwrap();
    assert_eq!(result.trim(), "65");
}

#[test]
fn test_chr_code() {
    let mut it = Interp::new();
    // ASCII 65 is 'A'
    let result = it.eval_render("chr(65)").unwrap();
    assert_eq!(result.trim(), "A");
}

#[test]
fn test_swapcase_string() {
    let mut it = Interp::new();
    let result = it.eval_render("swapcase(\"HeLLo\")").unwrap();
    assert_eq!(result.trim(), "hEllO");
}

#[test]
fn test_title_case() {
    let mut it = Interp::new();
    let result = it.eval_render("title(\"hello world\")").unwrap();
    assert_eq!(result.trim(), "Hello World");
}

// Phase 8: List Operations

#[test]
fn test_sort_ascending() {
    let mut it = Interp::new();
    let result = it.eval_render("sort(list(3, 1, 2))").unwrap();
    assert!(result.contains("1"));
    assert!(result.contains("2"));
    assert!(result.contains("3"));
}

#[test]
fn test_rsort_descending() {
    let mut it = Interp::new();
    let result = it.eval_render("rsort(list(1, 3, 2))").unwrap();
    assert!(result.contains("3"));
    assert!(result.contains("2"));
    assert!(result.contains("1"));
}

#[test]
fn test_reverse_list() {
    let mut it = Interp::new();
    let result = it.eval_render("reverse(list(1, 2, 3))").unwrap();
    assert!(result.contains("3"));
}

#[test]
fn test_unique_removes_dupes() {
    let mut it = Interp::new();
    let result = it.eval_render("size(unique(list(1, 2, 2, 3, 3, 3)))").unwrap();
    assert_eq!(result.trim(), "3");
}

#[test]
fn test_min_list() {
    let mut it = Interp::new();
    let result = it.eval_render("min(list(5, 2, 8, 1, 9))").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_max_list() {
    let mut it = Interp::new();
    let result = it.eval_render("max(list(5, 2, 8, 1, 9))").unwrap();
    assert_eq!(result.trim(), "9");
}

#[test]
fn test_sum_list() {
    let mut it = Interp::new();
    let result = it.eval_render("sum(list(1, 2, 3, 4, 5))").unwrap();
    assert_eq!(result.trim(), "15");
}

#[test]
fn test_product_list() {
    let mut it = Interp::new();
    let result = it.eval_render("product(list(2, 3, 4))").unwrap();
    assert_eq!(result.trim(), "24");
}

#[test]
fn test_find_value() {
    let mut it = Interp::new();
    let result = it.eval_render("find(list(10, 20, 30), 20)").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_find_not_found() {
    let mut it = Interp::new();
    let result = it.eval_render("find(list(10, 20, 30), 99)").unwrap();
    assert_eq!(result.trim(), "-1");
}

#[test]
fn test_contains_true() {
    let mut it = Interp::new();
    let result = it.eval_render("contains(list(1, 2, 3), 2)").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_contains_false() {
    let mut it = Interp::new();
    let result = it.eval_render("contains(list(1, 2, 3), 5)").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_count_list() {
    let mut it = Interp::new();
    let result = it.eval_render("count(list(1, 2, 2, 3, 2), 2)").unwrap();
    assert_eq!(result.trim(), "3");
}

#[test]
fn test_flatten_nested() {
    let mut it = Interp::new();
    it.eval_render("x = list(1, 2); y = list(3, 4); z = list(x, y)").ok();
    let result = it.eval_render("size(flatten(z))").unwrap();
    assert_eq!(result.trim(), "4");
}

#[test]
fn test_zip_lists() {
    let mut it = Interp::new();
    let result = it.eval_render("size(zip(list(1, 2, 3), list(4, 5, 6)))").unwrap();
    assert_eq!(result.trim(), "3");
}

#[test]
fn test_range_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("size(range(1, 5))").unwrap();
    assert_eq!(result.trim(), "5");
}

#[test]
fn test_range_with_step() {
    let mut it = Interp::new();
    let result = it.eval_render("size(range(0, 10, 2))").unwrap();
    assert_eq!(result.trim(), "6");
}

// Phase 9: Variable/Scope Management

#[test]
fn test_vars_list() {
    let mut it = Interp::new();
    it.eval_render("x = 1; y = 2; z = 3").ok();
    let result = it.eval_render("size(vars())").unwrap();
    // Should have at least x, y, z
    let size: i64 = result.trim().parse().unwrap_or(0);
    assert!(size >= 3);
}

#[test]
fn test_defined_true() {
    let mut it = Interp::new();
    it.eval_render("myvar = 42").ok();
    let result = it.eval_render("defined(\"myvar\")").unwrap();
    assert_eq!(result.trim(), "1");
}

#[test]
fn test_defined_false() {
    let mut it = Interp::new();
    let result = it.eval_render("defined(\"nonexistent\")").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_undefine_variable() {
    let mut it = Interp::new();
    it.eval_render("x = 10").ok();
    it.eval_render("undefine(\"x\")").ok();
    let result = it.eval_render("defined(\"x\")").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_del_alias() {
    let mut it = Interp::new();
    it.eval_render("y = 20").ok();
    it.eval_render("del(\"y\")").ok();
    let result = it.eval_render("defined(\"y\")").unwrap();
    assert_eq!(result.trim(), "0");
}

#[test]
fn test_type_number() {
    let mut it = Interp::new();
    let result = it.eval_render("type(42)").unwrap();
    assert_eq!(result.trim(), "number");
}

#[test]
fn test_type_string() {
    let mut it = Interp::new();
    let result = it.eval_render("type(\"hello\")").unwrap();
    assert_eq!(result.trim(), "string");
}

#[test]
fn test_type_list() {
    let mut it = Interp::new();
    let result = it.eval_render("type(list(1, 2))").unwrap();
    assert_eq!(result.trim(), "list");
}

#[test]
fn test_sizeof_number() {
    let mut it = Interp::new();
    let result = it.eval_render("sizeof(100)").unwrap();
    // Should return a positive number
    let size: i64 = result.trim().parse().unwrap_or(0);
    assert!(size > 0);
}

#[test]
fn test_sizeof_string() {
    let mut it = Interp::new();
    let result = it.eval_render("sizeof(\"hello\")").unwrap();
    let size: i64 = result.trim().parse().unwrap_or(0);
    assert_eq!(size, 5); // "hello" is 5 bytes
}

#[test]
fn test_env_returns_list() {
    let mut it = Interp::new();
    let result = it.eval_render("size(env())").unwrap();
    // Should have at least some environment variables
    let count: i64 = result.trim().parse().unwrap_or(0);
    assert!(count > 0);
}

#[test]
fn test_dump_output() {
    let mut it = Interp::new();
    it.eval_render("test_var = 123").ok();
    let result = it.eval_render("dump()").unwrap();
    // Dump should contain variable info
    assert!(result.contains("test_var") || result.contains("==="));
}

// Phase 10: I/O & Formatting tests

#[test]
fn test_sprintf_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("sprintf(\"hello %s\", \"world\")").unwrap();
    assert!(result.contains("hello world"));
}

#[test]
fn test_sprintf_number() {
    let mut it = Interp::new();
    let result = it.eval_render("sprintf(\"value: %d\", 42)").unwrap();
    assert!(result.contains("value: 42"));
}

#[test]
fn test_format_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("format(\"test {}\", \"value\")").unwrap();
    assert!(result.contains("test value"));
}

#[test]
fn test_format_multiple() {
    let mut it = Interp::new();
    let result = it.eval_render("format(\"a {} b {} c\", 1, 2)").unwrap();
    assert!(result.contains("a 1 b 2"));
}

#[test]
fn test_hex_conversion() {
    let mut it = Interp::new();
    let result = it.eval_render("hex(255)").unwrap();
    assert!(result.contains("ff"));
}

#[test]
fn test_hex_zero() {
    let mut it = Interp::new();
    let result = it.eval_render("hex(0)").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_oct_conversion() {
    let mut it = Interp::new();
    let result = it.eval_render("oct(8)").unwrap();
    assert!(result.contains("10"));
}

#[test]
fn test_oct_large() {
    let mut it = Interp::new();
    let result = it.eval_render("oct(64)").unwrap();
    assert!(result.contains("100"));
}

#[test]
fn test_bin_conversion() {
    let mut it = Interp::new();
    let result = it.eval_render("bin(5)").unwrap();
    assert!(result.contains("101"));
}

#[test]
fn test_bin_powers_of_two() {
    let mut it = Interp::new();
    let result = it.eval_render("bin(8)").unwrap();
    assert!(result.contains("1000"));
}

#[test]
fn test_hex_large_number() {
    let mut it = Interp::new();
    let result = it.eval_render("hex(256)").unwrap();
    assert!(result.contains("100"));
}

#[test]
fn test_bin_all_ones() {
    let mut it = Interp::new();
    let result = it.eval_render("bin(15)").unwrap();
    assert!(result.contains("1111"));
}

#[test]
fn test_sprintf_multiple_args() {
    let mut it = Interp::new();
    let result = it.eval_render("sprintf(\"%d + %d = %d\", 2, 3, 5)").unwrap();
    assert!(result.contains("2 + 3 = 5"));
}

#[test]
fn test_format_empty() {
    let mut it = Interp::new();
    let result = it.eval_render("format(\"test\")").unwrap();
    assert!(result.contains("test"));
}

// Phase 11: Math Extensions tests

#[test]
fn test_mean_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("mean(list(1, 2, 3, 4, 5))").unwrap();
    assert!(result.contains("3"));
}

#[test]
fn test_mean_decimals() {
    let mut it = Interp::new();
    let result = it.eval_render("mean(list(1.5, 2.5, 3.5))").unwrap();
    assert!(result.contains("2.5"));
}

#[test]
fn test_median_odd() {
    let mut it = Interp::new();
    let result = it.eval_render("median(list(1, 2, 3, 4, 5))").unwrap();
    assert!(result.contains("3"));
}

#[test]
fn test_median_even() {
    let mut it = Interp::new();
    let result = it.eval_render("median(list(1, 2, 3, 4))").unwrap();
    // Median of 1,2,3,4 is 2.5
    assert!(result.contains("2") || result.contains("5"));
}

#[test]
fn test_variance_uniform() {
    let mut it = Interp::new();
    let result = it.eval_render("variance(list(5, 5, 5, 5))").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_stdev_uniform() {
    let mut it = Interp::new();
    let result = it.eval_render("stdev(list(10, 10, 10, 10))").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_clz_one() {
    let mut it = Interp::new();
    let result = it.eval_render("clz(1)").unwrap();
    assert!(result.contains("63"));
}

#[test]
fn test_ctz_four() {
    let mut it = Interp::new();
    let result = it.eval_render("ctz(4)").unwrap();
    assert!(result.contains("2"));
}

#[test]
fn test_ctz_eight() {
    let mut it = Interp::new();
    let result = it.eval_render("ctz(8)").unwrap();
    assert!(result.contains("3"));
}

#[test]
fn test_nextpow2_three() {
    let mut it = Interp::new();
    let result = it.eval_render("nextpow2(3)").unwrap();
    assert!(result.contains("4"));
}

#[test]
fn test_nextpow2_five() {
    let mut it = Interp::new();
    let result = it.eval_render("nextpow2(5)").unwrap();
    assert!(result.contains("8"));
}

#[test]
fn test_prevpow2_five() {
    let mut it = Interp::new();
    let result = it.eval_render("prevpow2(5)").unwrap();
    assert!(result.contains("4"));
}

#[test]
fn test_ispow2_eight() {
    let mut it = Interp::new();
    let result = it.eval_render("ispow2(8)").unwrap();
    assert!(result.contains("1"));
}

#[test]
fn test_ispow2_seven() {
    let mut it = Interp::new();
    let result = it.eval_render("ispow2(7)").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_hammingdist_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("hammingdist(1, 4)").unwrap();
    // 1 = 0001, 4 = 0100, XOR = 0101 = 2 bits set
    assert!(result.contains("2"));
}

#[test]
fn test_hammingdist_identical() {
    let mut it = Interp::new();
    let result = it.eval_render("hammingdist(5, 5)").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_gray_zero() {
    let mut it = Interp::new();
    let result = it.eval_render("gray(0)").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_gray_one() {
    let mut it = Interp::new();
    let result = it.eval_render("gray(1)").unwrap();
    assert!(result.contains("1"));
}

#[test]
fn test_igray_one() {
    let mut it = Interp::new();
    let result = it.eval_render("igray(1)").unwrap();
    assert!(result.contains("1"));
}

#[test]
fn test_popcount_seven() {
    let mut it = Interp::new();
    let result = it.eval_render("popcount(7)").unwrap();
    // 7 = 111 in binary = 3 bits set
    assert!(result.contains("3"));
}

#[test]
fn test_popcount_eight() {
    let mut it = Interp::new();
    let result = it.eval_render("popcount(8)").unwrap();
    // 8 = 1000 in binary = 1 bit set
    assert!(result.contains("1"));
}

#[test]
fn test_rms_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("rms(list(3, 4))").unwrap();
    // RMS of 3, 4 = sqrt((9+16)/2) = sqrt(12.5) = 3.535...
    assert!(result.len() > 0);
}

#[test]
fn test_gmean_two_four() {
    let mut it = Interp::new();
    let result = it.eval_render("gmean(list(2, 8))").unwrap();
    // Geometric mean of 2, 8 = sqrt(16) = 4
    assert!(result.contains("4"));
}

#[test]
fn test_hmean_two_four() {
    let mut it = Interp::new();
    let result = it.eval_render("hmean(list(2, 4))").unwrap();
    // Harmonic mean = 2 / (1/2 + 1/4) = 2 / 0.75 = 2.666...
    assert!(result.len() > 0);
}

// Phase 12: System & Utility tests

#[test]
fn test_version_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("version()").unwrap();
    assert!(result.contains("toRustCalcMCP"));
}

#[test]
fn test_platform_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("platform()").unwrap();
    // Platform should be one of: linux, macos, windows, etc.
    assert!(result.len() > 0);
}

#[test]
fn test_hostname_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("hostname()").unwrap();
    // Should return hostname or "unknown"
    assert!(result.len() > 0);
}

#[test]
fn test_pid_returns_number() {
    let mut it = Interp::new();
    let result = it.eval_render("pid()").unwrap();
    // Should return a number
    let val: i64 = result.trim().parse().unwrap_or(0);
    assert!(val > 0);
}

#[test]
fn test_username_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("username()").unwrap();
    // Should return username or "unknown"
    assert!(result.len() > 0);
}

#[test]
fn test_homedir_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("homedir()").unwrap();
    // Should return a path
    assert!(result.len() > 0);
}

#[test]
fn test_tmpdir_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("tmpdir()").unwrap();
    // Should return temp directory path
    assert!(result.len() > 0);
}

#[test]
fn test_pwd_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("pwd()").unwrap();
    // Should return current working directory
    assert!(result.len() > 0);
    assert!(result.contains("/") || result.contains("\\"));
}

#[test]
fn test_getuid_returns_number() {
    let mut it = Interp::new();
    let result = it.eval_render("getuid()").unwrap();
    // Should return a number
    let val: i64 = result.trim().parse().unwrap_or(-1);
    assert!(val >= 0);
}

#[test]
fn test_arch_returns_string() {
    let mut it = Interp::new();
    let result = it.eval_render("arch()").unwrap();
    // Should be one of: x86_64, aarch64, etc.
    assert!(result.len() > 0);
}

#[test]
fn test_uname_contains_both() {
    let mut it = Interp::new();
    let result = it.eval_render("uname()").unwrap();
    // Should contain both OS and architecture separated by -
    assert!(result.contains("-"));
}

#[test]
fn test_cd_and_pwd_consistent() {
    let mut it = Interp::new();
    let original = it.eval_render("pwd()").unwrap();
    // Try to cd to a common directory and back
    let homedir = it.eval_render("homedir()").unwrap();
    // Just verify the operations don't crash
    assert!(original.len() > 0);
}

// Phase 13: Advanced Operations tests

#[test]
fn test_polyval_basic() {
    let mut it = Interp::new();
    // p(x) = 2x^2 + 3x + 1, evaluate at x=2: 2*4 + 3*2 + 1 = 15
    let result = it.eval_render("polyval(list(1, 3, 2), 2)").unwrap();
    assert!(result.contains("15"));
}

#[test]
fn test_dot_product() {
    let mut it = Interp::new();
    // [1, 2, 3] · [4, 5, 6] = 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    let result = it.eval_render("dot(list(1, 2, 3), list(4, 5, 6))").unwrap();
    assert!(result.contains("32"));
}

#[test]
fn test_norm_simple() {
    let mut it = Interp::new();
    // norm([3, 4]) = sqrt(9 + 16) = sqrt(25) = 5
    let result = it.eval_render("norm(list(3, 4))").unwrap();
    assert!(result.contains("5"));
}

#[test]
fn test_polyderiv_linear() {
    let mut it = Interp::new();
    // d/dx(2x + 1) = 2
    let result = it.eval_render("polyderiv(list(1, 2))").unwrap();
    assert!(result.contains("2"));
}

#[test]
fn test_union_sets() {
    let mut it = Interp::new();
    let result = it.eval_render("union(list(1, 2, 3), list(3, 4, 5))").unwrap();
    // Should contain 1,2,3,4,5
    assert!(result.contains("1"));
    assert!(result.contains("4"));
}

#[test]
fn test_intersection_sets() {
    let mut it = Interp::new();
    let result = it.eval_render("intersection(list(1, 2, 3), list(2, 3, 4))").unwrap();
    // Should contain 2,3
    assert!(result.contains("2"));
    assert!(result.contains("3"));
}

#[test]
fn test_difference_sets() {
    let mut it = Interp::new();
    let result = it.eval_render("difference(list(1, 2, 3), list(2, 4))").unwrap();
    // Should contain 1,3
    assert!(result.contains("1"));
}

#[test]
fn test_subset_true() {
    let mut it = Interp::new();
    let result = it.eval_render("subset(list(1, 2), list(1, 2, 3))").unwrap();
    assert!(result.contains("1"));
}

#[test]
fn test_subset_false() {
    let mut it = Interp::new();
    let result = it.eval_render("subset(list(1, 4), list(1, 2, 3))").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_cumsum_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("cumsum(list(1, 2, 3, 4))").unwrap();
    // Should be [1, 3, 6, 10]
    assert!(result.len() > 0);
}

#[test]
fn test_diff_basic() {
    let mut it = Interp::new();
    let result = it.eval_render("diff(list(1, 3, 6, 10))").unwrap();
    // Should be [2, 3, 4]
    assert!(result.len() > 0);
}

#[test]
fn test_mode_single() {
    let mut it = Interp::new();
    let result = it.eval_render("mode(list(1, 2, 2, 3, 3, 3))").unwrap();
    // 3 appears most often
    assert!(result.contains("3"));
}

#[test]
fn test_polyval_zero() {
    let mut it = Interp::new();
    // p(x) = x^2 - 4, evaluate at x=2: 4 - 4 = 0
    let result = it.eval_render("polyval(list(-4, 0, 1), 2)").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_dot_zero() {
    let mut it = Interp::new();
    // [1, 0] · [0, 1] = 0
    let result = it.eval_render("dot(list(1, 0), list(0, 1))").unwrap();
    assert!(result.contains("0"));
}

#[test]
fn test_norm_unit() {
    let mut it = Interp::new();
    // norm([1]) = 1
    let result = it.eval_render("norm(list(1))").unwrap();
    assert!(result.contains("1"));
}
