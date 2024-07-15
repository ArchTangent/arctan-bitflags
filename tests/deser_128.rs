//! (De)serialization Tests for the `BitFlags128` struct.
//!
//! The optional "serde-support" and "nanoserde-support" features are enabled for testing.
//! 
//! _Note_: (de)serializing `BitFlags128` with `serde` + `ron` requires the `ron` crate's 
//! `"integer128"` feature.

use arctan_bitflags::BitFlags128;
use nanoserde::{DeBin, SerBin};

#[rustfmt::skip]
const FLAG_BIN: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];

const FLAG_JSON: &str = "[0,1,18446744073709551616,340282366920938463463374607431768211455]";

const FLAG_RON: &str = "(0,1,18446744073709551616,340282366920938463463374607431768211455)";

const FLAG_ARRAY: [BitFlags128; 4] = [
    BitFlags128(0),
    BitFlags128(1),
    BitFlags128(18446744073709551616),
    BitFlags128(u128::MAX),
];

#[test]
fn bitflags128_serde() {
    // Serialize (JSON)
    let json_actual = serde_json::to_string(&FLAG_ARRAY).unwrap();
    let json_expected = FLAG_JSON;

    assert_eq!(json_actual, json_expected);

    // Deserialize (JSON)
    let array_actual: [BitFlags128; 4] = serde_json::from_str(&FLAG_JSON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);

    // Serialize (RON)
    let ron_actual = ron::to_string(&FLAG_ARRAY).unwrap();
    let ron_expected = FLAG_RON;

    assert_eq!(ron_actual, ron_expected);

    // Deserialize (RON)
    let array_actual: [BitFlags128; 4] = ron::from_str(&FLAG_RON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);      
}

#[test]
fn bitflags128_nanoserde_de() {
    // Deserialize (BIN)
    let array_actual = <[BitFlags128; 4]>::deserialize_bin(&FLAG_BIN).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);
}

#[test]
fn bitflags128_nanoserde_ser() {
    // Serialize (BIN)
    let bin_actual = <[BitFlags128; 4]>::serialize_bin(&FLAG_ARRAY);
    let bin_expected = FLAG_BIN;

    assert_eq!(bin_actual, bin_expected);
}
