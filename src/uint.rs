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
                #[allow(clippy::suspicious_arithmetic_impl)]
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

impl ops::Shl for UInt {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        let rhs: u64 = rhs.into();
        let mut bin = vec![Boolean::False; rhs as usize];
        bin.extend_from_slice(self.binary());
        Self::Output { _binary: bin }
    }
}

impl ops::ShlAssign for UInt {
    fn shl_assign(&mut self, rhs: Self) {
        let rhs: u64 = rhs.into();
        let mut bin = vec![Boolean::False; rhs as usize];
        bin.extend(self._binary.iter());
        self._binary = bin;
    }
}

impl ops::Shr for UInt {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        let rhs = cmp::min(u64::from(rhs) as usize, self.bit_len());
        Self::new(&self.binary()[rhs..])
    }
}

impl ops::ShrAssign for UInt {
    fn shr_assign(&mut self, rhs: Self) {
        let rhs = cmp::min(u64::from(rhs) as usize, self.bit_len());
        self._binary = Vec::from(&self._binary[rhs..]);
    }
}

impl ops::Add for UInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let bin1 = self.binary();
        let bin2 = rhs.binary();
        let dig1 = |i: usize| *bin1.get(i).unwrap_or(&Boolean::False);
        let dig2 = |i: usize| *bin2.get(i).unwrap_or(&Boolean::False);
        let mut sums = vec![];
        sums.push(dig1(0) ^ dig2(0));
        let mut carry = dig1(0) & dig2(0);
        let len = cmp::max(self.max_bit_digit(), rhs.max_bit_digit());
        for i in 0..len {
            let (a, b, c) = (dig1(i + 1), dig2(i + 1), carry);
            sums.push(a ^ b ^ c);
            carry = (a & b) | (b & c) | (c & a);
        }
        let mut res = Self { _binary: sums };
        res.trim_mut();
        res
    }
}

impl ops::AddAssign for UInt {
    fn add_assign(&mut self, rhs: Self) {
        let len = cmp::max(self.max_bit_digit(), rhs.max_bit_digit());
        let bin1 = &mut self._binary;
        let bin2 = rhs.binary();
        let dig2 = |i: usize| *bin2.get(i).unwrap_or(&Boolean::False);
        bin1.resize(len + 1, Boolean::False);
        let mut carry = bin1[0] & dig2(0);
        bin1[0] ^= dig2(0);
        let it = bin1
            .iter_mut()
            .enumerate()
            .skip(1)
            .map(|(i, bin)| (bin, dig2(i)));
        for (bin, b) in it {
            let a = *bin;
            *bin = a ^ b ^ carry;
            carry = (a & b) | (b & carry) | (carry & a);
        }
        self.trim_mut();
    }
}

impl ops::Mul for UInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = UInt::new([]);
        let bin2 = rhs.binary();
        let it = bin2.iter().enumerate();
        for (i, b) in it {
            if !bool::from(*b) {
                continue;
            }
            res += self.clone() << UInt::from(i as u64);
        }
        res
    }
}

impl ops::MulAssign for UInt {
    fn mul_assign(&mut self, rhs: Self) {
        self._binary = (self.clone() * rhs)._binary;
    }
}

impl ops::Sub for UInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let n1 = UInt::from(1);
        let mut le = UInt::from(0);
        let mut gt = self.clone() + n1.clone();
        if rhs.clone() + le.clone() > self {
            panic!("attempt to subtract with overflow");
        }
        // binary search
        while gt > le.clone() + n1.clone() {
            let mid = (gt.clone() + le.clone()) >> n1.clone();
            if mid.clone() + rhs.clone() > self {
                gt = mid;
            } else {
                le = mid;
            }
        }
        le
    }
}

impl ops::SubAssign for UInt {
    fn sub_assign(&mut self, rhs: Self) {
        self._binary = (self.clone() - rhs)._binary;
    }
}

impl ops::Div for UInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let n1 = UInt::from(1);
        let mut le = UInt::from(0);
        let mut gt = self.clone() + n1.clone();
        if rhs == UInt::from(0) {
            panic!("attempt to divide by zero");
        }
        // binary search
        while gt > le.clone() + n1.clone() {
            let mid = (gt.clone() + le.clone()) >> n1.clone();
            if mid.clone() * rhs.clone() > self {
                gt = mid;
            } else {
                le = mid;
            }
        }
        le
    }
}

impl ops::DivAssign for UInt {
    fn div_assign(&mut self, rhs: Self) {
        self._binary = (self.clone() / rhs)._binary;
    }
}

impl ops::Rem for UInt {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        // d, m = divmod(self, rhs)
        // self = rhs * d + m
        // m = self - rhs * d
        let d = self.clone() / rhs.clone();
        let r = d * rhs;
        self - r
    }
}

impl ops::RemAssign for UInt {
    fn rem_assign(&mut self, rhs: Self) {
        self._binary = (self.clone() % rhs)._binary;
    }
}
