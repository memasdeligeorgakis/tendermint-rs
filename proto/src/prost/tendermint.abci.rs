// This file is copied from <http://github.com/tendermint/abci>
// NOTE: When using custom types, mind the warnings.
// <https://github.com/gogo/protobuf/blob/master/custom_types.md#warnings-and-issues>

//----------------------------------------
// Request types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof="request::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19")]
    pub value: ::core::option::Option<request::Value>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="1")]
        Echo(super::RequestEcho),
        #[prost(message, tag="2")]
        Flush(super::RequestFlush),
        #[prost(message, tag="3")]
        Info(super::RequestInfo),
        #[prost(message, tag="4")]
        InitChain(super::RequestInitChain),
        #[prost(message, tag="5")]
        Query(super::RequestQuery),
        #[prost(message, tag="6")]
        BeginBlock(super::RequestBeginBlock),
        #[prost(message, tag="7")]
        CheckTx(super::RequestCheckTx),
        #[prost(message, tag="8")]
        DeliverTx(super::RequestDeliverTx),
        #[prost(message, tag="9")]
        EndBlock(super::RequestEndBlock),
        #[prost(message, tag="10")]
        Commit(super::RequestCommit),
        #[prost(message, tag="11")]
        ListSnapshots(super::RequestListSnapshots),
        #[prost(message, tag="12")]
        OfferSnapshot(super::RequestOfferSnapshot),
        #[prost(message, tag="13")]
        LoadSnapshotChunk(super::RequestLoadSnapshotChunk),
        #[prost(message, tag="14")]
        ApplySnapshotChunk(super::RequestApplySnapshotChunk),
        #[prost(message, tag="15")]
        PrepareProposal(super::RequestPrepareProposal),
        #[prost(message, tag="16")]
        ProcessProposal(super::RequestProcessProposal),
        #[prost(message, tag="17")]
        ExtendVote(super::RequestExtendVote),
        #[prost(message, tag="18")]
        VerifyVoteExtension(super::RequestVerifyVoteExtension),
        #[prost(message, tag="19")]
        FinalizeBlock(super::RequestFinalizeBlock),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEcho {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFlush {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInfo {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub block_version: u64,
    #[prost(uint64, tag="3")]
    pub p2p_version: u64,
    #[prost(string, tag="4")]
    pub abci_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestInitChain {
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<super::super::google::protobuf::Timestamp>,
    #[prost(string, tag="2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub consensus_params: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(message, repeated, tag="4")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes="vec", tag="5")]
    pub app_state_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="6")]
    pub initial_height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestQuery {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub height: i64,
    #[prost(bool, tag="4")]
    pub prove: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBeginBlock {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub header: ::core::option::Option<super::types::Header>,
    #[prost(message, optional, tag="3")]
    pub last_commit_info: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag="4")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<Misbehavior>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCheckTx {
    #[prost(bytes="vec", tag="1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="CheckTxType", tag="2")]
    pub r#type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestDeliverTx {
    #[prost(bytes="vec", tag="1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestEndBlock {
    #[prost(int64, tag="1")]
    pub height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestCommit {
}
/// lists available snapshots
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestListSnapshots {
}
/// offers a snapshot to the application
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestOfferSnapshot {
    /// snapshot offered by peers
    #[prost(message, optional, tag="1")]
    pub snapshot: ::core::option::Option<Snapshot>,
    /// light client-verified app hash for snapshot height
    #[prost(bytes="vec", tag="2")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
/// loads a snapshot chunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLoadSnapshotChunk {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(uint32, tag="2")]
    pub format: u32,
    #[prost(uint32, tag="3")]
    pub chunk: u32,
}
/// Applies a snapshot chunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestApplySnapshotChunk {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(bytes="vec", tag="2")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPrepareProposal {
    /// the modified transactions cannot exceed this size.
    #[prost(int64, tag="1")]
    pub max_tx_bytes: i64,
    /// txs is an array of transactions that will be included in a block,
    /// sent to the app for possible modifications.
    #[prost(bytes="vec", repeated, tag="2")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="3")]
    pub local_last_commit: ::core::option::Option<ExtendedCommitInfo>,
    #[prost(message, repeated, tag="4")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<Misbehavior>,
    #[prost(int64, tag="5")]
    pub height: i64,
    #[prost(message, optional, tag="6")]
    pub time: ::core::option::Option<super::super::google::protobuf::Timestamp>,
    #[prost(bytes="vec", tag="7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the validator proposing the block.
    #[prost(bytes="vec", tag="8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestProcessProposal {
    #[prost(bytes="vec", repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="2")]
    pub proposed_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag="3")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<Misbehavior>,
    /// hash is the merkle root hash of the fields of the proposed block.
    #[prost(bytes="vec", tag="4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub height: i64,
    #[prost(message, optional, tag="6")]
    pub time: ::core::option::Option<super::super::google::protobuf::Timestamp>,
    #[prost(bytes="vec", tag="7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// address of the public key of the original proposer of the block.
    #[prost(bytes="vec", tag="8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// Extends a vote with application-side injection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestExtendVote {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub height: i64,
}
/// Verify the vote extension
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestVerifyVoteExtension {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub height: i64,
    #[prost(bytes="vec", tag="4")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestFinalizeBlock {
    #[prost(bytes="vec", repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="2")]
    pub decided_last_commit: ::core::option::Option<CommitInfo>,
    #[prost(message, repeated, tag="3")]
    pub byzantine_validators: ::prost::alloc::vec::Vec<Misbehavior>,
    /// hash is the merkle root hash of the fields of the proposed block.
    #[prost(bytes="vec", tag="4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub height: i64,
    #[prost(message, optional, tag="6")]
    pub time: ::core::option::Option<super::super::google::protobuf::Timestamp>,
    #[prost(bytes="vec", tag="7")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// proposer_address is the address of the public key of the original proposer of the block.
    #[prost(bytes="vec", tag="8")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
//----------------------------------------
// Response types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Value", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20")]
    pub value: ::core::option::Option<response::Value>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="1")]
        Exception(super::ResponseException),
        #[prost(message, tag="2")]
        Echo(super::ResponseEcho),
        #[prost(message, tag="3")]
        Flush(super::ResponseFlush),
        #[prost(message, tag="4")]
        Info(super::ResponseInfo),
        #[prost(message, tag="5")]
        InitChain(super::ResponseInitChain),
        #[prost(message, tag="6")]
        Query(super::ResponseQuery),
        #[prost(message, tag="7")]
        BeginBlock(super::ResponseBeginBlock),
        #[prost(message, tag="8")]
        CheckTx(super::ResponseCheckTx),
        #[prost(message, tag="9")]
        DeliverTx(super::ResponseDeliverTx),
        #[prost(message, tag="10")]
        EndBlock(super::ResponseEndBlock),
        #[prost(message, tag="11")]
        Commit(super::ResponseCommit),
        #[prost(message, tag="12")]
        ListSnapshots(super::ResponseListSnapshots),
        #[prost(message, tag="13")]
        OfferSnapshot(super::ResponseOfferSnapshot),
        #[prost(message, tag="14")]
        LoadSnapshotChunk(super::ResponseLoadSnapshotChunk),
        #[prost(message, tag="15")]
        ApplySnapshotChunk(super::ResponseApplySnapshotChunk),
        #[prost(message, tag="16")]
        PrepareProposal(super::ResponsePrepareProposal),
        #[prost(message, tag="17")]
        ProcessProposal(super::ResponseProcessProposal),
        #[prost(message, tag="18")]
        ExtendVote(super::ResponseExtendVote),
        #[prost(message, tag="19")]
        VerifyVoteExtension(super::ResponseVerifyVoteExtension),
        #[prost(message, tag="20")]
        FinalizeBlock(super::ResponseFinalizeBlock),
    }
}
/// nondeterministic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseException {
    #[prost(string, tag="1")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEcho {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFlush {
}
#[derive(::serde::Deserialize, ::serde::Serialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInfo {
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
    /// this is the software version of the application. TODO: remove?
    #[prost(string, tag="2")]
    #[serde(default)]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    #[serde(with = "crate::serializers::from_str", default)]
    pub app_version: u64,
    #[prost(int64, tag="4")]
    #[serde(with = "crate::serializers::from_str")]
    pub last_block_height: i64,
    #[prost(bytes="vec", tag="5")]
    #[serde(skip_serializing_if = "::prost::alloc::vec::Vec::is_empty", with = "serde_bytes")]
    pub last_block_app_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseInitChain {
    #[prost(message, optional, tag="1")]
    pub consensus_params: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(message, repeated, tag="2")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(bytes="vec", tag="3")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseQuery {
    #[prost(uint32, tag="1")]
    pub code: u32,
    /// bytes data = 2; // use "value" instead.
    ///
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub index: i64,
    #[prost(bytes="vec", tag="6")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="8")]
    pub proof_ops: ::core::option::Option<super::crypto::ProofOps>,
    #[prost(int64, tag="9")]
    pub height: i64,
    #[prost(string, tag="10")]
    pub codespace: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseBeginBlock {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCheckTx {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    #[prost(message, repeated, tag="7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag="10")]
    pub priority: i64,
    // mempool_error is set by Tendermint.

    /// ABCI applications creating a ResponseCheckTX should not set mempool_error.
    #[prost(string, tag="11")]
    pub mempool_error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseDeliverTx {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    /// nondeterministic
    #[prost(message, repeated, tag="7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseEndBlock {
    #[prost(message, repeated, tag="1")]
    pub validator_updates: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(message, optional, tag="2")]
    pub consensus_param_updates: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseCommit {
    /// reserve 1
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub retain_height: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseListSnapshots {
    #[prost(message, repeated, tag="1")]
    pub snapshots: ::prost::alloc::vec::Vec<Snapshot>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseOfferSnapshot {
    #[prost(enumeration="response_offer_snapshot::Result", tag="1")]
    pub result: i32,
}
/// Nested message and enum types in `ResponseOfferSnapshot`.
pub mod response_offer_snapshot {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseLoadSnapshotChunk {
    #[prost(bytes="vec", tag="1")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseApplySnapshotChunk {
    #[prost(enumeration="response_apply_snapshot_chunk::Result", tag="1")]
    pub result: i32,
    /// Chunks to refetch and reapply
    #[prost(uint32, repeated, tag="2")]
    pub refetch_chunks: ::prost::alloc::vec::Vec<u32>,
    /// Chunk senders to reject and ban
    #[prost(string, repeated, tag="3")]
    pub reject_senders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ResponseApplySnapshotChunk`.
pub mod response_apply_snapshot_chunk {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unknown result, abort all snapshot restoration
        Unknown = 0,
        /// Chunk successfully accepted
        Accept = 1,
        /// Abort all snapshot restoration
        Abort = 2,
        /// Retry chunk (combine with refetch and reject)
        Retry = 3,
        /// Retry snapshot (combine with refetch and reject)
        RetrySnapshot = 4,
        /// Reject this snapshot, try others
        RejectSnapshot = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsePrepareProposal {
    #[prost(message, repeated, tag="1")]
    pub tx_records: ::prost::alloc::vec::Vec<TxRecord>,
    #[prost(bytes="vec", tag="2")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub tx_results: ::prost::alloc::vec::Vec<ExecTxResult>,
    #[prost(message, repeated, tag="4")]
    pub validator_updates: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(message, optional, tag="5")]
    pub consensus_param_updates: ::core::option::Option<super::types::ConsensusParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseProcessProposal {
    #[prost(enumeration="response_process_proposal::ProposalStatus", tag="1")]
    pub status: i32,
    #[prost(bytes="vec", tag="2")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub tx_results: ::prost::alloc::vec::Vec<ExecTxResult>,
    #[prost(message, repeated, tag="4")]
    pub validator_updates: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(message, optional, tag="5")]
    pub consensus_param_updates: ::core::option::Option<super::types::ConsensusParams>,
}
/// Nested message and enum types in `ResponseProcessProposal`.
pub mod response_process_proposal {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProposalStatus {
        Unknown = 0,
        Accept = 1,
        Reject = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseExtendVote {
    #[prost(bytes="vec", tag="1")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseVerifyVoteExtension {
    #[prost(enumeration="response_verify_vote_extension::VerifyStatus", tag="1")]
    pub status: i32,
}
/// Nested message and enum types in `ResponseVerifyVoteExtension`.
pub mod response_verify_vote_extension {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VerifyStatus {
        Unknown = 0,
        Accept = 1,
        Reject = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFinalizeBlock {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(message, repeated, tag="2")]
    pub tx_results: ::prost::alloc::vec::Vec<ExecTxResult>,
    #[prost(message, repeated, tag="3")]
    pub validator_updates: ::prost::alloc::vec::Vec<ValidatorUpdate>,
    #[prost(message, optional, tag="4")]
    pub consensus_param_updates: ::core::option::Option<super::types::ConsensusParams>,
    #[prost(bytes="vec", tag="5")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="6")]
    pub retain_height: i64,
}
//----------------------------------------
// Misc.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitInfo {
    #[prost(int32, tag="1")]
    pub round: i32,
    #[prost(message, repeated, tag="2")]
    pub votes: ::prost::alloc::vec::Vec<VoteInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedCommitInfo {
    /// The round at which the block proposer decided in the previous height.
    #[prost(int32, tag="1")]
    pub round: i32,
    /// List of validators' addresses in the last validator set with their voting
    /// information, including vote extensions.
    #[prost(message, repeated, tag="2")]
    pub votes: ::prost::alloc::vec::Vec<ExtendedVoteInfo>,
}
/// Event allows application developers to attach additional information to
/// ResponseBeginBlock, ResponseEndBlock, ResponseCheckTx and ResponseDeliverTx.
/// Later, transactions may be queried using these events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub attributes: ::prost::alloc::vec::Vec<EventAttribute>,
}
/// EventAttribute is a single key-value pair, associated with an event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttribute {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(bool, tag="3")]
    pub index: bool,
}
/// ExecTxResult contains results of executing one individual transaction.
///
/// * Its structure is equivalent to #ResponseDeliverTx which will be deprecated/deleted
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecTxResult {
    #[prost(uint32, tag="1")]
    pub code: u32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// nondeterministic
    #[prost(string, tag="3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag="4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub gas_wanted: i64,
    #[prost(int64, tag="6")]
    pub gas_used: i64,
    /// nondeterministic
    #[prost(message, repeated, tag="7")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(string, tag="8")]
    pub codespace: ::prost::alloc::string::String,
}
/// TxResult contains results of executing the transaction.
///
/// One usage is indexing transaction results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    #[prost(int64, tag="1")]
    pub height: i64,
    #[prost(uint32, tag="2")]
    pub index: u32,
    #[prost(bytes="vec", tag="3")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub result: ::core::option::Option<ExecTxResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRecord {
    #[prost(enumeration="tx_record::TxAction", tag="1")]
    pub action: i32,
    #[prost(bytes="vec", tag="2")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `TxRecord`.
pub mod tx_record {
    /// TxAction contains App-provided information on what to do with a transaction that is part of a raw proposal
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TxAction {
        /// Unknown action
        Unknown = 0,
        /// The Application did not modify this transaction.
        Unmodified = 1,
        /// The Application added this transaction.
        Added = 2,
        /// The Application wants this transaction removed from the proposal and the mempool.
        Removed = 3,
    }
}
//----------------------------------------
// Blockchain Types

/// Validator
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    /// The first 20 bytes of SHA256(public key)
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// PubKey pub_key = 2 \[(gogoproto.nullable)=false\];
    ///
    /// The voting power
    #[prost(int64, tag="3")]
    pub power: i64,
}
/// ValidatorUpdate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorUpdate {
    #[prost(message, optional, tag="1")]
    pub pub_key: ::core::option::Option<super::crypto::PublicKey>,
    #[prost(int64, tag="2")]
    pub power: i64,
}
/// VoteInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteInfo {
    #[prost(message, optional, tag="1")]
    pub validator: ::core::option::Option<Validator>,
    #[prost(bool, tag="2")]
    pub signed_last_block: bool,
}
/// ExtendedVoteInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedVoteInfo {
    /// The validator that sent the vote.
    #[prost(message, optional, tag="1")]
    pub validator: ::core::option::Option<Validator>,
    /// Indicates whether the validator signed the last block, allowing for rewards based on validator availability.
    #[prost(bool, tag="2")]
    pub signed_last_block: bool,
    /// Non-deterministic extension provided by the sending validator's application.
    #[prost(bytes="vec", tag="3")]
    pub vote_extension: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehavior {
    #[prost(enumeration="MisbehaviorType", tag="1")]
    pub r#type: i32,
    /// The offending validator
    #[prost(message, optional, tag="2")]
    pub validator: ::core::option::Option<Validator>,
    /// The height when the offense occurred
    #[prost(int64, tag="3")]
    pub height: i64,
    /// The corresponding time where the offense occurred
    #[prost(message, optional, tag="4")]
    pub time: ::core::option::Option<super::super::google::protobuf::Timestamp>,
    /// Total voting power of the validator set in case the ABCI application does
    /// not store historical validators.
    /// <https://github.com/tendermint/tendermint/issues/4581>
    #[prost(int64, tag="5")]
    pub total_voting_power: i64,
}
//----------------------------------------
// State Sync Types

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// The height at which the snapshot was taken
    #[prost(uint64, tag="1")]
    pub height: u64,
    /// The application-specific snapshot format
    #[prost(uint32, tag="2")]
    pub format: u32,
    /// Number of chunks in the snapshot
    #[prost(uint32, tag="3")]
    pub chunks: u32,
    /// Arbitrary snapshot hash, equal only if identical
    #[prost(bytes="vec", tag="4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Arbitrary application metadata
    #[prost(bytes="vec", tag="5")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CheckTxType {
    New = 0,
    Recheck = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MisbehaviorType {
    Unknown = 0,
    DuplicateVote = 1,
    LightClientAttack = 2,
}
