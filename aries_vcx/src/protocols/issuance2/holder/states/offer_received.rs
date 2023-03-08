use std::sync::Arc;

use messages::protocols::issuance::credential_offer::CredentialOffer;

use crate::{
    core::profile::profile::Profile,
    errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult},
    protocols::{
        issuance::{holder::state_machine::parse_cred_def_id_from_cred_offer, is_cred_def_revokable},
        issuance2::holder::trait_bounds::{GetAttachment, GetAttributes, IsTerminalState},
    },
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OfferReceived {
    pub offer: CredentialOffer,
}

impl OfferReceived {
    pub fn new(offer: CredentialOffer) -> Self {
        OfferReceived { offer }
    }

    pub async fn is_revokable(&self, profile: &Arc<dyn Profile>) -> VcxResult<bool> {
        let offer = self.offer.offers_attach.content().map_err(|err| {
            AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidJson,
                format!("Failed to get credential offer attachment content: {}", err),
            )
        })?;
        let cred_def_id = parse_cred_def_id_from_cred_offer(&offer).map_err(|err| {
            AriesVcxError::from_msg(
                AriesVcxErrorKind::InvalidJson,
                format!(
                    "Failed to parse credential definition id from credential offer: {}",
                    err
                ),
            )
        })?;
        is_cred_def_revokable(profile, &cred_def_id).await
    }
}

impl GetAttributes for OfferReceived {
    fn get_attributes(&self) -> VcxResult<String> {
        let mut new_map = serde_json::map::Map::new();
        self.offer.credential_preview.attributes.iter().for_each(|attribute| {
            new_map.insert(
                attribute.name.clone(),
                serde_json::Value::String(attribute.value.clone()),
            );
        });
        Ok(serde_json::Value::Object(new_map).to_string())
    }
}

impl GetAttachment for OfferReceived {
    fn get_attachment(&self) -> VcxResult<String> {
        Ok(self.offer.offers_attach.content()?)
    }
}

impl IsTerminalState for OfferReceived {
    fn is_terminal_state(&self) -> bool {
        false
    }
}
