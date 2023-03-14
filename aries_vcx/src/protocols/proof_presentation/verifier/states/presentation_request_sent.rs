use std::sync::Arc;

use messages::{
    concepts::problem_report::ProblemReport,
    protocols::proof_presentation::{presentation::Presentation, presentation_request::PresentationRequest},
    status::Status,
};

use crate::{
    common::proofs::verifier::verifier::validate_indy_proof,
    core::profile::profile::Profile,
    errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult},
    global::settings,
    protocols::proof_presentation::verifier::{state_machine::RevocationStatus, states::finished::FinishedState},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationRequestSentState {
    pub presentation_request: PresentationRequest,
}

impl PresentationRequestSentState {
    pub async fn verify_presentation(
        &self,
        profile: &Arc<dyn Profile>,
        presentation: &Presentation,
        thread_id: &str,
    ) -> VcxResult<()> {
        if !settings::indy_mocks_enabled() && !presentation.from_thread(thread_id) {
            return Err(AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidJson,
                format!(
                    "Cannot handle proof presentation: thread id does not match: {:?}",
                    presentation.thread
                ),
            ));
        };

        let valid = validate_indy_proof(
            profile,
            &presentation.presentations_attach.content()?,
            &self.presentation_request.request_presentations_attach.content()?,
        )
        .await?;

        if !valid {
            return Err(AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidProof,
                "Presentation verification failed",
            ));
        }

        Ok(())
    }
}

impl From<(PresentationRequestSentState, Presentation, RevocationStatus)> for FinishedState {
    fn from(
        (state, presentation, was_revoked): (PresentationRequestSentState, Presentation, RevocationStatus),
    ) -> Self {
        trace!("transit state from PresentationRequestSentState to FinishedState");
        FinishedState {
            presentation_request: Some(state.presentation_request),
            presentation: Some(presentation),
            status: Status::Success,
            revocation_status: Some(was_revoked),
        }
    }
}

impl From<(PresentationRequestSentState, ProblemReport)> for FinishedState {
    fn from((state, problem_report): (PresentationRequestSentState, ProblemReport)) -> Self {
        trace!("transit state from PresentationRequestSentState to FinishedState");
        FinishedState {
            presentation_request: Some(state.presentation_request),
            presentation: None,
            status: Status::Failed(problem_report),
            revocation_status: None,
        }
    }
}
