#[macro_use]
mod macros;

use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[serde(transparent)]
pub struct Hex<T> {
    internal: T,
}

impl<T: Deref> Deref for Hex<T> {
    type Target = T::Target;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<T: DerefMut> DerefMut for Hex<T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.internal
    }
}

impl<T: fmt::Display> fmt::Display for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.internal)
    }
}

macro_rules! impl_for_ty {
    ($target_type: ty) => {
        paste! {
            mod [<hex_ $target_type _lower>] {
                serde_hex_mod_with_target_type!($target_type, "{:x}");
            }
            use [<hex_ $target_type _lower>]::HexInternal as [<Hex $target_type:camel LowerInternal>];
            pub type [<Hex $target_type:camel Lower>] = Hex<[<Hex $target_type:camel LowerInternal>]>;
            impl From<$target_type> for [<Hex $target_type:camel Lower>] {
                fn from(value: $target_type) -> Self {
                    Self {
                        internal: [<Hex $target_type:camel LowerInternal>]::from(value)
                    }
                }
            }
            mod [<hex_ $target_type _upper>] {
                serde_hex_mod_with_target_type!($target_type, "{:X}");
            }
            use [<hex_ $target_type _upper>]::HexInternal as [<Hex $target_type:camel UpperInternal>];
            pub type [<Hex $target_type:camel Upper>] = Hex<[<Hex $target_type:camel UpperInternal>]>;
            impl From<$target_type> for [<Hex $target_type:camel Upper>] {
                fn from(value: $target_type) -> Self {
                    Self {
                        internal: [<Hex $target_type:camel UpperInternal>]::from(value)
                    }
                }
            }
            mod [<hex_ $target_type _prefix_lower>] {
                serde_hex_prefix_mod_with_target_type!($target_type, "{:#x}");
            }
            use [<hex_ $target_type _prefix_lower>]::HexInternal as [<Hex $target_type:camel PrefixLowerInternal>];
            pub type [<Hex $target_type:camel PrefixLower>] = Hex<[<Hex $target_type:camel PrefixLowerInternal>]>;
            impl From<$target_type> for [<Hex $target_type:camel PrefixLower>] {
                fn from(value: $target_type) -> Self {
                    Self {
                        internal: [<Hex $target_type:camel PrefixLowerInternal>]::from(value)
                    }
                }
            }
            mod [<hex_ $target_type _prefix_upper>] {
                serde_hex_prefix_mod_with_target_type!($target_type, "{:#X}");
            }
            use [<hex_ $target_type _prefix_upper>]::HexInternal as [<Hex $target_type:camel PrefixUpperInternal>];
            pub type [<Hex $target_type:camel PrefixUpper>] = Hex<[<Hex $target_type:camel PrefixUpperInternal>]>;
            impl From<$target_type> for [<Hex $target_type:camel PrefixUpper>] {
                fn from(value: $target_type) -> Self {
                    Self {
                        internal: [<Hex $target_type:camel PrefixUpperInternal>]::from(value)
                    }
                }
            }
        }
    };
}
impl_for_ty!(u8);
impl_for_ty!(u16);
impl_for_ty!(u32);
impl_for_ty!(u64);
impl_for_ty!(usize);
