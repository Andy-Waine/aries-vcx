use derive_more::From;
use messages_macros::MessageType;
use strum_macros::{AsRefStr, EnumString};
use transitive::TransitiveFrom;

use super::{
    traits::{MajorVersion, MinorVersion, ProtocolName},
    Protocol,
};
use crate::msg_types::{actor::Actor, registry::get_supported_version};

#[derive(Copy, Clone, Debug, From, PartialEq, MessageType)]
#[semver(protocol = "report-problem")]
pub enum ReportProblem {
    V1(ReportProblemV1),
}

#[derive(Copy, Clone, Debug, From, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(ReportProblem, Protocol)))]
#[semver(major = 1, parent = "ReportProblem", actors(Actor::Notified, Actor::Notifier))]
pub enum ReportProblemV1 {
    V1_0(ReportProblemV1_0),
}

#[derive(Copy, Clone, Debug, PartialEq, TransitiveFrom, MessageType)]
#[transitive(into(all(ReportProblemV1, ReportProblem, Protocol)))]
#[semver(minor = 0, parent = "ReportProblemV1")]
pub struct ReportProblemV1_0;

#[derive(Copy, Clone, Debug, AsRefStr, EnumString, PartialEq, MessageType)]
#[strum(serialize_all = "kebab-case")]
#[semver(parent = "ReportProblemV1_0")]
pub enum ReportProblemV1_0Kind {
    ProblemReport,
}

#[cfg(test)]
mod tests {
    use crate::misc::test_utils;

    use super::ReportProblemV1_0;

    const PROTOCOL: &str = "https://didcomm.org/report-problem/1.0";
    const VERSION_RESOLUTION_PROTOCOL: &str = "https://didcomm.org/report-problem/1.255";
    const UNSUPPORTED_VERSION_PROTOCOL: &str = "https://didcomm.org/report-problem/2.0";

    const KIND_PROBLEM_REPORT: &str = "problem-report";

    #[test]
    fn test_protocol_report_problem() {
        test_utils::test_protocol(PROTOCOL, ReportProblemV1_0)
    }

    #[test]
    fn test_version_resolution_report_problem() {
        test_utils::test_protocol(VERSION_RESOLUTION_PROTOCOL, ReportProblemV1_0)
    }

    #[test]
    #[should_panic]
    fn test_unsupported_version_report_problem() {
        test_utils::test_protocol(UNSUPPORTED_VERSION_PROTOCOL, ReportProblemV1_0)
    }

    #[test]
    fn test_msg_type_problem_report() {
        test_utils::test_msg_type(PROTOCOL, KIND_PROBLEM_REPORT, ReportProblemV1_0)
    }
}
