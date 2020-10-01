#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn bcd_table_test() {
        assert_eq!(super::STD8421.get_nibble('0' as u8), Some(0));
        assert_eq!(super::STD8421.get_nibble('1' as u8), Some(1));
        assert_eq!(super::STD8421.get_nibble('2' as u8), Some(2));
        assert_eq!(super::STD8421.get_nibble('3' as u8), Some(3));
        assert_eq!(super::STD8421.get_nibble('4' as u8), Some(4));
        assert_eq!(super::STD8421.get_nibble('5' as u8), Some(5));
        assert_eq!(super::STD8421.get_nibble('6' as u8), Some(6));
        assert_eq!(super::STD8421.get_nibble('7' as u8), Some(7));
        assert_eq!(super::STD8421.get_nibble('8' as u8), Some(8));
        assert_eq!(super::STD8421.get_nibble('9' as u8), Some(9));
    }
}

enum EncodeError {
    ErrNonEncodable,
}

#[derive(Copy, Clone)]
enum Digit {
    Char(u8), // symbol
    Filler,   // filler nibble if bytes count is odd
    Empty,    // invalid nibble
}

#[derive(Copy, Clone)]
struct BcdTable {
    table: [Digit; 16],
    swap_nibbles: bool,
}

const STD8421: BcdTable = BcdTable {
    table: [
        Digit::Char(b'0'), // 0
        Digit::Char(b'1'), // 1
        Digit::Char(b'2'), // 2
        Digit::Char(b'3'), // 3
        Digit::Char(b'4'), // 4
        Digit::Char(b'5'), // 5
        Digit::Char(b'6'), // 6
        Digit::Char(b'7'), // 7
        Digit::Char(b'8'), // 8
        Digit::Char(b'9'), // 9
        Digit::Empty,      // A
        Digit::Empty,      // B
        Digit::Empty,      // C
        Digit::Empty,      // D
        Digit::Empty,      // E
        Digit::Filler,     // F
    ],

    swap_nibbles: false,
};

const TELEPHONY: BcdTable = BcdTable {
    table: [
        Digit::Char(b'0'), // 0
        Digit::Char(b'1'), // 1
        Digit::Char(b'2'), // 2
        Digit::Char(b'3'), // 3
        Digit::Char(b'4'), // 4
        Digit::Char(b'5'), // 5
        Digit::Char(b'6'), // 6
        Digit::Char(b'7'), // 7
        Digit::Char(b'8'), // 8
        Digit::Char(b'9'), // 9
        Digit::Char(b'*'), // A
        Digit::Char(b'#'), // B
        Digit::Char(b'a'), // C
        Digit::Char(b'b'), // D
        Digit::Char(b'c'), // E
        Digit::Filler,     // F
    ],

    swap_nibbles: true,
};

const AIKEN: BcdTable = BcdTable {
    table: [
        Digit::Char(b'0'), // 0
        Digit::Char(b'1'), // 1
        Digit::Char(b'2'), // 2
        Digit::Char(b'3'), // 3
        Digit::Char(b'4'), // 4
        Digit::Empty,      // 5
        Digit::Empty,      // 6
        Digit::Empty,      // 7
        Digit::Empty,      // 8
        Digit::Empty,      // 9
        Digit::Empty,      // A
        Digit::Char(b'5'), // B
        Digit::Char(b'6'), // C
        Digit::Char(b'7'), // D
        Digit::Char(b'8'), // E
        Digit::Char(b'9'), // F
    ],

    swap_nibbles: false,
};

impl BcdTable {
    fn new_telephony() -> BcdTable {
        TELEPHONY
    }

    fn new_aiken() -> BcdTable {
        AIKEN
    }

    fn new_std8421() -> BcdTable {
        STD8421
    }

    // retrieve a nibble that encodes given symbol
    // e.g. '7' -> 7 for 8421 encoding
    fn get_nibble(&self, c: u8) -> Option<u8> {
        for i in 0..self.table.len() {
            if let Digit::Char(x) = &self.table[i] {
                if *x == c {
                    return Some(i as u8);
                }
            }
        }

        None
    }

    fn encode(&self, s: &str, v: &mut Vec<u8>) -> Option<EncodeError> {
        Some(EncodeError::ErrNonEncodable)
    }
}

//
//trait WithConstructor {
//fn new_with_param(param: usize) -> Self;

//fn new() -> Self
//where
//Self: Sized,
//{
//Self::new_with_param(0)
//}
//}
//
