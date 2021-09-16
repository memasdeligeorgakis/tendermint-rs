use crate::prelude::*;
use tendermint_proto::types;
use tendermint_proto::Protobuf;

/// The data sent via the vote extension method of ABCI++
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VoteExtension {
    /// App data to be signed
    pub app_data_to_sign: Vec<u8>,
    /// Self-authenticating app data
    pub app_data_self_authenticating: Vec<u8>,
}

/// The data sent via the vote extension method of ABCI++
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VoteExtensionToSign {
    /// App data that is to be signed
    pub app_data_to_sign: Vec<u8>,
}

impl VoteExtension {
    /// Create a new vote extension
    pub fn new() -> VoteExtension {
        Default::default()
    }
}

impl VoteExtensionToSign {
    /// Create a new vote extension whose data is to be signed
    pub fn new() -> VoteExtensionToSign {
        Default::default()
    }
}

impl From<types::VoteExtension> for VoteExtension {
    fn from(value: types::VoteExtension) -> Self {
        Self {
            app_data_to_sign: value.app_data_to_sign,
            app_data_self_authenticating: value.app_data_self_authenticating,
        }
    }
}

impl From<VoteExtension> for types::VoteExtension {
    fn from(value: VoteExtension) -> Self {
        Self {
            app_data_to_sign: value.app_data_to_sign,
            app_data_self_authenticating: value.app_data_self_authenticating,
        }
    }
}

impl From<types::VoteExtensionToSign> for VoteExtensionToSign {
    fn from(value: types::VoteExtensionToSign) -> Self {
        Self {
            app_data_to_sign: value.app_data_to_sign,
        }
    }
}

impl From<VoteExtensionToSign> for types::VoteExtensionToSign {
    fn from(value: VoteExtensionToSign) -> Self {
        Self {
            app_data_to_sign: value.app_data_to_sign,
        }
    }
}

impl From<VoteExtension> for VoteExtensionToSign {
    fn from(value: VoteExtension) -> Self {
        Self {
            app_data_to_sign: value.app_data_to_sign,
        }
    }
}

impl From<VoteExtensionToSign> for VoteExtension {
    fn from(value: VoteExtensionToSign) -> Self {
        Self {
            app_data_to_sign: value.app_data_to_sign,
            ..Default::default()
        }
    }
}

impl Protobuf<types::VoteExtension> for VoteExtension {}
impl Protobuf<types::VoteExtensionToSign> for VoteExtensionToSign {}
