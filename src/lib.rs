use anyhow::Result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn put_nibble_test() {
        let mut b: u8 = 0x34;
        super::put_nibble(&mut b, 0x5, true);
        assert_eq!(b, 0x54);

        super::put_nibble(&mut b, 0x9, false);
        assert_eq!(b, 0x59);
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

    #[test]
    fn bcd_table_encode_even() {
        let a: &str = "1234567890";
        let b: [u8; 5] = [0x12, 0x34, 0x56, 0x78, 0x90];

        let enc = super::Table::new(super::Encoding::Std8421);
        let mut v = Vec::new();
        let r = enc.encode_str(a, &mut v);

        assert_eq!(Ok(()), r);
        assert!(b.to_vec().eq(&v))
    }

    #[test]
    fn bcd_table_decode_even() {
        let a: &str = "1234567890";
        let b: [u8; 5] = [0x12, 0x34, 0x56, 0x78, 0x90];

        let enc = super::Table::new(super::Encoding::Std8421);
        let mut s = String::new();
        let r = enc.decode_bytes(&b, &mut s);

        assert_eq!(Ok(()), r);
        assert!(a.eq(&s));
    }

    #[test]
    fn bcd_table_decode_odd() {
        let a: &str = "12345678901";
        let b: [u8; 6] = [0x12, 0x34, 0x56, 0x78, 0x90, 0x1F];

        let enc = super::Table::new(super::Encoding::Std8421);
        let mut s = String::new();
        let r = enc.decode_bytes(&b, &mut s);

        assert_eq!(Ok(()), r);
        assert!(a.eq(&s));
    }

    #[test]
    fn bcd_table_encode_odd() {
        let a: &str = "12345678901";
        let b: [u8; 6] = [0x12, 0x34, 0x56, 0x78, 0x90, 0x1F];

        let enc = super::Table::new(super::Encoding::Std8421);
        let mut v = Vec::new();
        let r = enc.encode_str(a, &mut v);

        assert_eq!(Ok(()), r);
        assert!(b.to_vec().eq(&v))
    }

    #[test]
    fn bcd_table_encode_odd_tbcd() {
        let a = String::from("12345678901");
        let b: [u8; 6] = [0x21, 0x43, 0x65, 0x87, 0x09, 0xF1];

        let enc = super::Table::new(super::Encoding::Telephony);
        let mut v = Vec::new();
        let r = enc.encode_str(&a, &mut v);

        assert_eq!(Ok(()), r);
        assert!(b.to_vec().eq(&v))
    }

    #[test]
    fn bcd_table_decode_odd_tbcd() {
        let a = String::from("12345678901");
        let b: [u8; 6] = [0x21, 0x43, 0x65, 0x87, 0x09, 0xF1];

        let enc = super::Table::new(super::Encoding::Telephony);
        let mut s = String::new();
        let r = enc.decode_bytes(&b, &mut s);

        assert_eq!(Ok(()), r);
        assert!(a.eq(&s));
    }

    #[test]
    fn bcd_table_encode_err() {
        let a: &str = "hello";

        let enc = super::Table::new(super::Encoding::Telephony);
        let mut v = Vec::new();
        let r = enc.encode_str(a, &mut v);

        assert_eq!(Err(super::EncodeError::NonEncodable), r);
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Encoding {
    Std8421,
    Aiken,
    Telephony,
    Excess3,
}

#[derive(Copy, Clone)]
struct Table {
    table: [Option<u8>; 16],
    filler_nibble: Option<u8>,
    swap_nibbles: bool,
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
                filler_nibble: Some(0xf),
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
}

#[derive(Debug, PartialEq)]
enum EncodeError {
    NonEncodable,
}

#[derive(Debug, PartialEq)]
enum DecodeError {
    NonDecodable,
}

fn put_nibble(byte: &mut u8, nibble: u8, big: bool) {
    if big {
        *byte &= 0x0F;
        *byte |= (nibble << 4) & 0xF0;
    } else {
        *byte &= 0xF0;
        *byte |= nibble & 0x0F;
    }
}

#[allow(dead_code)]
impl Table {
    // retrieve a nibble that encodes given symbol
    // e.g. '7' -> 7 for 8421 encoding
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

    fn encode_str(&self, s: &str, v: &mut Vec<u8>) -> Result<(), EncodeError> {
        let mut chars = s.chars();

        loop {
            let mut byte: u8 = 0;

            // Get an odd byte
            let mut item = chars.next();
            if let Some(c) = item {
                if let Some(nibble) = self.get_nibble(c as u8) {
                    put_nibble(&mut byte, nibble, !self.swap_nibbles);
                } else {
                    return Err(EncodeError::NonEncodable);
                }
            } else {
                // no more data
                return Ok(());
            }

            // Get an even byte
            item = chars.next();
            if let Some(c) = item {
                if let Some(nibble) = self.get_nibble(c as u8) {
                    put_nibble(&mut byte, nibble, self.swap_nibbles);
                } else {
                    return Err(EncodeError::NonEncodable);
                }
            } else if let Some(c) = self.filler_nibble {
                // put filler; no more data
                put_nibble(&mut byte, c as u8, self.swap_nibbles);
                v.push(byte);
                break;
            } else {
                break;
            }

            v.push(byte);
        }

        Ok(())
    }

    pub(crate) fn decode_bytes(&self, v: &[u8], s: &mut String) -> Result<(), DecodeError> {
        for byte in v {
            let mut pair: [u8; 2] = [0, 0];

            if self.swap_nibbles {
                pair[1] = (byte & 0xF0) >> 4;
                pair[0] = byte & 0xF;
            } else {
                pair[0] = (byte & 0xF0) >> 4;
                pair[1] = byte & 0xF;
            }

            for x in &pair {
                if let Some(c) = self.table[*x as usize] {
                    s.push(c as char);
                } else if Some(*x) == self.filler_nibble {
                    // don't push anything
                } else {
                    return Err(DecodeError::NonDecodable);
                }
            }
        }
        Ok(())
    }
}
