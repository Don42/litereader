use nom::{IResult, Needed};


#[warn(dead_code)]
fn parse_varint(i: &[u8]) -> IResult<&[u8], u64> {
    let mut content: u64 = 0;
    let mut count: usize = 0;
    for &c in i {
        if count < 8 {
            content <<= 7;
            content += (c & 0x7Fu8) as u64
        } else {
            content <<= 8;
            content += c as u64;
        }
        if (0x80u8 & c) == 0 || count == 8 {
            return IResult::Done(&i[(count+1)..], content);
        }
        count += 1;
    }
    return IResult::Incomplete(Needed::Size(9 - count))
}

#[cfg(test)]
mod tests {
    #[test]
    fn varint_simple() {
        let buffer: [u8; 16] = [0x81, 0x3e, 0x05, 0x07, 0x17, 0x29, 0x29, 0x01, 0x82,
            0x37, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x6f];
        let (i, res) = super::parse_varint(&buffer).unwrap();
        assert_eq!(190, res);
        assert_eq!(&buffer[2..], i)
    }
}
