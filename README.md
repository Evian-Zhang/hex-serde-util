# hex-serde-util

This crate provides a convenient approach to serializing/deserializing hex numbers with hex string of several formats.

## Examples

```rust
use hex_serde_util::{HexUsizeUpper, HexUsizeLower, HexUsizePrefixUpper, HexUsizePrefixLower};

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
assert_eq!(
    serde_json::from_str::<HexUsizeLower>(r#""1a""#).unwrap(),
    0x1ausize.into()
);
assert_eq!(
    serde_json::from_str::<HexUsizeUpper>(r#""1A""#).unwrap(),
    0x1ausize.into()
);
assert_eq!(
    serde_json::from_str::<HexUsizePrefixLower>(r#""0x1a""#).unwrap(),
    0x1ausize.into()
);
assert_eq!(
    serde_json::from_str::<HexUsizePrefixUpper>(r#""0x1A""#).unwrap(),
    0x1ausize.into()
);
```

In summary, this crate provides a way to serialize/deserialize hex number from/to hex string with:

* lower case without `0x` prefix (`1a` for example)
* upper case without `0x` prefix (`1A` for example)
* lower case with `0x` prefix (`0x1a` for example)
* upper case with `0x` prefix (`0x1A` for example)

Generally, a project may only use one format of hex string, so just use an alias when using this crate in your project for convenience:

```rust
use hex_serde_util::HexUsizePrefixUpper as Hex;
use serde::Deserialize;

#[derive(Deserialize)]
struct AnalysisInfo {
    base_addr: Hex,
}
```

## Necessity

`serde` provides many attributes for customizing serialization/deserialization, such as

* `[serde(serialize_with = "path")]`
* `[serde(deserialize_with = "path")]`
* `[serde(with = "module")]`

These attributes can be used to work with hex strings, but there are some disadvantages:

* Boilerplate needed

   You need to write some boilerplates in your project to use these attributes.
* Can't use these attributes when in a wrapper such as `Option`

   It is a common practice to use `Option` to wrap some fields if these type may be null when processing. However, if this field is designed to be a hex string, you have to write additional boilerplates for just serializing/deserializing those fields.

This crate handles these problems.

## Usages

Just as shown above, users can just use types provided by this crate when serialize/deserialize hex strings with hex numbers. Moreover, these types also implements some convenient traits:

* `Deref`, `DerefMut`

   After deserializing, users can just use these traits to get the real number:

   ```rust
   use hex_serde_util::HexUsizePrefixUpper as Hex;
   let hex_wrapper = serde_json::from_str::<Hex>(r#""0x1A""#).unwrap();
   let value = *hex_wrapper; // `value` is 26 with type `usize`
   assert_eq!(value, 26);
   ```
* `From`, `Into`

   Before serializing, users can just use `.into` to construct corresponding types.
* `Display`

   When printing those structs with `println!("{}");`, the field is displayed as if it is a hex string:

   ```rust
   use hex_serde_util::HexUsizePrefixUpper as Hex;
   let hex_wrapper: Hex = 0x1a.into();
   let hex_str = hex_wrapper.to_string();
   assert_eq!(&hex_str, "0x1A");
   ```
* `Eq`, `PartialEq`, `Ord`, `PartialOrd`

   For comparing.
