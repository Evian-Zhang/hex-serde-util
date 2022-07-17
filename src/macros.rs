macro_rules! serde_hex_mod_with_target_type {
    ($target_type: ty, $format_str: expr) => {
        mod serde_hex {
            use serde::{self, Deserialize, Deserializer, Serializer};
            pub fn serialize<S>(hex_num: &$target_type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let s = format!($format_str, hex_num);
                serializer.serialize_str(&s)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<$target_type, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                <$target_type>::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
            }
        }
        impl_serde_hex!($target_type, $format_str);
    };
}

macro_rules! serde_hex_prefix_mod_with_target_type {
    ($target_type: ty, $format_str: expr) => {
        mod serde_hex {
            use serde::{self, Deserialize, Deserializer, Serializer};
            pub fn serialize<S>(hex_num: &$target_type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let s = format!($format_str, hex_num);
                serializer.serialize_str(&s)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<$target_type, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let s = if let Some(s) = s.strip_prefix("0x") {
                    s
                } else {
                    return Err(serde::de::Error::custom(format!(
                        "Unexpected hex string {s} without 0x prefix."
                    )));
                };
                <$target_type>::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
            }
        }
        impl_serde_hex!($target_type, $format_str);
    };
}

macro_rules! impl_serde_hex {
    ($target_type: ty, $format_str: expr) => {
        use serde::{Deserialize, Serialize};
        use std::fmt;
        use std::ops::Deref;

        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        #[serde(transparent)]
        pub struct HexInternal {
            #[serde(with = "serde_hex")]
            value: $target_type,
        }

        impl Deref for HexInternal {
            type Target = $target_type;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl fmt::Display for HexInternal {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $format_str, &self.value)
            }
        }
    };
}
