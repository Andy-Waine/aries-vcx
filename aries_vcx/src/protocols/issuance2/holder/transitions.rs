use std::sync::Arc;

use messages::{
    a2a::{A2AMessage, MessageId},
    concepts::problem_report::ProblemReport,
    protocols::issuance::{
        credential::Credential,
        credential_offer::CredentialOffer,
        credential_proposal::{CredentialProposal, CredentialProposalData},
        credential_request::CredentialRequest,
    },
};

use crate::{
    core::profile::profile::Profile,
    errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult},
    global::settings,
    handlers::util::verify_thread_id,
    protocols::SendClosure,
};

use super::{
    states::{
        failed::Failed, finished::Finished, initial::Initial, offer_received::OfferReceived,
        proposal_sent::ProposalSent, request_sent::RequestSent,
    },
    Holder,
};

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
        // send proposal where thread_id is the existing thread_id (as this is not the first msg in the protocol)
        let proposal = CredentialProposal::from(proposal_data).set_thread_id(&self.thread_id);
        self.send_proposal_message(proposal, send_message).await
    }

    pub async fn send_request(
        self,
        profile: &Arc<dyn Profile>,
        prover_did: String,
        send_message: SendClosure,
    ) -> VcxResult<Holder<RequestSent>> {
        // TODO - actually, how do we handle this.... it could go into requestSent or Failed..
        match _make_credential_request(profile, self.thread_id.clone(), prover_did, &self.state.offer).await {
            Ok((cred_request, request_metadata, cred_def_json)) => {
                send_message(cred_request.to_a2a_message()).await?;
                let state = RequestSent::new(request_metadata, cred_def_json);
            }
            Err(_) => todo!(),
        }
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
        // TODO - actually, how do we handle this.... it could be finished or failed
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

    /// Internal only, used by both proposalsent and requestsent
    fn receive_problem_report_message(self, problem_report: ProblemReport) -> Holder<Failed> {
        Holder {
            source_id: self.source_id,
            thread_id: self.thread_id,
            state: Failed::new(problem_report),
        }
    }
}

// TODO - idk where to put these functions

fn parse_cred_def_id_from_cred_offer(cred_offer: &str) -> VcxResult<String> {
    trace!(
        "Holder::parse_cred_def_id_from_cred_offer >>> cred_offer: {:?}",
        cred_offer
    );

    let parsed_offer: serde_json::Value = serde_json::from_str(cred_offer).map_err(|err| {
        AriesVcxError::from_msg(
            AriesVcxErrorKind::InvalidJson,
            format!("Invalid Credential Offer Json: {:?}", err),
        )
    })?;

    let cred_def_id = parsed_offer["cred_def_id"].as_str().ok_or_else(|| {
        AriesVcxError::from_msg(
            AriesVcxErrorKind::InvalidJson,
            "Invalid Credential Offer Json: cred_def_id not found",
        )
    })?;

    Ok(cred_def_id.to_string())
}

async fn create_credential_request(
    profile: &Arc<dyn Profile>,
    cred_def_id: &str,
    prover_did: &str,
    cred_offer: &str,
) -> VcxResult<(String, String, String, String)> {
    let ledger = Arc::clone(profile).inject_ledger();
    let anoncreds = Arc::clone(profile).inject_anoncreds();
    let cred_def_json = ledger.get_cred_def(cred_def_id, None).await?;

    let master_secret_id = settings::DEFAULT_LINK_SECRET_ALIAS;
    anoncreds
        .prover_create_credential_req(prover_did, cred_offer, &cred_def_json, master_secret_id)
        .await
        .map_err(|err| err.extend("Cannot create credential request"))
        .map(|(s1, s2)| (s1, s2, cred_def_id.to_string(), cred_def_json))
}

async fn _make_credential_request(
    profile: &Arc<dyn Profile>,
    thread_id: String,
    my_pw_did: String,
    offer: &CredentialOffer,
) -> VcxResult<(CredentialRequest, String, String)> {
    trace!(
        "Holder::_make_credential_request >>> my_pw_did: {:?}, offer: {:?}",
        my_pw_did,
        offer
    );

    let cred_offer = offer.offers_attach.content()?;
    trace!("Parsed cred offer attachment: {}", cred_offer);
    let cred_def_id = parse_cred_def_id_from_cred_offer(&cred_offer)?;
    let (req, req_meta, _cred_def_id, cred_def_json) =
        create_credential_request(profile, &cred_def_id, &my_pw_did, &cred_offer).await?;
    trace!("Created cred def json: {}", cred_def_json);
    let credential_request_msg = CredentialRequest::create()
        .set_thread_id(&thread_id)
        .set_out_time()
        .set_requests_attach(req)?;

    Ok((credential_request_msg, req_meta, cred_def_json))
}
