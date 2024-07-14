//! (De)serialization Tests for the `BitFlags8` struct.
//!
//! The optional "serde-support" and "nanoserde-support" features are enabled for testing.

use arctan_bitflags::BitFlags8;
use nanoserde::{DeBin, DeJson, DeRon, SerBin, SerJson, SerRon};

const FLAG_BIN: &[u8] = &[0, 1, 2, 3, 4, 8, 16, 32, 64, 128, 255];

const FLAG_JSON: &str = "[0,1,2,3,4,8,16,32,64,128,255]";

const FLAG_RON: &str = "(0, 1, 2, 3, 4, 8, 16, 32, 64, 128, 255)";

const FLAG_ARRAY: [BitFlags8; 11] = [
    BitFlags8(0),
    BitFlags8(1),
    BitFlags8(2),
    BitFlags8(3),
    BitFlags8(4),
    BitFlags8(8),
    BitFlags8(16),
    BitFlags8(32),
    BitFlags8(64),
    BitFlags8(128),
    BitFlags8(255),
];

#[test]
fn bitflags8_serde() {
    // Serialize
    let json_actual = serde_json::to_string(&FLAG_ARRAY).unwrap();
    let json_expected = FLAG_JSON;

    assert_eq!(json_actual, json_expected);

    // Deserialize
    let array_actual: [BitFlags8; 11] = serde_json::from_str(&FLAG_JSON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);
}

#[test]
fn bitflags8_nanoserde_de() {
    // Deserialize (BIN)
    let array_actual = <[BitFlags8; 11]>::deserialize_bin(&FLAG_BIN).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);

    // Deserialize (JSON)
    let array_actual = <[BitFlags8; 11]>::deserialize_json(&FLAG_JSON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);

    // Deserialize (RON)
    let array_actual = <[BitFlags8; 11]>::deserialize_ron(&FLAG_RON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);
}

#[test]
fn bitflags8_nanoserde_ser() {
    // Serialize (BIN)
    let bin_actual = <[BitFlags8; 11]>::serialize_bin(&FLAG_ARRAY);
    let bin_expected = FLAG_BIN;

    assert_eq!(bin_actual, bin_expected);

    // Serialize (JSON)
    let json_actual = <[BitFlags8; 11]>::serialize_json(&FLAG_ARRAY);
    let json_expected = FLAG_JSON;

    assert_eq!(json_actual, json_expected);

    // Serialize (RON)
    let ron_actual = <[BitFlags8; 11]>::serialize_ron(&FLAG_ARRAY);
    let ron_expected = FLAG_RON;

    assert_eq!(ron_actual, ron_expected);
}
