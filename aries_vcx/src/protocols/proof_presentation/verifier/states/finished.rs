use serde::{Deserialize, Deserializer};

use messages::concepts::problem_report::ProblemReport;
use messages::protocols::proof_presentation::presentation::Presentation;
use messages::protocols::proof_presentation::presentation_request::PresentationRequest;
use messages::status::Status;

use crate::protocols::proof_presentation::verifier::verification_status::PresentationVerificationStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinishedState {
    pub presentation_request: Option<PresentationRequest>,
    pub presentation: Option<Presentation>,
    pub status: Status,
    #[serde(
        default = "PresentationVerificationStatus::Unavailable",
        deserialize_with = "null_to_unavailable"
    )]
    #[serde(alias = "revocation_status")]
    pub verification_status: PresentationVerificationStatus,
}

// For backwards compatibility, if "revocation_status / verification_status" is null, we deserialize as Unavailable
fn null_to_unavailable<'de, D>(deserializer: D) -> Result<PresentationVerificationStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(PresentationVerificationStatus::Unavailable))
}

impl FinishedState {
    pub fn declined(problem_report: ProblemReport) -> Self {
        trace!("transit state to FinishedState due to a rejection");
        FinishedState {
            presentation_request: None,
            presentation: None,
            status: Status::Declined(problem_report),
            verification_status: PresentationVerificationStatus::Unavailable(),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "general_test")]
pub mod unit_tests {
    use std::str::FromStr;

    use messages::protocols::proof_presentation::presentation::test_utils::{_presentation, _presentation_1};
    use messages::protocols::proof_presentation::presentation_proposal::test_utils::_presentation_proposal;
    use messages::protocols::proof_presentation::presentation_request::test_utils::_presentation_request;
    use messages::protocols::proof_presentation::test_utils::{_ack, _problem_report};

    use crate::common::proofs::proof_request::test_utils::_presentation_request_data;
    use crate::common::test_utils::mock_profile;
    use crate::test::source_id;
    use crate::utils::devsetup::{SetupEmpty, SetupMocks};

    use super::*;

    #[test]
    fn test_verifier_state_finished_ser() {
        let state = FinishedState {
            presentation_request: None,
            presentation: None,
            status: Status::Success,
            verification_status: PresentationVerificationStatus::Valid,
        };
        let serialized = serde_json::to_string(&state).unwrap();
        let expected =
            r#"{"presentation_request":null,"presentation":null,"status":"Success","revocation_status":"Valid"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_verifier_state_finished_deser() {
        {
            let serialized =
                r#"{"presentation":null,"presentation_request":null,"status":"Success","revocation_status":"Invalid"}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(deserialized.verification_status, PresentationVerificationStatus::Invalid)
        }
        {
            let serialized =
                r#"{"presentation":null,"presentation_request":null,"status":"Success","revocation_status":"Valid"}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(deserialized.verification_status, PresentationVerificationStatus::Valid)
        }
    }

    #[test]
    fn test_verifier_state_finished_deser_legacy() {
        {
            let serialized = r#"{"presentation":null,"presentation_request":null,"status":"Success"}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(
                deserialized.verification_status,
                PresentationVerificationStatus::Unavailable()
            )
        }
        {
            let serialized =
                r#"{"presentation":null,"presentation_request":null,"status":"Success","verification_status":null}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(
                deserialized.verification_status,
                PresentationVerificationStatus::Unavailable()
            )
        }
        {
            let serialized =
                r#"{"presentation":null,"presentation_request":null,"status":"Success","verification_status":"Revoked"}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(deserialized.verification_status, PresentationVerificationStatus::Invalid)
        }
        {
            let serialized = r#"{"presentation":null,"presentation_request":null,"status":"Success","verification_status":"NonRevoked"}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(deserialized.verification_status, PresentationVerificationStatus::Valid)
        }
    }

    #[test]
    fn test_verifier_state_finished_deser_legacy_2() {
        {
            let serialized =
                r#"{"presentation":null,"presentation_request":null,"status":"Success","revocation_status":null}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(
                deserialized.verification_status,
                PresentationVerificationStatus::Unavailable()
            )
        }
        {
            let serialized =
                r#"{"presentation":null,"presentation_request":null,"status":"Success","revocation_status":"Revoked"}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(deserialized.verification_status, PresentationVerificationStatus::Invalid)
        }
        {
            let serialized = r#"{"presentation":null,"presentation_request":null,"status":"Success","revocation_status":"NonRevoked"}"#;
            let deserialized: FinishedState = serde_json::from_str(serialized).unwrap();
            assert_eq!(deserialized.verification_status, PresentationVerificationStatus::Valid)
        }
    }
}
