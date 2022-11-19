use std::cmp::Ordering::*;
use std::{cmp, fmt, ops};

use crate::Boolean;

#[derive(Eq, Clone)]
pub struct UInt {
    _binary: Vec<Boolean>,
}

impl UInt {
    pub fn new<T>(binary: T) -> Self
    where
        T: Into<Vec<Boolean>>,
    {
        UInt {
            _binary: binary.into(),
        }
    }

    pub fn binary(&self) -> &Vec<Boolean> {
        &self._binary
    }

    pub fn bit_len(&self) -> usize {
        self._binary.len()
    }

    pub fn max_bit_digit(&self) -> usize {
        let ln = self.bit_len();
        for i in (0..ln).rev() {
            let b = self._binary[i];
            if b.into() {
                return i + 1;
            }
        }
        0
    }

    pub fn trim(&self) -> Self {
        if self._binary.is_empty() {
            return UInt::new([]);
        }
        let last_true_index = self.max_bit_digit();
        if last_true_index == 0 {
            // self._binary is [False, False, ...]
            return UInt::new([]);
        }
        UInt::new(&self._binary[0..last_true_index])
    }

    pub fn trim_mut(&mut self) {
        let ln = self.max_bit_digit();
        self._binary.resize(ln, Boolean::False);
    }
}

impl From<u64> for UInt {
    fn from(n: u64) -> Self {
        let len = u64::BITS - n.leading_zeros();
        let it = (0..len).map(|i| Boolean::from(n >> i & 1 == 1));
        Self {
            _binary: it.collect(),
        }
    }
}

impl From<UInt> for u64 {
    fn from(n: UInt) -> Self {
        let mut res = 0;
        for (i, &b) in n.binary().iter().enumerate() {
            res |= u64::from(<Boolean as std::convert::Into<bool>>::into(b)) << i;
        }
        res
    }
}

impl fmt::Debug for UInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UInt")
            .field("binary", &self._binary)
            .finish()
    }
}

impl cmp::PartialEq for UInt {
    fn eq(&self, other: &Self) -> bool {
        let len = self.max_bit_digit();
        if len != other.max_bit_digit() {
            return false;
        }
        self.binary()[..len]
            .iter()
            .zip(other.binary()[..len].iter())
            .all(|(&b1, &b2)| b1 == b2)
    }
}

impl cmp::Ord for UInt {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let len = self.max_bit_digit();
        match len.cmp(&other.max_bit_digit()) {
            Equal => {
                let bin1 = self.binary();
                let bin2 = other.binary();
                (0..len)
                    .rev()
                    .map(|i| (bin1[i] as u8, bin2[i] as u8))
                    .map(|(b1, b2)| b1.cmp(&b2))
                    .fold(Equal, |acc, m| match (acc, m) {
                        (Equal, x) => x,
                        (x, _) => x,
                    })
            }
            x => x,
        }
    }
}

impl cmp::PartialOrd for UInt {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::BitAnd for UInt {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let len = cmp::min(self.max_bit_digit(), rhs.max_bit_digit());
        let bin1 = self.binary();
        let bin2 = rhs.binary();
        let binary: Vec<_> = (0..len)
            .map(|i| {
                (
                    bin1.get(i).unwrap_or(&Boolean::False),
                    bin2.get(i).unwrap_or(&Boolean::False),
                )
            })
            .map(|(&b1, &b2)| b1 & b2)
            .collect();
        Self::Output { _binary: binary }
    }
}

impl ops::BitAndAssign for UInt {
    fn bitand_assign(&mut self, rhs: Self) {
        let len = cmp::min(self.max_bit_digit(), rhs.max_bit_digit());
        self._binary.resize(len, Boolean::False);
        let bin1 = &mut self._binary;
        let bin2 = rhs.binary();
        let it = bin1.iter_mut().zip(bin2.iter()).take(len);
        for (b1, b2) in it {
            *b1 &= *b2;
        }
    }
}

impl ops::BitOr for UInt {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.max_bit_digit(), rhs.max_bit_digit());
        let bin1 = self.binary();
        let bin2 = rhs.binary();
        let binary: Vec<_> = (0..len)
            .map(|i| {
                (
                    bin1.get(i).unwrap_or(&Boolean::False),
                    bin2.get(i).unwrap_or(&Boolean::False),
                )
            })
            .map(|(&b1, &b2)| b1 | b2)
            .collect();
        Self::Output { _binary: binary }
    }
}

impl ops::BitOrAssign for UInt {
    fn bitor_assign(&mut self, rhs: Self) {
        let len = cmp::max(self.max_bit_digit(), rhs.max_bit_digit());
        self._binary.resize(len, Boolean::False);
        let bin1 = &mut self._binary;
        let bin2 = rhs.binary();
        let it = bin1.iter_mut().enumerate().take(len);
        for (i, b) in it {
            *b |= *bin2.get(i).unwrap_or(&Boolean::False);
        }
    }
}

impl ops::BitXor for UInt {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.max_bit_digit(), rhs.max_bit_digit());
        let bin1 = self.binary();
        let bin2 = rhs.binary();
        let mut binary: Vec<_> = (0..len)
            .map(|i| {
                (
                    bin1.get(i).unwrap_or(&Boolean::False),
                    bin2.get(i).unwrap_or(&Boolean::False),
                )
            })
            .map(|(&b1, &b2)| b1 ^ b2)
            .collect();
        for i in (0..len).rev() {
            if binary[i].into() {
                binary.resize(i + 1, Boolean::False);
                break;
            }
        }
        Self::Output { _binary: binary }
    }
}

impl ops::BitXorAssign for UInt {
    fn bitxor_assign(&mut self, rhs: Self) {
        let len = cmp::max(self.max_bit_digit(), rhs.max_bit_digit());
        let bin1 = &mut self._binary;
        let bin2 = rhs.binary();
        bin1.resize(len, Boolean::False);
        let it = bin1.iter_mut().enumerate().take(len);
        for (i, b) in it {
            *b ^= *bin2.get(i).unwrap_or(&Boolean::False);
        }
        self.trim_mut();
    }
}
