use derive_more::From;
use messages_macros::MessageType;
use strum_macros::{AsRefStr, EnumString};
use transitive::TransitiveInto;

use crate::{
    error::{MsgTypeError, MsgTypeResult},
    message_type::MessageType,
};

use super::{
    traits::{ResolveMajorVersion, ResolveMinorVersion, ResolveMsgKind},
    MessageFamily,
};

#[derive(Copy, Clone, Debug, From, PartialEq, TransitiveInto, MessageType)]
#[transitive(MessageFamily, MessageType)]
#[semver(family = "trust_ping")]
pub enum TrustPing {
    V1(TrustPingV1),
}

#[derive(Copy, Clone, Debug, From, PartialEq, TransitiveInto, MessageType)]
#[transitive(all(TrustPing, MessageFamily, MessageType))]
#[semver(major = 1)]
pub enum TrustPingV1 {
    V1_0(TrustPingV1_0),
}

#[derive(Copy, Clone, Debug, AsRefStr, EnumString, PartialEq, TransitiveInto, MessageType)]
#[transitive(all(TrustPingV1, TrustPing, MessageFamily, MessageType))]
#[strum(serialize_all = "kebab-case")]
#[semver(minor = 0)]
pub enum TrustPingV1_0 {
    Ping,
    #[strum(serialize = "ping_response")]
    PingResponse,
}
