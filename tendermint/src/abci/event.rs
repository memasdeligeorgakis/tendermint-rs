/// An event that occurred while processing a request.
///
/// Application developers can attach additional information to
/// [`BeginBlock`](super::response::BeginBlock),
/// [`EndBlock`](super::response::EndBlock),
/// [`CheckTx`](super::response::CheckTx), and
/// [`DeliverTx`](super::response::DeliverTx) responses. Later, transactions may
/// be queried using these events.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#events)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Event {
    /// The kind of event.
    ///
    /// Tendermint calls this the `type`, but we use `kind` to avoid confusion
    /// with Rust types and follow Rust conventions.
    pub kind: String,
    /// A list of [`EventAttribute`]s describing the event.
    pub attributes: Vec<EventAttribute>,
}

/// A key-value pair describing an [`Event`].
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#events)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EventAttribute {
    /// The event key.
    pub key: String,
    /// The event value.
    pub value: String,
    /// Whether Tendermint's indexer should index this event.
    ///
    /// **This field is nondeterministic**.
    pub index: bool,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// XXX(hdevalence): these all use &'static str for now, this should be fixed
// to align with the crate's error-handling strategy.

use std::convert::{TryFrom, TryInto};

use tendermint_proto::abci as pb;
use tendermint_proto::Protobuf;

impl From<EventAttribute> for pb::EventAttribute {
    fn from(event: EventAttribute) -> Self {
        Self {
            key: event.key.into_bytes().into(),
            value: event.value.into_bytes().into(),
            index: event.index,
        }
    }
}

impl TryFrom<pb::EventAttribute> for EventAttribute {
    type Error = crate::Error;

    fn try_from(event: pb::EventAttribute) -> Result<Self, Self::Error> {
        Ok(Self {
            key: String::from_utf8(event.key.to_vec())?,
            value: String::from_utf8(event.value.to_vec())?,
            index: event.index,
        })
    }
}

impl Protobuf<pb::EventAttribute> for EventAttribute {}

impl From<Event> for pb::Event {
    fn from(event: Event) -> Self {
        Self {
            r#type: event.kind,
            attributes: event.attributes.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<pb::Event> for Event {
    type Error = crate::Error;

    fn try_from(event: pb::Event) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: event.r#type,
            attributes: event
                .attributes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::Event> for Event {}
