use std::sync::Arc;

use messages::{
    a2a::{A2AMessage, MessageId},
    concepts::problem_report::ProblemReport,
    protocols::issuance::{
        credential::Credential,
        credential_offer::CredentialOffer,
        credential_proposal::{CredentialProposal, CredentialProposalData},
    },
};

use crate::{
    core::profile::profile::Profile, errors::error::VcxResult, handlers::util::verify_thread_id, protocols::SendClosure,
};

use self::states::{
    failed::Failed, finished::Finished, initial::Initial, offer_received::OfferReceived, proposal_sent::ProposalSent,
    request_sent::RequestSent,
};

pub mod states;

pub struct Holder<S> {
    source_id: String,
    thread_id: String,
    state: S,
}

impl Holder<Initial> {
    pub fn create(source_id: String) -> Self {
        let thread_id = MessageId::new().0;
        Holder {
            source_id,
            thread_id,
            state: Initial,
        }
    }

    pub async fn send_proposal(
        self,
        proposal_data: CredentialProposalData,
        send_message: SendClosure,
    ) -> VcxResult<Holder<ProposalSent>> {
        // send proposal where ID is the thread_id (as this is the first msg in the protocol)
        let proposal = CredentialProposal::from(proposal_data).set_id(&self.thread_id);
        self.send_proposal_message(proposal, send_message).await
    }
}

impl Holder<ProposalSent> {
    pub fn receive_offer(self, credential_offer: CredentialOffer) -> VcxResult<Holder<OfferReceived>> {
        verify_thread_id(&self.thread_id, &A2AMessage::CredentialOffer(credential_offer.clone()))?;

        Ok(Holder {
            source_id: self.source_id,
            thread_id: self.thread_id,
            state: OfferReceived::new(credential_offer),
        })
    }

    pub fn receive_problem_report(self, problem_report: ProblemReport) -> Holder<Failed> {
        self.receive_problem_report_message(problem_report)
    }
}

impl Holder<OfferReceived> {
    pub fn create_from_offer(source_id: String, credential_offer: CredentialOffer) -> Self {
        let thread_id = credential_offer.get_thread_id();
        Self {
            source_id,
            thread_id,
            state: OfferReceived::new(credential_offer),
        }
    }

    pub async fn send_proposal(
        self,
        proposal_data: CredentialProposalData,
        send_message: SendClosure,
    ) -> VcxResult<Holder<ProposalSent>> {
        // send proposal where ID is the existing thread_id (as this is not the first msg in the protocol)
        let proposal = CredentialProposal::from(proposal_data).set_thread_id(&self.thread_id);
        self.send_proposal_message(proposal, send_message).await
    }

    pub async fn send_request(
        self,
        _profile: &Arc<dyn Profile>,
        _prover_did: String,
        _send_message: SendClosure,
    ) -> VcxResult<Holder<RequestSent>> {
        let state = RequestSent::new(todo!(), todo!());
        Ok(Holder {
            source_id: self.source_id,
            thread_id: self.thread_id,
            state,
        })
    }

    pub async fn decline_offer(self, comment: Option<String>, send_message: SendClosure) -> VcxResult<Holder<Failed>> {
        // build problem report
        let problem_report = todo!();
        // send..

        let state = Failed::new(problem_report);
        Ok(Holder {
            source_id: self.source_id,
            thread_id: self.thread_id,
            state,
        })
    }
}

impl Holder<RequestSent> {
    pub async fn receive_credential(
        self,
        profile: &Arc<dyn Profile>,
        credential: Credential,
        send_message: SendClosure,
    ) -> VcxResult<Holder<Finished>> {
        // store cred...
        let (cred_id, rev_reg_def_json) = todo!(); // send ack...

        let state = Finished::new(cred_id, credential, rev_reg_def_json);
        Ok(Holder {
            source_id: self.source_id,
            thread_id: self.thread_id,
            state,
        })
    }

    pub fn receive_problem_report(self, problem_report: ProblemReport) -> Holder<Failed> {
        self.receive_problem_report_message(problem_report)
    }
}

impl Holder<Finished> {}

impl<S> Holder<S> {
    /// Internal only, used by both initial and offerreceived states
    async fn send_proposal_message(
        self,
        proposal: CredentialProposal,
        send_message: SendClosure,
    ) -> VcxResult<Holder<ProposalSent>> {
        send_message(proposal.to_a2a_message()).await?;

        let state = ProposalSent::new(proposal);
        Ok(Holder {
            source_id: self.source_id,
            thread_id: self.thread_id,
            state,
        })
    }

    pub fn receive_problem_report_message(self, problem_report: ProblemReport) -> Holder<Failed> {
        Holder {
            source_id: self.source_id,
            thread_id: self.thread_id,
            state: Failed::new(problem_report),
        }
    }
}
