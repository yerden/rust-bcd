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

#[test]
fn bcd_table_decode_err() {
    let b: [u8; 6] = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

    let enc = super::Table::new(super::Encoding::Std8421);
    let mut s = String::new();
    let r = enc.decode_bytes(&b, &mut s);

    assert_eq!(Err(super::DecodeError::NonDecodable), r);
}
