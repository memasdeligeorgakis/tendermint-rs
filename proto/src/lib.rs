//! tendermint-proto library gives the developer access to the Tendermint proto-defined structs.

#![deny(warnings, trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![allow(clippy::large_enum_variant)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/tendermint-proto/0.18.0")]

// Built-in prost_types with slight customization to enable JSON-encoding
pub mod google {
    pub mod protobuf {
        include!("prost/google.protobuf.rs");
        // custom Timeout and Duration types that have valid doctest documentation texts
        include!("protobuf.rs");
    }
}

mod tendermint;
pub use tendermint::*;

mod error;
use anomaly::BoxError;
use bytes::{Buf, BufMut};
pub use error::{Error, Kind};
use prost::encoding::encoded_len_varint;
use prost::Message;
use std::convert::{TryFrom, TryInto};

pub mod serializers;

/// Allows for easy Google Protocol Buffers encoding and decoding of domain
/// types with validation.
///
/// ## Examples
///
/// ```rust
/// use bytes::BufMut;
/// use prost::Message;
/// use std::convert::TryFrom;
/// use tendermint_proto::Protobuf;
///
/// // This struct would ordinarily be automatically generated by prost.
/// #[derive(Clone, PartialEq, Message)]
/// pub struct MyRawType {
///     #[prost(uint64, tag="1")]
///     pub a: u64,
///     #[prost(string, tag="2")]
///     pub b: String,
/// }
///
/// #[derive(Clone)]
/// pub struct MyDomainType {
///     a: u64,
///     b: String,
/// }
///
/// impl MyDomainType {
///     /// Trivial constructor with basic validation logic.
///     pub fn new(a: u64, b: String) -> Result<Self, String> {
///         if a < 1 {
///             return Err("a must be greater than 0".to_owned());
///         }
///         Ok(Self { a, b })
///     }
/// }
///
/// impl TryFrom<MyRawType> for MyDomainType {
///     type Error = String;
///
///     fn try_from(value: MyRawType) -> Result<Self, Self::Error> {
///         Self::new(value.a, value.b)
///     }
/// }
///
/// impl From<MyDomainType> for MyRawType {
///     fn from(value: MyDomainType) -> Self {
///         Self { a: value.a, b: value.b }
///     }
/// }
///
/// impl Protobuf<MyRawType> for MyDomainType {}
///
///
/// // Simulate an incoming valid raw message
/// let valid_raw = MyRawType { a: 1, b: "Hello!".to_owned() };
/// let mut valid_raw_bytes: Vec<u8> = Vec::new();
/// valid_raw.encode(&mut valid_raw_bytes).unwrap();
/// assert!(!valid_raw_bytes.is_empty());
///
/// // Try to decode the simulated incoming message
/// let valid_domain = MyDomainType::decode(valid_raw_bytes.clone().as_ref()).unwrap();
/// assert_eq!(1, valid_domain.a);
/// assert_eq!("Hello!".to_owned(), valid_domain.b);
///
/// // Encode it to compare the serialized form to what we received
/// let mut valid_domain_bytes: Vec<u8> = Vec::new();
/// valid_domain.encode(&mut valid_domain_bytes).unwrap();
/// assert_eq!(valid_raw_bytes, valid_domain_bytes);
///
/// // Simulate an incoming invalid raw message
/// let invalid_raw = MyRawType { a: 0, b: "Hello!".to_owned() };
/// let mut invalid_raw_bytes: Vec<u8> = Vec::new();
/// invalid_raw.encode(&mut invalid_raw_bytes).unwrap();
///
/// // We expect a validation error here
/// assert!(MyDomainType::decode(invalid_raw_bytes.as_ref()).is_err());
/// ```
pub trait Protobuf<T: Message + From<Self> + Default>
where
    Self: Sized + Clone + TryFrom<T>,
    <Self as TryFrom<T>>::Error: Into<BoxError>,
{
    /// Encode into a buffer in Protobuf format.
    ///
    /// Uses [`prost::Message::encode`] after converting into its counterpart
    /// Protobuf data structure.
    ///
    /// [`prost::Message::encode`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encode
    fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), Error> {
        T::from(self.clone())
            .encode(buf)
            .map_err(|e| Kind::EncodeMessage.context(e).into())
    }

    /// Encode with a length-delimiter to a buffer in Protobuf format.
    ///
    /// An error will be returned if the buffer does not have sufficient capacity.
    ///
    /// Uses [`prost::Message::encode_length_delimited`] after converting into
    /// its counterpart Protobuf data structure.
    ///
    /// [`prost::Message::encode_length_delimited`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encode_length_delimited
    fn encode_length_delimited<B: BufMut>(&self, buf: &mut B) -> Result<(), Error> {
        T::from(self.clone())
            .encode_length_delimited(buf)
            .map_err(|e| Kind::EncodeMessage.context(e).into())
    }

    /// Constructor that attempts to decode an instance from a buffer.
    ///
    /// The entire buffer will be consumed.
    ///
    /// Similar to [`prost::Message::decode`] but with additional validation
    /// prior to constructing the destination type.
    ///
    /// [`prost::Message::decode`]: https://docs.rs/prost/*/prost/trait.Message.html#method.decode
    fn decode<B: Buf>(buf: B) -> Result<Self, Error> {
        T::decode(buf).map_or_else(
            |e| Err(Kind::DecodeMessage.context(e).into()),
            |t| Self::try_from(t).map_err(|e| Kind::TryFromProtobuf.context(e).into()),
        )
    }

    /// Constructor that attempts to decode a length-delimited instance from
    /// the buffer.
    ///
    /// The entire buffer will be consumed.
    ///
    /// Similar to [`prost::Message::decode_length_delimited`] but with
    /// additional validation prior to constructing the destination type.
    ///
    /// [`prost::Message::decode_length_delimited`]: https://docs.rs/prost/*/prost/trait.Message.html#method.decode_length_delimited
    fn decode_length_delimited<B: Buf>(buf: B) -> Result<Self, Error> {
        T::decode_length_delimited(buf).map_or_else(
            |e| Err(Kind::DecodeMessage.context(e).into()),
            |t| Self::try_from(t).map_err(|e| Kind::TryFromProtobuf.context(e).into()),
        )
    }

    /// Returns the encoded length of the message without a length delimiter.
    ///
    /// Uses [`prost::Message::encoded_len`] after converting to its
    /// counterpart Protobuf data structure.
    ///
    /// [`prost::Message::encoded_len`]: https://docs.rs/prost/*/prost/trait.Message.html#method.encoded_len
    fn encoded_len(&self) -> usize {
        T::from(self.clone()).encoded_len()
    }

    /// Encodes into a Protobuf-encoded `Vec<u8>`.
    fn encode_vec(&self) -> Result<Vec<u8>, Error> {
        let mut wire = Vec::with_capacity(self.encoded_len());
        self.encode(&mut wire).map(|_| wire)
    }

    /// Constructor that attempts to decode a Protobuf-encoded instance from a
    /// `Vec<u8>` (or equivalent).
    fn decode_vec(v: &[u8]) -> Result<Self, Error> {
        Self::decode(v)
    }

    /// Encode with a length-delimiter to a `Vec<u8>` Protobuf-encoded message.
    fn encode_length_delimited_vec(&self) -> Result<Vec<u8>, Error> {
        let len = self.encoded_len();
        let lenu64 = len.try_into().map_err(|e| Kind::EncodeMessage.context(e))?;
        let mut wire = Vec::with_capacity(len + encoded_len_varint(lenu64));
        self.encode_length_delimited(&mut wire).map(|_| wire)
    }

    /// Constructor that attempts to decode a Protobuf-encoded instance with a
    /// length-delimiter from a `Vec<u8>` or equivalent.
    fn decode_length_delimited_vec(v: &[u8]) -> Result<Self, Error> {
        Self::decode_length_delimited(v)
    }
}
