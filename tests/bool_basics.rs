#[cfg(test)]
mod bool_basics {
    use std::str::FromStr;

    use binum::Boolean;
    use Boolean::*;

    #[test]
    fn test_not() {
        assert_eq!(!False, True);
        assert_eq!(!True, False);
    }

    #[test]
    fn test_and() {
        assert_eq!(False & False, False);
        assert_eq!(False & True, False);
        assert_eq!(True & False, False);
        assert_eq!(True & True, True);
    }

    #[test]
    fn test_andassign() {
        let mut b = True;
        b &= True; // True &= True
        assert_eq!(b, True);
        b &= False; // True &= False
        assert_eq!(b, False);
        b &= True; // False &= True
        assert_eq!(b, False);
        b &= False; // False &= False
        assert_eq!(b, False);
    }

    #[test]
    fn test_or() {
        assert_eq!(False | False, False);
        assert_eq!(False | True, True);
        assert_eq!(True | False, True);
        assert_eq!(True | True, True);
    }

    #[test]
    fn test_orassign() {
        let mut b = False;
        b |= False; // False |= False
        assert_eq!(b, False);
        b |= True; // False |= True
        assert_eq!(b, True);
        b |= False; // True |= False
        assert_eq!(b, True);
        b |= True; // True |= True
        assert_eq!(b, True);
    }

    #[test]
    fn test_xor() {
        assert_eq!(False ^ False, False);
        assert_eq!(False ^ True, True);
        assert_eq!(True ^ False, True);
        assert_eq!(True ^ True, False);
    }

    #[test]
    fn test_xorassign() {
        let mut b = False;
        b ^= False; // False ^= False
        assert_eq!(b, False);
        b ^= True; // False ^= True
        assert_eq!(b, True);
        b ^= False; // True ^= False
        assert_eq!(b, True);
        b ^= True; // True ^= True
        assert_eq!(b, False);
    }

    #[test]
    fn test_boolcnv() {
        assert!(!bool::from(False));
        assert!(bool::from(True));
        assert_eq!(Boolean::from(false), False);
        assert_eq!(Boolean::from(true), True);
    }

    #[test]
    fn test_fromstr() {
        match Boolean::from_str("") {
            Err(_) => (), // OK
            _ => unreachable!(),
        };
        match Boolean::from_str("True") {
            Ok(True) => (),
            _ => unreachable!(),
        };
        match Boolean::from_str("False") {
            Ok(False) => (),
            _ => unreachable!(),
        };
        match Boolean::from_str("ture") {
            Err(_) => (),
            _ => unreachable!(),
        };
        match Boolean::from_str("fasle") {
            Err(_) => (),
            _ => unreachable!(),
        };
    }
}
