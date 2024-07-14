//! (De)serialization Tests for the `BitFlags32` struct.
//!
//! The optional "serde-support" and "nanoserde-support" features are enabled for testing.

use arctan_bitflags::BitFlags32;
use nanoserde::{DeBin, DeJson, DeRon, SerBin, SerJson, SerRon};

#[rustfmt::skip]
const FLAG_BIN: &[u8] = &[
    0, 0, 0, 0, 
    1, 0, 0, 0, 
    0, 0, 1, 0, 
    255, 255, 255, 255
];

const FLAG_JSON: &str = "[0,1,65536,4294967295]";

const FLAG_RON: &str = "(0, 1, 65536, 4294967295)";

const FLAG_ARRAY: [BitFlags32; 4] = [
    BitFlags32(0),
    BitFlags32(1),
    BitFlags32(65536),
    BitFlags32(4294967295),
];

#[test]
fn bitflags32_serde() {
    // Serialize
    let json_actual = serde_json::to_string(&FLAG_ARRAY).unwrap();
    let json_expected = FLAG_JSON;

    assert_eq!(json_actual, json_expected);

    // Deserialize
    let array_actual: [BitFlags32; 4] = serde_json::from_str(&FLAG_JSON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);
}

#[test]
fn bitflags32_nanoserde_de() {
    // Deserialize (BIN)
    let array_actual = <[BitFlags32; 4]>::deserialize_bin(&FLAG_BIN).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);

    // Deserialize (JSON)
    let array_actual = <[BitFlags32; 4]>::deserialize_json(&FLAG_JSON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);

    // Deserialize (RON)
    let array_actual = <[BitFlags32; 4]>::deserialize_ron(&FLAG_RON).unwrap();
    let array_expected = FLAG_ARRAY;

    assert_eq!(array_actual, array_expected);
}

#[test]
fn bitflags32_nanoserde_ser() {
    // Serialize (BIN)
    let bin_actual = <[BitFlags32; 4]>::serialize_bin(&FLAG_ARRAY);
    let bin_expected = FLAG_BIN;

    assert_eq!(bin_actual, bin_expected);

    // Serialize (JSON)
    let json_actual = <[BitFlags32; 4]>::serialize_json(&FLAG_ARRAY);
    let json_expected = FLAG_JSON;

    assert_eq!(json_actual, json_expected);

    // Serialize (RON)
    let ron_actual = <[BitFlags32; 4]>::serialize_ron(&FLAG_ARRAY);
    let ron_expected = FLAG_RON;

    assert_eq!(ron_actual, ron_expected);
}
