use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};

use crate::{
    composite_message::Message,
    msg_types::types::revocation::RevocationV2_0Kind,
    protocols::{
        notification::{AckContent, AckDecorators, AckStatus},
        traits::ConcreteMessage,
    },
};

pub type AckRevoke = Message<AckRevokeContent, AckDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent)]
#[message(kind = "RevocationV2_0Kind::Ack")]
#[serde(transparent)]
pub struct AckRevokeContent(pub AckContent);

impl AckRevokeContent {
    pub fn new(status: AckStatus) -> Self {
        Self(AckContent::new(status))
    }
}
