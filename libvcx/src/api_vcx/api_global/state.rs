use aries_vcx::{
    global::settings::{
        reset_config_values, CONFIG_POOL_NAME, CONFIG_WALLET_KEY, CONFIG_WALLET_KEY_DERIVATION, CONFIG_WALLET_NAME,
        CONFIG_WALLET_TYPE, DEFAULT_POOL_NAME, DEFAULT_WALLET_NAME, UNINITIALIZED_WALLET_KEY, WALLET_KDF_DEFAULT,
    },
    indy::{
        ledger::pool,
        wallet::{delete_wallet, WalletConfig},
    },
};

use crate::api_vcx::api_global::{
    agency_client::reset_main_agency_client,
    pool::{close_main_pool, reset_main_pool_handle},
    settings::get_config_value,
    wallet::close_main_wallet,
};

pub fn state_vcx_shutdown(delete: bool) {
    info!("vcx_shutdown >>>");
    trace!("vcx_shutdown(delete: {})", delete);

    if let Ok(()) = futures::executor::block_on(close_main_wallet()) {}
    if let Ok(()) = futures::executor::block_on(close_main_pool()) {}

    crate::api_vcx::api_handle::schema::release_all();
    crate::api_vcx::api_handle::mediated_connection::release_all();
    crate::api_vcx::api_handle::issuer_credential::release_all();
    crate::api_vcx::api_handle::credential_def::release_all();
    crate::api_vcx::api_handle::proof::release_all();
    crate::api_vcx::api_handle::disclosed_proof::release_all();
    crate::api_vcx::api_handle::credential::release_all();

    if delete {
        let pool_name = get_config_value(CONFIG_POOL_NAME).unwrap_or(DEFAULT_POOL_NAME.to_string());
        let wallet_name = get_config_value(CONFIG_WALLET_NAME).unwrap_or(DEFAULT_WALLET_NAME.to_string());
        let wallet_type = get_config_value(CONFIG_WALLET_TYPE).ok();
        let wallet_key = get_config_value(CONFIG_WALLET_KEY).unwrap_or(UNINITIALIZED_WALLET_KEY.into());
        let wallet_key_derivation = get_config_value(CONFIG_WALLET_KEY_DERIVATION).unwrap_or(WALLET_KDF_DEFAULT.into());

        let _res = futures::executor::block_on(close_main_wallet());

        let wallet_config = WalletConfig {
            wallet_name,
            wallet_key,
            wallet_key_derivation,
            wallet_type,
            storage_config: None,
            storage_credentials: None,
            rekey: None,
            rekey_derivation_method: None,
        };

        if let Ok(()) = futures::executor::block_on(delete_wallet(&wallet_config)) {}
        if let Ok(()) = futures::executor::block_on(pool::delete(&pool_name)) {}
    }

    let _ = reset_config_values();
    reset_main_agency_client();
    reset_main_pool_handle();
}

#[cfg(test)]
pub mod tests {
    use aries_vcx::{
        utils::{
            devsetup::SetupMocks,
            mockdata::{mockdata_credex::ARIES_CREDENTIAL_OFFER, mockdata_proof::ARIES_PROOF_REQUEST_PRESENTATION},
        },
        vdrtools::INVALID_WALLET_HANDLE,
    };

    use crate::{
        api_c::vcx::vcx_shutdown,
        api_vcx::{
            api_global::wallet::get_main_wallet_handle,
            api_handle::{
                credential, credential::credential_create_with_offer, credential_def, disclosed_proof,
                disclosed_proof::create_with_proof_request, issuer_credential, mediated_connection, proof, schema,
                schema::create_and_publish_schema,
            },
        },
    };

    #[tokio::test]
    #[cfg(feature = "general_test")]
    async fn test_shutdown() {
        let _setup = SetupMocks::init();

        let data = r#"["name","male"]"#;
        let connection = mediated_connection::tests::build_test_connection_inviter_invited().await;
        let credentialdef = credential_def::create("SID".to_string(), "id".to_string(), "tag".to_string(), false)
            .await
            .unwrap();
        let issuer_credential = issuer_credential::issuer_credential_create("1".to_string()).unwrap();
        let proof = proof::create_proof(
            "1".to_string(),
            "[]".to_string(),
            "[]".to_string(),
            r#"{"support_revocation":false}"#.to_string(),
            "Optional".to_owned(),
        )
        .await
        .unwrap();
        let schema = create_and_publish_schema("5", "name".to_string(), "0.1".to_string(), data.to_string())
            .await
            .unwrap();
        let disclosed_proof = create_with_proof_request("id", ARIES_PROOF_REQUEST_PRESENTATION).unwrap();
        let credential = credential_create_with_offer("name", ARIES_CREDENTIAL_OFFER).unwrap();

        vcx_shutdown(true);
        assert_eq!(mediated_connection::is_valid_handle(connection), false);
        assert_eq!(issuer_credential::is_valid_handle(issuer_credential), false);
        assert_eq!(schema::is_valid_handle(schema), false);
        assert_eq!(proof::is_valid_handle(proof), false);
        assert_eq!(credential_def::is_valid_handle(credentialdef), false);
        assert_eq!(credential::is_valid_handle(credential), false);
        assert_eq!(disclosed_proof::is_valid_handle(disclosed_proof), false);
        assert_eq!(get_main_wallet_handle(), INVALID_WALLET_HANDLE);
    }
}
