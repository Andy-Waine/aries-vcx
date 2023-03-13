use derive_more::From;
use messages_macros::MessageType;
use strum_macros::{AsRefStr, EnumString};
use transitive::TransitiveFrom;

use crate::{
    error::{MsgTypeError, MsgTypeResult},
    message_type::actor::Actor,
    message_type::registry::get_supported_version,
};

use super::{
    traits::{MajorVersion, MessageKind, MinorVersion, ProtocolName},
    MessageFamily,
};

#[derive(Copy, Clone, Debug, From, PartialEq, MessageType)]
#[semver(family = "routing")]
pub enum Routing {
    V1(RoutingV1),
}

#[derive(Copy, Clone, Debug, From, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(Routing, MessageFamily)))]
#[semver(major = 1, parent = "Routing", actors(Actor::Mediator))]
pub enum RoutingV1 {
    V1_0(RoutingV1_0),
}

#[derive(Copy, Clone, Debug, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(RoutingV1, Routing, MessageFamily)))]
#[semver(minor = 0, parent = "RoutingV1")]
pub struct RoutingV1_0;

#[derive(Copy, Clone, Debug, AsRefStr, EnumString, PartialEq, MessageType)]
#[strum(serialize_all = "kebab-case")]
#[semver(parent = "RoutingV1_0")]
pub enum RoutingV1_0Kind {
    Forward,
}