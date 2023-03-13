use std::{fmt::Display, str::FromStr};

use derive_more::{From, TryInto};

use crate::error::{MsgTypeError, MsgTypeResult};

use self::{
    basic_message::BasicMessage, connection::Connection, cred_issuance::CredentialIssuance,
    discover_features::DiscoverFeatures, notification::Notification, out_of_band::OutOfBand,
    present_proof::PresentProof, report_problem::ReportProblem, revocation::Revocation, routing::Routing,
    traits::ProtocolName, trust_ping::TrustPing,
};

pub mod basic_message;
pub mod connection;
pub mod cred_issuance;
pub mod discover_features;
pub mod notification;
pub mod out_of_band;
pub mod present_proof;
pub mod report_problem;
pub mod revocation;
pub mod routing;
pub mod traits;
pub mod trust_ping;

#[derive(Clone, Debug, From, TryInto, PartialEq)]
pub enum MessageFamily {
    Routing(Routing),
    Connection(Connection),
    Revocation(Revocation),
    CredentialIssuance(CredentialIssuance),
    ReportProblem(ReportProblem),
    PresentProof(PresentProof),
    TrustPing(TrustPing),
    DiscoverFeatures(DiscoverFeatures),
    BasicMessage(BasicMessage),
    OutOfBand(OutOfBand),
    Notification(Notification),
}

macro_rules! resolve_major_ver {
    ($type:ident, $family:expr, $major:expr, $minor:expr) => {
        if $family == $type::FAMILY {
            return Ok(Self::$type($type::resolve_version($major, $minor)?));
        }
    };
}

impl MessageFamily {
    pub const DID_COM_ORG_PREFIX: &'static str = "https://didcomm.org";
    pub const DID_SOV_PREFIX: &'static str = "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec";

    pub fn from_parts(family: &str, major: u8, minor: u8) -> MsgTypeResult<Self> {
        resolve_major_ver!(Routing, family, major, minor);
        resolve_major_ver!(Connection, family, major, minor);
        resolve_major_ver!(Revocation, family, major, minor);
        resolve_major_ver!(CredentialIssuance, family, major, minor);
        resolve_major_ver!(ReportProblem, family, major, minor);
        resolve_major_ver!(PresentProof, family, major, minor);
        resolve_major_ver!(TrustPing, family, major, minor);
        resolve_major_ver!(DiscoverFeatures, family, major, minor);
        resolve_major_ver!(BasicMessage, family, major, minor);
        resolve_major_ver!(OutOfBand, family, major, minor);
        resolve_major_ver!(Notification, family, major, minor);

        Err(MsgTypeError::unknown_family(family.to_owned()))
    }

    pub fn as_parts(&self) -> (&str, u8, u8) {
        match &self {
            Self::Routing(v) => v.as_protocol_parts(),
            Self::Connection(v) => v.as_protocol_parts(),
            Self::Revocation(v) => v.as_protocol_parts(),
            Self::CredentialIssuance(v) => v.as_protocol_parts(),
            Self::ReportProblem(v) => v.as_protocol_parts(),
            Self::PresentProof(v) => v.as_protocol_parts(),
            Self::TrustPing(v) => v.as_protocol_parts(),
            Self::DiscoverFeatures(v) => v.as_protocol_parts(),
            Self::BasicMessage(v) => v.as_protocol_parts(),
            Self::OutOfBand(v) => v.as_protocol_parts(),
            Self::Notification(v) => v.as_protocol_parts(),
        }
    }

    /// Steps the provided iterator of parts and returns the string slice element.
    ///
    /// # Errors:
    ///
    /// Will return an error if the iterator returns [`None`].
    pub fn next_part<'a, I>(iter: &mut I, name: &'static str) -> MsgTypeResult<&'a str>
    where
        I: Iterator<Item = &'a str>,
    {
        iter.next().ok_or_else(|| MsgTypeError::not_found(name))
    }
}

impl Display for MessageFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix_str = Self::DID_COM_ORG_PREFIX;
        let (family, major, minor) = self.as_parts();
        write!(f, "{prefix_str}/{family}/{major}.{minor}")
    }
}

impl FromStr for MessageFamily {
    type Err = MsgTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The type is segmented by forward slashes, but the HTTPS
        // prefix includes two as well (https://), so we'll accommodate that
        // when we skip elements from the split string.
        //
        // We always skip at least one element, the prefix itself.
        let skip_slash = match s {
            _ if s.starts_with(Self::DID_COM_ORG_PREFIX) => Ok(3),
            _ if s.starts_with(Self::DID_SOV_PREFIX) => Ok(1),
            _ => Err(MsgTypeError::unknown_prefix(s.to_owned())),
        }?;

        // We'll get the next components in order
        let mut iter = s.split('/').skip(skip_slash);

        let family = MessageFamily::next_part(&mut iter, "family")?;
        let version = MessageFamily::next_part(&mut iter, "protocol version")?;

        // We'll parse the version to its major and minor parts
        let mut version_iter = version.split('.');

        let major = MessageFamily::next_part(&mut version_iter, "protocol major version")?.parse()?;
        let minor = MessageFamily::next_part(&mut version_iter, "protocol minor version")?.parse()?;

        MessageFamily::from_parts(family, major, minor)
    }
}