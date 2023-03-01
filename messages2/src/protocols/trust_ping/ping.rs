use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};
use transitive::TransitiveFrom;

use crate::{
    aries_message::AriesMessage,
    decorators::{Thread, Timing},
    macros::threadlike_opt_impl,
    message_type::message_family::trust_ping::TrustPingV1_0,
    protocols::traits::MessageKind,
};

use super::TrustPing;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent, TransitiveFrom)]
#[message(kind = "TrustPingV1_0::PingResponse")]
#[transitive(into(TrustPing, AriesMessage))]
pub struct Ping {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(default)]
    pub response_requested: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "~thread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

threadlike_opt_impl!(Ping);
