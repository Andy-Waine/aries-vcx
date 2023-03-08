use std::sync::Arc;

use messages::{
    a2a::A2AMessage,
    concepts::problem_report::ProblemReport,
    protocols::{
        issuance::{credential_offer::CredentialOffer, credential_proposal::CredentialProposal},
        revocation_notification::revocation_notification::RevocationNotification,
    },
};

use crate::{
    common::credentials::{get_cred_rev_id, is_cred_revoked},
    core::profile::profile::Profile,
    errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult},
    handlers::revocation_notification::receiver::RevocationNotificationReceiver,
    protocols::SendClosure,
};

use self::{
    states::{
        failed::Failed, finished::Finished, offer_received::OfferReceived, proposal_sent::ProposalSent,
        request_sent::RequestSent,
    },
    trait_bounds::{GetAttachment, GetAttributes, IsTerminalState},
};

pub mod states;
pub mod trait_bounds;
pub mod transitions;

pub struct Holder<S> {
    source_id: String,
    thread_id: String,
    state: S,
}

impl Holder<ProposalSent> {
    pub async fn is_revokable(&self, profile: &Arc<dyn Profile>) -> VcxResult<bool> {
        self.state.is_revokable(profile).await
    }
}

impl Holder<OfferReceived> {
    pub fn get_offer(&self) -> CredentialOffer {
        self.state.offer.clone()
    }

    pub async fn is_revokable(&self, profile: &Arc<dyn Profile>) -> VcxResult<bool> {
        self.state.is_revokable(profile).await
    }
}

impl Holder<RequestSent> {
    pub fn is_revokable(&self) -> VcxResult<bool> {
        self.state.is_revokable()
    }
}

impl Holder<Finished> {
    // cred_id and credential as a2a message?
    pub fn get_credential(&self) -> (String, A2AMessage) {
        (self.state.cred_id.clone(), self.state.credential.to_a2a_message())
    }

    pub fn get_tails_location(&self) -> VcxResult<String> {
        self.state.get_tails_location()
    }

    pub fn get_tails_hash(&self) -> VcxResult<String> {
        self.state.get_tails_hash()
    }

    pub fn get_rev_reg_id(&self) -> VcxResult<String> {
        self.state.get_rev_reg_id()
    }

    pub fn get_cred_id(&self) -> String {
        self.state.cred_id.clone()
    }

    pub fn is_revokable(&self) -> bool {
        self.state.is_revokable()
    }

    pub async fn is_revoked(&self, profile: &Arc<dyn Profile>) -> VcxResult<bool> {
        if self.is_revokable() {
            let rev_reg_id = self.get_rev_reg_id()?;
            let cred_id = self.get_cred_id();
            let rev_id = get_cred_rev_id(profile, &cred_id).await?;
            is_cred_revoked(profile, &rev_reg_id, &rev_id).await
        } else {
            Err(AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidState,
                "Unable to check revocation status - this credential is not revokable",
            ))
        }
    }

    pub async fn delete_credential(&self, profile: &Arc<dyn Profile>) -> VcxResult<()> {
        let cred_id = self.get_cred_id();
        trace!("Holder::delete_credential >>> cred_id: {}", cred_id);

        let anoncreds = Arc::clone(profile).inject_anoncreds();
        anoncreds.prover_delete_credential(&cred_id).await
    }

    pub async fn get_cred_rev_id(&self, profile: &Arc<dyn Profile>) -> VcxResult<String> {
        get_cred_rev_id(profile, &self.get_cred_id()).await
    }

    pub async fn handle_revocation_notification(
        &self,
        profile: &Arc<dyn Profile>,
        notification: RevocationNotification,
        send_message: SendClosure,
    ) -> VcxResult<()> {
        if self.is_revokable() {
            // TODO: Store to remember notification was received along with details
            RevocationNotificationReceiver::build(self.get_rev_reg_id()?, self.get_cred_rev_id(profile).await?)
                .handle_revocation_notification(notification, send_message)
                .await?;
            Ok(())
        } else {
            Err(AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidState,
                "Unexpected revocation notification, credential is not revokable".to_string(),
            ))
        }
    }
}

// generic methods
impl<S> Holder<S> {
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn thread_id(&self) -> &str {
        &self.thread_id
    }
}

impl<S> Holder<S>
where
    S: GetAttributes,
{
    pub fn get_attributes(&self) -> VcxResult<String> {
        self.state.get_attributes()
    }
}

impl<S> Holder<S>
where
    S: GetAttachment,
{
    pub fn get_attachment(&self) -> VcxResult<String> {
        self.state.get_attachment()
    }
}

impl<S> Holder<S>
where
    S: IsTerminalState,
{
    pub fn is_terminal_state(&self) -> bool {
        self.state.is_terminal_state()
    }
}
