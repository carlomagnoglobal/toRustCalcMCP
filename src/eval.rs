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
    pub rng_seed: u64, // Seed for RNG (LCG-style)
}

impl Default for Interp {
    fn default() -> Self {
        let mut it = Interp {
            cfg: Config::default(),
            global_vars: HashMap::new(),
            scope_stack: Vec::new(),
            builtins: HashMap::new(),
            rng_seed: 1, // Default seed
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
                Ok(match op {
                    UnOp::Neg => match v {
                        Value::Number(n) => Value::Number(-n),
                        Value::Complex(r, i) => Value::Complex(-r, -i),
                        _ => return Err("not a number".to_string()),
                    }
                    UnOp::Pos => match v {
                        Value::Number(n) => Value::Number(n),
                        Value::Complex(r, i) => Value::Complex(r, i),
                        _ => return Err("not a number".to_string()),
                    }
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

        // Extract real and imaginary parts
        let (lr, li) = match &lv {
            Value::Number(n) => (n.clone(), Num::zero()),
            Value::Complex(r, i) => (r.clone(), i.clone()),
            _ => return Err("not a number".to_string()),
        };
        let (rr, ri) = match &rv {
            Value::Number(n) => (n.clone(), Num::zero()),
            Value::Complex(r, i) => (r.clone(), i.clone()),
            _ => return Err("not a number".to_string()),
        };

        // Perform operation on complex numbers
        let result = match op {
            BinOp::Add => {
                Value::Complex(&lr + &rr, &li + &ri)
            }
            BinOp::Sub => {
                Value::Complex(&lr - &rr, &li - &ri)
            }
            BinOp::Mul => {
                // (a+bi)(c+di) = (ac-bd) + (ad+bc)i
                let real = &lr * &rr - &li * &ri;
                let imag = &lr * &ri + &li * &rr;
                Value::Complex(real, imag)
            }
            BinOp::Div => {
                // (a+bi)/(c+di) = ((ac+bd) + (bc-ad)i) / (c^2 + d^2)
                if rr.is_zero() && ri.is_zero() {
                    return Err("division by zero".into());
                }
                let denom = &rr * &rr + &ri * &ri;
                let real = (&lr * &rr + &li * &ri) / &denom;
                let imag = (&li * &rr - &lr * &ri) / &denom;
                Value::Complex(real, imag)
            }
            BinOp::Pow => {
                // For now, only support integer/real powers, not general complex powers
                if !ri.is_zero() || !rr.is_integer() {
                    return Err("complex exponentiation not supported".to_string());
                }
                if li.is_zero() {
                    Value::Number(number::pow(&lr, &rr)?)
                } else {
                    return Err("complex base exponentiation not supported".to_string());
                }
            }
            BinOp::IntDiv | BinOp::Mod => {
                return Err(format!("{:?} not supported for complex numbers", op));
            }
            BinOp::Eq => Value::boolean(&lr == &rr && &li == &ri),
            BinOp::Ne => Value::boolean(&lr != &rr || &li != &ri),
            BinOp::Lt => {
                if !li.is_zero() || !ri.is_zero() {
                    return Err("comparison not defined for complex numbers".to_string());
                }
                Value::boolean(&lr < &rr)
            }
            BinOp::Le => {
                if !li.is_zero() || !ri.is_zero() {
                    return Err("comparison not defined for complex numbers".to_string());
                }
                Value::boolean(&lr <= &rr)
            }
            BinOp::Gt => {
                if !li.is_zero() || !ri.is_zero() {
                    return Err("comparison not defined for complex numbers".to_string());
                }
                Value::boolean(&lr > &rr)
            }
            BinOp::Ge => {
                if !li.is_zero() || !ri.is_zero() {
                    return Err("comparison not defined for complex numbers".to_string());
                }
                Value::boolean(&lr >= &rr)
            }
        };

        // Simplify: if imaginary part is zero, return just the real part
        if let Value::Complex(r, i) = &result {
            if i.is_zero() {
                Ok(Value::Number(r.clone()))
            } else {
                Ok(result)
            }
        } else {
            Ok(result)
        }
    }

    pub fn epsilon(&self) -> Num {
        self.cfg.epsilon.clone()
    }
}
