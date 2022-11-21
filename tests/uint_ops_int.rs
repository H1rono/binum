#[cfg(test)]
mod uint_ops_int {
    use binum::{Boolean, UInt};
    use Boolean::*;

    #[test]
    fn test_add() {
        // basic
        assert_eq!(UInt::from(9) + UInt::from(16), UInt::from(25));
        // corner case
        let n0 = UInt::from(0);
        let n1 = UInt::from(u64::MAX);
        assert_eq!(n0.clone() + n0.clone(), UInt::new([]));
        assert_eq!(n0.clone() + n1, UInt::new([True; u64::BITS as usize]));
        // unity, commutativity, associativity
        let n1 = UInt::from(0b10011011);
        // unity
        assert_eq!(n1.clone() + n0.clone(), n0.clone() + n1.clone());
        assert_eq!(n1.clone() + n0, n1);
        let n2 = UInt::from(0b10100000);
        assert_eq!(n1.clone() + n2.clone(), n2.clone() + n1.clone()); // commutativity
        let n3 = UInt::from(0b11000000);
        assert_eq!((n1.clone() + n2.clone()) + n3.clone(), n1 + (n2 + n3)); // associativity
    }

    #[test]
    fn test_add_assign() {
        // basic
        let mut n1 = UInt::from(9);
        n1 += UInt::from(16);
        assert_eq!(n1, UInt::from(25));
        // corner case
        let mut n0 = UInt::new([]);
        let mut n1 = UInt::from(u64::MAX);
        n0 += n0.clone();
        assert_eq!(n0, UInt::from(0));
        n1 += n0;
        assert_eq!(n1, UInt::new([True; u64::BITS as usize]));
        // unity, commutativity, associativity
        let unit = UInt::from(0);
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        let n3 = UInt::from(0b11000000);
        let mut n4 = n1.clone();
        let mut n5 = unit.clone();
        n4 += unit;
        n5 += n1.clone();
        // unity
        assert_eq!(n4, n5);
        assert_eq!(n4, n1);
        let mut n4 = n1.clone();
        let mut n5 = n2.clone();
        n4 += n5.clone();
        n5 += n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 += n2.clone();
        n4 += n3.clone();
        let mut n5 = n1;
        let mut n6 = n2;
        n6 += n3;
        n5 += n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }

    #[test]
    fn test_mul() {
        // basic
        assert_eq!(UInt::from(9) * UInt::from(6), UInt::from(54));
        // corner case
        let n0 = UInt::from(0);
        let n1 = UInt::from(u64::MAX);
        assert_eq!(n0.clone() * n0.clone(), UInt::new([]));
        assert_eq!(n0 * n1, UInt::new([]));
        // unity, commutativity, associativity
        let n1 = UInt::from(1);
        let n2 = UInt::from(0b10011011);
        // unity
        assert_eq!(n2.clone() * n1.clone(), n1.clone() * n2.clone());
        assert_eq!(n2.clone() * n1, n2);
        let n3 = UInt::from(0b10100000);
        assert_eq!(n2.clone() * n3.clone(), n3.clone() * n2.clone()); // commutativity
        let n4 = UInt::from(0b11000000);
        assert_eq!((n2.clone() * n3.clone()) * n4.clone(), n2 * (n3 * n4)); // associativity
    }

    #[test]
    fn test_mul_assign() {
        // basic
        let mut n1 = UInt::from(9);
        n1 *= UInt::from(6);
        assert_eq!(n1, UInt::from(54));
        // corner case
        let mut n0 = UInt::new([]);
        let mut n1 = UInt::from(u64::MAX);
        n0 *= n0.clone();
        assert_eq!(n0, UInt::from(0));
        n1 *= n0;
        assert_eq!(n1, UInt::new([]));
        // unity, commutativity, associativity
        let unit = UInt::from(1);
        let n1 = UInt::from(0b10011011);
        let n2 = UInt::from(0b10100000);
        let n3 = UInt::from(0b11000000);
        let mut n4 = n1.clone();
        let mut n5 = unit.clone();
        n4 *= unit;
        n5 *= n1.clone();
        // unity
        assert_eq!(n4, n5);
        assert_eq!(n4, n1);
        let mut n4 = n1.clone();
        let mut n5 = n2.clone();
        n4 *= n5.clone();
        n5 *= n1.clone();
        assert_eq!(n4, n5); // commutativity
        let mut n4 = n1.clone();
        n4 *= n2.clone();
        n4 *= n3.clone();
        let mut n5 = n1;
        let mut n6 = n2;
        n6 *= n3;
        n5 *= n6;
        assert_eq!(n4, n5); // (pseudo) associativity
    }

    #[test]
    fn test_sub() {
        // basic
        assert_eq!(UInt::from(16) - UInt::from(9), UInt::from(7));
        assert_eq!(UInt::from(1) - UInt::from(1), UInt::from(0));
        // corner case
        let n0 = UInt::from(0);
        let n1 = UInt::from(u64::MAX);
        assert_eq!(n0.clone() - n0.clone(), UInt::new([]));
        assert_eq!(n1 - n0.clone(), UInt::new([True; u64::BITS as usize]));
        // unity
        let unit = UInt::from(0);
        let n1 = UInt::from(0b10011011);
        assert_eq!(n1.clone() - unit, n1);
        // inverse
        assert_eq!(n1.clone() - n1, n0);
    }

    #[test]
    #[should_panic]
    fn test_sub_fails() {
        // this should cause panic
        let _ = UInt::from(9) - UInt::from(16);
    }

    #[test]
    fn test_sub_assign() {
        // basic
        let mut n1 = UInt::from(16);
        n1 -= UInt::from(9);
        assert_eq!(n1, UInt::from(7));
        // corner case
        let mut n0 = UInt::from(0);
        let mut n1 = UInt::from(u64::MAX);
        n0 -= n0.clone();
        assert_eq!(n0, UInt::new([]));
        n1 -= n0;
        assert_eq!(n1, UInt::new([True; u64::BITS as usize]));
        // unity
        let unit = UInt::from(0);
        let mut n1 = UInt::from(0b10011011);
        n1 -= unit.clone();
        assert_eq!(n1, UInt::from(0b10011011));
        // inverse
        n1 -= n1.clone();
        assert_eq!(n1, unit);
    }

    #[test]
    #[should_panic]
    fn test_sub_assign_fails() {
        let mut n1 = UInt::from(9);
        n1 -= UInt::from(16);
    }

    #[test]
    fn test_div() {
        // basic
        assert_eq!(UInt::from(55) / UInt::from(9), UInt::from(6));
        // corner case
        let n0 = UInt::from(1);
        let n1 = UInt::from(u64::MAX);
        assert_eq!(n0.clone() / n0.clone(), UInt::new([True]));
        assert_eq!(n1 / n0, UInt::new([True; u64::BITS as usize]));
        // unity
        let unit = UInt::from(1);
        let n2 = UInt::from(0b10011011);
        assert_eq!(n2.clone() / unit.clone(), n2);
        // inverse
        assert_eq!(n2.clone() / n2, unit);
    }

    #[test]
    #[should_panic]
    fn test_div_fails() {
        // this should cause ZeroDivisionError
        let _ = UInt::from(1) / UInt::from(0);
    }

    #[test]
    fn test_div_assign() {
        // basic
        let mut n1 = UInt::from(16);
        n1 /= UInt::from(9);
        assert_eq!(n1, UInt::from(1));
        // corner case
        let mut n1 = UInt::from(1);
        let mut n2 = UInt::from(u64::MAX);
        n1 /= n1.clone();
        assert_eq!(n1, UInt::new([True]));
        n2 /= n1;
        assert_eq!(n2, UInt::new([True; u64::BITS as usize]));
        // unity
        let unit = UInt::from(1);
        let mut n1 = UInt::from(0b10011011);
        n1 /= unit.clone();
        assert_eq!(n1, UInt::from(0b10011011));
        // inverse
        n1 /= n1.clone();
        assert_eq!(n1, unit);
    }

    #[test]
    #[should_panic]
    fn test_div_assign_fails() {
        let mut n1 = UInt::from(1);
        // ZeroDivisionError
        n1 /= UInt::from(0);
    }

    #[test]
    fn test_rem() {
        // basic
        assert_eq!(UInt::from(16) % UInt::from(9), UInt::from(7));
        // corner case
        let n0 = UInt::from(1);
        let n1 = UInt::from(u64::MAX);
        assert_eq!(n0.clone() % n0.clone(), UInt::new([]));
        assert_eq!(n1 % n0, UInt::new([]));
        // (maybe) inverse
        let n1 = UInt::from(0b10011011);
        assert_eq!(n1.clone() % n1, UInt::from(0));
    }

    #[test]
    #[should_panic]
    fn test_rem_fails() {
        // ZeroDivisionError
        let _ = UInt::from(16) % UInt::from(0);
    }

    #[test]
    fn test_rem_assign() {
        // basic
        let mut n1 = UInt::from(16);
        n1 %= UInt::from(9);
        assert_eq!(n1, UInt::from(7));
        // corner case
        let mut n1 = UInt::from(1);
        let mut n2 = UInt::from(u64::MAX);
        n1 %= n1.clone();
        assert_eq!(n1, UInt::new([]));
        n2 %= UInt::from(1);
        assert_eq!(n2, UInt::new([]));
        // (maybe) inverse
        let mut n1 = UInt::from(0b10011011);
        n1 %= n1.clone();
        assert_eq!(n1, UInt::from(0));
    }

    #[test]
    #[should_panic]
    fn test_rem_assign_fails() {
        // ZeroDivisionError
        let mut n1 = UInt::from(15);
        n1 %= UInt::from(0);
    }
}
