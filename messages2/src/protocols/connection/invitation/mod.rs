pub mod pairwise;
pub mod public;

use derive_more::From;
use messages_macros::MessageContent;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

use super::Connection;
use crate::aries_message::MsgWithType;
use crate::composite_message::{transit_to_aries_msg, Message};
use crate::delayed_serde::DelayedSerde;
use crate::message_type::message_family::connection::ConnectionV1_0;
use crate::protocols::traits::MessageKind;

use self::pairwise::{PairwiseInvitationContent, PwInvitationDecorators};
use self::public::PublicInvitationContent;
pub use self::{
    pairwise::{PairwiseDidInvitation, PairwiseInvitation},
    public::PublicInvitation,
};

/// Type used to encapsulate a fully resolved invitation, which
/// contains all the information necessary for generating a [`crate::protocols::connection::request::Request`].
///
/// Other invitation types would get resolved to this.
// We rely on the URL version of the pairwise invitation because, coincidentally,
// that's what a fully resolved invitation looks like.
// If other fields are needed in the future, this type could be adapted.
pub struct CompleteInvitationContent(PairwiseInvitationContent<Url>);

// We implement the message kind on this type as we have to rely on
// untagged deserialization, since we cannot know the invitation format
// ahead of time.
//
// However, to have the capability of setting different decorators
// based on the invitation format, we don't wrap the [`Invitation`]
// in a [`Message`], but rather its variants.
#[derive(Debug, Clone, From, Deserialize, Serialize, MessageContent)]
#[message(kind = "ConnectionV1_0::Invitation")]
#[serde(untagged)]
pub enum Invitation {
    Public(PublicInvitation),
    Pairwise(PairwiseInvitation),
    PairwiseDID(PairwiseDidInvitation),
}

/// We need a custom [`DelayedSerde`] impl to take advantage of
/// serde's untagged deserialization.
impl DelayedSerde for Invitation {
    type MsgType = ConnectionV1_0;

    fn delayed_deserialize<'de, D>(msg_type: Self::MsgType, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let expected = Self::kind();
        if msg_type == expected {
            Self::deserialize(deserializer)
        } else {
            let const_msg = concat!("Failed deserializing ", stringify!(Invitation));
            let msg = format!("{const_msg}; Expected kind: {expected:?}, found: {msg_type:?}");
            Err(D::Error::custom(msg))
        }
    }

    fn delayed_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        MsgWithType::from(self).serialize(serializer)
    }
}

transit_to_aries_msg!(PublicInvitationContent, Invitation, Connection);
transit_to_aries_msg!(PairwiseInvitationContent<Url>:PwInvitationDecorators, Invitation, Connection);
transit_to_aries_msg!(PairwiseInvitationContent<String>:PwInvitationDecorators, Invitation, Connection);