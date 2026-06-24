//! Session configuration: precision, display mode, number of digits.

use crate::number::{self, Num};
use num_traits::{Signed, Zero};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Real,
    Frac,
    Int,
}

impl Mode {
    pub fn parse(s: &str) -> Option<Mode> {
        match s {
            "real" => Some(Mode::Real),
            "frac" => Some(Mode::Frac),
            "int" => Some(Mode::Int),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Real => "real",
            Mode::Frac => "frac",
            Mode::Int => "int",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub epsilon: Num,
    pub display: usize,
    pub mode: Mode,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            epsilon: Num::from_float(1e-20).unwrap(),
            display: 20,
            mode: Mode::Real,
        }
    }
}

impl Config {
    pub fn set_epsilon_from_str(&mut self, s: &str) -> Result<(), String> {
        let n = number::parse_number(s).ok_or("invalid epsilon")?;
        if n.is_negative() || n.is_zero() {
            return Err("epsilon must be positive".to_string());
        }
        self.epsilon = n;
        Ok(())
    }
}
