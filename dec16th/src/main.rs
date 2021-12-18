extern crate hex;

#[derive(Debug)]
struct Hdr {
    version: u8,
    typeid: u8,
}
#[derive(Debug)]
enum Payload {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Greater(Vec<Packet>),
    Less(Vec<Packet>),
    Equal(Vec<Packet>),
}
#[derive(Debug)]
struct Packet {
    hdr: Hdr,
    pyl: Payload,
}

fn main() {
    let packets: Vec<Vec<u8>> = include_str!("input.txt")
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| hex::decode(x).expect("Decoding failed"))
        .collect();

    for rawpacket in &packets {
        let packet = parse_packet(rawpacket, 0);
        //println!("{:#?}", packet);
        println!("1: {}", version_sum(&packet.0));
        println!("1: {}", solve(&packet.0));
    }
}

fn parse_packet(raw: &[u8], ptr: usize) -> (Packet, usize) {
    let hdr = get_hdr(&raw, ptr);
    if hdr.typeid == 4 {
        let mut literal = 0u64;
        let mut idx = 0;
        loop {
            let tmp = getu8(raw, ptr + 6 + idx * 5, 5);
            literal = (literal << 4) | (tmp & 0xF) as u64;
            idx += 1;
            if tmp >> 4 == 0 {
                break;
            }
        }
        (
            Packet {
                hdr: hdr,
                pyl: Payload::Literal(literal),
            },
            6 + idx * 5,
        )
    } else {
        let ltypeid = getu8(raw, ptr + 6, 1);
        let mut subp = Vec::new();
        let mut pyllen = 0usize;
        let mut innerptr = 0usize;
        if ltypeid == 0 {
            let bitslen = getu16(raw, ptr + 7, 15) as usize;
            pyllen += 15 + bitslen as usize;
            while innerptr < bitslen {
                let (packet, size) = parse_packet(raw, ptr + 7 + 15 + innerptr);
                subp.push(packet);
                innerptr += size;
            }
        } else {
            let subp_nb = getu16(raw, ptr + 7, 11);
            pyllen += 11;
            for _ in 0..subp_nb {
                let (packet, size) = parse_packet(raw, ptr + 7 + 11 + innerptr);
                subp.push(packet);
                innerptr += size;
                pyllen += size;
            }
        }
        (
            Packet {
                pyl: {
                    if hdr.typeid == 0 {            Payload::Sum(subp)
                    } else if hdr.typeid == 1 {     Payload::Product(subp)
                    } else if hdr.typeid == 2 {     Payload::Minimum(subp)
                    } else if hdr.typeid == 3 {     Payload::Maximum(subp)
                    } else if hdr.typeid == 5 {     Payload::Greater(subp)
                    } else if hdr.typeid == 6 {     Payload::Less(subp)
                    } else if hdr.typeid == 7 {     Payload::Equal(subp)
                    } else {                        unreachable!();
                    }
                },
                hdr: hdr,
            },
            6 + 1 + pyllen,
        )
    }
}

fn version_sum(p: &Packet) -> u32 {
    let mut s = 0;
    match &p.pyl {
        Payload::Sum(v)
        | Payload::Product(v)
        | Payload::Minimum(v)
        | Payload::Maximum(v)
        | Payload::Greater(v)
        | Payload::Less(v)
        | Payload::Equal(v) => {
            for i in v {
                s += version_sum(&i);
            }
        }
        _ => {}
    }
    s + p.hdr.version as u32
}

fn solve(p: &Packet) -> u64 {
    match &p.pyl {
        Payload::Sum(v)     => {v.iter().map(|i| solve(i)).sum()},
        Payload::Product(v) => {v.iter().map(|i| solve(i)).product()},
        Payload::Minimum(v) => {v.iter().map(|i| solve(i)).min().unwrap()},
        Payload::Maximum(v) => {v.iter().map(|i| solve(i)).max().unwrap()},
        Payload::Greater(v) => {let t = v.iter().map(|i| solve(i)).collect::<Vec<u64>>(); if t[0]>t[1] {1}else{0}},
        Payload::Less(v)    => {let t = v.iter().map(|i| solve(i)).collect::<Vec<u64>>(); if t[0]<t[1] {1}else{0}},
        Payload::Equal(v)   => {let t = v.iter().map(|i| solve(i)).collect::<Vec<u64>>(); if t[0]==t[1]{1}else{0}},
        Payload::Literal(x) => {*x}
    }
}

fn get_hdr(raw: &[u8], ptr: usize) -> Hdr {
    Hdr {
        version: getu8(raw, ptr + 0, 3),
        typeid: getu8(raw, ptr + 3, 3),
    }
}

fn getu8(input: &[u8], off: usize, len: usize) -> u8 {
    assert_eq!(len <= 8, true);
    if input.len() * 8 - off > 8 {
        let window: [u8; 2] = input[off / 8..off / 8 + 2].try_into().unwrap();
        let d = off - ((off / 8) * 8);
        ((u16::from_be_bytes(window) >> (16 - d - len)) & (2u16.pow(len as u32) - 1)) as u8
    } else {
        let window: [u8; 1] = input[off / 8..off / 8 + 1].try_into().unwrap();
        let d = off - ((off / 8) * 8);
        (u8::from_be_bytes(window) >> (8 - d - len)) & (2u16.pow(len as u32) - 1) as u8
    }
}

fn getu16(input: &[u8], off: usize, len: usize) -> u16 {
    assert_eq!(len <= 16, true);
    if input.len() * 8 - off > 16 {
        let window: [u8; 4] = input[off / 8..off / 8 + 4].try_into().unwrap();
        let d = off - ((off / 8) * 8);
        ((u32::from_be_bytes(window) >> (32 - d - len)) & (2u32.pow(len as u32) - 1))as u16
    } else {
        unimplemented!();
    }
}

#[allow(dead_code)]
fn getu32(input: &[u8], off: usize, len: usize) -> u32 {
    assert_eq!(len <= 32, true);
    if input.len() * 8 - off > 16 {
        let window: [u8; 8] = input[off / 8..off / 8 + 8].try_into().unwrap();
        let d = off - ((off / 8) * 8);
        ((u64::from_be_bytes(window) >> (64 - d - len)) & (2u64.pow(len as u32) - 1)) as u32
    } else {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
    ];

    #[test]
    fn test_getu8() {
        assert_eq!(0x1, getu8(&INPUT, 64 + 2, 3));
        assert_eq!(0x4, getu8(&INPUT, 64, 3));
        assert_eq!(0x2, getu8(&INPUT, 64 + 7, 3));
        assert_eq!(0b01001100, getu8(&INPUT, 64 + 7, 8));
        assert_eq!(0x0, getu8(&INPUT, 64 + 7, 1));
        assert_eq!(0x1, getu8(&INPUT, 15 * 8, 1));
        assert_eq!(0xFF, getu8(&INPUT, 15 * 8, 8));
    }

    #[test]
    #[should_panic]
    fn test_getu8_panic() {
        getu8(&INPUT, 0, 9);
    }

    #[test]
    fn test_getu16() {
        assert_eq!(0x1, getu16(&INPUT, 64 + 2, 3));
        assert_eq!(0x4, getu16(&INPUT, 64, 3));
        assert_eq!(0x2, getu16(&INPUT, 64 + 7, 3));
        assert_eq!(0b01001100, getu16(&INPUT, 64 + 7, 8));
        assert_eq!(0x0, getu16(&INPUT, 64 + 7, 1));

        assert_eq!(0b0100110011010101, getu16(&INPUT, 64 + 7, 16));
        assert_eq!(0b10001000100, getu16(&INPUT, 64, 11));
    }

    #[test]
    #[should_panic]
    fn test_getu16_panic() {
        getu16(&INPUT, 0, 17);
    }

    #[test]
    fn test_getu32() {
        assert_eq!(0x1, getu32(&INPUT, 64 + 2, 3));
        assert_eq!(0x4, getu32(&INPUT, 64, 3));
        assert_eq!(0x2, getu32(&INPUT, 64 + 7, 3));
        assert_eq!(0b01001100, getu32(&INPUT, 64 + 7, 8));
        assert_eq!(0x0, getu32(&INPUT, 64 + 7, 1));

        assert_eq!(0b0100110011010101, getu32(&INPUT, 64 + 7, 16));
        assert_eq!(0b10001000100, getu32(&INPUT, 64, 11));

        assert_eq!(0b0100110011010101010111011, getu32(&INPUT, 64 + 7, 25));
        assert_eq!(0b1000100010011001101, getu32(&INPUT, 64, 19));
    }

    #[test]
    #[should_panic]
    fn test_getu32_panic() {
        getu32(&INPUT, 0, 33);
    }
}
