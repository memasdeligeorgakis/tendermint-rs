/// ABCIResponses retains the responses
/// of the various ABCI calls during block processing.
/// It is persisted to disk for each height before calling Commit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciResponses {
    #[prost(message, repeated, tag="1")]
    pub finalize_block: ::std::vec::Vec<super::abci::ResponseFinalizeBlock>,
}
/// ValidatorsInfo represents the latest validator set, or the last height it changed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorsInfo {
    #[prost(message, optional, tag="1")]
    pub validator_set: ::std::option::Option<super::types::ValidatorSet>,
    #[prost(int64, tag="2")]
    pub last_height_changed: i64,
}
/// ConsensusParamsInfo represents the latest consensus params, or the last height it changed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusParamsInfo {
    #[prost(message, optional, tag="1")]
    pub consensus_params: ::std::option::Option<super::types::ConsensusParams>,
    #[prost(int64, tag="2")]
    pub last_height_changed: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(message, optional, tag="1")]
    pub consensus: ::std::option::Option<super::version::Consensus>,
    #[prost(string, tag="2")]
    pub software: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(message, optional, tag="1")]
    pub version: ::std::option::Option<Version>,
    /// immutable
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
    #[prost(int64, tag="14")]
    pub initial_height: i64,
    /// LastBlockHeight=0 at genesis (ie. block(H=0) does not exist)
    #[prost(int64, tag="3")]
    pub last_block_height: i64,
    #[prost(message, optional, tag="4")]
    pub last_block_id: ::std::option::Option<super::types::BlockId>,
    #[prost(message, optional, tag="5")]
    pub last_block_time: ::std::option::Option<super::super::google::protobuf::Timestamp>,
    /// LastValidators is used to validate block.LastCommit.
    /// Validators are persisted to the database separately every time they change,
    /// so we can query for historical validator sets.
    /// Note that if s.LastBlockHeight causes a valset change,
    /// we set s.LastHeightValidatorsChanged = s.LastBlockHeight + 1 + 1
    /// Extra +1 due to nextValSet delay.
    #[prost(message, optional, tag="6")]
    pub next_validators: ::std::option::Option<super::types::ValidatorSet>,
    #[prost(message, optional, tag="7")]
    pub validators: ::std::option::Option<super::types::ValidatorSet>,
    #[prost(message, optional, tag="8")]
    pub last_validators: ::std::option::Option<super::types::ValidatorSet>,
    #[prost(int64, tag="9")]
    pub last_height_validators_changed: i64,
    /// Consensus parameters used for validating blocks.
    /// Changes returned by FinalizeBlock and updated after Commit.
    #[prost(message, optional, tag="10")]
    pub consensus_params: ::std::option::Option<super::types::ConsensusParams>,
    #[prost(int64, tag="11")]
    pub last_height_consensus_params_changed: i64,
    /// Merkle root of the results from executing prev block
    #[prost(bytes, tag="12")]
    pub last_results_hash: std::vec::Vec<u8>,
    /// the latest AppHash we've received from calling abci.Commit()
    #[prost(bytes, tag="13")]
    pub app_hash: std::vec::Vec<u8>,
}
