use crate::{error::Error, serializers};
use core::convert::{TryFrom, TryInto};
use serde::{Deserialize, Serialize};
use tendermint_proto::google::protobuf::Duration as RawDuration;
use tendermint_proto::Protobuf;

/// Duration is a wrapper around core::time::Duration
/// essentially, to keep the usages look cleaner
/// i.e. you can avoid using serde annotations everywhere
/// Todo: harmonize google::protobuf::Duration, core::time::Duration and this. Too many structs.
/// <https://github.com/informalsystems/tendermint-rs/issues/741>
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct Duration(#[serde(with = "serializers::time_duration")] pub core::time::Duration);

impl Duration {
    pub fn new(seconds: u64, nanos: u32) -> Self {
        Duration(core::time::Duration::new(seconds, nanos))
    }

    pub fn from_secs(secs: u64) -> Self {
        Duration(core::time::Duration::from_secs(secs))
    }

    pub fn from_millis(millis: u64) -> Self {
        Duration(core::time::Duration::from_millis(millis))
    }

    pub fn from_nanos(nanos: u64) -> Self {
        Duration(core::time::Duration::from_nanos(nanos))
    }
}

impl From<Duration> for core::time::Duration {
    fn from(d: Duration) -> core::time::Duration {
        d.0
    }
}

impl Protobuf<RawDuration> for Duration {}

impl TryFrom<RawDuration> for Duration {
    type Error = Error;

    fn try_from(value: RawDuration) -> Result<Self, Self::Error> {
        Ok(Self(core::time::Duration::new(
            value.seconds.try_into().map_err(Error::integer_overflow)?,
            value.nanos.try_into().map_err(Error::integer_overflow)?,
        )))
    }
}

impl From<Duration> for RawDuration {
    fn from(value: Duration) -> Self {
        // Todo: make the struct into a proper domain type so this becomes infallible.
        Self {
            seconds: value.0.as_secs() as i64,
            nanos: value.0.subsec_nanos() as i32,
        }
    }
}
