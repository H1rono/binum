#[cfg(test)]
mod uint_ops {
    use binum::{num::UInt, Boolean};
    use Boolean::*;

    #[test]
    fn test_bitor() {
        // corner case
        let n1 = UInt::new([]);
        let n2 = UInt::from(u64::MAX);
        assert_eq!(n1 | n1, UInt::new([]));
        assert_eq!(n1 | n2, n2);
        // truth table
        let n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        assert_eq!(n1 | n2, UInt::from(0b1011));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        assert_eq!(n1 | n1, n1); // idempotency
        let n2 = UInt::from(0b10100000);
        assert_eq!(n1 | n2, n2 | n1); // commutativity
        let n3 = UInt::from(0b11000000);
        assert_eq!((n1 | n2) | n3, n1 | (n2 | n3)); // associativity
    }

    #[test]
    fn test_bitor_assign() {
        // corner case
        let mut n1 = UInt::new([]);
        let mut n2 = UInt::from(u64::MAX);
        n1 |= n1;
        assert_eq!(n1, UInt::new([]));
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
        n4 |= n4;
        assert_eq!(n4, n1.clone()); // idempotency
        let mut n5 = n2.clone();
        n4 |= n5;
        n5 |= n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 |= n2.clone();
        n4 |= n3.clone();
        let mut n5 = n1.clone();
        let mut n6 = n2.clone();
        n6 |= n3.clone();
        n5 |= n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }

    #[test]
    fn test_bitand() {
        // corner case
        let n1 = UInt::from(0);
        let n2 = UInt::from(u64::MAX);
        assert_eq!(n1 & n1, UInt::new([]));
        assert_eq!(n1 & n2, UInt::new([]));
        // truth table
        let n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        assert_eq!(n1 & n2, UInt::from(0b0001));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        assert_eq!(n1 & n1, n1); // idempotency
        let n2 = UInt::from(0b10100000);
        assert_eq!(n1 & n2, n2 & n1); // commutativity
        let n3 = UInt::from(0b11000000);
        assert_eq!((n1 & n2) & n3, n1 & (n2 & n3)); // associativity
    }

    #[test]
    fn test_bitor_assign() {
        // corner case
        let mut n1 = UInt::new([]);
        let mut n2 = UInt::from(u64::MAX);
        n1 &= n1;
        assert_eq!(n1, UInt::from(0));
        n2 &= n1;
        assert_eq!(n2, UInt::from(0));
        // truth table
        let mut n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        n1 &= n2;
        assert_eq!(n1, UInt::from(0b0001));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        let n3 = UInt::from(0b11000000);
        let mut n4 = n1.clone();
        n4 &= n4;
        assert_eq!(n4, n1.clone()); // idempotency
        let mut n5 = n2.clone();
        n4 &= n5;
        n5 &= n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 &= n2.clone();
        n4 &= n3.clone();
        let mut n5 = n1.clone();
        let mut n6 = n2.clone();
        n6 &= n3.clone();
        n5 &= n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }

    #[test]
    fn test_bitxor() {
        // corner case
        let n1 = UInt::from(0);
        let n2 = UInt::from(u64::MAX);
        assert_eq!(n1 ^ n1, UInt::new([]));
        assert_eq!(n1 ^ n2, UInt::new([]));
        // truth table
        let n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        assert_eq!(n1 ^ n2, UInt::from(0b1001));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        assert_eq!(n1 ^ n1, n1); // idempotency
        let n2 = UInt::from(0b10100000);
        assert_eq!(n1 ^ n2, n2 ^ n1); // commutativity
        let n3 = UInt::from(0b11000000);
        assert_eq!((n1 ^ n2) ^ n3, n1 ^ (n2 ^ n3)); // associativity
    }

    #[test]
    fn test_bitor_assign() {
        // corner case
        let mut n1 = UInt::new([]);
        let mut n2 = UInt::from(u64::MAX);
        n1 ^= n1;
        assert_eq!(n1, UInt::from(0));
        n2 ^= n1;
        assert_eq!(n2, UInt::new([True; u64::BITS as usize]));
        // truth table
        let mut n1 = UInt::from(0b0011);
        let n2 = UInt::from(0b1010);
        n1 ^= n2;
        assert_eq!(n1, UInt::from(0b1001));
        // idempotency, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        let n3 = UInt::from(0b11000000);
        let mut n4 = n1.clone();
        n4 ^= n4;
        assert_eq!(n4, n1.clone()); // idempotency
        let mut n5 = n2.clone();
        n4 ^= n5;
        n5 ^= n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 ^= n2.clone();
        n4 ^= n3.clone();
        let mut n5 = n1.clone();
        let mut n6 = n2.clone();
        n6 ^= n3.clone();
        n5 ^= n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }
}
