use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};
use transitive::TransitiveFrom;

use crate::{
    aries_message::AriesMessage,
    decorators::{MsgLocalization, Thread, Timing},
    macros::threadlike_impl,
    message_type::message_family::connection::ConnectionV1_0,
};

use crate::protocols::traits::MessageKind;

use super::Connection;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent, TransitiveFrom)]
#[message(kind = "ConnectionV1_0::ProblemReport")]
#[transitive(into(Connection, AriesMessage))]
pub struct ProblemReport {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "problem-code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem_code: Option<ProblemCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain: Option<String>,
    #[serde(rename = "~l10n")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localization: Option<MsgLocalization>,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

threadlike_impl!(ProblemReport);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProblemCode {
    RequestNotAccepted,
    RequestProcessingError,
    ResponseNotAccepted,
    ResponseProcessingError,
}
