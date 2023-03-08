use messages::protocols::issuance::credential_proposal::CredentialProposal;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProposalSent {
    pub credential_proposal: CredentialProposal,
}

impl ProposalSent {
    pub fn new(credential_proposal: CredentialProposal) -> Self {
        Self { credential_proposal }
    }
}
