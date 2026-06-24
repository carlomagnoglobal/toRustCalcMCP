//! Tree-walking evaluator.

use crate::builtins;
use crate::config::Config;
use crate::number::{self, Num};
use crate::parser::{BinOp, Expr, UnOp};
use crate::value::Value;
use num_traits::{ToPrimitive, Zero};
use std::collections::HashMap;

pub type BuiltinFn = fn(&mut Interp, &[Value]) -> Result<Value, String>;

pub struct Interp {
    pub cfg: Config,
    pub global_vars: HashMap<String, Value>,
    pub scope_stack: Vec<HashMap<String, Value>>, // Stack of local scopes
    pub builtins: HashMap<String, BuiltinFn>,
}

impl Default for Interp {
    fn default() -> Self {
        let mut it = Interp {
            cfg: Config::default(),
            global_vars: HashMap::new(),
            scope_stack: Vec::new(),
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

    // Get a variable from the current scope or parent scopes
    fn get_var(&self, name: &str) -> Option<Value> {
        // Check local scopes from innermost to outermost
        for scope in self.scope_stack.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        // Check global scope
        self.global_vars.get(name).cloned()
    }

    // Set a variable in the current scope
    fn set_var(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(name, value);
        } else {
            self.global_vars.insert(name, value);
        }
    }

    pub fn eval_str(&mut self, src: &str) -> Result<Value, String> {
        Ok(self.eval_all(src)?.pop().unwrap_or(Value::Null))
    }

    pub fn eval_all(&mut self, src: &str) -> Result<Vec<Value>, String> {
        let stmts = crate::parser::parse(src)?;
        let mut out = Vec::with_capacity(stmts.len());
        for s in stmts {
            out.push(self.eval(&s)?);
        }
        Ok(out)
    }

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
                .get_var(name)
                .ok_or_else(|| format!("undefined variable: {name}")),
            Expr::Assign(name, rhs) => {
                let v = self.eval(rhs)?;
                self.set_var(name.clone(), v.clone());
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
                // Check if it's a user-defined function first
                if let Some(func_val) = self.get_var(name) {
                    if let Value::Function(params, body) = func_val {
                        if params.len() != argv.len() {
                            return Err(format!("{}() expects {} args, got {}", name, params.len(), argv.len()));
                        }
                        // Create new scope for function call
                        let mut local_scope = HashMap::new();
                        for (param, arg) in params.iter().zip(argv.iter()) {
                            local_scope.insert(param.clone(), arg.clone());
                        }
                        self.scope_stack.push(local_scope);
                        let result = self.eval(&body);
                        self.scope_stack.pop();
                        return result;
                    }
                }
                // Otherwise look for builtin
                let Some(f) = self.builtins.get(name).copied() else {
                    return Err(format!("undefined function: {name}()"));
                };
                f(self, &argv)
            }
            Expr::Define(name, params, body) => {
                let func = Value::Function(params.clone(), std::rc::Rc::new((**body).clone()));
                self.set_var(name.clone(), func);
                Ok(Value::Null)
            }
            Expr::If(cond, then_b, else_b) => {
                let cond_val = self.eval(cond)?;
                let is_true = match cond_val {
                    Value::Number(n) => !n.is_zero(),
                    Value::Null => false,
                    _ => true,
                };
                if is_true {
                    self.eval(then_b)
                } else if let Some(else_branch) = else_b {
                    self.eval(else_branch)
                } else {
                    Ok(Value::Null)
                }
            }
            Expr::While(cond, body) => {
                let mut result = Value::Null;
                loop {
                    let cond_val = self.eval(cond)?;
                    let is_true = match cond_val {
                        Value::Number(n) => !n.is_zero(),
                        Value::Null => false,
                        _ => true,
                    };
                    if !is_true {
                        break;
                    }
                    result = self.eval(body)?;
                }
                Ok(result)
            }
            Expr::For(var, start, end, body) => {
                let start_val = self.eval(start)?;
                let end_val = self.eval(end)?;
                let start_num = start_val.as_number()?.clone();
                let end_num = end_val.as_number()?.clone();

                let mut result = Value::Null;
                let mut current = start_num.clone();
                let one = Num::from_integer(num_bigint::BigInt::from(1));

                while &current <= &end_num {
                    self.set_var(var.clone(), Value::Number(current.clone()));
                    result = self.eval(body)?;
                    current = &current + &one;
                }
                Ok(result)
            }
            Expr::Block(stmts) => {
                let mut result = Value::Null;
                for stmt in stmts {
                    result = self.eval(stmt)?;
                }
                Ok(result)
            }
            Expr::Print(args) => {
                let values: Vec<Value> = args.iter().map(|a| self.eval(a)).collect::<Result<_, _>>()?;
                let mut output = Vec::new();
                for v in &values {
                    match v {
                        Value::Str(s) => output.push(s.clone()),
                        Value::Number(n) => output.push(number::to_decimal_string(n, self.cfg.display)),
                        Value::Null => {},
                        _ => output.push(format!("{:?}", v)),
                    }
                }
                Ok(Value::Str(output.join(" ")))
            }
            Expr::Index(list_expr, index_expr) => {
                let list_val = self.eval(list_expr)?;
                let index_val = self.eval(index_expr)?;

                match list_val {
                    Value::List(items) => {
                        let idx_num = index_val.as_number()?;
                        if !idx_num.is_integer() {
                            return Err("index must be an integer".to_string());
                        }
                        let idx = idx_num.numer();
                        let idx_i64 = idx.to_i64()
                            .ok_or("index out of range".to_string())?;
                        let idx_usize = if idx_i64 < 0 {
                            // Negative indexing: -1 is last element
                            let len = items.len() as i64;
                            ((len + idx_i64) as usize)
                        } else {
                            idx_i64 as usize
                        };

                        items.get(idx_usize)
                            .cloned()
                            .ok_or("index out of bounds".to_string())
                    }
                    _ => Err("cannot index non-list value".to_string()),
                }
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
