//! Blocks within the chains of a Tendermint network

mod commit;
pub mod commit_sig;
pub mod header;
mod height;
mod id;
mod meta;
pub mod parts;
mod round;
pub mod signed_header;
mod size;

pub use self::{
    commit::*,
    commit_sig::*,
    header::Header,
    height::*,
    id::{Id, ParseId},
    meta::Meta,
    round::*,
    size::Size,
};
use crate::prelude::*;
use crate::{abci::transaction, error::Error, evidence};
use core::convert::{TryFrom, TryInto};
use serde::{Deserialize, Serialize};
use tendermint_proto::types::Block as RawBlock;
use tendermint_proto::Protobuf;

/// Blocks consist of a header, transactions, votes (the commit), and a list of
/// evidence of malfeasance (i.e. signing conflicting votes).
///
/// <https://github.com/tendermint/spec/blob/d46cd7f573a2c6a2399fcab2cde981330aa63f37/spec/core/data_structures.md#block>
// Default serialization - all fields serialize; used by /block endpoint
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Block {
    /// Block header
    pub header: Header,

    /// Transaction data
    pub data: transaction::Data,

    /// Evidence of malfeasance
    #[serde(default)]
    pub evidence: Option<Vec<evidence::Evidence>>,

    /// Last commit
    #[serde(with = "crate::serializers::optional")]
    pub last_commit: Option<Commit>,
}

impl Protobuf<RawBlock> for Block {}

impl TryFrom<RawBlock> for Block {
    type Error = Error;

    fn try_from(value: RawBlock) -> Result<Self, Self::Error> {
        let header: Header = value.header.ok_or_else(Error::missing_header)?.try_into()?;
        // if last_commit is Commit::Default, it is considered nil by Go.
        let last_commit = value
            .last_commit
            .map(TryInto::try_into)
            .transpose()?
            .filter(|c| c != &Commit::default());
        if last_commit.is_none() && header.height.value() != 1 {
            return Err(Error::invalid_block(
                "last_commit is empty on non-first block".to_string(),
            ));
        }
        // Todo: Figure out requirements.
        //if last_commit.is_some() && header.height.value() == 1 {
        //    return Err(Kind::InvalidFirstBlock.context("last_commit is not null on first
        // height").into());
        //}
        let evidence: evidence::Data = value
                .evidence
                .ok_or_else(Error::missing_evidence)?
                .try_into()?;
        Ok(Block {
            header,
            data: value.data.ok_or_else(Error::missing_data)?.into(),
            evidence: Some(evidence.into_vec()),
            last_commit,
        })
    }
}

impl From<Block> for RawBlock {
    fn from(value: Block) -> Self {
        let data = evidence::Data::new(value.evidence.unwrap_or_default());
        RawBlock {
            header: Some(value.header.into()),
            data: Some(value.data.into()),
            evidence: Some(data.into()),
            last_commit: value.last_commit.map(Into::into),
        }
    }
}

impl Block {
    /// constructor
    pub fn new(
        header: Header,
        data: transaction::Data,
        evidence: evidence::Data,
        last_commit: Option<Commit>,
    ) -> Result<Self, Error> {
        if last_commit.is_none() && header.height.value() != 1 {
            return Err(Error::invalid_block(
                "last_commit is empty on non-first block".to_string(),
            ));
        }
        if last_commit.is_some() && header.height.value() == 1 {
            return Err(Error::invalid_block(
                "last_commit is filled on first block".to_string(),
            ));
        }
        Ok(Block {
            header,
            data,
            evidence: Some(evidence.into_vec()),
            last_commit,
        })
    }

    /// Get header
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Get data
    pub fn data(&self) -> &transaction::Data {
        &self.data
    }

    /// Get evidence
    pub fn evidence(&self) -> evidence::Data {
        evidence::Data::new(self.evidence.clone().unwrap_or_default())
    }

    /// Get last commit
    pub fn last_commit(&self) -> &Option<Commit> {
        &self.last_commit
    }
}
