use crate::Boolean;

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