use derive_more::{From, TryInto};
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

#[derive(Copy, Clone, Debug, From, TryInto, PartialEq, MessageType)]
#[semver(family = "connections")]
pub enum Connection {
    V1(ConnectionV1),
}

#[derive(Copy, Clone, Debug, From, TryInto, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(Connection, MessageFamily)))]
#[semver(major = 1, parent = "Connection", actors(Actor::Inviter, Actor::Invitee))]
pub enum ConnectionV1 {
    V1_0(ConnectionV1_0),
}

#[derive(Copy, Clone, Debug, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(ConnectionV1, Connection, MessageFamily)))]
#[semver(minor = 0, parent = "ConnectionV1")]
pub struct ConnectionV1_0;

#[derive(Copy, Clone, Debug, AsRefStr, EnumString, PartialEq, MessageType)]
#[strum(serialize_all = "snake_case")]
#[semver(parent = "ConnectionV1_0")]
pub enum ConnectionV1_0Kind {
    Invitation,
    Request,
    Response,
    ProblemReport,
    #[strum(serialize = "ed25519Sha512_single")]
    Ed25519Sha512Single,
}