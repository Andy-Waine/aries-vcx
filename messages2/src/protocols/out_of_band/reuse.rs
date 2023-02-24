use messages_macros::Message;
use serde::{Deserialize, Serialize};
use transitive::TransitiveFrom;

use crate::{
    aries_message::AriesMessage,
    decorators::{Thread, Timing},
    message_type::message_family::out_of_band::OutOfBandV1_1,
    protocols::traits::ConcreteMessage, macros::threadlike_impl,
};

use super::OutOfBand;

#[derive(Clone, Debug, Deserialize, Serialize, Message, TransitiveFrom)]
#[message(kind = "OutOfBandV1_1::HandshakeReuse")]
#[transitive(into(OutOfBand, AriesMessage))]
pub struct HandshakeReuse {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

threadlike_impl!(HandshakeReuse);