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
#[semver(family = "notification")]
pub enum Notification {
    V1(NotificationV1),
}

#[derive(Copy, Clone, Debug, From, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(Notification, MessageFamily)))]
#[semver(major = 1, parent = "Notification", actors(Actor::Notified, Actor::Notifier))]
pub enum NotificationV1 {
    V1_0(NotificationV1_0),
}

#[derive(Copy, Clone, Debug, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(NotificationV1, Notification, MessageFamily)))]
#[semver(minor = 0, parent = "NotificationV1")]
pub struct NotificationV1_0;

#[derive(Copy, Clone, Debug, AsRefStr, EnumString, PartialEq, MessageType)]
#[strum(serialize_all = "kebab-case")]
#[semver(parent = "NotificationV1_0")]
pub enum NotificationV1_0Kind {
    Ack,
}