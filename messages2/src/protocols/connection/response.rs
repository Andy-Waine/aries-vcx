use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};
use transitive::{TransitiveFrom, TransitiveTryFrom};

use crate::{
    composite_message::Message,
    decorators::{PleaseAck, Thread, Timing},
    message_type::{
        message_family::connection::{Connection as ConnectionKind, ConnectionV1, ConnectionV1_0},
        MessageFamily, MessageType,
    },
};

use crate::protocols::traits::MessageKind;

pub type Response = Message<ResponseContent, ResponseDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent)]
#[message(kind = "ConnectionV1_0::Response")]
pub struct ResponseContent {
    #[serde(rename = "connection~sig")]
    pub connection_sig: ConnectionSignature,
}

impl ResponseContent {
    pub fn new(connection_sig: ConnectionSignature) -> Self {
        Self { connection_sig }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConnectionSignature {
    #[serde(rename = "@type")]
    msg_type: SigEd25519Sha512Single,
    pub signature: String,
    pub sig_data: String,
    pub signer: String,
}

impl ConnectionSignature {
    pub fn new(signature: String, sig_data: String, signer: String) -> Self {
        Self {
            msg_type: SigEd25519Sha512Single,
            signature,
            sig_data,
            signer,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseDecorators {
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(rename = "~please_ack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub please_ack: Option<PleaseAck>,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

impl ResponseDecorators {
    pub fn new(thread: Thread) -> Self {
        Self {
            thread,
            please_ack: None,
            timing: None,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, TransitiveFrom, TransitiveTryFrom)]
#[serde(into = "MessageType", try_from = "MessageType")]
#[transitive(into(all(ConnectionV1_0, MessageType)))]
#[transitive(try_from(MessageFamily, ConnectionKind, ConnectionV1, ConnectionV1_0))]
struct SigEd25519Sha512Single;

impl From<SigEd25519Sha512Single> for ConnectionV1_0 {
    fn from(_value: SigEd25519Sha512Single) -> Self {
        ConnectionV1_0::Ed25519Sha512Single
    }
}

impl TryFrom<ConnectionV1_0> for SigEd25519Sha512Single {
    type Error = &'static str;

    fn try_from(value: ConnectionV1_0) -> Result<Self, Self::Error> {
        match value {
            ConnectionV1_0::Ed25519Sha512Single => Ok(Self),
            _ => Err("message kind is not \"ed25519Sha512_single\""),
        }
    }
}

impl TryFrom<MessageType> for SigEd25519Sha512Single {
    type Error = &'static str;

    fn try_from(value: MessageType) -> Result<Self, Self::Error> {
        let interm = MessageFamily::from(value);
        SigEd25519Sha512Single::try_from(interm)
    }
}