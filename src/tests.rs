//! Testing for `arctan-bitflags`.

use super::*;

//   ######             #######   ########  ########
//  ##    ##            ##    ##     ##        ##
//   ######   #######   #######      ##        ##
//  ##    ##            ##    ##     ##        ##
//   ######             #######   ########     ##

#[test]
fn bitflags8_creation() {
    let f1a = BitFlags8::new();
    let f1b = BitFlags8(0);
    let f1c = BitFlags8::from(0);
    let f1d: BitFlags8 = 0.into();
    assert_eq!(f1a, f1b);
    assert_eq!(f1b, f1c);
    assert_eq!(f1c, f1d);

    let f2a = BitFlags8::from_slice(&[2, 4, 5]);
    let f2b = BitFlags8::from_bits(0b0011_0100);
    assert_eq!(f2a, f2b);
}

#[test]
fn bitflags8_ops() {
    let flags1 = BitFlags8(0b0001);
    let flags2 = BitFlags8(0b1001);

    assert_eq!(flags1 | flags2, BitFlags8(0b1001));
    assert_eq!(flags1 & flags2, BitFlags8(0b0001));
}

#[test]
fn bitflags8_general() {
    let v0 = BitFlags8(0b00000000);
    let v1 = BitFlags8(0b00000100);
    let va = BitFlags8(0b11111111);

    assert!(v0.is_empty());
    assert!(va.is_all());
    assert!(BitFlags8(0b00000010).intersects(BitFlags8(0b00000111)));
    assert!(!BitFlags8(0b00000010).intersects(BitFlags8(0b00100000)));
    assert!(BitFlags8(0b00000111).contains(BitFlags8(0b00000011)));
    assert!(!BitFlags8(0b00000111).contains(BitFlags8(0b00100011)));
    assert_eq!(BitFlags8(0b00000010), BitFlags8::from_index(1));
    assert_eq!(v1.bit_at_index(2), true);
    assert_eq!(v1.bit_at_index(3), false);
    assert_eq!(v1.get_bit_at_index(2), Some(true));
    assert_eq!(v1.get_bit_at_index(3), Some(false));
    assert_eq!(v1.get_bit_at_index(8), None);
}

#[test]
fn bitflags8_intersection() {
    let f1 = BitFlags8(0b0000);
    let f2 = BitFlags8(0b0001);
    let f3 = BitFlags8(0b1001);

    assert_eq!(f1.intersection(f2), BitFlags8(0b0000));
    assert_eq!(f1.intersection(f3), BitFlags8(0b0000));
    assert_eq!(f2.intersection(f3), BitFlags8(0b0001));
}

#[test]
fn bitflags8_union() {
    let f1 = BitFlags8(0b0000);
    let f2 = BitFlags8(0b0001);
    let f3 = BitFlags8(0b1001);

    assert_eq!(f1.union(f2), BitFlags8(0b0001));
    assert_eq!(f1.union(f3), BitFlags8(0b1001));
    assert_eq!(f2.union(f3), BitFlags8(0b1001));
}

#[test]
fn bitflags8_highest_set_bit() {
    let values = &[
        BitFlags8(0),
        BitFlags8(1),
        BitFlags8(2),
        BitFlags8(3),
        BitFlags8(4),
        BitFlags8(5),
        BitFlags8(8),
        BitFlags8(16),
        BitFlags8(32),
        BitFlags8(64),
    ];

    let expected = vec![0, 1, 2, 2, 4, 4, 8, 16, 32, 64];
    let returned = values.iter().map(|f| f.highest_set_bit()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags8_highest_set_bit_ix() {
    let values = &[
        BitFlags8(0),
        BitFlags8(1),
        BitFlags8(2),
        BitFlags8(3),
        BitFlags8(4),
        BitFlags8(5),
        BitFlags8(8),
        BitFlags8(16),
        BitFlags8(32),
        BitFlags8(64),
    ];
    let expected = vec![
        None,
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ];
    let returned = values.iter().map(|f| f.highest_set_bit_index()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags8_count_ones() {
    let v0 = BitFlags8(0b00000000);
    let v1 = BitFlags8(0b01000000);
    let v2 = BitFlags8(0b00100001);
    let v3 = BitFlags8(0b00011001);
    let v4 = BitFlags8(0b10101001);
    let v5 = BitFlags8(0b10111001);

    assert_eq!(v0.count_ones(), 0);
    assert_eq!(v1.count_ones(), 1);
    assert_eq!(v2.count_ones(), 2);
    assert_eq!(v3.count_ones(), 3);
    assert_eq!(v4.count_ones(), 4);
    assert_eq!(v5.count_ones(), 5);
}

#[test]
fn bitflags8_iter() {
    let v0 = BitFlags8(0b00000000);
    let v1 = BitFlags8(0b00001001);
    let v2 = BitFlags8(0b01001001);
    let v3 = BitFlags8(0b11111111);

    let v0_bits: Vec<usize> = v0.iter().collect();
    let v1_bits: Vec<usize> = v1.iter().collect();
    let v2_bits: Vec<usize> = v2.iter().collect();
    let v3_bits: Vec<usize> = v3.iter().collect();

    assert_eq!(&v0_bits, &[] as &[usize; 0]);
    assert_eq!(&v1_bits, &[0, 3]);
    assert_eq!(&v2_bits, &[0, 3, 6]);
    assert_eq!(&v3_bits, &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn bitflags8_insert_at_index() {
    let mut v0 = BitFlags8(0b00000000);
    let mut v1 = BitFlags8(0b00000000);
    let mut v2 = BitFlags8(0b00000000);
    let mut v3 = BitFlags8(0b00000000);
    let mut va = BitFlags8(0b11111111);

    v0.insert_at_index(0);
    v1.insert_at_index(1);
    v2.insert_at_index(2);
    v3.insert_at_index(3);
    va.insert_at_index(7);

    assert_eq!(v0, BitFlags8(0b00000001));
    assert_eq!(v1, BitFlags8(0b00000010));
    assert_eq!(v2, BitFlags8(0b00000100));
    assert_eq!(v3, BitFlags8(0b00001000));
    assert_eq!(va, BitFlags8(0b11111111));
}

#[test]
fn bitflags8_remove_at_index() {
    let mut v0 = BitFlags8(0b11111111);
    let mut v1 = BitFlags8(0b11111111);
    let mut v2 = BitFlags8(0b11111111);
    let mut v3 = BitFlags8(0b11111111);
    let mut va = BitFlags8(0b00000000);

    v0.remove_at_index(0);
    v1.remove_at_index(1);
    v2.remove_at_index(2);
    v3.remove_at_index(3);
    va.remove_at_index(7);

    assert_eq!(v0, BitFlags8(0b11111110));
    assert_eq!(v1, BitFlags8(0b11111101));
    assert_eq!(v2, BitFlags8(0b11111011));
    assert_eq!(v3, BitFlags8(0b11110111));
    assert_eq!(va, BitFlags8(0b00000000));
}

#[test]
fn bitflags8_set() {
    let mut f0 = BitFlags8(0b0000);
    let f1 = BitFlags8(0b0001);
    let f2 = BitFlags8(0b1001);
    let f3 = BitFlags8(0b1111);

    f0.set(f1, true);
    assert_eq!(f0, BitFlags8(0b0001));
    
    f0.set(f2, true);
    assert_eq!(f0, BitFlags8(0b1001));

    f0.set(f3, true);
    assert_eq!(f0, BitFlags8(0b1111));
}

#[test]
fn bitflags8_unset() {
    let mut f0 = BitFlags8(0b1111);
    let f1 = BitFlags8(0b0001);
    let f2 = BitFlags8(0b1001);
    let f3 = BitFlags8(0b1111);

    f0.set(f1, false);
    assert_eq!(f0, BitFlags8(0b1110));
    
    f0.set(f2, false);
    assert_eq!(f0, BitFlags8(0b0110));

    f0.set(f3, false);
    assert_eq!(f0, BitFlags8(0b0000));
}

#[test]
fn bitflags8_set_at_index() {
    let mut v0 = BitFlags8(0b00000000);
    let mut v1 = BitFlags8(0b00000000);
    let mut v2 = BitFlags8(0b00000000);
    let mut v3 = BitFlags8(0b00000000);
    let mut va = BitFlags8(0b11111111);

    v0.set_at_index(0, true);
    v1.set_at_index(1, true);
    v2.set_at_index(2, true);
    v3.set_at_index(3, true);
    va.set_at_index(7, true);

    assert_eq!(v0, BitFlags8(0b00000001));
    assert_eq!(v1, BitFlags8(0b00000010));
    assert_eq!(v2, BitFlags8(0b00000100));
    assert_eq!(v3, BitFlags8(0b00001000));
    assert_eq!(va, BitFlags8(0b11111111));

    v0.set_at_index(0, false);
    v1.set_at_index(1, false);
    v2.set_at_index(2, false);
    v3.set_at_index(3, false);
    va.set_at_index(7, false);

    assert_eq!(v0, BitFlags8(0b00000000));
    assert_eq!(v1, BitFlags8(0b00000000));
    assert_eq!(v2, BitFlags8(0b00000000));
    assert_eq!(v3, BitFlags8(0b00000000));
    assert_eq!(va, BitFlags8(0b01111111));
}

#[test]
fn bitflags8_toggle_at_index() {
    let mut v0 = BitFlags8(0b00000000);
    let mut v1 = BitFlags8(0b00000000);
    let mut v2 = BitFlags8(0b00000000);
    let mut v3 = BitFlags8(0b00000000);
    let mut va = BitFlags8(0b11111111);

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags8(0b00000001));
    assert_eq!(v1, BitFlags8(0b00000010));
    assert_eq!(v2, BitFlags8(0b00000100));
    assert_eq!(v3, BitFlags8(0b00001000));
    assert_eq!(va, BitFlags8(0b01111111));

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags8(0b00000000));
    assert_eq!(v1, BitFlags8(0b00000000));
    assert_eq!(v2, BitFlags8(0b00000000));
    assert_eq!(v3, BitFlags8(0b00000000));
    assert_eq!(va, BitFlags8(0b11111111));
}

//     ##        ###              #######   ########  ########
//   ####      ##                 ##    ##     ##        ##
//     ##     #######   #######   #######      ##        ##
//     ##     ##    ##            ##    ##     ##        ##
//   ######    ######             #######   ########     ##

#[test]
fn bitflags16_creation() {
    let f1a = BitFlags16::new();
    let f1b = BitFlags16(0);
    let f1c = BitFlags16::from(0);
    let f1d: BitFlags16 = 0.into();
    assert_eq!(f1a, f1b);
    assert_eq!(f1b, f1c);
    assert_eq!(f1c, f1d);

    let f2a = BitFlags16::from_slice(&[2, 4, 5]);
    let f2b = BitFlags16::from_bits(0b0011_0100);
    assert_eq!(f2a, f2b);
}

#[test]
fn bitflags16_ops() {
    let flags1 = BitFlags16(0b0001);
    let flags2 = BitFlags16(0b1001);

    assert_eq!(flags1 | flags2, BitFlags16(0b1001));
    assert_eq!(flags1 & flags2, BitFlags16(0b0001));
}

#[test]
fn bitflags16_general() {
    let v0 = BitFlags16(0b00000000);
    let v1 = BitFlags16(0b00000100);
    let va = BitFlags16(u16::MAX);

    assert!(v0.is_empty());
    assert!(va.is_all());
    assert!(BitFlags16(0b00000010).intersects(BitFlags16(0b00000111)));
    assert!(!BitFlags16(0b00000010).intersects(BitFlags16(0b00100000)));
    assert!(BitFlags16(0b00000111).contains(BitFlags16(0b00000011)));
    assert!(!BitFlags16(0b00000111).contains(BitFlags16(0b00100011)));
    assert_eq!(BitFlags16(0b00000010), BitFlags16::from_index(1));
    assert_eq!(v1.bit_at_index(2), true);
    assert_eq!(v1.bit_at_index(3), false);
    assert_eq!(v1.get_bit_at_index(2), Some(true));
    assert_eq!(v1.get_bit_at_index(3), Some(false));
    assert_eq!(v1.get_bit_at_index(16), None);
}

#[test]
fn bitflags16_intersection() {
    let f1 = BitFlags16(0b0000);
    let f2 = BitFlags16(0b0001);
    let f3 = BitFlags16(0b1001);

    assert_eq!(f1.intersection(f2), BitFlags16(0b0000));
    assert_eq!(f1.intersection(f3), BitFlags16(0b0000));
    assert_eq!(f2.intersection(f3), BitFlags16(0b0001));
}

#[test]
fn bitflags16_union() {
    let f1 = BitFlags16(0b0000);
    let f2 = BitFlags16(0b0001);
    let f3 = BitFlags16(0b1001);

    assert_eq!(f1.union(f2), BitFlags16(0b0001));
    assert_eq!(f1.union(f3), BitFlags16(0b1001));
    assert_eq!(f2.union(f3), BitFlags16(0b1001));
}

#[test]
fn bitflags16_highest_set_bit() {
    let values = &[
        BitFlags16(0),
        BitFlags16(1),
        BitFlags16(2),
        BitFlags16(3),
        BitFlags16(4),
        BitFlags16(5),
        BitFlags16(8),
        BitFlags16(16),
        BitFlags16(32),
        BitFlags16(64),
    ];

    let expected = vec![0, 1, 2, 2, 4, 4, 8, 16, 32, 64];
    let returned = values.iter().map(|f| f.highest_set_bit()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags16_highest_set_bit_ix() {
    let values = &[
        BitFlags8(0),
        BitFlags8(1),
        BitFlags8(2),
        BitFlags8(3),
        BitFlags8(4),
        BitFlags8(5),
        BitFlags8(8),
        BitFlags8(16),
        BitFlags8(32),
        BitFlags8(64),
    ];
    let expected = vec![
        None,
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ];
    let returned = values.iter().map(|f| f.highest_set_bit_index()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags16_count_ones() {
    let v0 = BitFlags16(0b00000000);
    let v1 = BitFlags16(0b01000000);
    let v2 = BitFlags16(0b00100001);
    let v3 = BitFlags16(0b00011001);
    let v4 = BitFlags16(0b10101001);
    let v5 = BitFlags16(0b10111001);

    assert_eq!(v0.count_ones(), 0);
    assert_eq!(v1.count_ones(), 1);
    assert_eq!(v2.count_ones(), 2);
    assert_eq!(v3.count_ones(), 3);
    assert_eq!(v4.count_ones(), 4);
    assert_eq!(v5.count_ones(), 5);
}

#[test]
fn bitflags16_iter() {
    let v0 = BitFlags16(0b00000000);
    let v1 = BitFlags16(0b00001001);
    let v2 = BitFlags16(0b01001001);
    let v3 = BitFlags16(0b11111111);

    let v0_bits: Vec<usize> = v0.iter().collect();
    let v1_bits: Vec<usize> = v1.iter().collect();
    let v2_bits: Vec<usize> = v2.iter().collect();
    let v3_bits: Vec<usize> = v3.iter().collect();

    assert_eq!(&v0_bits, &[] as &[usize; 0]);
    assert_eq!(&v1_bits, &[0, 3]);
    assert_eq!(&v2_bits, &[0, 3, 6]);
    assert_eq!(&v3_bits, &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn bitflags16_insert_at_index() {
    let mut v0 = BitFlags16(0b00000000);
    let mut v1 = BitFlags16(0b00000000);
    let mut v2 = BitFlags16(0b00000000);
    let mut v3 = BitFlags16(0b00000000);
    let mut va = BitFlags16(0b11111111);

    v0.insert_at_index(0);
    v1.insert_at_index(1);
    v2.insert_at_index(2);
    v3.insert_at_index(3);
    va.insert_at_index(7);

    assert_eq!(v0, BitFlags16(0b00000001));
    assert_eq!(v1, BitFlags16(0b00000010));
    assert_eq!(v2, BitFlags16(0b00000100));
    assert_eq!(v3, BitFlags16(0b00001000));
    assert_eq!(va, BitFlags16(0b11111111));
}

#[test]
fn bitflags16_remove_at_index() {
    let mut v0 = BitFlags16(0b11111111);
    let mut v1 = BitFlags16(0b11111111);
    let mut v2 = BitFlags16(0b11111111);
    let mut v3 = BitFlags16(0b11111111);
    let mut va = BitFlags16(0b00000000);

    v0.remove_at_index(0);
    v1.remove_at_index(1);
    v2.remove_at_index(2);
    v3.remove_at_index(3);
    va.remove_at_index(7);

    assert_eq!(v0, BitFlags16(0b11111110));
    assert_eq!(v1, BitFlags16(0b11111101));
    assert_eq!(v2, BitFlags16(0b11111011));
    assert_eq!(v3, BitFlags16(0b11110111));
    assert_eq!(va, BitFlags16(0b00000000));
}

#[test]
fn bitflags16_set() {
    let mut f0 = BitFlags16(0b0000);
    let f1 = BitFlags16(0b0001);
    let f2 = BitFlags16(0b1001);
    let f3 = BitFlags16(0b1111);

    f0.set(f1, true);
    assert_eq!(f0, BitFlags16(0b0001));
    
    f0.set(f2, true);
    assert_eq!(f0, BitFlags16(0b1001));

    f0.set(f3, true);
    assert_eq!(f0, BitFlags16(0b1111));
}

#[test]
fn bitflags16_unset() {
    let mut f0 = BitFlags16(0b1111);
    let f1 = BitFlags16(0b0001);
    let f2 = BitFlags16(0b1001);
    let f3 = BitFlags16(0b1111);

    f0.set(f1, false);
    assert_eq!(f0, BitFlags16(0b1110));
    
    f0.set(f2, false);
    assert_eq!(f0, BitFlags16(0b0110));

    f0.set(f3, false);
    assert_eq!(f0, BitFlags16(0b0000));
}

#[test]
fn bitflags16_set_at_index() {
    let mut v0 = BitFlags16(0b00000000);
    let mut v1 = BitFlags16(0b00000000);
    let mut v2 = BitFlags16(0b00000000);
    let mut v3 = BitFlags16(0b00000000);
    let mut va = BitFlags16(0b11111111);

    v0.set_at_index(0, true);
    v1.set_at_index(1, true);
    v2.set_at_index(2, true);
    v3.set_at_index(3, true);
    va.set_at_index(7, true);

    assert_eq!(v0, BitFlags16(0b00000001));
    assert_eq!(v1, BitFlags16(0b00000010));
    assert_eq!(v2, BitFlags16(0b00000100));
    assert_eq!(v3, BitFlags16(0b00001000));
    assert_eq!(va, BitFlags16(0b11111111));

    v0.set_at_index(0, false);
    v1.set_at_index(1, false);
    v2.set_at_index(2, false);
    v3.set_at_index(3, false);
    va.set_at_index(7, false);

    assert_eq!(v0, BitFlags16(0b00000000));
    assert_eq!(v1, BitFlags16(0b00000000));
    assert_eq!(v2, BitFlags16(0b00000000));
    assert_eq!(v3, BitFlags16(0b00000000));
    assert_eq!(va, BitFlags16(0b01111111));
}

#[test]
fn bitflags16_toggle_at_index() {
    let mut v0 = BitFlags16(0b00000000);
    let mut v1 = BitFlags16(0b00000000);
    let mut v2 = BitFlags16(0b00000000);
    let mut v3 = BitFlags16(0b00000000);
    let mut va = BitFlags16(0b11111111);

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags16(0b00000001));
    assert_eq!(v1, BitFlags16(0b00000010));
    assert_eq!(v2, BitFlags16(0b00000100));
    assert_eq!(v3, BitFlags16(0b00001000));
    assert_eq!(va, BitFlags16(0b01111111));

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags16(0b00000000));
    assert_eq!(v1, BitFlags16(0b00000000));
    assert_eq!(v2, BitFlags16(0b00000000));
    assert_eq!(v3, BitFlags16(0b00000000));
    assert_eq!(va, BitFlags16(0b11111111));
}

//  #######    ######            #######   ########  ########
//        ##  ##    ##           ##    ##     ##        ##
//    #####      ####   #######  #######      ##        ##
//        ##   ##                ##    ##     ##        ##
//  #######   ########           #######   ########     ##

#[test]
fn bitflags32_creation() {
    let f1a = BitFlags32::new();
    let f1b = BitFlags32(0);
    let f1c = BitFlags32::from(0);
    let f1d: BitFlags32 = 0.into();
    assert_eq!(f1a, f1b);
    assert_eq!(f1b, f1c);
    assert_eq!(f1c, f1d);

    let f2a = BitFlags32::from_slice(&[2, 4, 5]);
    let f2b = BitFlags32::from_bits(0b0011_0100);
    assert_eq!(f2a, f2b);
}

#[test]
fn bitflags32_ops() {
    let flags1 = BitFlags32(0b0001);
    let flags2 = BitFlags32(0b1001);

    assert_eq!(flags1 | flags2, BitFlags32(0b1001));
    assert_eq!(flags1 & flags2, BitFlags32(0b0001));
}

#[test]
fn bitflags32_general() {
    let v0 = BitFlags32(0b00000000);
    let v1 = BitFlags32(0b00000100);
    let va = BitFlags32(u32::MAX);

    assert!(v0.is_empty());
    assert!(va.is_all());
    assert!(BitFlags32(0b00000010).intersects(BitFlags32(0b00000111)));
    assert!(!BitFlags32(0b00000010).intersects(BitFlags32(0b00100000)));
    assert!(BitFlags32(0b00000111).contains(BitFlags32(0b00000011)));
    assert!(!BitFlags32(0b00000111).contains(BitFlags32(0b00100011)));
    assert_eq!(BitFlags32(0b00000010), BitFlags32::from_index(1));
    assert_eq!(v1.bit_at_index(2), true);
    assert_eq!(v1.bit_at_index(3), false);
    assert_eq!(v1.get_bit_at_index(2), Some(true));
    assert_eq!(v1.get_bit_at_index(3), Some(false));
    assert_eq!(v1.get_bit_at_index(32), None);
}

#[test]
fn bitflags32_intersection() {
    let f1 = BitFlags32(0b0000);
    let f2 = BitFlags32(0b0001);
    let f3 = BitFlags32(0b1001);

    assert_eq!(f1.intersection(f2), BitFlags32(0b0000));
    assert_eq!(f1.intersection(f3), BitFlags32(0b0000));
    assert_eq!(f2.intersection(f3), BitFlags32(0b0001));
}

#[test]
fn bitflags32_union() {
    let f1 = BitFlags32(0b0000);
    let f2 = BitFlags32(0b0001);
    let f3 = BitFlags32(0b1001);

    assert_eq!(f1.union(f2), BitFlags32(0b0001));
    assert_eq!(f1.union(f3), BitFlags32(0b1001));
    assert_eq!(f2.union(f3), BitFlags32(0b1001));
}

#[test]
fn bitflags32_highest_set_bit() {
    let values = &[
        BitFlags32(0),
        BitFlags32(1),
        BitFlags32(2),
        BitFlags32(3),
        BitFlags32(4),
        BitFlags32(5),
        BitFlags32(8),
        BitFlags32(16),
        BitFlags32(32),
        BitFlags32(64),
    ];

    let expected = vec![0, 1, 2, 2, 4, 4, 8, 16, 32, 64];
    let returned = values.iter().map(|f| f.highest_set_bit()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags32_highest_set_bit_ix() {
    let values = &[
        BitFlags32(0),
        BitFlags32(1),
        BitFlags32(2),
        BitFlags32(3),
        BitFlags32(4),
        BitFlags32(5),
        BitFlags32(8),
        BitFlags32(16),
        BitFlags32(32),
        BitFlags32(64),
    ];
    let expected = vec![
        None,
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ];
    let returned = values.iter().map(|f| f.highest_set_bit_index()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags32_count_ones() {
    let v0 = BitFlags32(0b00000000);
    let v1 = BitFlags32(0b01000000);
    let v2 = BitFlags32(0b00100001);
    let v3 = BitFlags32(0b00011001);
    let v4 = BitFlags32(0b10101001);
    let v5 = BitFlags32(0b10111001);

    assert_eq!(v0.count_ones(), 0);
    assert_eq!(v1.count_ones(), 1);
    assert_eq!(v2.count_ones(), 2);
    assert_eq!(v3.count_ones(), 3);
    assert_eq!(v4.count_ones(), 4);
    assert_eq!(v5.count_ones(), 5);
}

#[test]
fn bitflags32_iter() {
    let v0 = BitFlags32(0b00000000);
    let v1 = BitFlags32(0b00001001);
    let v2 = BitFlags32(0b01001001);
    let v3 = BitFlags32(0b11111111);

    let v0_bits: Vec<usize> = v0.iter().collect();
    let v1_bits: Vec<usize> = v1.iter().collect();
    let v2_bits: Vec<usize> = v2.iter().collect();
    let v3_bits: Vec<usize> = v3.iter().collect();

    assert_eq!(&v0_bits, &[] as &[usize; 0]);
    assert_eq!(&v1_bits, &[0, 3]);
    assert_eq!(&v2_bits, &[0, 3, 6]);
    assert_eq!(&v3_bits, &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn bitflags32_insert_at_index() {
    let mut v0 = BitFlags32(0b00000000);
    let mut v1 = BitFlags32(0b00000000);
    let mut v2 = BitFlags32(0b00000000);
    let mut v3 = BitFlags32(0b00000000);
    let mut va = BitFlags32(0b11111111);

    v0.insert_at_index(0);
    v1.insert_at_index(1);
    v2.insert_at_index(2);
    v3.insert_at_index(3);
    va.insert_at_index(7);

    assert_eq!(v0, BitFlags32(0b00000001));
    assert_eq!(v1, BitFlags32(0b00000010));
    assert_eq!(v2, BitFlags32(0b00000100));
    assert_eq!(v3, BitFlags32(0b00001000));
    assert_eq!(va, BitFlags32(0b11111111));
}

#[test]
fn bitflags32_remove_at_index() {
    let mut v0 = BitFlags32(0b11111111);
    let mut v1 = BitFlags32(0b11111111);
    let mut v2 = BitFlags32(0b11111111);
    let mut v3 = BitFlags32(0b11111111);
    let mut va = BitFlags32(0b00000000);

    v0.remove_at_index(0);
    v1.remove_at_index(1);
    v2.remove_at_index(2);
    v3.remove_at_index(3);
    va.remove_at_index(7);

    assert_eq!(v0, BitFlags32(0b11111110));
    assert_eq!(v1, BitFlags32(0b11111101));
    assert_eq!(v2, BitFlags32(0b11111011));
    assert_eq!(v3, BitFlags32(0b11110111));
    assert_eq!(va, BitFlags32(0b00000000));
}

#[test]
fn bitflags32_set() {
    let mut f0 = BitFlags32(0b0000);
    let f1 = BitFlags32(0b0001);
    let f2 = BitFlags32(0b1001);
    let f3 = BitFlags32(0b1111);

    f0.set(f1, true);
    assert_eq!(f0, BitFlags32(0b0001));
    
    f0.set(f2, true);
    assert_eq!(f0, BitFlags32(0b1001));

    f0.set(f3, true);
    assert_eq!(f0, BitFlags32(0b1111));
}

#[test]
fn bitflags32_unset() {
    let mut f0 = BitFlags32(0b1111);
    let f1 = BitFlags32(0b0001);
    let f2 = BitFlags32(0b1001);
    let f3 = BitFlags32(0b1111);

    f0.set(f1, false);
    assert_eq!(f0, BitFlags32(0b1110));
    
    f0.set(f2, false);
    assert_eq!(f0, BitFlags32(0b0110));

    f0.set(f3, false);
    assert_eq!(f0, BitFlags32(0b0000));
}

#[test]
fn bitflags32_set_at_index() {
    let mut v0 = BitFlags32(0b00000000);
    let mut v1 = BitFlags32(0b00000000);
    let mut v2 = BitFlags32(0b00000000);
    let mut v3 = BitFlags32(0b00000000);
    let mut va = BitFlags32(0b11111111);

    v0.set_at_index(0, true);
    v1.set_at_index(1, true);
    v2.set_at_index(2, true);
    v3.set_at_index(3, true);
    va.set_at_index(7, true);

    assert_eq!(v0, BitFlags32(0b00000001));
    assert_eq!(v1, BitFlags32(0b00000010));
    assert_eq!(v2, BitFlags32(0b00000100));
    assert_eq!(v3, BitFlags32(0b00001000));
    assert_eq!(va, BitFlags32(0b11111111));

    v0.set_at_index(0, false);
    v1.set_at_index(1, false);
    v2.set_at_index(2, false);
    v3.set_at_index(3, false);
    va.set_at_index(7, false);

    assert_eq!(v0, BitFlags32(0b00000000));
    assert_eq!(v1, BitFlags32(0b00000000));
    assert_eq!(v2, BitFlags32(0b00000000));
    assert_eq!(v3, BitFlags32(0b00000000));
    assert_eq!(va, BitFlags32(0b01111111));
}

#[test]
fn bitflags32_toggle_at_index() {
    let mut v0 = BitFlags32(0b00000000);
    let mut v1 = BitFlags32(0b00000000);
    let mut v2 = BitFlags32(0b00000000);
    let mut v3 = BitFlags32(0b00000000);
    let mut va = BitFlags32(0b11111111);

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags32(0b00000001));
    assert_eq!(v1, BitFlags32(0b00000010));
    assert_eq!(v2, BitFlags32(0b00000100));
    assert_eq!(v3, BitFlags32(0b00001000));
    assert_eq!(va, BitFlags32(0b01111111));

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags32(0b00000000));
    assert_eq!(v1, BitFlags32(0b00000000));
    assert_eq!(v2, BitFlags32(0b00000000));
    assert_eq!(v3, BitFlags32(0b00000000));
    assert_eq!(va, BitFlags32(0b11111111));
}

//    ###     ##    ##            #######   ########  ########
//   ##       ##    ##            ##    ##     ##        ##
//  #######   ########  #######   #######      ##        ##
//  ##    ##        ##            ##    ##     ##        ##
//   ######         ##            #######   ########     ##

#[test]
fn bitflags64_creation() {
    let f1a = BitFlags64::new();
    let f1b = BitFlags64(0);
    let f1c = BitFlags64::from(0);
    let f1d: BitFlags64 = 0.into();
    assert_eq!(f1a, f1b);
    assert_eq!(f1b, f1c);
    assert_eq!(f1c, f1d);

    let f2a = BitFlags64::from_slice(&[2, 4, 5]);
    let f2b = BitFlags64::from_bits(0b0011_0100);
    assert_eq!(f2a, f2b);
}

#[test]
fn bitflags64_ops() {
    let flags1 = BitFlags64(0b0001);
    let flags2 = BitFlags64(0b1001);

    assert_eq!(flags1 | flags2, BitFlags64(0b1001));
    assert_eq!(flags1 & flags2, BitFlags64(0b0001));
}

#[test]
fn bitflags64_general() {
    let v0 = BitFlags64(0b00000000);
    let v1 = BitFlags64(0b00000100);
    let va = BitFlags64(u64::MAX);

    assert!(v0.is_empty());
    assert!(va.is_all());
    assert!(BitFlags64(0b00000010).intersects(BitFlags64(0b00000111)));
    assert!(!BitFlags64(0b00000010).intersects(BitFlags64(0b00100000)));
    assert!(BitFlags64(0b00000111).contains(BitFlags64(0b00000011)));
    assert!(!BitFlags64(0b00000111).contains(BitFlags64(0b00100011)));
    assert_eq!(BitFlags64(0b00000010), BitFlags64::from_index(1));
    assert_eq!(v1.bit_at_index(2), true);
    assert_eq!(v1.bit_at_index(3), false);
    assert_eq!(v1.get_bit_at_index(2), Some(true));
    assert_eq!(v1.get_bit_at_index(3), Some(false));
    assert_eq!(v1.get_bit_at_index(64), None);
}

#[test]
fn bitflags64_intersection() {
    let f1 = BitFlags64(0b0000);
    let f2 = BitFlags64(0b0001);
    let f3 = BitFlags64(0b1001);

    assert_eq!(f1.intersection(f2), BitFlags64(0b0000));
    assert_eq!(f1.intersection(f3), BitFlags64(0b0000));
    assert_eq!(f2.intersection(f3), BitFlags64(0b0001));
}

#[test]
fn bitflags64_union() {
    let f1 = BitFlags64(0b0000);
    let f2 = BitFlags64(0b0001);
    let f3 = BitFlags64(0b1001);

    assert_eq!(f1.union(f2), BitFlags64(0b0001));
    assert_eq!(f1.union(f3), BitFlags64(0b1001));
    assert_eq!(f2.union(f3), BitFlags64(0b1001));
}

#[test]
fn bitflags64_highest_set_bit() {
    let values = &[
        BitFlags64(0),
        BitFlags64(1),
        BitFlags64(2),
        BitFlags64(3),
        BitFlags64(4),
        BitFlags64(5),
        BitFlags64(8),
        BitFlags64(16),
        BitFlags64(32),
        BitFlags64(64),
    ];

    let expected = vec![0, 1, 2, 2, 4, 4, 8, 16, 32, 64];
    let returned = values.iter().map(|f| f.highest_set_bit()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags64_highest_set_bit_ix() {
    let values = &[
        BitFlags64(0),
        BitFlags64(1),
        BitFlags64(2),
        BitFlags64(3),
        BitFlags64(4),
        BitFlags64(5),
        BitFlags64(8),
        BitFlags64(16),
        BitFlags64(32),
        BitFlags64(64),
    ];
    let expected = vec![
        None,
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ];
    let returned = values.iter().map(|f| f.highest_set_bit_index()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags64_count_ones() {
    let v0 = BitFlags64(0b00000000);
    let v1 = BitFlags64(0b01000000);
    let v2 = BitFlags64(0b00100001);
    let v3 = BitFlags64(0b00011001);
    let v4 = BitFlags64(0b10101001);
    let v5 = BitFlags64(0b10111001);

    assert_eq!(v0.count_ones(), 0);
    assert_eq!(v1.count_ones(), 1);
    assert_eq!(v2.count_ones(), 2);
    assert_eq!(v3.count_ones(), 3);
    assert_eq!(v4.count_ones(), 4);
    assert_eq!(v5.count_ones(), 5);
}

#[test]
fn bitflags64_iter() {
    let v0 = BitFlags64(0b00000000);
    let v1 = BitFlags64(0b00001001);
    let v2 = BitFlags64(0b01001001);
    let v3 = BitFlags64(0b11111111);

    let v0_bits: Vec<usize> = v0.iter().collect();
    let v1_bits: Vec<usize> = v1.iter().collect();
    let v2_bits: Vec<usize> = v2.iter().collect();
    let v3_bits: Vec<usize> = v3.iter().collect();

    assert_eq!(&v0_bits, &[] as &[usize; 0]);
    assert_eq!(&v1_bits, &[0, 3]);
    assert_eq!(&v2_bits, &[0, 3, 6]);
    assert_eq!(&v3_bits, &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn bitflags64_insert_at_index() {
    let mut v0 = BitFlags64(0b00000000);
    let mut v1 = BitFlags64(0b00000000);
    let mut v2 = BitFlags64(0b00000000);
    let mut v3 = BitFlags64(0b00000000);
    let mut va = BitFlags64(0b11111111);

    v0.insert_at_index(0);
    v1.insert_at_index(1);
    v2.insert_at_index(2);
    v3.insert_at_index(3);
    va.insert_at_index(7);

    assert_eq!(v0, BitFlags64(0b00000001));
    assert_eq!(v1, BitFlags64(0b00000010));
    assert_eq!(v2, BitFlags64(0b00000100));
    assert_eq!(v3, BitFlags64(0b00001000));
    assert_eq!(va, BitFlags64(0b11111111));
}

#[test]
fn bitflags64_remove_at_index() {
    let mut v0 = BitFlags64(0b11111111);
    let mut v1 = BitFlags64(0b11111111);
    let mut v2 = BitFlags64(0b11111111);
    let mut v3 = BitFlags64(0b11111111);
    let mut va = BitFlags64(0b00000000);

    v0.remove_at_index(0);
    v1.remove_at_index(1);
    v2.remove_at_index(2);
    v3.remove_at_index(3);
    va.remove_at_index(7);

    assert_eq!(v0, BitFlags64(0b11111110));
    assert_eq!(v1, BitFlags64(0b11111101));
    assert_eq!(v2, BitFlags64(0b11111011));
    assert_eq!(v3, BitFlags64(0b11110111));
    assert_eq!(va, BitFlags64(0b00000000));
}

#[test]
fn bitflags64_set() {
    let mut f0 = BitFlags64(0b0000);
    let f1 = BitFlags64(0b0001);
    let f2 = BitFlags64(0b1001);
    let f3 = BitFlags64(0b1111);

    f0.set(f1, true);
    assert_eq!(f0, BitFlags64(0b0001));
    
    f0.set(f2, true);
    assert_eq!(f0, BitFlags64(0b1001));

    f0.set(f3, true);
    assert_eq!(f0, BitFlags64(0b1111));
}

#[test]
fn bitflags64_unset() {
    let mut f0 = BitFlags64(0b1111);
    let f1 = BitFlags64(0b0001);
    let f2 = BitFlags64(0b1001);
    let f3 = BitFlags64(0b1111);

    f0.set(f1, false);
    assert_eq!(f0, BitFlags64(0b1110));
    
    f0.set(f2, false);
    assert_eq!(f0, BitFlags64(0b0110));

    f0.set(f3, false);
    assert_eq!(f0, BitFlags64(0b0000));
}

#[test]
fn bitflags64_set_at_index() {
    let mut v0 = BitFlags64(0b00000000);
    let mut v1 = BitFlags64(0b00000000);
    let mut v2 = BitFlags64(0b00000000);
    let mut v3 = BitFlags64(0b00000000);
    let mut va = BitFlags64(0b11111111);

    v0.set_at_index(0, true);
    v1.set_at_index(1, true);
    v2.set_at_index(2, true);
    v3.set_at_index(3, true);
    va.set_at_index(7, true);

    assert_eq!(v0, BitFlags64(0b00000001));
    assert_eq!(v1, BitFlags64(0b00000010));
    assert_eq!(v2, BitFlags64(0b00000100));
    assert_eq!(v3, BitFlags64(0b00001000));
    assert_eq!(va, BitFlags64(0b11111111));

    v0.set_at_index(0, false);
    v1.set_at_index(1, false);
    v2.set_at_index(2, false);
    v3.set_at_index(3, false);
    va.set_at_index(7, false);

    assert_eq!(v0, BitFlags64(0b00000000));
    assert_eq!(v1, BitFlags64(0b00000000));
    assert_eq!(v2, BitFlags64(0b00000000));
    assert_eq!(v3, BitFlags64(0b00000000));
    assert_eq!(va, BitFlags64(0b01111111));
}

#[test]
fn bitflags64_toggle_at_index() {
    let mut v0 = BitFlags64(0b00000000);
    let mut v1 = BitFlags64(0b00000000);
    let mut v2 = BitFlags64(0b00000000);
    let mut v3 = BitFlags64(0b00000000);
    let mut va = BitFlags64(0b11111111);

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags64(0b00000001));
    assert_eq!(v1, BitFlags64(0b00000010));
    assert_eq!(v2, BitFlags64(0b00000100));
    assert_eq!(v3, BitFlags64(0b00001000));
    assert_eq!(va, BitFlags64(0b01111111));

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags64(0b00000000));
    assert_eq!(v1, BitFlags64(0b00000000));
    assert_eq!(v2, BitFlags64(0b00000000));
    assert_eq!(v3, BitFlags64(0b00000000));
    assert_eq!(va, BitFlags64(0b11111111));
}

//     ##      ######    ######            #######   ########  ########
//   ####     ##    ##  ##    ##           ##    ##     ##        ##
//     ##        ####	 ######   #######  #######      ##        ##
//     ##      ##       ##    ##           ##    ##     ##        ##
//   ######   ########   ######            #######   ########     ##

#[test]
fn bitflags128_creation() {
    let f1a = BitFlags128::new();
    let f1b = BitFlags128(0);
    let f1c = BitFlags128::from(0);
    let f1d: BitFlags128 = 0.into();
    assert_eq!(f1a, f1b);
    assert_eq!(f1b, f1c);
    assert_eq!(f1c, f1d);

    let f2a = BitFlags128::from_slice(&[2, 4, 5]);
    let f2b = BitFlags128::from_bits(0b0011_0100);
    assert_eq!(f2a, f2b);
}

#[test]
fn bitflags128_ops() {
    let flags1 = BitFlags128(0b0001);
    let flags2 = BitFlags128(0b1001);

    assert_eq!(flags1 | flags2, BitFlags128(0b1001));
    assert_eq!(flags1 & flags2, BitFlags128(0b0001));
}

#[test]
fn bitflags128_general() {
    let v0 = BitFlags128(0b00000000);
    let v1 = BitFlags128(0b00000100);
    let va = BitFlags128(u128::MAX);

    assert!(v0.is_empty());
    assert!(va.is_all());
    assert!(BitFlags128(0b00000010).intersects(BitFlags128(0b00000111)));
    assert!(!BitFlags128(0b00000010).intersects(BitFlags128(0b00100000)));
    assert!(BitFlags128(0b00000111).contains(BitFlags128(0b00000011)));
    assert!(!BitFlags128(0b00000111).contains(BitFlags128(0b00100011)));
    assert_eq!(BitFlags128(0b00000010), BitFlags128::from_index(1));
    assert_eq!(v1.bit_at_index(2), true);
    assert_eq!(v1.bit_at_index(3), false);
    assert_eq!(v1.get_bit_at_index(2), Some(true));
    assert_eq!(v1.get_bit_at_index(3), Some(false));
    assert_eq!(v1.get_bit_at_index(128), None);
}

#[test]
fn bitflags128_intersection() {
    let f1 = BitFlags128(0b0000);
    let f2 = BitFlags128(0b0001);
    let f3 = BitFlags128(0b1001);

    assert_eq!(f1.intersection(f2), BitFlags128(0b0000));
    assert_eq!(f1.intersection(f3), BitFlags128(0b0000));
    assert_eq!(f2.intersection(f3), BitFlags128(0b0001));
}

#[test]
fn bitflags128_union() {
    let f1 = BitFlags128(0b0000);
    let f2 = BitFlags128(0b0001);
    let f3 = BitFlags128(0b1001);

    assert_eq!(f1.union(f2), BitFlags128(0b0001));
    assert_eq!(f1.union(f3), BitFlags128(0b1001));
    assert_eq!(f2.union(f3), BitFlags128(0b1001));
}

#[test]
fn bitflags128_highest_set_bit() {
    let values = &[
        BitFlags128(0),
        BitFlags128(1),
        BitFlags128(2),
        BitFlags128(3),
        BitFlags128(4),
        BitFlags128(5),
        BitFlags128(8),
        BitFlags128(16),
        BitFlags128(32),
        BitFlags128(64),
    ];

    let expected = vec![0, 1, 2, 2, 4, 4, 8, 16, 32, 64];
    let returned = values.iter().map(|f| f.highest_set_bit()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags128_highest_set_bit_ix() {
    let values = &[
        BitFlags128(0),
        BitFlags128(1),
        BitFlags128(2),
        BitFlags128(3),
        BitFlags128(4),
        BitFlags128(5),
        BitFlags128(8),
        BitFlags128(16),
        BitFlags128(32),
        BitFlags128(64),
    ];
    let expected = vec![
        None,
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
    ];
    let returned = values.iter().map(|f| f.highest_set_bit_index()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
}

#[test]
fn bitflags128_count_ones() {
    let v0 = BitFlags128(0b00000000);
    let v1 = BitFlags128(0b01000000);
    let v2 = BitFlags128(0b00100001);
    let v3 = BitFlags128(0b00011001);
    let v4 = BitFlags128(0b10101001);
    let v5 = BitFlags128(0b10111001);

    assert_eq!(v0.count_ones(), 0);
    assert_eq!(v1.count_ones(), 1);
    assert_eq!(v2.count_ones(), 2);
    assert_eq!(v3.count_ones(), 3);
    assert_eq!(v4.count_ones(), 4);
    assert_eq!(v5.count_ones(), 5);
}

#[test]
fn bitflags128_iter() {
    let v0 = BitFlags128(0b00000000);
    let v1 = BitFlags128(0b00001001);
    let v2 = BitFlags128(0b01001001);
    let v3 = BitFlags128(0b11111111);

    let v0_bits: Vec<usize> = v0.iter().collect();
    let v1_bits: Vec<usize> = v1.iter().collect();
    let v2_bits: Vec<usize> = v2.iter().collect();
    let v3_bits: Vec<usize> = v3.iter().collect();

    assert_eq!(&v0_bits, &[] as &[usize; 0]);
    assert_eq!(&v1_bits, &[0, 3]);
    assert_eq!(&v2_bits, &[0, 3, 6]);
    assert_eq!(&v3_bits, &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn bitflags128_insert_at_index() {
    let mut v0 = BitFlags128(0b00000000);
    let mut v1 = BitFlags128(0b00000000);
    let mut v2 = BitFlags128(0b00000000);
    let mut v3 = BitFlags128(0b00000000);
    let mut va = BitFlags128(0b11111111);

    v0.insert_at_index(0);
    v1.insert_at_index(1);
    v2.insert_at_index(2);
    v3.insert_at_index(3);
    va.insert_at_index(7);

    assert_eq!(v0, BitFlags128(0b00000001));
    assert_eq!(v1, BitFlags128(0b00000010));
    assert_eq!(v2, BitFlags128(0b00000100));
    assert_eq!(v3, BitFlags128(0b00001000));
    assert_eq!(va, BitFlags128(0b11111111));
}

#[test]
fn bitflags128_remove_at_index() {
    let mut v0 = BitFlags128(0b11111111);
    let mut v1 = BitFlags128(0b11111111);
    let mut v2 = BitFlags128(0b11111111);
    let mut v3 = BitFlags128(0b11111111);
    let mut va = BitFlags128(0b00000000);

    v0.remove_at_index(0);
    v1.remove_at_index(1);
    v2.remove_at_index(2);
    v3.remove_at_index(3);
    va.remove_at_index(7);

    assert_eq!(v0, BitFlags128(0b11111110));
    assert_eq!(v1, BitFlags128(0b11111101));
    assert_eq!(v2, BitFlags128(0b11111011));
    assert_eq!(v3, BitFlags128(0b11110111));
    assert_eq!(va, BitFlags128(0b00000000));
}

#[test]
fn bitflags128_set() {
    let mut f0 = BitFlags128(0b0000);
    let f1 = BitFlags128(0b0001);
    let f2 = BitFlags128(0b1001);
    let f3 = BitFlags128(0b1111);

    f0.set(f1, true);
    assert_eq!(f0, BitFlags128(0b0001));
    
    f0.set(f2, true);
    assert_eq!(f0, BitFlags128(0b1001));

    f0.set(f3, true);
    assert_eq!(f0, BitFlags128(0b1111));
}

#[test]
fn bitflags128_unset() {
    let mut f0 = BitFlags128(0b1111);
    let f1 = BitFlags128(0b0001);
    let f2 = BitFlags128(0b1001);
    let f3 = BitFlags128(0b1111);

    f0.set(f1, false);
    assert_eq!(f0, BitFlags128(0b1110));
    
    f0.set(f2, false);
    assert_eq!(f0, BitFlags128(0b0110));

    f0.set(f3, false);
    assert_eq!(f0, BitFlags128(0b0000));
}

#[test]
fn bitflags128_set_at_index() {
    let mut v0 = BitFlags128(0b00000000);
    let mut v1 = BitFlags128(0b00000000);
    let mut v2 = BitFlags128(0b00000000);
    let mut v3 = BitFlags128(0b00000000);
    let mut va = BitFlags128(0b11111111);

    v0.set_at_index(0, true);
    v1.set_at_index(1, true);
    v2.set_at_index(2, true);
    v3.set_at_index(3, true);
    va.set_at_index(7, true);

    assert_eq!(v0, BitFlags128(0b00000001));
    assert_eq!(v1, BitFlags128(0b00000010));
    assert_eq!(v2, BitFlags128(0b00000100));
    assert_eq!(v3, BitFlags128(0b00001000));
    assert_eq!(va, BitFlags128(0b11111111));

    v0.set_at_index(0, false);
    v1.set_at_index(1, false);
    v2.set_at_index(2, false);
    v3.set_at_index(3, false);
    va.set_at_index(7, false);

    assert_eq!(v0, BitFlags128(0b00000000));
    assert_eq!(v1, BitFlags128(0b00000000));
    assert_eq!(v2, BitFlags128(0b00000000));
    assert_eq!(v3, BitFlags128(0b00000000));
    assert_eq!(va, BitFlags128(0b01111111));
}

#[test]
fn bitflags128_toggle_at_index() {
    let mut v0 = BitFlags128(0b00000000);
    let mut v1 = BitFlags128(0b00000000);
    let mut v2 = BitFlags128(0b00000000);
    let mut v3 = BitFlags128(0b00000000);
    let mut va = BitFlags128(0b11111111);

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags128(0b00000001));
    assert_eq!(v1, BitFlags128(0b00000010));
    assert_eq!(v2, BitFlags128(0b00000100));
    assert_eq!(v3, BitFlags128(0b00001000));
    assert_eq!(va, BitFlags128(0b01111111));

    v0.toggle_at_index(0);
    v1.toggle_at_index(1);
    v2.toggle_at_index(2);
    v3.toggle_at_index(3);
    va.toggle_at_index(7);

    assert_eq!(v0, BitFlags128(0b00000000));
    assert_eq!(v1, BitFlags128(0b00000000));
    assert_eq!(v2, BitFlags128(0b00000000));
    assert_eq!(v3, BitFlags128(0b00000000));
    assert_eq!(va, BitFlags128(0b11111111));
}
