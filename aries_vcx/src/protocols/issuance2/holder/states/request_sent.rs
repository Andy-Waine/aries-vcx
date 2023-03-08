use crate::{
    errors::error::{AriesVcxError, AriesVcxErrorKind, VcxResult},
    protocols::issuance2::holder::trait_bounds::IsTerminalState,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestSent {
    cred_request_metadata: String,
    cred_def_json: String,
}

impl RequestSent {
    pub fn new(cred_request_metadata: String, cred_def_json: String) -> Self {
        RequestSent {
            cred_request_metadata,
            cred_def_json,
        }
    }

    pub fn is_revokable(&self) -> VcxResult<bool> {
        let parsed_cred_def: serde_json::Value = serde_json::from_str(&self.cred_def_json).map_err(|err| {
            AriesVcxError::from_msg(
                AriesVcxErrorKind::SerializationError,
                format!(
                    "Failed deserialize credential definition json {}\nError: {}",
                    self.cred_def_json, err
                ),
            )
        })?;
        Ok(!parsed_cred_def["value"]["revocation"].is_null())
    }
}

impl IsTerminalState for RequestSent {
    fn is_terminal_state(&self) -> bool {
        false
    }
}
