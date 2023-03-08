use std::sync::Arc;

use messages::protocols::issuance::credential_proposal::CredentialProposal;

use crate::{
    core::profile::profile::Profile,
    errors::error::VcxResult,
    protocols::{issuance::is_cred_def_revokable, issuance2::holder::trait_bounds::IsTerminalState},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProposalSent {
    pub credential_proposal: CredentialProposal,
}

impl ProposalSent {
    pub fn new(credential_proposal: CredentialProposal) -> Self {
        Self { credential_proposal }
    }

    pub async fn is_revokable(&self, profile: &Arc<dyn Profile>) -> VcxResult<bool> {
        is_cred_def_revokable(profile, &self.credential_proposal.cred_def_id).await
    }
}

impl IsTerminalState for ProposalSent {
    fn is_terminal_state(&self) -> bool {
        false
    }
}
