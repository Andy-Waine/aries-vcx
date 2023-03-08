use messages::protocols::issuance::credential_offer::CredentialOffer;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OfferReceived {
    pub offer: CredentialOffer,
}

impl OfferReceived {
    pub fn new(offer: CredentialOffer) -> Self {
        OfferReceived { offer }
    }
}
