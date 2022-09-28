//! `/events` endpoint JSON-RPC wrapper

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::event::Event;
use crate::prelude::*;

/// Request ABCI events.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Request {
    pub query: String,
    pub max_wait_time: String,
}

impl Request {
    /// Query the Tendermint events with the given `query` until
    /// `timeout` expires.
    pub fn new(query: String, max_wait_time: Duration) -> Self {
        let max_wait_time = format!("{}s", max_wait_time.as_secs());
        Self {
            query,
            max_wait_time,
        }
    }
}

impl crate::Request for Request {
    type Response = Response;

    fn method(&self) -> crate::Method {
        crate::Method::Events
    }
}

impl crate::SimpleRequest for Request {}

/// Contains events in response to a [`Request`].
#[derive(Clone, Debug, Deserialize, Serialize)]
#[repr(transparent)]
pub struct Response(pub Event);

impl crate::Response for Response {}

impl From<Response> for Event {
    #[inline]
    fn from(response: Response) -> Event {
        response.0
    }
}
