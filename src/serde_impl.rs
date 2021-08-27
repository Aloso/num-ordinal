use crate::*;

macro_rules! impl_serde {
    ($t:ident, $f:ident( $($call:tt)*)) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.$f(self $($call)*)
            }
        }

        impl<'de> serde::Deserialize<'de> for $t {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                serde::Deserialize::deserialize(deserializer).map($t)
            }
        }
    };
}

impl_serde!(Osize, serialize_u64(.0 as u64));
impl_serde!(O128, serialize_u128(.0));
impl_serde!(O64, serialize_u64(.0));
impl_serde!(O32, serialize_u32(.0));
impl_serde!(O16, serialize_u16(.0));
impl_serde!(O8, serialize_u8(.0));
