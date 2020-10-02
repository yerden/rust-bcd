#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn bcd_table_test() {
        let enc = super::Table::new(super::Encoding::Std8421);
        assert_eq!(enc.get_nibble('0' as u8), Some(0));
        assert_eq!(enc.get_nibble('1' as u8), Some(1));
        assert_eq!(enc.get_nibble('2' as u8), Some(2));
        assert_eq!(enc.get_nibble('3' as u8), Some(3));
        assert_eq!(enc.get_nibble('4' as u8), Some(4));
        assert_eq!(enc.get_nibble('5' as u8), Some(5));
        assert_eq!(enc.get_nibble('6' as u8), Some(6));
        assert_eq!(enc.get_nibble('7' as u8), Some(7));
        assert_eq!(enc.get_nibble('8' as u8), Some(8));
        assert_eq!(enc.get_nibble('9' as u8), Some(9));

        assert_eq!(enc.get_nibble('X' as u8), None);
    }
}

enum EncodeError {
    NonEncodable,
}

#[derive(Copy, Clone)]
struct Table {
    table: [Option<u8>; 16],
    filler_nibble: Option<u8>,
    swap_nibbles: bool,
}

enum Encoding {
    Std8421,
    Aiken,
    Telephony,
    Excess3,
}

impl Table {
    #[allow(dead_code)]
    fn new(enc: Encoding) -> Table {
        match enc {
            Encoding::Std8421 => Table {
                table: [
                    Some(b'0'), // 0
                    Some(b'1'), // 1
                    Some(b'2'), // 2
                    Some(b'3'), // 3
                    Some(b'4'), // 4
                    Some(b'5'), // 5
                    Some(b'6'), // 6
                    Some(b'7'), // 7
                    Some(b'8'), // 8
                    Some(b'9'), // 9
                    None,       // A
                    None,       // B
                    None,       // C
                    None,       // D
                    None,       // E
                    None,       // F
                ],
                filler_nibble: None,
                swap_nibbles: false,
            },

            Encoding::Telephony => Table {
                table: [
                    Some(b'0'), // 0
                    Some(b'1'), // 1
                    Some(b'2'), // 2
                    Some(b'3'), // 3
                    Some(b'4'), // 4
                    Some(b'5'), // 5
                    Some(b'6'), // 6
                    Some(b'7'), // 7
                    Some(b'8'), // 8
                    Some(b'9'), // 9
                    Some(b'*'), // A
                    Some(b'#'), // B
                    Some(b'a'), // C
                    Some(b'b'), // D
                    Some(b'c'), // E
                    None,       // F
                ],
                filler_nibble: Some(0xf),
                swap_nibbles: true,
            },

            Encoding::Aiken => Table {
                table: [
                    Some(b'0'), // 0
                    Some(b'1'), // 1
                    Some(b'2'), // 2
                    Some(b'3'), // 3
                    Some(b'4'), // 4
                    None,       // 5
                    None,       // 6
                    None,       // 7
                    None,       // 8
                    None,       // 9
                    None,       // A
                    Some(b'5'), // B
                    Some(b'6'), // C
                    Some(b'7'), // D
                    Some(b'8'), // E
                    Some(b'9'), // F
                ],
                filler_nibble: None,
                swap_nibbles: false,
            },

            Encoding::Excess3 => Table {
                table: [
                    None,       // 0
                    None,       // 1
                    None,       // 2
                    Some(b'0'), // 3
                    Some(b'1'), // 4
                    Some(b'2'), // 5
                    Some(b'3'), // 6
                    Some(b'4'), // 7
                    Some(b'5'), // 8
                    Some(b'6'), // 9
                    Some(b'7'), // A
                    Some(b'8'), // B
                    Some(b'9'), // C
                    None,       // D
                    None,       // E
                    None,       // F
                ],
                filler_nibble: None,
                swap_nibbles: false,
            },
        }
    }

    // retrieve a nibble that encodes given symbol
    // e.g. '7' -> 7 for 8421 encoding
    #[allow(dead_code)]
    fn get_nibble(&self, c: u8) -> Option<u8> {
        for i in 0..self.table.len() {
            if let Some(x) = &self.table[i] {
                if *x == c {
                    return Some(i as u8);
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn encode(&self, s: &str, v: &mut Vec<u8>) -> Option<EncodeError> {
        Some(EncodeError::NonEncodable)
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
