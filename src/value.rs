//! Values: numbers, strings, or null.

use crate::config::Config;
use crate::number::{self, Num};
use num_bigint::BigInt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(Num),
    Str(String),
    Null,
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
                crate::config::Mode::Real => number::to_decimal_string(n, cfg.display),
                crate::config::Mode::Frac => {
                    if n.is_integer() {
                        n.numer().to_string()
                    } else {
                        format!("{}/{}", n.numer(), n.denom())
                    }
                }
                crate::config::Mode::Int => number::trunc(n).numer().to_string(),
            },
            Value::Str(s) => s.clone(),
            Value::Null => String::new(),
        }
    }
}
