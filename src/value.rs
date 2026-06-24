//! Values: numbers, strings, null, or functions.

use crate::config::Config;
use crate::number::{self, Num};
use crate::parser::Expr;
use num_bigint::BigInt;
use num_traits::{Signed, Zero};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Number(Num),
    Complex(Num, Num), // real, imaginary
    Str(String),
    Null,
    Function(Vec<String>, Rc<Expr>), // params, body (as Expr)
    List(Vec<Value>), // homogeneous list
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Complex(ar, ai), Value::Complex(br, bi)) => ar == br && ai == bi,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::List(a), Value::List(b)) => a == b,
            // Functions are not compared for equality
            _ => false,
        }
    }
}

impl Value {
    pub fn as_number(&self) -> Result<&Num, String> {
        match self {
            Value::Number(n) => Ok(n),
            _ => Err("not a number".to_string()),
        }
    }

    pub fn boolean(b: bool) -> Value {
        Value::Number(if b {
            Num::from_integer(BigInt::from(1))
        } else {
            Num::from_integer(BigInt::from(0))
        })
    }

    pub fn render(&self, cfg: &Config) -> String {
        match self {
            Value::Number(n) => match cfg.mode {
                crate::config::Mode::Real => {
                    if cfg.obase != 10 {
                        number::to_string_in_base(n, cfg.obase, cfg.display)
                    } else {
                        number::to_decimal_string(n, cfg.display)
                    }
                }
                crate::config::Mode::Frac => {
                    if n.is_integer() {
                        if cfg.obase != 10 {
                            number::to_base(n.numer(), cfg.obase)
                        } else {
                            n.numer().to_string()
                        }
                    } else {
                        format!("{}/{}", number::to_base(n.numer(), cfg.obase), number::to_base(n.denom(), cfg.obase))
                    }
                }
                crate::config::Mode::Int => {
                    let int_val = number::trunc(n).numer().clone();
                    if cfg.obase != 10 {
                        number::to_base(&int_val, cfg.obase)
                    } else {
                        int_val.to_string()
                    }
                }
            },
            Value::Complex(r, i) => {
                let real_str = if cfg.obase != 10 {
                    number::to_string_in_base(r, cfg.obase, cfg.display)
                } else {
                    number::to_decimal_string(r, cfg.display)
                };
                let imag_str = if cfg.obase != 10 {
                    number::to_string_in_base(i, cfg.obase, cfg.display)
                } else {
                    number::to_decimal_string(i, cfg.display)
                };
                // Remove ~ prefix for imaginary part if present
                let imag_clean = imag_str.trim_start_matches('~');
                if i.is_zero() {
                    real_str
                } else if r.is_zero() {
                    format!("{}i", imag_clean)
                } else if i.numer().is_positive() {
                    format!("{}+{}i", real_str, imag_clean)
                } else {
                    format!("{}-{}i", real_str, imag_clean.trim_start_matches('-'))
                }
            }
            Value::Str(s) => s.clone(),
            Value::Null => String::new(),
            Value::Function(_, _) => String::new(), // Functions don't render
            Value::List(items) => {
                let rendered: Vec<String> = items.iter()
                    .map(|v| v.render(cfg))
                    .collect();
                format!("[{}]", rendered.join(", "))
            }
        }
    }
}
