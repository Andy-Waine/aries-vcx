use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::msg_types::types::routing::RoutingV1_0Kind;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent)]
#[message(kind = "RoutingV1_0Kind::Forward")]
pub struct Forward {
    pub to: String,
    #[serde(rename = "msg")]
    pub msg: Box<RawValue>,
}

impl PartialEq for Forward {
    fn eq(&self, other: &Self) -> bool {
        self.to == other.to && self.msg.get() == other.msg.get()
    }
}

impl Forward {
    pub fn new(to: String, msg: Box<RawValue>) -> Self {
        Self { to, msg }
    }
}
