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
}
