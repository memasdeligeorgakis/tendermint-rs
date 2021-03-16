//! ABCI application interface.

#[cfg(feature = "echo-app")]
pub mod echo;
#[cfg(feature = "kvstore-app")]
pub mod kvstore;

use tendermint_proto::abci::request::Value;
use tendermint_proto::abci::{
    response, Request, RequestApplySnapshotChunk, RequestCheckTx, RequestEcho, RequestExtendVote,
    RequestFinalizeBlock, RequestInfo, RequestInitChain, RequestLoadSnapshotChunk,
    RequestOfferSnapshot, RequestPrepareProposal, RequestProcessProposal, RequestQuery,
    RequestRevertProposal, RequestVerifyHeader, RequestVerifyVoteExtension, Response,
    ResponseApplySnapshotChunk, ResponseCheckTx, ResponseCommit, ResponseEcho, ResponseExtendVote,
    ResponseFinalizeBlock, ResponseFlush, ResponseInfo, ResponseInitChain, ResponseListSnapshots,
    ResponseLoadSnapshotChunk, ResponseOfferSnapshot, ResponsePrepareProposal,
    ResponseProcessProposal, ResponseQuery, ResponseRevertProposal, ResponseVerifyHeader,
    ResponseVerifyVoteExtension,
};

/// An ABCI application.
///
/// Applications are `Send` + `Clone` + `'static` because they are cloned for
/// each incoming connection to the ABCI [`Server`]. It is up to the
/// application developer to manage shared state between these clones of their
/// application.
///
/// [`Server`]: crate::Server
pub trait Application: Send + Clone + 'static {
    /// Echo back the same message as provided in the request.
    fn echo(&self, request: RequestEcho) -> ResponseEcho {
        ResponseEcho {
            message: request.message,
        }
    }

    /// Provide information about the ABCI application.
    fn info(&self, _request: RequestInfo) -> ResponseInfo {
        Default::default()
    }

    /// Called once upon genesis.
    fn init_chain(&self, _request: RequestInitChain) -> ResponseInitChain {
        Default::default()
    }

    /// Query the application for data at the current or past height.
    fn query(&self, _request: RequestQuery) -> ResponseQuery {
        Default::default()
    }

    /// Check the given transaction before putting it into the local mempool.
    fn check_tx(&self, _request: RequestCheckTx) -> ResponseCheckTx {
        Default::default()
    }

    /// Finalize block
    fn finalize_block(&self, _request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        Default::default()
    }

    /// Prepare proposal
    fn prepare_proposal(&self, _request: RequestPrepareProposal) -> ResponsePrepareProposal {
        Default::default()
    }

    /// Verify header
    fn verify_header(&self, _request: RequestVerifyHeader) -> ResponseVerifyHeader {
        Default::default()
    }

    /// Process proposal
    fn process_proposal(&self, _request: RequestProcessProposal) -> ResponseProcessProposal {
        Default::default()
    }

    /// Process proposal
    fn revert_proposal(&self, _request: RequestRevertProposal) -> ResponseRevertProposal {
        Default::default()
    }

    /// Extend vote
    fn extend_vote(&self, _request: RequestExtendVote) -> ResponseExtendVote {
        Default::default()
    }

    /// Verify vote extension
    fn verify_vote_extension(
        &self,
        _request: RequestVerifyVoteExtension,
    ) -> ResponseVerifyVoteExtension {
        Default::default()
    }

    /// Signals that messages queued on the client should be flushed to the server.
    fn flush(&self) -> ResponseFlush {
        ResponseFlush {}
    }

    /// Commit the current state at the current height.
    fn commit(&self) -> ResponseCommit {
        Default::default()
    }

    /// Used during state sync to discover available snapshots on peers.
    fn list_snapshots(&self) -> ResponseListSnapshots {
        Default::default()
    }

    /// Called when bootstrapping the node using state sync.
    fn offer_snapshot(&self, _request: RequestOfferSnapshot) -> ResponseOfferSnapshot {
        Default::default()
    }

    /// Used during state sync to retrieve chunks of snapshots from peers.
    fn load_snapshot_chunk(&self, _request: RequestLoadSnapshotChunk) -> ResponseLoadSnapshotChunk {
        Default::default()
    }

    /// Apply the given snapshot chunk to the application's state.
    fn apply_snapshot_chunk(
        &self,
        _request: RequestApplySnapshotChunk,
    ) -> ResponseApplySnapshotChunk {
        Default::default()
    }
}

/// Provides a mechanism for the [`Server`] to execute incoming requests while
/// expecting the correct response types.
pub trait RequestDispatcher {
    /// Executes the relevant application method based on the type of the
    /// request, and produces the corresponding response.
    fn handle(&self, request: Request) -> Response;
}

// Implement `RequestDispatcher` for all `Application`s.
impl<A: Application> RequestDispatcher for A {
    fn handle(&self, request: Request) -> Response {
        tracing::debug!("Incoming request: {:?}", request);
        Response {
            value: Some(match request.value.unwrap() {
                Value::Echo(req) => response::Value::Echo(self.echo(req)),
                Value::Flush(_) => response::Value::Flush(self.flush()),
                Value::Info(req) => response::Value::Info(self.info(req)),
                Value::InitChain(req) => response::Value::InitChain(self.init_chain(req)),
                Value::Query(req) => response::Value::Query(self.query(req)),
                Value::CheckTx(req) => response::Value::CheckTx(self.check_tx(req)),
                Value::Commit(_) => response::Value::Commit(self.commit()),
                Value::ListSnapshots(_) => response::Value::ListSnapshots(self.list_snapshots()),
                Value::OfferSnapshot(req) => {
                    response::Value::OfferSnapshot(self.offer_snapshot(req))
                }
                Value::LoadSnapshotChunk(req) => {
                    response::Value::LoadSnapshotChunk(self.load_snapshot_chunk(req))
                }
                Value::ApplySnapshotChunk(req) => {
                    response::Value::ApplySnapshotChunk(self.apply_snapshot_chunk(req))
                }
                Value::FinalizeBlock(req) => {
                    response::Value::FinalizeBlock(self.finalize_block(req))
                }
                Value::PrepareProposal(req) => {
                    response::Value::PrepareProposal(self.prepare_proposal(req))
                }
                Value::VerifyHeader(req) => response::Value::VerifyHeader(self.verify_header(req)),
                Value::ProcessProposal(req) => {
                    response::Value::ProcessProposal(self.process_proposal(req))
                }
                Value::RevertProposal(req) => {
                    response::Value::RevertProposal(self.revert_proposal(req))
                }
                Value::ExtendVote(req) => response::Value::ExtendVote(self.extend_vote(req)),
                Value::VerifyVoteExtension(req) => {
                    response::Value::VerifyVoteExtension(self.verify_vote_extension(req))
                }
            }),
        }
    }
}
