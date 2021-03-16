//! ABCI responses and response data.
//!
//! The [`Response`] enum records all possible ABCI responses. Responses that
//! contain data are modeled as a separate struct, to avoid duplication of field
//! definitions.

// IMPORTANT NOTE ON DOCUMENTATION:
//
// The documentation for each response type is adapted from the ABCI Methods and
// Types spec document. However, the same logical response may appear three
// times, as a struct with the response data, as a Response variant, and as a
// CategoryResponse variant. Ideally, the documentation would be copied between
// these automatically, but doing this requires using #[doc = include_str!],
// which is unstable. For now, the Response enum is the source of truth; please
// change the docs there and copy as required.

use std::convert::{TryFrom, TryInto};

use bytes::Bytes;

/// XXX(hdevalence): hide merkle::proof and re-export its contents from merkle?
use crate::merkle::proof as merkle;

use super::{
    //Event,
    event::Event,
    params::ConsensusParams,
    types::{Evidence, Snapshot, ValidatorUpdate},
};

/// Returns an exception (undocumented, nondeterministic).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Exception {
    /// Undocumented.
    pub error: String,
}

/// Returns a string sent in the request, to test an ABCI implementation.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Echo {
    /// The message sent in the request.
    pub message: String,
}

/// Returns information about the application state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Info {
    /// Some arbitrary information.
    pub data: String,
    /// The application software semantic version.
    pub version: String,
    /// The application protocol version.
    pub app_version: u64,
    /// The latest block for which the app has called [`Commit`](super::request::Commit).
    pub last_block_height: i64,
    /// The latest result of [`Commit`](super::request::Commit).
    pub last_block_app_hash: Bytes,
}

/// Returned on genesis after initializing chain state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct InitChain {
    /// Initial consensus-critical parameters (optional).
    pub consensus_params: Option<ConsensusParams>,
    /// Initial validator set (optional).
    ///
    /// If this list is empty, the initial validator set will be the one given in
    /// [`request::InitChain::validators`](super::request::InitChain::validators).
    ///
    /// If this list is nonempty, it will be the initial validator set, instead
    /// of the one given in
    /// [`request::InitChain::validators`](super::request::InitChain::validators).
    pub validators: Vec<ValidatorUpdate>,
    /// Initial application hash.
    pub app_hash: Bytes,
}

/// Returns data queried from the application.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Query {
    /// The response code for the query.
    pub code: u32,
    /// The output of the application's logger.
    ///
    /// **May be non-deterministic**.
    pub log: String,
    /// Additional information.
    ///
    /// **May be non-deterministic**.
    pub info: String,
    /// The index of the key in the tree.
    pub index: i64,
    /// The key of the matching data.
    pub key: Bytes,
    /// The value of the matching data.
    pub value: Bytes,
    /// Serialized proof for the value data, if requested, to be verified against
    /// the app hash for the given `height`.
    pub proof: Option<merkle::Proof>,
    /// The block height from which data was derived.
    ///
    /// Note that this is the height of the block containing the application's
    /// Merkle root hash, which represents the state as it was after committing
    /// the block at `height - 1`.
    pub height: i64,
    /// The namespace for the `code`.
    pub codespace: String,
}

/// Finalize block
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct FinalizeBlock {
    //FIXME(ash): this should not be optional
    /// consenus updates
    pub updates: Option<ConsensusUpdates>,
    /// tx results
    pub tx_results: Vec<ResponseTx>,
}

/// Prepare proposal
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PrepareProposal {
    //FIXME(Ash): add BlockData field, header field
}

/// Verify header
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct VerifyHeader {
    /// accept header
    pub accept_header: bool,
    /// evidence
    pub evidence: Vec<Evidence>,
}

/// Process proposal
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ProcessProposal {
    /// accept block
    pub accept_block: bool,
    /// evidence
    pub evidence: Vec<Evidence>,
}

/// Revert proposal
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct RevertProposal {}

/// Extend vote
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ExtendVote {
    /// unsigned app vote data
    pub unsigned_app_vote_data: Bytes,
    /// self-authenticating app vote data
    pub self_authenticating_app_data: Bytes,
}

/// Verify vote extension
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct VerifyVoteExtension {
    /// result
    pub result: bool,
}

/// Returns the result of checking a transaction for mempool inclusion.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct CheckTx {
    /// The response code.
    ///
    /// Transactions where `code != 0` will be rejected; these transactions will
    /// not be broadcast to other nodes or included in a proposal block.
    /// Tendermint attributes no other value to the response code.
    pub code: u32,
    /// Result bytes, if any.
    pub data: Bytes,
    /// The output of the application's logger.
    ///
    /// **May be non-deterministic**.
    pub log: String,
    /// Additional information.
    ///
    /// **May be non-deterministic**.
    pub info: String,
    /// Amount of gas requested for the transaction.
    pub gas_wanted: i64,
    /// Amount of gas consumed by the transaction.
    pub gas_used: i64,
    /// Events that occurred while checking the transaction.
    pub events: Vec<Event>,
    /// The namespace for the `code`.
    pub codespace: String,
}

/// Returns events that occurred while executing a transaction against the
/// application state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ResponseTx {
    /// The response code.
    ///
    /// This code should be `0` only if the transaction is fully valid. However,
    /// invalid transactions included in a block will still be executed against
    /// the application state.
    pub code: u32,
    /// Result bytes, if any.
    pub data: Bytes,
    /// The output of the application's logger.
    ///
    /// **May be non-deterministic**.
    pub log: String,
    /// Additional information.
    ///
    /// **May be non-deterministic**.
    pub info: String,
    /// Amount of gas requested for the transaction.
    pub gas_wanted: i64,
    /// Amount of gas consumed by the transaction.
    pub gas_used: i64,
    /// Events that occurred while executing the transaction.
    pub events: Vec<Event>,
    /// The namespace for the `code`.
    pub codespace: String,
}

/// Returns validator updates that occur after the end of a block.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ConsensusUpdates {
    /// Changes to the validator set, if any.
    ///
    /// Setting the voting power to 0 removes a validator.
    pub validator_updates: Vec<ValidatorUpdate>,
    /// Changes to consensus parameters (optional).
    pub consensus_param_updates: Option<ConsensusParams>,
    /// Events that occurred while ending the block.
    pub events: Vec<Event>,
}

/// Returns the result of persisting the application state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Commit {
    /// The Merkle root hash of the application state
    ///
    /// XXX(hdevalence) - is this different from an app hash?
    /// XXX(hdevalence) - rename to app_hash ?
    pub data: Bytes,
    /// Blocks below this height may be removed.
    pub retain_height: i64,
}

/// Returns a list of local state snapshots.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ListSnapshots {
    /// A list of local state snapshots.
    pub snapshots: Vec<Snapshot>,
}

/// Returns the application's response to a snapshot offer.
///
/// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
///
/// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum OfferSnapshot {
    /// Unknown result, abort all snapshot restoration
    Unknown = 0,
    /// Snapshot accepted, apply chunks
    Accept = 1,
    /// Abort all snapshot restoration
    Abort = 2,
    /// Reject this specific snapshot, try others
    Reject = 3,
    /// Reject all snapshots of this format, try others
    RejectFormat = 4,
    /// Reject all snapshots from the sender(s), try others
    RejectSender = 5,
}

impl Default for OfferSnapshot {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Returns a snapshot chunk from the application.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct LoadSnapshotChunk {
    /// The binary chunk contents, in an arbitrary format.
    ///
    /// Chunk messages cannot be larger than 16MB *including metadata*, so 10MB
    /// is a good starting point.
    pub chunk: Bytes,
}
/// Returns the result of applying a snapshot chunk and associated data.
///
/// The application can choose to refetch chunks and/or ban P2P peers as
/// appropriate. Tendermint will not do this unless instructed by the
/// application.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ApplySnapshotChunk {
    /// The result of applying the snapshot chunk.
    pub result: ApplySnapshotChunkResult,
    /// Refetch and reapply the given chunks, regardless of `result`.
    ///
    /// Only the listed chunks will be refetched, and reapplied in sequential
    /// order.
    pub refetch_chunks: Vec<u32>,
    /// Reject the given P2P senders, regardless of `result`.
    ///
    /// Any chunks already applied will not be refetched unless explicitly
    /// requested, but queued chunks from these senders will be discarded, and
    /// new chunks or other snapshots rejected.
    pub reject_senders: Vec<String>,
}

/// The result of applying a snapshot chunk.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ApplySnapshotChunkResult {
    /// Unknown result, abort all snapshot restoration.
    Unknown = 0,
    /// The chunk was accepted.
    Accept = 1,
    /// Abort snapshot restoration, and don't try any other snapshots.
    Abort = 2,
    /// Reapply this chunk, combine with
    /// [`refetch_chunks`](ApplySnapshotChunk::refetch_chunks) and
    /// [`reject_senders`](ApplySnapshotChunk::reject_senders) as appropriate.
    Retry = 3,
    /// Restart this snapshot from
    /// [`OfferSnapshot`](super::request::OfferSnapshot), reusing chunks unless
    /// instructed otherwise.
    RetrySnapshot = 4,
    /// Reject this snapshot, try a different one.
    RejectSnapshot = 5,
}

impl Default for ApplySnapshotChunkResult {
    fn default() -> Self {
        Self::Unknown
    }
}

/// All possible ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Response {
    /// Undocumented, nondeterministic.
    Exception(Exception),
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
    /// Indicates that all pending requests have been completed with their responses flushed.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#flush)
    Flush,
    /// Returns information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Returned on genesis after initializing chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Returns data queried from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
    /// Returns the result of checking a transaction for mempool inclusion.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
    /// Finalize block
    FinalizeBlock(FinalizeBlock),
    /// Returns the result of persisting the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit(Commit),
    /// Returns a list of local state snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots(ListSnapshots),
    /// Returns the application's response to a snapshot offer.
    ///
    /// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
    ///
    /// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
    OfferSnapshot(OfferSnapshot),
    /// Returns a snapshot chunk from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Returns the result of applying a snapshot chunk and associated data.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
    /// Prepare proposal
    PrepareProposal(PrepareProposal),
    /// Verify header
    VerifyHeader(VerifyHeader),
    /// Process proposal
    ProcessProposal(ProcessProposal),
    /// Revert proposal
    RevertProposal(RevertProposal),
    /// Extend vote
    ExtendVote(ExtendVote),
    /// Verify vote extension
    VerifyVoteExtension(VerifyVoteExtension),
}

/// The consensus category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConsensusResponse {
    /// Returned on genesis after initializing chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Returns the result of persisting the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit(Commit),
    /// Finalize block
    FinalizeBlock(FinalizeBlock),
    /// Prepare proposal
    PrepareProposal(PrepareProposal),
    /// Verify header
    VerifyHeader(VerifyHeader),
    /// Process proposal
    ProcessProposal(ProcessProposal),
    /// Revert proposal
    RevertProposal(RevertProposal),
    /// Extend vote
    ExtendVote(ExtendVote),
    /// Verify vote extension
    VerifyVoteExtension(VerifyVoteExtension),
}

impl From<ConsensusResponse> for Response {
    fn from(req: ConsensusResponse) -> Self {
        match req {
            ConsensusResponse::InitChain(x) => Self::InitChain(x),
            ConsensusResponse::Commit(x) => Self::Commit(x),
            ConsensusResponse::FinalizeBlock(x) => Self::FinalizeBlock(x),
            ConsensusResponse::PrepareProposal(x) => Self::PrepareProposal(x),
            ConsensusResponse::VerifyHeader(x) => Self::VerifyHeader(x),
            ConsensusResponse::ProcessProposal(x) => Self::ProcessProposal(x),
            ConsensusResponse::RevertProposal(x) => Self::RevertProposal(x),
            ConsensusResponse::ExtendVote(x) => Self::ExtendVote(x),
            ConsensusResponse::VerifyVoteExtension(x) => Self::VerifyVoteExtension(x),
        }
    }
}

impl TryFrom<Response> for ConsensusResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::InitChain(x) => Ok(Self::InitChain(x)),
            Response::Commit(x) => Ok(Self::Commit(x)),
            Response::FinalizeBlock(x) => Ok(Self::FinalizeBlock(x)),
            Response::PrepareProposal(x) => Ok(Self::PrepareProposal(x)),
            Response::VerifyHeader(x) => Ok(Self::VerifyHeader(x)),
            Response::ProcessProposal(x) => Ok(Self::ProcessProposal(x)),
            Response::RevertProposal(x) => Ok(Self::RevertProposal(x)),
            Response::ExtendVote(x) => Ok(Self::ExtendVote(x)),
            Response::VerifyVoteExtension(x) => Ok(Self::VerifyVoteExtension(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The mempool category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MempoolResponse {
    /// Returns the result of checking a transaction for mempool inclusion.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
}

impl From<MempoolResponse> for Response {
    fn from(req: MempoolResponse) -> Self {
        match req {
            MempoolResponse::CheckTx(x) => Self::CheckTx(x),
        }
    }
}

impl TryFrom<Response> for MempoolResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::CheckTx(x) => Ok(Self::CheckTx(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The info category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InfoResponse {
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
    /// Returns information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Returns data queried from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
}

impl From<InfoResponse> for Response {
    fn from(req: InfoResponse) -> Self {
        match req {
            InfoResponse::Echo(x) => Self::Echo(x),
            InfoResponse::Info(x) => Self::Info(x),
            InfoResponse::Query(x) => Self::Query(x),
        }
    }
}

impl TryFrom<Response> for InfoResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::Echo(x) => Ok(Self::Echo(x)),
            Response::Info(x) => Ok(Self::Info(x)),
            Response::Query(x) => Ok(Self::Query(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The snapshot category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SnapshotResponse {
    /// Returns a list of local state snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots(ListSnapshots),
    /// Returns the application's response to a snapshot offer.
    ///
    /// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
    ///
    /// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
    OfferSnapshot(OfferSnapshot),
    /// Returns a snapshot chunk from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Returns the result of applying a snapshot chunk and associated data.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
}

impl From<SnapshotResponse> for Response {
    fn from(req: SnapshotResponse) -> Self {
        match req {
            SnapshotResponse::ListSnapshots(x) => Self::ListSnapshots(x),
            SnapshotResponse::OfferSnapshot(x) => Self::OfferSnapshot(x),
            SnapshotResponse::LoadSnapshotChunk(x) => Self::LoadSnapshotChunk(x),
            SnapshotResponse::ApplySnapshotChunk(x) => Self::ApplySnapshotChunk(x),
        }
    }
}

impl TryFrom<Response> for SnapshotResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::ListSnapshots(x) => Ok(Self::ListSnapshots(x)),
            Response::OfferSnapshot(x) => Ok(Self::OfferSnapshot(x)),
            Response::LoadSnapshotChunk(x) => Ok(Self::LoadSnapshotChunk(x)),
            Response::ApplySnapshotChunk(x) => Ok(Self::ApplySnapshotChunk(x)),
            _ => Err("wrong request type"),
        }
    }
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// XXX(hdevalence): these all use &'static str for now, this should be fixed
// to align with the crate's error-handling strategy.

use tendermint_proto::abci as pb;
use tendermint_proto::Protobuf;

impl From<Exception> for pb::ResponseException {
    fn from(exception: Exception) -> Self {
        Self {
            error: exception.error,
        }
    }
}

impl TryFrom<pb::ResponseException> for Exception {
    type Error = &'static str;

    fn try_from(exception: pb::ResponseException) -> Result<Self, Self::Error> {
        Ok(Self {
            error: exception.error,
        })
    }
}

impl Protobuf<pb::ResponseException> for Exception {}

impl From<Echo> for pb::ResponseEcho {
    fn from(echo: Echo) -> Self {
        Self {
            message: echo.message,
        }
    }
}

impl TryFrom<pb::ResponseEcho> for Echo {
    type Error = &'static str;

    fn try_from(echo: pb::ResponseEcho) -> Result<Self, Self::Error> {
        Ok(Self {
            message: echo.message,
        })
    }
}

impl Protobuf<pb::ResponseEcho> for Echo {}

impl From<Info> for pb::ResponseInfo {
    fn from(info: Info) -> Self {
        Self {
            data: info.data,
            version: info.version,
            app_version: info.app_version,
            last_block_height: info.last_block_height,
            last_block_app_hash: info.last_block_app_hash,
        }
    }
}

impl TryFrom<pb::ResponseInfo> for Info {
    type Error = &'static str;

    fn try_from(info: pb::ResponseInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            data: info.data,
            version: info.version,
            app_version: info.app_version,
            last_block_height: info.last_block_height,
            last_block_app_hash: info.last_block_app_hash,
        })
    }
}

impl Protobuf<pb::ResponseInfo> for Info {}

impl From<InitChain> for pb::ResponseInitChain {
    fn from(init_chain: InitChain) -> Self {
        Self {
            consensus_params: init_chain.consensus_params.map(Into::into),
            validators: init_chain.validators.into_iter().map(Into::into).collect(),
            app_hash: init_chain.app_hash,
        }
    }
}

impl TryFrom<pb::ResponseInitChain> for InitChain {
    type Error = crate::Error;

    fn try_from(init_chain: pb::ResponseInitChain) -> Result<Self, Self::Error> {
        Ok(Self {
            consensus_params: init_chain
                .consensus_params
                .map(TryInto::try_into)
                .transpose()?,
            validators: init_chain
                .validators
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            app_hash: init_chain.app_hash,
        })
    }
}

impl Protobuf<pb::ResponseInitChain> for InitChain {}

impl From<Query> for pb::ResponseQuery {
    fn from(query: Query) -> Self {
        Self {
            code: query.code,
            log: query.log,
            info: query.info,
            index: query.index,
            key: query.key,
            value: query.value,
            proof_ops: query.proof.map(Into::into),
            height: query.height,
            codespace: query.codespace,
        }
    }
}

impl TryFrom<pb::ResponseQuery> for Query {
    type Error = crate::Error;

    fn try_from(query: pb::ResponseQuery) -> Result<Self, Self::Error> {
        Ok(Self {
            code: query.code,
            log: query.log,
            info: query.info,
            index: query.index,
            key: query.key,
            value: query.value,
            proof: query.proof_ops.map(TryInto::try_into).transpose()?,
            height: query.height,
            codespace: query.codespace,
        })
    }
}

impl Protobuf<pb::ResponseQuery> for Query {}

impl From<Commit> for pb::ResponseCommit {
    fn from(commit: Commit) -> Self {
        Self {
            data: commit.data,
            retain_height: commit.retain_height,
        }
    }
}

impl TryFrom<pb::ResponseCommit> for Commit {
    type Error = crate::Error;

    fn try_from(commit: pb::ResponseCommit) -> Result<Self, Self::Error> {
        Ok(Self {
            data: commit.data,
            retain_height: commit.retain_height,
        })
    }
}

impl Protobuf<pb::ResponseCommit> for Commit {}

impl From<CheckTx> for pb::ResponseCheckTx {
    fn from(check_tx: CheckTx) -> Self {
        Self {
            code: check_tx.code,
            data: check_tx.data,
            log: check_tx.log,
            info: check_tx.info,
            gas_wanted: check_tx.gas_wanted,
            gas_used: check_tx.gas_used,
            events: check_tx.events.into_iter().map(Into::into).collect(),
            codespace: check_tx.codespace,
        }
    }
}

impl TryFrom<pb::ResponseCheckTx> for CheckTx {
    type Error = crate::Error;

    fn try_from(check_tx: pb::ResponseCheckTx) -> Result<Self, Self::Error> {
        Ok(Self {
            code: check_tx.code,
            data: check_tx.data,
            log: check_tx.log,
            info: check_tx.info,
            gas_wanted: check_tx.gas_wanted,
            gas_used: check_tx.gas_used,
            events: check_tx
                .events
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            codespace: check_tx.codespace,
        })
    }
}

impl Protobuf<pb::ResponseCheckTx> for CheckTx {}

impl From<OfferSnapshot> for pb::ResponseOfferSnapshot {
    fn from(offer_snapshot: OfferSnapshot) -> Self {
        Self {
            result: offer_snapshot as i32,
        }
    }
}

impl TryFrom<pb::ResponseOfferSnapshot> for OfferSnapshot {
    type Error = crate::Error;

    fn try_from(offer_snapshot: pb::ResponseOfferSnapshot) -> Result<Self, Self::Error> {
        Ok(match offer_snapshot.result {
            0 => OfferSnapshot::Unknown,
            1 => OfferSnapshot::Accept,
            2 => OfferSnapshot::Abort,
            3 => OfferSnapshot::Reject,
            4 => OfferSnapshot::RejectFormat,
            5 => OfferSnapshot::RejectSender,
            _ => Err("unknown offer snapshot result code")?,
        })
    }
}

impl Protobuf<pb::ResponseOfferSnapshot> for OfferSnapshot {}

impl From<LoadSnapshotChunk> for pb::ResponseLoadSnapshotChunk {
    fn from(load_snapshot_chunk: LoadSnapshotChunk) -> Self {
        Self {
            chunk: load_snapshot_chunk.chunk,
        }
    }
}

impl TryFrom<pb::ResponseLoadSnapshotChunk> for LoadSnapshotChunk {
    type Error = crate::Error;

    fn try_from(load_snapshot_chunk: pb::ResponseLoadSnapshotChunk) -> Result<Self, Self::Error> {
        Ok(Self {
            chunk: load_snapshot_chunk.chunk,
        })
    }
}

impl Protobuf<pb::ResponseLoadSnapshotChunk> for LoadSnapshotChunk {}

impl From<ListSnapshots> for pb::ResponseListSnapshots {
    fn from(list_snapshots: ListSnapshots) -> Self {
        Self {
            snapshots: list_snapshots
                .snapshots
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<pb::ResponseListSnapshots> for ListSnapshots {
    type Error = crate::Error;

    fn try_from(list_snapshots: pb::ResponseListSnapshots) -> Result<Self, Self::Error> {
        Ok(Self {
            snapshots: list_snapshots
                .snapshots
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::ResponseListSnapshots> for ListSnapshots {}

impl From<ApplySnapshotChunk> for pb::ResponseApplySnapshotChunk {
    fn from(apply_snapshot_chunk: ApplySnapshotChunk) -> Self {
        Self {
            result: apply_snapshot_chunk.result as i32,
            refetch_chunks: apply_snapshot_chunk.refetch_chunks,
            reject_senders: apply_snapshot_chunk.reject_senders,
        }
    }
}

impl TryFrom<pb::ResponseApplySnapshotChunk> for ApplySnapshotChunk {
    type Error = crate::Error;

    fn try_from(apply_snapshot_chunk: pb::ResponseApplySnapshotChunk) -> Result<Self, Self::Error> {
        let result = match apply_snapshot_chunk.result {
            0 => ApplySnapshotChunkResult::Unknown,
            1 => ApplySnapshotChunkResult::Accept,
            2 => ApplySnapshotChunkResult::Abort,
            3 => ApplySnapshotChunkResult::Retry,
            4 => ApplySnapshotChunkResult::RetrySnapshot,
            5 => ApplySnapshotChunkResult::RejectSnapshot,
            _ => Err("unknown snapshot chunk result")?,
        };
        Ok(Self {
            result,
            refetch_chunks: apply_snapshot_chunk.refetch_chunks,
            reject_senders: apply_snapshot_chunk.reject_senders,
        })
    }
}

impl Protobuf<pb::ResponseApplySnapshotChunk> for ApplySnapshotChunk {}

impl From<ResponseTx> for pb::ResponseTx {
    fn from(response_tx: ResponseTx) -> Self {
        Self {
            code: response_tx.code,
            data: response_tx.data,
            log: response_tx.log,
            info: response_tx.info,
            gas_wanted: response_tx.gas_wanted,
            gas_used: response_tx.gas_used,
            events: response_tx.events.into_iter().map(Into::into).collect(),
            codespace: response_tx.codespace,
        }
    }
}

impl TryFrom<pb::ResponseTx> for ResponseTx {
    type Error = crate::Error;

    fn try_from(response_tx: pb::ResponseTx) -> Result<Self, Self::Error> {
        Ok(Self {
            code: response_tx.code,
            data: response_tx.data,
            log: response_tx.log,
            info: response_tx.info,
            gas_wanted: response_tx.gas_wanted,
            gas_used: response_tx.gas_used,
            events: response_tx
                .events
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            codespace: response_tx.codespace,
        })
    }
}

impl Protobuf<pb::ResponseTx> for ResponseTx {}

impl From<ConsensusUpdates> for pb::ConsensusUpdates {
    fn from(consensus_updates: ConsensusUpdates) -> Self {
        Self {
            validator_updates: consensus_updates
                .validator_updates
                .into_iter()
                .map(Into::into)
                .collect(),
            consensus_param_updates: consensus_updates.consensus_param_updates.map(Into::into),
            events: consensus_updates
                .events
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<pb::ConsensusUpdates> for ConsensusUpdates {
    type Error = crate::Error;

    fn try_from(consensus_updates: pb::ConsensusUpdates) -> Result<Self, Self::Error> {
        Ok(Self {
            validator_updates: consensus_updates
                .validator_updates
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
            consensus_param_updates: consensus_updates
                .consensus_param_updates
                .map(TryInto::try_into)
                .transpose()?,
            events: consensus_updates
                .events
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::ConsensusUpdates> for ConsensusUpdates {}

impl From<FinalizeBlock> for pb::ResponseFinalizeBlock {
    fn from(finalize_block: FinalizeBlock) -> Self {
        Self {
            updates: finalize_block
                .updates
                .and_then(|updates| updates.try_into().ok()),
            tx_results: finalize_block
                .tx_results
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<pb::ResponseFinalizeBlock> for FinalizeBlock {
    type Error = crate::Error;

    fn try_from(finalize_block: pb::ResponseFinalizeBlock) -> Result<Self, Self::Error> {
        Ok(Self {
            updates: finalize_block
                .updates
                .and_then(|updates| updates.try_into().ok()),
            tx_results: finalize_block
                .tx_results
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::ResponseFinalizeBlock> for FinalizeBlock {}

impl From<PrepareProposal> for pb::ResponsePrepareProposal {
    fn from(_prepare_proposal: PrepareProposal) -> Self {
        Self {}
    }
}

impl TryFrom<pb::ResponsePrepareProposal> for PrepareProposal {
    type Error = crate::Error;

    fn try_from(_prepare_proposal: pb::ResponsePrepareProposal) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl Protobuf<pb::ResponsePrepareProposal> for PrepareProposal {}

impl From<VerifyHeader> for pb::ResponseVerifyHeader {
    fn from(verify_header: VerifyHeader) -> Self {
        Self {
            accept_header: verify_header.accept_header,
            evidence: verify_header.evidence.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<pb::ResponseVerifyHeader> for VerifyHeader {
    type Error = crate::Error;

    fn try_from(verify_header: pb::ResponseVerifyHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            accept_header: verify_header.accept_header,
            evidence: verify_header
                .evidence
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::ResponseVerifyHeader> for VerifyHeader {}

impl From<ProcessProposal> for pb::ResponseProcessProposal {
    fn from(process_proposal: ProcessProposal) -> Self {
        Self {
            accept_block: process_proposal.accept_block,
            evidence: process_proposal
                .evidence
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl TryFrom<pb::ResponseProcessProposal> for ProcessProposal {
    type Error = crate::Error;

    fn try_from(process_proposal: pb::ResponseProcessProposal) -> Result<Self, Self::Error> {
        Ok(Self {
            accept_block: process_proposal.accept_block,
            evidence: process_proposal
                .evidence
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::ResponseProcessProposal> for ProcessProposal {}

impl From<RevertProposal> for pb::ResponseRevertProposal {
    fn from(_revert_proposal: RevertProposal) -> Self {
        Self {}
    }
}

impl TryFrom<pb::ResponseRevertProposal> for RevertProposal {
    type Error = crate::Error;

    fn try_from(_revert_proposal: pb::ResponseRevertProposal) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl From<ExtendVote> for pb::ResponseExtendVote {
    fn from(extend_vote: ExtendVote) -> Self {
        Self {
            unsigned_app_vote_data: extend_vote.unsigned_app_vote_data.into(),
            self_authenticating_app_data: extend_vote.self_authenticating_app_data.into(),
        }
    }
}

impl TryFrom<pb::ResponseExtendVote> for ExtendVote {
    type Error = crate::Error;

    fn try_from(extend_vote: pb::ResponseExtendVote) -> Result<Self, Self::Error> {
        Ok(Self {
            unsigned_app_vote_data: extend_vote.unsigned_app_vote_data.into(),
            self_authenticating_app_data: extend_vote.self_authenticating_app_data.into(),
        })
    }
}

impl From<VerifyVoteExtension> for pb::ResponseVerifyVoteExtension {
    fn from(verify_vote_extension: VerifyVoteExtension) -> Self {
        Self {
            result: verify_vote_extension.result,
        }
    }
}

impl TryFrom<pb::ResponseVerifyVoteExtension> for VerifyVoteExtension {
    type Error = crate::Error;

    fn try_from(
        verify_vote_extension: pb::ResponseVerifyVoteExtension,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            result: verify_vote_extension.result,
        })
    }
}

impl From<Response> for pb::Response {
    fn from(response: Response) -> pb::Response {
        use pb::response::Value;
        let value = match response {
            Response::Exception(x) => Some(Value::Exception(x.into())),
            Response::Echo(x) => Some(Value::Echo(x.into())),
            Response::Flush => Some(Value::Flush(Default::default())),
            Response::Info(x) => Some(Value::Info(x.into())),
            Response::InitChain(x) => Some(Value::InitChain(x.into())),
            Response::Query(x) => Some(Value::Query(x.into())),
            Response::FinalizeBlock(x) => Some(Value::FinalizeBlock(x.into())),
            Response::CheckTx(x) => Some(Value::CheckTx(x.into())),
            Response::PrepareProposal(x) => Some(Value::PrepareProposal(x.into())),
            Response::VerifyHeader(x) => Some(Value::VerifyHeader(x.into())),
            Response::ProcessProposal(x) => Some(Value::ProcessProposal(x.into())),
            Response::RevertProposal(x) => Some(Value::RevertProposal(x.into())),
            Response::ExtendVote(x) => Some(Value::ExtendVote(x.into())),
            Response::VerifyVoteExtension(x) => Some(Value::VerifyVoteExtension(x.into())),
            Response::Commit(x) => Some(Value::Commit(x.into())),
            Response::ListSnapshots(x) => Some(Value::ListSnapshots(x.into())),
            Response::OfferSnapshot(x) => Some(Value::OfferSnapshot(x.into())),
            Response::LoadSnapshotChunk(x) => Some(Value::LoadSnapshotChunk(x.into())),
            Response::ApplySnapshotChunk(x) => Some(Value::ApplySnapshotChunk(x.into())),
        };
        pb::Response { value }
    }
}

impl TryFrom<pb::Response> for Response {
    type Error = crate::Error;

    fn try_from(response: pb::Response) -> Result<Self, Self::Error> {
        use pb::response::Value;
        match response.value {
            Some(Value::Exception(x)) => Ok(Response::Exception(x.try_into()?)),
            Some(Value::Echo(x)) => Ok(Response::Echo(x.try_into()?)),
            Some(Value::Flush(_)) => Ok(Response::Flush),
            Some(Value::Info(x)) => Ok(Response::Info(x.try_into()?)),
            Some(Value::InitChain(x)) => Ok(Response::InitChain(x.try_into()?)),
            Some(Value::Query(x)) => Ok(Response::Query(x.try_into()?)),
            Some(Value::FinalizeBlock(x)) => Ok(Response::FinalizeBlock(x.try_into()?)),
            Some(Value::CheckTx(x)) => Ok(Response::CheckTx(x.try_into()?)),
            Some(Value::PrepareProposal(x)) => Ok(Response::PrepareProposal(x.try_into()?)),
            Some(Value::VerifyHeader(x)) => Ok(Response::VerifyHeader(x.try_into()?)),
            Some(Value::ProcessProposal(x)) => Ok(Response::ProcessProposal(x.try_into()?)),
            Some(Value::RevertProposal(x)) => Ok(Response::RevertProposal(x.try_into()?)),
            Some(Value::ExtendVote(x)) => Ok(Response::ExtendVote(x.try_into()?)),
            Some(Value::VerifyVoteExtension(x)) => Ok(Response::VerifyVoteExtension(x.try_into()?)),
            Some(Value::Commit(x)) => Ok(Response::Commit(x.try_into()?)),
            Some(Value::ListSnapshots(x)) => Ok(Response::ListSnapshots(x.try_into()?)),
            Some(Value::OfferSnapshot(x)) => Ok(Response::OfferSnapshot(x.try_into()?)),
            Some(Value::LoadSnapshotChunk(x)) => Ok(Response::LoadSnapshotChunk(x.try_into()?)),
            Some(Value::ApplySnapshotChunk(x)) => Ok(Response::ApplySnapshotChunk(x.try_into()?)),
            None => Err("no response in proto".into()),
        }
    }
}

impl Protobuf<pb::Response> for Response {}
