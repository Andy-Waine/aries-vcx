use messages::protocols::issuance::credential::Credential;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Finished {
    cred_id: String,
    credential: Credential,
    rev_reg_def_json: Option<String>,
}

impl Finished {
    pub fn new(cred_id: String, credential: Credential, rev_reg_def_json: Option<String>) -> Self {
        Self {
            cred_id,
            credential,
            rev_reg_def_json,
        }
    }
}
