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
    pub ibase: u32, // input base (2-36, default 10)
    pub obase: u32, // output base (2-36, default 10)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // Exact 1/10^20 (not Num::from_float, which yields a messy binary
            // rational p/2^k and makes round_to_epsilon snap clean values like
            // cos(0)=1 to nearby non-clean multiples, printing "~-0" etc.).
            epsilon: number::parse_number("1e-20").expect("valid default epsilon"),
            display: 20,
            mode: Mode::Real,
            ibase: 10,
            obase: 10,
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
