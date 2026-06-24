//! Tree-walking evaluator.

use crate::builtins;
use crate::config::Config;
use crate::number::{self, Num};
use crate::parser::{BinOp, Expr, UnOp};
use crate::value::Value;
use num_traits::Zero;
use std::collections::HashMap;

pub type BuiltinFn = fn(&mut Interp, &[Value]) -> Result<Value, String>;

pub struct Interp {
    pub cfg: Config,
    pub vars: HashMap<String, Value>,
    pub builtins: HashMap<String, BuiltinFn>,
}

impl Default for Interp {
    fn default() -> Self {
        let mut it = Interp {
            cfg: Config::default(),
            vars: HashMap::new(),
            builtins: HashMap::new(),
        };
        builtins::register(&mut it.builtins);
        it
    }
}

impl Interp {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluate a full source string; returns the value of the last statement.
    pub fn eval_str(&mut self, src: &str) -> Result<Value, String> {
        Ok(self.eval_all(src)?.pop().unwrap_or(Value::Null))
    }

    /// Evaluate every `;`-separated statement, returning each value (calc prints
    /// the value of each expression statement, including assignments).
    pub fn eval_all(&mut self, src: &str) -> Result<Vec<Value>, String> {
        let stmts = crate::parser::parse(src)?;
        let mut out = Vec::with_capacity(stmts.len());
        for s in stmts {
            out.push(self.eval(&s)?);
        }
        Ok(out)
    }

    /// Evaluate and render in one shot, one rendered value per line.
    pub fn eval_render(&mut self, src: &str) -> Result<String, String> {
        let vals = self.eval_all(src)?;
        let lines: Vec<String> = vals
            .iter()
            .map(|v| v.render(&self.cfg))
            .filter(|s| !s.is_empty())
            .collect();
        Ok(lines.join("\n"))
    }

    pub fn eval(&mut self, e: &Expr) -> Result<Value, String> {
        match e {
            Expr::Number(s) => number::parse_number(s)
                .map(Value::Number)
                .ok_or_else(|| format!("invalid number literal: {s}")),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Var(name) => self
                .vars
                .get(name)
                .cloned()
                .ok_or_else(|| format!("undefined variable: {name}")),
            Expr::Assign(name, rhs) => {
                let v = self.eval(rhs)?;
                self.vars.insert(name.clone(), v.clone());
                Ok(v)
            }
            Expr::Unary(op, inner) => {
                let v = self.eval(inner)?;
                let n = v.as_number()?.clone();
                Ok(match op {
                    UnOp::Neg => Value::Number(-n),
                    UnOp::Pos => Value::Number(n),
                })
            }
            Expr::Binary(op, l, r) => self.eval_binary(*op, l, r),
            Expr::Call(name, args) => {
                let argv: Vec<Value> =
                    args.iter().map(|a| self.eval(a)).collect::<Result<_, _>>()?;
                let Some(f) = self.builtins.get(name).copied() else {
                    return Err(format!("undefined function: {name}()"));
                };
                f(self, &argv)
            }
        }
    }

    fn eval_binary(&mut self, op: BinOp, l: &Expr, r: &Expr) -> Result<Value, String> {
        let lv = self.eval(l)?;
        let rv = self.eval(r)?;
        let a = lv.as_number()?.clone();
        let b = rv.as_number()?.clone();
        Ok(match op {
            BinOp::Add => Value::Number(a + b),
            BinOp::Sub => Value::Number(a - b),
            BinOp::Mul => Value::Number(a * b),
            BinOp::Div => {
                if b.is_zero() {
                    return Err("division by zero".into());
                }
                Value::Number(a / b)
            }
            BinOp::IntDiv => {
                if b.is_zero() {
                    return Err("division by zero".into());
                }
                Value::Number(number::trunc(&(a / b)))
            }
            BinOp::Mod => {
                if b.is_zero() {
                    return Err("modulus by zero".into());
                }
                // calc's % : a - b*int(a/b)
                let q = number::trunc(&(&a / &b));
                Value::Number(&a - &b * q)
            }
            BinOp::Pow => Value::Number(number::pow(&a, &b)?),
            BinOp::Eq => Value::boolean(a == b),
            BinOp::Ne => Value::boolean(a != b),
            BinOp::Lt => Value::boolean(a < b),
            BinOp::Le => Value::boolean(a <= b),
            BinOp::Gt => Value::boolean(a > b),
            BinOp::Ge => Value::boolean(a >= b),
        })
    }

    pub fn epsilon(&self) -> Num {
        self.cfg.epsilon.clone()
    }
}
