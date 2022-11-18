extern crate binum;

#[cfg(test)]
mod bool_basics {
    use binum::Boolean;
    use Boolean::*;

    #[test]
    fn test_add() {
        assert_eq!(False & False, False);
        assert_eq!(False & True, False);
        assert_eq!(True & False, False);
        assert_eq!(True & True, True);
    }

    #[test]
    fn test_or() {
        assert_eq!(False | False, False);
        assert_eq!(False | True, True);
        assert_eq!(True | False, True);
        assert_eq!(True | True, True);
    }

    #[test]
    fn test_xor() {
        assert_eq!(False ^ False, False);
        assert_eq!(False ^ True, True);
        assert_eq!(True ^ False, True);
        assert_eq!(True ^ True, False);
    }
}
