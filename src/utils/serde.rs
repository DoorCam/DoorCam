//! Implements utilities for Deserialization

use duration_str::deserialize_duration;
use serde::{Deserialize, Deserializer};
use std::time::Duration;

/// Implements the function for `serde(deserialize_with = "$deserialize_name")` where
/// `$optional_type` is wrapped in a `Option`.
/// To use it, add `serde(default, deserialize_with = "$deserialize_optional_name")`.
macro_rules! deserialize_optional_with {
    ($visibility:vis $deserialize_optional_name:ident, $optional_type:ty, $deserialize_name:literal, $comment:literal) => {
        #[doc = $comment]
        $visibility fn $deserialize_optional_name<'de, D>(
            deserializer: D,
        ) -> Result<Option<$optional_type>, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(Deserialize)]
            struct Wrapper(#[serde(deserialize_with = $deserialize_name)] $optional_type);

            Ok(Option::<Wrapper>::deserialize(deserializer)?.map(|Wrapper(w)| w))
        }
    };
}

deserialize_optional_with! {pub deserialize_optional_duration, Duration, "deserialize_duration",
"Implements the function for `serde(deserialize_with = \"deserialize_duration\")` where `Duration` is wrapped in a `Option`.
To use it, add `serde(default, deserialize_with = \"deserialize_optional_duration\")`."
}
