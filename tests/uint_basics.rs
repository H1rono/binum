#[cfg(test)]
mod uint_basics {
    use binum::{num::*, Boolean};
    use Boolean::*;

    const T: Boolean = True;
    const F: Boolean = False;

    #[test]
    fn test_u64_to_uint() {
        // u64 -> UInt
        let nu64: u64 = 0;
        let nuint: UInt = nu64.into();
        assert_eq!(nuint.binary(), &vec![]);
        let nu64: u64 = 0b10011011;
        let nuint: UInt = nu64.into();
        assert_eq!(
            nuint.binary(),
            &("10011011"
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>())
        );
        let nu64 = u64::MAX;
        let nuint: UInt = nu64.into();
        assert_eq!(nuint.binary(), &vec![True; u64::BITS as usize]);
    }

    #[test]
    fn test_uint_to_u64() {
        // UInt -> u64
        let nuint = UInt::new([]);
        let nu64: u64 = nuint.into();
        assert_eq!(nu64, 0);
        let nuint = UInt::new(
            "10011011"
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>(),
        );
        let nu64: u64 = nuint.into();
        assert_eq!(nu64, 0b10011011);
        let nuint = UInt::new([True; u64::BITS as usize]);
        let nu64: u64 = nuint.into();
        assert_eq!(nu64, u64::MAX);
    }

    #[test]
    fn test_max_bit_digit() {
        let n = UInt::new([]);
        assert_eq!(n.max_bit_digit(), 0);
        let n = UInt::new([False; 8]);
        assert_eq!(n.max_bit_digit(), 0);
        let n = UInt::new([True; 8]);
        assert_eq!(n.max_bit_digit(), 8);
        let n = UInt::new(
            "00011011"
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>(),
        );
        assert_eq!(n.max_bit_digit(), 5);
        let n = UInt::new([True; u64::BITS as usize]);
        assert_eq!(n.max_bit_digit(), u64::BITS as usize);
    }

    #[test]
    fn test_trim() {
        let n = UInt::new([]);
        assert_eq!(n.trim().binary(), &vec![]);
        let n = UInt::new([False; 8]);
        assert_eq!(n.trim().binary(), &vec![]);
        let n = UInt::new([True; 8]);
        assert_eq!(n.trim().binary(), &vec![True; 8]);
        let n = UInt::new(
            "00011011"
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>(),
        );
        assert_eq!(n.trim().binary(), &vec![T, T, F, T, T]);
        let n = UInt::new([True; u64::BITS as usize]);
        assert_eq!(n.trim().binary(), &vec![True; u64::BITS as usize]);
    }

    #[test]
    fn test_trim_mut() {
        let mut n = UInt::new([]);
        n.trim_mut();
        assert_eq!(n.binary(), &vec![]);
        let mut n = UInt::new([False; 8]);
        n.trim_mut();
        assert_eq!(n.binary(), &vec![]);
        let mut n = UInt::new([True; 8]);
        n.trim_mut();
        assert_eq!(n.binary(), &vec![True; 8]);
        let mut n = UInt::new(
            "00011011"
                .chars()
                .rev()
                .map(|c| Boolean::from(c == '1'))
                .collect::<Vec<_>>(),
        );
        n.trim_mut();
        assert_eq!(n.binary(), &vec![T, T, F, T, T]);
        let mut n = UInt::new([True; u64::BITS as usize]);
        n.trim_mut();
        assert_eq!(n.binary(), &vec![True; u64::BITS as usize]);
    }
}
