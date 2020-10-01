use anyhow::Result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

enum EncodeError {
    ErrNonEncodable,
}

#[derive(Copy, Clone)]
pub(crate) enum Digit {
    Char(u8),
    Filler,
    Empty,
}

#[derive(Copy, Clone)]
pub(crate) struct BcdTable {
    table: [Digit; 16],
    swap_nibbles: bool,
}

pub(crate) const STD8421: BcdTable = BcdTable {
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

pub(crate) const TELEPHONY: BcdTable = BcdTable {
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

pub(crate) const AIKEN: BcdTable = BcdTable {
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
    fn get_nibble(&self, c: u8) -> Digit {
        for d in &self.table {
            if let Digit::Char(x) = d {
                if x == c {
                    *d
                }
            }
        }
        Digit::Empty
    }
    //fn encode(&self, s: &str, v: &mut Vec<u8>) -> Option<EncodeError> {
    //for c in s.chars() {
    //let x = c as u8
    //if let Digit::Char(x) = c {
    //Some(EncodeError::ErrNonEncodable)
    //}
    //}
    //None
    //}
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
