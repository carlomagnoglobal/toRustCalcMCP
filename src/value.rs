//! Values: numbers, strings, null, or functions.

use crate::config::Config;
use crate::number::{self, Num};
use crate::parser::Expr;
use num_bigint::BigInt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Number(Num),
    Str(String),
    Null,
    Function(Vec<String>, Rc<Expr>), // params, body (as Expr)
    List(Vec<Value>), // homogeneous list
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
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
