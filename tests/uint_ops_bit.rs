#[cfg(test)]
mod uint_ops {
    use binum::{Boolean, UInt};
    use Boolean::*;

    #[test]
    fn test_bitand() {
        // corner case
        let n1 = UInt::from(0);
        let n2 = UInt::from(u64::MAX);
        assert_eq!(n1.clone() & n1.clone(), UInt::new([]));
        assert_eq!(n1 & n2, UInt::new([]));
        // truth table
        let n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        assert_eq!(n1 & n2, UInt::from(0b0010));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        assert_eq!(n1.clone() & n1.clone(), n1); // idempotency
        let n2 = UInt::from(0b10100000);
        assert_eq!(n1.clone() & n2.clone(), n2.clone() & n1.clone()); // commutativity
        let n3 = UInt::from(0b11000000);
        assert_eq!((n1.clone() & n2.clone()) & n3.clone(), n1 & (n2 & n3)); // associativity
    }

    #[test]
    fn test_bitand_assign() {
        // corner case
        let mut n1 = UInt::new([]);
        let mut n2 = UInt::from(u64::MAX);
        n1 &= n1.clone();
        assert_eq!(n1, UInt::from(0));
        n2 &= n1;
        assert_eq!(n2, UInt::from(0));
        // truth table
        let mut n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        n1 &= n2;
        assert_eq!(n1, UInt::from(0b0010));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        let n3 = UInt::from(0b11000000);
        let mut n4 = n1.clone();
        n4 &= n4.clone();
        assert_eq!(n4, n1); // idempotency
        let mut n5 = n2.clone();
        n4 &= n5.clone();
        n5 &= n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 &= n2.clone();
        n4 &= n3.clone();
        let mut n5 = n1;
        let mut n6 = n2;
        n6 &= n3;
        n5 &= n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }

    #[test]
    fn test_bitor() {
        // corner case
        let n1 = UInt::new([]);
        let n2 = UInt::from(u64::MAX);
        assert_eq!(n1.clone() | n1.clone(), UInt::new([]));
        assert_eq!(n1 | n2, UInt::new([True; u64::BITS as usize]));
        // truth table
        let n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        assert_eq!(n1 | n2, UInt::from(0b1011));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        assert_eq!(n1.clone() | n1.clone(), n1); // idempotency
        let n2 = UInt::from(0b10100000);
        assert_eq!(n1.clone() | n2.clone(), n2.clone() | n1.clone()); // commutativity
        let n3 = UInt::from(0b11000000);
        assert_eq!((n1.clone() | n2.clone()) | n3.clone(), n1 | (n2 | n3)); // associativity
    }

    #[test]
    fn test_bitor_assign() {
        // corner case
        let mut n1 = UInt::new([]);
        let mut n2 = UInt::from(u64::MAX);
        n1 |= n1.clone();
        assert_eq!(n1.clone(), UInt::new([]));
        n2 |= n1;
        assert_eq!(n2, UInt::new([True; u64::BITS as usize]));
        // truth table
        let mut n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        n1 |= n2;
        assert_eq!(n1, UInt::from(0b1011));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        let n3 = UInt::from(0b11000000);
        let mut n4 = n1.clone();
        n4 |= n4.clone();
        assert_eq!(n4, n1); // idempotency
        let mut n5 = n2.clone();
        n4 |= n5.clone();
        n5 |= n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 |= n2.clone();
        n4 |= n3.clone();
        let mut n5 = n1;
        let mut n6 = n2;
        n6 |= n3;
        n5 |= n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }

    #[test]
    fn test_bitxor() {
        // corner case
        let n1 = UInt::from(0);
        let n2 = UInt::from(u64::MAX);
        assert_eq!(n1.clone() ^ n1.clone(), UInt::new([]));
        assert_eq!(n1 ^ n2, UInt::new([True; u64::BITS as usize]));
        // truth table
        let n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        assert_eq!(n1 ^ n2, UInt::from(0b1001));
        // commutativity, associativity
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        assert_eq!(n1.clone() ^ n2.clone(), n2.clone() ^ n1.clone()); // commutativity
        let n3 = UInt::from(0b11000000);
        assert_eq!((n1.clone() ^ n2.clone()) ^ n3.clone(), n1 ^ (n2 ^ n3)); // associativity
    }

    #[test]
    fn test_bitxor_assign() {
        // corner case
        let mut n1 = UInt::new([]);
        let mut n2 = UInt::from(u64::MAX);
        n1 ^= n1.clone();
        assert_eq!(n1, UInt::from(0));
        n2 ^= n1;
        assert_eq!(n2, UInt::new([True; u64::BITS as usize]));
        // truth table
        let mut n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        n1 ^= n2;
        assert_eq!(n1, UInt::from(0b1001));
        // commutativity, associativity
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        let n3 = UInt::from(0b11000000);
        let mut n4 = n1.clone();
        let mut n5 = n2.clone();
        n4 ^= n5.clone();
        n5 ^= n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 ^= n2.clone();
        n4 ^= n3.clone();
        let mut n5 = n1;
        let mut n6 = n2;
        n6 ^= n3;
        n5 ^= n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }

    #[test]
    fn test_shl() {
        // basics
        assert_eq!(UInt::from(3) << UInt::from(2), UInt::from(12));
        // corner case
        let left = UInt::from(1) << UInt::from(u64::BITS as u64);
        let right = UInt::new(
            format!("1{:>064}", 0)
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>(),
        );
        assert_eq!(left, right);
        // unity
        let unit = UInt::from(0);
        let n = UInt::from(9);
        assert_eq!(n.clone() << unit, n);
    }

    #[test]
    fn test_shl_assign() {
        // basics
        let mut n = UInt::from(3);
        n <<= UInt::from(2);
        assert_eq!(n, UInt::from(12));
        // corner case
        let mut left = UInt::from(1);
        left <<= UInt::from(u64::BITS as u64);
        let right = UInt::new(
            format!("1{:>064}", 0)
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>(),
        );
        assert_eq!(left, right);
        // unity
        let unit = UInt::from(0);
        let mut n = UInt::from(9);
        n <<= unit;
        assert_eq!(n, UInt::from(9));
    }

    #[test]
    fn test_shr() {
        // basics
        assert_eq!(UInt::from(0b1101) >> UInt::from(2), UInt::from(0b11));
        // corner case
        let left = UInt::from(1) >> UInt::from(10);
        let right = UInt::from(0);
        assert_eq!(left, right);
        // unity
        let unit = UInt::from(0);
        let n = UInt::from(9);
        assert_eq!(n.clone() >> unit, n);
    }

    #[test]
    fn test_shr_assign() {
        // basics
        let mut n = UInt::from(0b1101);
        n >>= UInt::from(2);
        assert_eq!(n, UInt::from(0b11));
        // corner case
        let mut left = UInt::from(1);
        left >>= UInt::from(10);
        let right = UInt::from(0);
        assert_eq!(left, right);
        // unity
        let unit = UInt::from(0);
        let mut n = UInt::from(9);
        n >>= unit;
        assert_eq!(n, UInt::from(9));
    }
}
