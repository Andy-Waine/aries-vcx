use messages_macros::Message;
use serde::{Deserialize, Serialize};
use transitive::TransitiveFrom;

use crate::{
    aries_message::AriesMessage,
    decorators::{Attachment, Timing},
    message_type::message_family::out_of_band::OutOfBandV1_1,
    mime_type::MimeType,
    protocols::{common::service::Service, traits::ConcreteMessage},
};

use super::{OobGoalCode, OutOfBand};

#[derive(Clone, Debug, Deserialize, Serialize, Message, TransitiveFrom)]
#[message(kind = "OutOfBandV1_1::Invitation")]
#[transitive(into(OutOfBand, AriesMessage))]
pub struct Invitation {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_code: Option<OobGoalCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<MimeType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake_protocols: Option<Vec<()>>, // TODO: Make a separate type
    pub services: Vec<Service>,
    #[serde(rename = "requests~attach")]
    pub requests_attach: Vec<Attachment>,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}