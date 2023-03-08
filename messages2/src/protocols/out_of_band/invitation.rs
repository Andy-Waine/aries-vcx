use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};

use crate::{
    composite_message::Message,
    decorators::{Attachment, Timing},
    message_type::message_family::out_of_band::OutOfBandV1_1,
    mime_type::MimeType,
    protocols::{common::service::Service, traits::MessageKind},
};

use super::OobGoalCode;

pub type Invitation = Message<InvitationContent, InvitationDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent)]
#[message(kind = "OutOfBandV1_1::Invitation")]
pub struct InvitationContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_code: Option<OobGoalCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<MimeType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake_protocols: Option<Vec<String>>, // TODO: Protocol Registry
    pub services: Vec<Service>,
    #[serde(rename = "requests~attach")]
    pub requests_attach: Vec<Attachment>,
}

impl InvitationContent {
    pub fn new(services: Vec<Service>, requests_attach: Vec<Attachment>) -> Self {
        Self {
            label: None,
            goal_code: None,
            goal: None,
            accept: None,
            handshake_protocols: None,
            services,
            requests_attach,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct InvitationDecorators {
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}