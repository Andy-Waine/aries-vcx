use messages_macros::MessageContent;
use serde::{Deserialize, Serialize};

use crate::{
    composite_message::Message,
    decorators::Timing,
    message_type::{
        message_family::discover_features::DiscoverFeaturesV1_0,
        registry::{ProtocolDescriptor, PROTOCOL_REGISTRY},
    },
    protocols::traits::MessageKind,
};

pub type Query = Message<QueryContent, QueryDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, MessageContent)]
#[message(kind = "DiscoverFeaturesV1_0::Query")]
pub struct QueryContent {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl QueryContent {
    pub fn new(query: String) -> Self {
        Self { query, comment: None }
    }

    pub fn lookup(&self) -> Vec<&ProtocolDescriptor> {
        let mut protocols = Vec::new();

        for versions in PROTOCOL_REGISTRY.values() {
            for minor in versions.values() {
                for pd in minor.values() {
                    if pd.pid.starts_with(&self.query) {
                        protocols.push(pd);
                    }
                }
            }
        }

        protocols
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct QueryDecorators {
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}