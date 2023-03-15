use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};

use crate::{
    msg_types::types::cred_issuance::CredentialIssuanceV1_0Kind,
    protocols::notification::{AckContent, AckDecorators, AckStatus},
    Message,
};

pub type AckCredential = Message<AckCredentialContent, AckDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent, PartialEq)]
#[message(kind = "CredentialIssuanceV1_0Kind::Ack")]
#[serde(transparent)]
pub struct AckCredentialContent(pub AckContent);

impl AckCredentialContent {
    pub fn new(status: AckStatus) -> Self {
        Self(AckContent::new(status))
    }
}
