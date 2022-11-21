use std::convert::From;
use std::fmt;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Boolean {
    False = 0,
    True,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::False => "False",
                Self::True => "True",
            }
        )
    }
}

impl ops::Not for Boolean {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Boolean::False => Boolean::True,
            Boolean::True => Boolean::False,
        }
    }
}

impl ops::BitAnd for Boolean {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::True, Self::True) => Self::True,
            _ => Self::False,
        }
    }
}

impl ops::BitAndAssign for Boolean {
    fn bitand_assign(&mut self, rhs: Self) {
        let lhs = *self;
        *self = match (lhs, rhs) {
            (Self::True, Self::True) => Self::True,
            _ => Self::False,
        };
    }
}

impl ops::BitOr for Boolean {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::False, Self::False) => Self::False,
            _ => Self::True,
        }
    }
}

impl ops::BitOrAssign for Boolean {
    fn bitor_assign(&mut self, rhs: Self) {
        let lhs = *self;
        *self = match (lhs, rhs) {
            (Self::False, Self::False) => Self::False,
            _ => Self::True,
        };
    }
}

impl ops::BitXor for Boolean {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::False, Self::False) | (Self::True, Self::True) => Self::False,
            _ => Self::True,
        }
    }
}

impl ops::BitXorAssign for Boolean {
    fn bitxor_assign(&mut self, rhs: Self) {
        let lhs = *self;
        *self = match (lhs, rhs) {
            (Self::False, Self::False) | (Self::True, Self::True) => Self::False,
            _ => Self::True,
        };
    }
}

impl From<Boolean> for bool {
    fn from(item: Boolean) -> Self {
        match item {
            Boolean::False => false,
            Boolean::True => true,
        }
    }
}

impl From<bool> for Boolean {
    fn from(item: bool) -> Self {
        match item {
            false => Self::False,
            true => Self::True,
        }
    }
}

impl FromStr for Boolean {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let s = s.to_lowercase();
        if len == 4 && s == "true" {
            Ok(Self::True)
        } else if len == 5 && s == "false" {
            Ok(Self::False)
        } else {
            Err(format!("`{}` cannot be converted to Boolean", s))
        }
    }
}

mod num;
pub use num::UInt;
