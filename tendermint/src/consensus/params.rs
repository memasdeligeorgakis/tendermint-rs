//! Tendermint consensus parameters

use crate::error::Error;
use crate::prelude::*;
use crate::{block, duration::Duration, evidence, public_key};
use core::convert::{TryFrom, TryInto};
use serde::{Deserialize, Serialize};
use tendermint_proto::types::AbciParams as RawAbciParams;
use tendermint_proto::types::ConsensusParams as RawParams;
use tendermint_proto::types::SynchronyParams as RawSynchronyParams;
use tendermint_proto::types::TimeoutParams as RawTimeoutParams;
use tendermint_proto::types::ValidatorParams as RawValidatorParams;
use tendermint_proto::types::VersionParams as RawVersionParams;
use tendermint_proto::Protobuf;

/// Tendermint consensus parameters
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Params {
    /// Block size parameters
    pub block: block::Size,

    /// Evidence parameters
    pub evidence: evidence::Params,

    /// Validator parameters
    pub validator: ValidatorParams,

    /// Version parameters
    #[serde(skip)] // Todo: FIXME kvstore /genesis returns '{}' instead of '{app_version: "0"}'
    pub version: Option<VersionParams>,

    pub synchrony: SynchronyParams,

    pub timeout: TimeoutParams,

    pub abci: AbciParams,
}

impl Protobuf<RawParams> for Params {}

impl TryFrom<RawParams> for Params {
    type Error = Error;

    fn try_from(value: RawParams) -> Result<Self, Self::Error> {
        Ok(Self {
            block: value
                .block
                .ok_or_else(|| Error::invalid_block("missing block".to_string()))?
                .try_into()?,
            evidence: value
                .evidence
                .ok_or_else(Error::invalid_evidence)?
                .try_into()?,
            validator: value
                .validator
                .ok_or_else(Error::invalid_validator_params)?
                .try_into()?,
            version: value.version.map(TryFrom::try_from).transpose()?,
            synchrony: value
                .synchrony
                .ok_or_else(Error::invalid_synchrony_params)?
                .try_into()?,
            timeout: value
                .timeout
                .ok_or_else(Error::invalid_timeout_params)?
                .try_into()?,
            abci: value
                .abci
                .ok_or_else(Error::missing_abci_params)?
                .try_into()?,
        })
    }
}

impl From<Params> for RawParams {
    fn from(value: Params) -> Self {
        RawParams {
            block: Some(value.block.into()),
            evidence: Some(value.evidence.into()),
            validator: Some(value.validator.into()),
            version: value.version.map(From::from),
            synchrony: Some(value.synchrony.into()),
            timeout: Some(value.timeout.into()),
            abci: Some(value.abci.into()),
        }
    }
}

/// Validator consensus parameters
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ValidatorParams {
    /// Allowed algorithms for validator signing
    pub pub_key_types: Vec<public_key::Algorithm>,
}

impl Protobuf<RawValidatorParams> for ValidatorParams {}

impl TryFrom<RawValidatorParams> for ValidatorParams {
    type Error = Error;

    fn try_from(value: RawValidatorParams) -> Result<Self, Self::Error> {
        Ok(Self {
            pub_key_types: value.pub_key_types.iter().map(|f| key_type(f)).collect(),
        })
    }
}

// Todo: How are these key types created?
fn key_type(s: &str) -> public_key::Algorithm {
    if s == "Ed25519" || s == "ed25519" {
        return public_key::Algorithm::Ed25519;
    }
    if s == "Secp256k1" || s == "secp256k1" {
        return public_key::Algorithm::Secp256k1;
    }
    public_key::Algorithm::Ed25519 // Todo: Shall we error out for invalid key types?
}

impl From<ValidatorParams> for RawValidatorParams {
    fn from(value: ValidatorParams) -> Self {
        RawValidatorParams {
            pub_key_types: value
                .pub_key_types
                .into_iter()
                .map(|k| match k {
                    public_key::Algorithm::Ed25519 => "ed25519".to_string(),
                    public_key::Algorithm::Secp256k1 => "secp256k1".to_string(),
                })
                .collect(),
        }
    }
}

/// Version Parameters
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct VersionParams {
    #[serde(with = "crate::serializers::from_str")]
    app_version: u64,
}

impl Protobuf<RawVersionParams> for VersionParams {}

impl TryFrom<RawVersionParams> for VersionParams {
    type Error = Error;

    fn try_from(value: RawVersionParams) -> Result<Self, Self::Error> {
        Ok(Self {
            app_version: value.app_version,
        })
    }
}

impl From<VersionParams> for RawVersionParams {
    fn from(value: VersionParams) -> Self {
        RawVersionParams {
            app_version: value.app_version,
        }
    }
}

/// Synchrony Parameters
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct SynchronyParams {
    pub message_delay: Duration,
    pub precision: Duration,
}

impl Protobuf<RawSynchronyParams> for SynchronyParams {}

impl TryFrom<RawSynchronyParams> for SynchronyParams {
    type Error = Error;

    fn try_from(value: RawSynchronyParams) -> Result<Self, Self::Error> {
        Ok(Self {
            message_delay: value
                .message_delay
                .map(Duration::try_from)
                .transpose()?
                .ok_or_else(Error::missing_synchrony_params)?,
            precision: value
                .precision
                .map(Duration::try_from)
                .transpose()?
                .ok_or_else(Error::missing_synchrony_params)?,
        })
    }
}

impl From<SynchronyParams> for RawSynchronyParams {
    fn from(value: SynchronyParams) -> Self {
        RawSynchronyParams {
            message_delay: Some(value.message_delay.into()),
            precision: Some(value.precision.into()),
        }
    }
}

/// Timeout Parameters
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Default)]
pub struct TimeoutParams {
    pub propose: Duration,
    pub propose_delta: Duration,
    pub vote: Duration,
    pub vote_delta: Duration,
    pub commit: Duration,
    pub bypass_commit_timeout: bool,
}

impl Protobuf<RawTimeoutParams> for TimeoutParams {}

impl TryFrom<RawTimeoutParams> for TimeoutParams {
    type Error = Error;

    fn try_from(value: RawTimeoutParams) -> Result<Self, Self::Error> {
        Ok(Self {
            propose: value
                .propose
                .map(Duration::try_from)
                .transpose()?
                .ok_or_else(Error::missing_timeout_params)?,
            propose_delta: value
                .propose_delta
                .map(Duration::try_from)
                .transpose()?
                .ok_or_else(Error::missing_timeout_params)?,
            vote: value
                .vote
                .map(Duration::try_from)
                .transpose()?
                .ok_or_else(Error::missing_timeout_params)?,
            vote_delta: value
                .vote_delta
                .map(Duration::try_from)
                .transpose()?
                .ok_or_else(Error::missing_timeout_params)?,
            commit: value
                .commit
                .map(Duration::try_from)
                .transpose()?
                .ok_or_else(Error::missing_timeout_params)?,
            bypass_commit_timeout: value.bypass_commit_timeout,
        })
    }
}

impl From<TimeoutParams> for RawTimeoutParams {
    fn from(value: TimeoutParams) -> Self {
        Self {
            propose: Some(value.propose.into()),
            propose_delta: Some(value.propose_delta.into()),
            vote: Some(value.vote.into()),
            vote_delta: Some(value.vote_delta.into()),
            commit: Some(value.commit.into()),
            bypass_commit_timeout: value.bypass_commit_timeout,
        }
    }
}

/// ABCI Params configure functionality specific to the Application Blockchain Interface.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct AbciParams {
    pub vote_extensions_enable_height: u64,
    pub recheck_tx: bool,
}

impl Protobuf<RawAbciParams> for AbciParams {}

impl TryFrom<RawAbciParams> for AbciParams {
    type Error = Error;

    fn try_from(params: RawAbciParams) -> Result<Self, Self::Error> {
        Ok(Self {
            vote_extensions_enable_height: params
                .vote_extensions_enable_height
                .try_into()
                .map_err(Error::negative_height)?,
            recheck_tx: params.recheck_tx,
        })
    }
}

impl From<AbciParams> for RawAbciParams {
    fn from(params: AbciParams) -> Self {
        RawAbciParams {
            vote_extensions_enable_height: params
                .vote_extensions_enable_height
                .try_into()
                .unwrap_or_default(),
            recheck_tx: params.recheck_tx,
        }
    }
}

impl Default for AbciParams {
    fn default() -> Self {
        Self {
            vote_extensions_enable_height: 0,
            recheck_tx: true,
        }
    }
}
