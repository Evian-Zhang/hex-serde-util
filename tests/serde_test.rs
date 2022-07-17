use hex_serde_util::*;

#[test]
fn test_serialize() {
    let lower_data: HexUsizeLower = 0x1ausize.into();
    let upper_data: HexUsizeUpper = 0x1ausize.into();
    let prefix_lower_data: HexUsizePrefixLower = 0x1ausize.into();
    let prefix_upper_data: HexUsizePrefixUpper = 0x1ausize.into();
    assert_eq!(&serde_json::to_string(&lower_data).unwrap(), r#""1a""#);
    assert_eq!(&serde_json::to_string(&upper_data).unwrap(), r#""1A""#);
    assert_eq!(
        &serde_json::to_string(&prefix_lower_data).unwrap(),
        r#""0x1a""#
    );
    assert_eq!(
        &serde_json::to_string(&prefix_upper_data).unwrap(),
        r#""0x1A""#
    );
}

#[test]
fn test_deserialize() {
    let lower_str = r#""1a""#;
    let upper_str = r#""1A""#;
    let prefix_lower_str = r#""0x1a""#;
    let prefix_upper_str = r#""0x1A""#;
    assert_eq!(
        serde_json::from_str::<HexUsizeLower>(&lower_str).unwrap(),
        0x1ausize.into()
    );
    assert_eq!(
        serde_json::from_str::<HexUsizeUpper>(&upper_str).unwrap(),
        0x1ausize.into()
    );
    assert_eq!(
        serde_json::from_str::<HexUsizePrefixLower>(&prefix_lower_str).unwrap(),
        0x1ausize.into()
    );
    assert_eq!(
        serde_json::from_str::<HexUsizePrefixUpper>(&prefix_upper_str).unwrap(),
        0x1ausize.into()
    );
}

#[test]
#[should_panic]
fn test_overflow_deserialize() {
    let str = r#""0xaaaaaaaa""#;
    assert!(serde_json::from_str::<HexU8PrefixLower>(&str).is_ok());
}

#[test]
#[should_panic]
fn test_prefix_to_nonprefix_deserialize() {
    let str = r#""0x1a""#;
    assert!(serde_json::from_str::<HexUsizeLower>(&str).is_ok());
}

#[test]
#[should_panic]
fn test_nonprefix_to_prefix_deserialize() {
    let str = r#""1a""#;
    assert!(serde_json::from_str::<HexUsizePrefixLower>(&str).is_ok());
}
