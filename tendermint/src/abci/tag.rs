//! Tags

use crate::error::Error;
use crate::prelude::*;
use core::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};

/// Tags
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    /// Key
    pub key: Key,

    /// Value
    pub value: Value,
}

/// Tag keys
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Key(
    String,
);

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(Key(s.into()))
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Tag values
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Value(
    String,
);

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(Value(s.into()))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}
