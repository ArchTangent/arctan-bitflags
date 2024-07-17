//! Tests for the `BitFlags32` struct.

use arctan_bitflags::BitFlags32;

#[test]
fn bitflags32_contains() {
    let f1 = BitFlags32(0b0000);
    let f2 = BitFlags32(0b0001);
    let f3 = BitFlags32(0b1001);

    assert!(!f1.contains(f2));
    assert!(!f1.contains(f3));
    assert!(!f2.contains(f3));
    assert!(f3.contains(f2));
    assert!(f3.contains(f1));
    assert!(f2.contains(f1));
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
fn bitflags32_creation() {
    let f1a = BitFlags32::new();
    let f1b = BitFlags32(0);
    let f1c = BitFlags32::from(0);
    let f1d: BitFlags32 = 0.into();
    let f1e = BitFlags32(0b0010);
    let f1f = BitFlags32::from_index(1);
    assert_eq!(f1a, f1b);
    assert_eq!(f1b, f1c);
    assert_eq!(f1c, f1d);
    assert_eq!(f1e, f1f);

    let f2a = BitFlags32::from_slice(&[2, 4, 5]);
    let f2b = BitFlags32::from_u32(0b0011_0100);
    assert_eq!(f2a, f2b);
}

#[test]
fn bitflags32_empty_full() {
    let f1 = BitFlags32::empty();
    let f2 = BitFlags32::full();

    assert!(f1.is_empty());
    assert!(f2.is_full());
}

#[test]
fn bitflags32_ops() {
    let f1 = BitFlags32(0b0001);
    let f2 = BitFlags32(0b0010);
    let f3 = BitFlags32(0b1001);

    assert_eq!(f1 | f2, BitFlags32(0b0011));
    assert_eq!(f1 | f3, BitFlags32(0b1001));

    assert_eq!(f1 & f2, BitFlags32(0b0000));
    assert_eq!(f1 & f3, BitFlags32(0b0001));

    assert_eq!(f1 ^ f2, BitFlags32(0b0011));
    assert_eq!(f1 ^ f3, BitFlags32(0b1000));

    assert_eq!(!f1, BitFlags32(0b11111111_11111111_11111111_1111_1110));
    assert_eq!(!f2, BitFlags32(0b11111111_11111111_11111111_1111_1101));
    assert_eq!(!f3, BitFlags32(0b11111111_11111111_11111111_1111_0110));

    assert_eq!(f1.complement(), BitFlags32(0b11111111_11111111_11111111_1111_1110));
    assert_eq!(f2.complement(), BitFlags32(0b11111111_11111111_11111111_1111_1101));
    assert_eq!(f3.complement(), BitFlags32(0b11111111_11111111_11111111_1111_0110));
}

#[test]
fn bitflags32_ops_assign() {
    let mut f1 = BitFlags32(0b0001);
    let f2 = BitFlags32(0b0010);
    let f3 = BitFlags32(0b1001);

    f1 |= f2;
    assert_eq!(f1, BitFlags32(0b0011));

    f1 |= f3;
    assert_eq!(f1, BitFlags32(0b1011));

    let mut f1 = BitFlags32(0b0001);

    f1 &= f2;
    assert_eq!(f1, BitFlags32(0b0000));

    f1 &= f3;
    assert_eq!(f1, BitFlags32(0b0000));

    let mut f1 = BitFlags32(0b0001);

    f1 ^= f2;
    assert_eq!(f1, BitFlags32(0b0011));

    f1 ^= f3;
    assert_eq!(f1, BitFlags32(0b1010));
}

#[test]
fn bitflags32_get_index() {
    let f1 = BitFlags32::full();

    let ix_first = f1.get_bit_at_index(0);
    let ix_last = f1.get_bit_at_index(31);
    let ix_oob = f1.get_bit_at_index(32);

    assert_eq!(ix_first, Some(true));
    assert_eq!(ix_last, Some(true));
    assert_eq!(ix_oob, None);
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

    let expected = vec![
        BitFlags32(0),
        BitFlags32(1),
        BitFlags32(2),
        BitFlags32(2),
        BitFlags32(4),
        BitFlags32(4),
        BitFlags32(8),
        BitFlags32(16),
        BitFlags32(32),
        BitFlags32(64),
    ];

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
fn bitflags32_index() {
    let f1 = BitFlags32(0b10000000);

    let ix_0 = f1.bit_at_index(0);
    let ix_7 = f1.bit_at_index(7);

    assert_eq!(ix_0, false);
    assert_eq!(ix_7, true);
}

#[test]
#[should_panic]
fn bitflags32_index_oob() {
    let f1 = BitFlags32(0b10000000);

    f1.bit_at_index(32);
}

#[test]
fn bitflags32_intersection() {
    let f1 = BitFlags32(0b0000);
    let f2 = BitFlags32(0b0001);
    let f3 = BitFlags32(0b1001);

    assert_eq!(f1.intersection(f2), BitFlags32(0b0000));
    assert_eq!(f1.intersection(f3), BitFlags32(0b0000));
    assert_eq!(f2.intersection(f3), BitFlags32(0b0001));

    assert!(!f1.intersects(f2));
    assert!(!f1.intersects(f3));
    assert!(f2.intersects(f3));
}

#[test]
fn bitflags32_difference() {
    let f1 = BitFlags32(0b0000);
    let f2 = BitFlags32(0b0001);
    let f3 = BitFlags32(0b1001);

    assert_eq!(f1.difference(f2), BitFlags32(0b0000));
    assert_eq!(f1.difference(f3), BitFlags32(0b0000));
    assert_eq!(f2.difference(f3), BitFlags32(0b0000));
    assert_eq!(f3.difference(f2), BitFlags32(0b1000));
    assert_eq!(f3.difference(f1), BitFlags32(0b1001));
    assert_eq!(f2.difference(f1), BitFlags32(0b0001));
}

#[test]
fn bitflags32_symmetric_difference() {
    let f1 = BitFlags32(0b0000);
    let f2 = BitFlags32(0b0001);
    let f3 = BitFlags32(0b1001);

    assert_eq!(f1.symmetric_difference(f2), BitFlags32(0b0001));
    assert_eq!(f1.symmetric_difference(f3), BitFlags32(0b1001));
    assert_eq!(f2.symmetric_difference(f3), BitFlags32(0b1000));
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
fn bitflags32_toggle() {
    let mut f1 = BitFlags32(0b0101);
    f1.toggle(BitFlags32(0b0000));
    assert_eq!(f1, BitFlags32(0b0101));

    let mut f1 = BitFlags32(0b0101);
    f1.toggle(BitFlags32(0b0011));
    assert_eq!(f1, BitFlags32(0b0110));

    let mut f1 = BitFlags32(0b0101);
    f1.toggle(BitFlags32(0b1111));
    assert_eq!(f1, BitFlags32(0b1010));
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
