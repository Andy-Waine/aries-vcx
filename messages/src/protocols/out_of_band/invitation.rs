use crate::{
    a2a::{message_type::MessageType, A2AMessage, MessageId},
    a2a_message,
    concepts::{attachment::Attachments, mime_type::MimeType, timing::Timing},
    errors::error::prelude::*,
    protocols::out_of_band::{service_oob::ServiceOob, GoalCode},
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct OutOfBandInvitation {
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_code: Option<GoalCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<MimeType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handshake_protocols: Option<Vec<MessageType>>, // TODO: Make a separate type
    pub services: Vec<ServiceOob>,
    #[serde(rename = "requests~attach")]
    pub requests_attach: Attachments,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

a2a_message!(OutOfBandInvitation);

impl OutOfBandInvitation {
    pub fn to_string(&self) -> String {
        json!(self).to_string()
    }

    pub fn from_string(oob_data: &str) -> MessagesResult<OutOfBandInvitation> {
        serde_json::from_str(oob_data).map_err(|err| {
            MessagesError::from_msg(
                MessagesErrorKind::InvalidJson,
                format!("Cannot deserialize out of band message: {:?}", err),
            )
        })
    }
}
