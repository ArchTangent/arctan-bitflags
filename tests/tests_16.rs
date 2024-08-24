//! Tests for the `BitFlags16` struct.

use arctan_bitflags::BitFlags16;

#[test]
fn bitflags16_contains() {
    let f1 = BitFlags16(0b0000);
    let f2 = BitFlags16(0b0001);
    let f3 = BitFlags16(0b1001);

    assert!(!f1.contains(f2));
    assert!(!f1.contains(f3));
    assert!(!f2.contains(f3));
    assert!(f3.contains(f2));
    assert!(f3.contains(f1));
    assert!(f2.contains(f1));
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
fn bitflags16_creation() {
    let f1a = BitFlags16::new();
    let f1b = BitFlags16(0);
    let f1c = BitFlags16::from(0);
    let f1d: BitFlags16 = 0.into();
    let f1e = BitFlags16(0b0010);
    let f1f = BitFlags16::from_index(1);
    assert_eq!(f1a, f1b);
    assert_eq!(f1b, f1c);
    assert_eq!(f1c, f1d);
    assert_eq!(f1e, f1f);

    let f2a = BitFlags16::from_slice(&[2, 4, 5]);
    let f2b = BitFlags16::from_u16(0b0011_0100);
    assert_eq!(f2a, f2b);
}

#[test]
fn bitflags16_empty_full() {
    let f1 = BitFlags16::empty();
    let f2 = BitFlags16::full();

    assert!(f1.is_empty());
    assert!(f2.is_full());
}

#[test]
fn bitflags16_ops() {
    let f1 = BitFlags16(0b0001);
    let f2 = BitFlags16(0b0010);
    let f3 = BitFlags16(0b1001);

    assert_eq!(f1 | f2, BitFlags16(0b0011));
    assert_eq!(f1 | f3, BitFlags16(0b1001));

    assert_eq!(f1 & f2, BitFlags16(0b0000));
    assert_eq!(f1 & f3, BitFlags16(0b0001));

    assert_eq!(f1 ^ f2, BitFlags16(0b0011));
    assert_eq!(f1 ^ f3, BitFlags16(0b1000));

    assert_eq!(!f1, BitFlags16(0b11111111_1111_1110));
    assert_eq!(!f2, BitFlags16(0b11111111_1111_1101));
    assert_eq!(!f3, BitFlags16(0b11111111_1111_0110));

    assert_eq!(f1.complement(), BitFlags16(0b11111111_1111_1110));
    assert_eq!(f2.complement(), BitFlags16(0b11111111_1111_1101));
    assert_eq!(f3.complement(), BitFlags16(0b11111111_1111_0110));
}

#[test]
fn bitflags16_ops_assign() {
    let mut f1 = BitFlags16(0b0001);
    let f2 = BitFlags16(0b0010);
    let f3 = BitFlags16(0b1001);

    f1 |= f2;
    assert_eq!(f1, BitFlags16(0b0011));

    f1 |= f3;
    assert_eq!(f1, BitFlags16(0b1011));

    let mut f1 = BitFlags16(0b0001);

    f1 &= f2;
    assert_eq!(f1, BitFlags16(0b0000));

    f1 &= f3;
    assert_eq!(f1, BitFlags16(0b0000));

    let mut f1 = BitFlags16(0b0001);

    f1 ^= f2;
    assert_eq!(f1, BitFlags16(0b0011));

    f1 ^= f3;
    assert_eq!(f1, BitFlags16(0b1010));
}

#[test]
fn bitflags16_get_index() {
    let f1 = BitFlags16::full();

    let ix_first = f1.get_bit_at_index(0);
    let ix_last = f1.get_bit_at_index(15);
    let ix_oob = f1.get_bit_at_index(16);

    assert_eq!(ix_first, Some(true));
    assert_eq!(ix_last, Some(true));
    assert_eq!(ix_oob, None);
}

#[test]
fn bitflags16_highest_set_bit_ix() {
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
fn bitflags16_index() {
    let f1 = BitFlags16(0b10000000);

    let ix_0 = f1.bit_at_index(0);
    let ix_7 = f1.bit_at_index(7);

    assert_eq!(ix_0, false);
    assert_eq!(ix_7, true);
}

#[test]
#[should_panic]
fn bitflags16_index_oob() {
    let f1 = BitFlags16(0b10000000);

    f1.bit_at_index(16);
}

#[test]
fn bitflags16_intersection() {
    let f1 = BitFlags16(0b0000);
    let f2 = BitFlags16(0b0001);
    let f3 = BitFlags16(0b1001);

    assert_eq!(f1.intersection(f2), BitFlags16(0b0000));
    assert_eq!(f1.intersection(f3), BitFlags16(0b0000));
    assert_eq!(f2.intersection(f3), BitFlags16(0b0001));

    assert!(!f1.intersects(f2));
    assert!(!f1.intersects(f3));
    assert!(f2.intersects(f3));
}

#[test]
fn bitflags16_difference() {
    let f1 = BitFlags16(0b0000);
    let f2 = BitFlags16(0b0001);
    let f3 = BitFlags16(0b1001);

    assert_eq!(f1.difference(f2), BitFlags16(0b0000));
    assert_eq!(f1.difference(f3), BitFlags16(0b0000));
    assert_eq!(f2.difference(f3), BitFlags16(0b0000));
    assert_eq!(f3.difference(f2), BitFlags16(0b1000));
    assert_eq!(f3.difference(f1), BitFlags16(0b1001));
    assert_eq!(f2.difference(f1), BitFlags16(0b0001));
}

#[test]
fn bitflags16_symmetric_difference() {
    let f1 = BitFlags16(0b0000);
    let f2 = BitFlags16(0b0001);
    let f3 = BitFlags16(0b1001);

    assert_eq!(f1.symmetric_difference(f2), BitFlags16(0b0001));
    assert_eq!(f1.symmetric_difference(f3), BitFlags16(0b1001));
    assert_eq!(f2.symmetric_difference(f3), BitFlags16(0b1000));
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

    let expected = vec![
        BitFlags16(0),
        BitFlags16(1),
        BitFlags16(2),
        BitFlags16(2),
        BitFlags16(4),
        BitFlags16(4),
        BitFlags16(8),
        BitFlags16(16),
        BitFlags16(32),
        BitFlags16(64),
    ];
    
    let returned = values.iter().map(|f| f.highest_set_bit()).collect::<Vec<_>>();

    assert_eq!(expected, returned)
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
fn bitflags16_set_bit_range() {
    let mut actual = [
        BitFlags16::new(),
        BitFlags16::new(),
        BitFlags16::new(),
        BitFlags16::new(),
        BitFlags16::new(),
        BitFlags16::new(),
    ];
    
    actual[0].set_bit_range(0, 0);
    actual[1].set_bit_range(0, 1);
    actual[2].set_bit_range(0, 2);
    actual[3].set_bit_range(1, 3);
    actual[4].set_bit_range(6, 7);
    actual[5].set_bit_range(0, 15);

    let expected = [
        BitFlags16(0b0000_0001),
        BitFlags16(0b0000_0011),
        BitFlags16(0b0000_0111),
        BitFlags16(0b0000_1110),
        BitFlags16(0b1100_0000),
        BitFlags16::full(),
    ];

    assert_eq!(actual, expected);
}


#[test]
fn bitflags16_toggle() {
    let mut f1 = BitFlags16(0b0101);
    f1.toggle(BitFlags16(0b0000));
    assert_eq!(f1, BitFlags16(0b0101));

    let mut f1 = BitFlags16(0b0101);
    f1.toggle(BitFlags16(0b0011));
    assert_eq!(f1, BitFlags16(0b0110));

    let mut f1 = BitFlags16(0b0101);
    f1.toggle(BitFlags16(0b1111));
    assert_eq!(f1, BitFlags16(0b1010));
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

#[test]
fn bitflags16_with_set_bit_range() {
    let actual = [
        BitFlags16::with_set_bit_range(0, 0),
        BitFlags16::with_set_bit_range(0, 1),
        BitFlags16::with_set_bit_range(0, 2),
        BitFlags16::with_set_bit_range(1, 3),
        BitFlags16::with_set_bit_range(6, 7),
        BitFlags16::with_set_bit_range(0, 15),
    ];
    let expected = [
        BitFlags16(0b0000_0001),
        BitFlags16(0b0000_0011),
        BitFlags16(0b0000_0111),
        BitFlags16(0b0000_1110),
        BitFlags16(0b1100_0000),
        BitFlags16::full(),
    ];

    assert_eq!(actual, expected);
}