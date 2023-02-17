use derive_more::From;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{message_type::message_family::{
    discover_features::{DiscoverFeatures as DiscoverFeaturesKind, DiscoverFeaturesV1, DiscoverFeaturesV1_0},
}, delayed_serde::DelayedSerde};

use super::traits::ConcreteMessage;

#[derive(Clone, Debug, From)]
pub enum DiscoverFeatures {
    Query(Query),
    Disclose(Disclose),
}

impl DelayedSerde for DiscoverFeatures {
    type MsgType = DiscoverFeaturesKind;

    fn delayed_deserialize<'de, D>(seg: Self::MsgType, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let DiscoverFeaturesKind::V1(major) = seg;
        let DiscoverFeaturesV1::V1_0(minor) = major;

        match minor {
            DiscoverFeaturesV1_0::Query => Query::deserialize(deserializer).map(From::from),
            DiscoverFeaturesV1_0::Disclose => Disclose::deserialize(deserializer).map(From::from),
        }
    }

    fn delayed_serialize<'a, M, F, S>(&self, state: &'a mut M, closure: &mut F) -> Result<S::Ok, S::Error>
    where
        M: serde::ser::SerializeMap,
        F: FnMut(&'a mut M) -> S,
        S: serde::Serializer,
        S::Error: From<M::Error>,
    {
        match self {
            Self::Query(v) => v.delayed_serialize(state, closure),
            Self::Disclose(v) => v.delayed_serialize(state, closure),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Query;

impl ConcreteMessage for Query {
    type Kind = DiscoverFeaturesV1_0;

    fn kind() -> Self::Kind {
        Self::Kind::Query
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Disclose;

impl ConcreteMessage for Disclose {
    type Kind = DiscoverFeaturesV1_0;

    fn kind() -> Self::Kind {
        Self::Kind::Disclose
    }
}
