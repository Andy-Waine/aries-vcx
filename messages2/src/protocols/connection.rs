use derive_more::From;
use messages_macros::Message;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    delayed_serde::DelayedSerde,
    message_type::message_family::connection::{Connection as ConnectionKind, ConnectionV1, ConnectionV1_0},
};

use super::traits::ConcreteMessage;

#[derive(Clone, Debug, From)]
pub enum Connection {
    Invitation(Invitation),
    Request(Request),
    Response(Response),
    ProblemReport(ProblemReport),
}

impl DelayedSerde for Connection {
    type MsgType = ConnectionKind;

    fn delayed_deserialize<'de, D>(seg: Self::MsgType, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ConnectionKind::V1(major) = seg;
        let ConnectionV1::V1_0(minor) = major;

        match minor {
            ConnectionV1_0::Invitation => Invitation::deserialize(deserializer).map(From::from),
            ConnectionV1_0::Request => Request::deserialize(deserializer).map(From::from),
            ConnectionV1_0::Response => Response::deserialize(deserializer).map(From::from),
            ConnectionV1_0::ProblemReport => ProblemReport::deserialize(deserializer).map(From::from),
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
            Self::Invitation(v) => v.delayed_serialize(state, closure),
            Self::Request(v) => v.delayed_serialize(state, closure),
            Self::Response(v) => v.delayed_serialize(state, closure),
            Self::ProblemReport(v) => v.delayed_serialize(state, closure),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Message)]
#[message(kind = "ConnectionV1_0::Invitation")]
pub struct Invitation;

#[derive(Clone, Debug, Deserialize, Serialize, Message)]
#[message(kind = "ConnectionV1_0::Request")]
pub struct Request;

#[derive(Clone, Debug, Deserialize, Serialize, Message)]
#[message(kind = "ConnectionV1_0::Response")]
pub struct Response;

#[derive(Clone, Debug, Deserialize, Serialize, Message)]
#[message(kind = "ConnectionV1_0::ProblemReport")]
pub struct ProblemReport;
