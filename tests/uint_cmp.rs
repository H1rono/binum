#[cfg(test)]
mod uint_cmp {
    use binum::{Boolean, UInt};
    use Boolean::*;

    #[test]
    fn test_eq() {
        let n1 = UInt::from(255);
        let n2 = UInt::from(0b11111111);
        assert_eq!(n1, n1); // reflexivity
        let n3 = UInt::new([True; 8]);
        // transitivity
        assert_eq!(n1, n2);
        assert_eq!(n2, n3);
        assert_eq!(n3, n1);
        assert_eq!(n1, n3); // symmetry
        let n4 = UInt::from(0);
        let n5 = UInt::from(u64::MAX);
        assert_ne!(n1, n4);
        assert_ne!(n3, n5);
    }

    #[test]
    fn test_ord() {
        let n1 = UInt::from(0b100111010);
        let n2 = UInt::new(
            "100111010"
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>(),
        );
        // reflexivity
        assert!(n1 <= n1);
        // anti-symmetry
        assert!(n1 <= n2 && n2 <= n1 && n1 == n2);
        // transitivity
        let n3 = UInt::from(315);
        assert!(n1 < n3);
        assert!(n3 > n1);
        let n4 = UInt::from(0);
        assert!(n4 < n1);
        assert!(n4 < n3);
    }
}
