use diesel::{Identifiable, Insertable, Queryable};
use error_stack::{IntoReport, ResultExt};

use crate::schema::google_pay_credentials;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Identifiable, Queryable)]
#[diesel(table_name = google_pay_credentials)]
pub struct GooglePayCredentials {
    #[serde(skip_serializing)]
    pub id: i32,
    pub merchant_id: String,
    pub connector_name: String,
    pub allowed_payment_methods: Vec<String>,
    pub allowed_auth: Vec<String>,
    pub credentials: serde_json::Value,
}

#[derive(Debug, serde::Serialize, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = google_pay_credentials)]
#[serde(deny_unknown_fields)]
pub struct GooglePayCredentialsNewInternal {
    merchant_id: String,
    connector_name: String,
    allowed_payment_methods: String,
    allowed_auth: String,
    credentials: Option<serde_json::Value>,
}

pub struct GooglePayCredentialsNew {
    pub merchant_id: String,
    pub connector_name: String,
    pub allowed_payment_methods: Vec<String>,
    pub allowed_auth: Vec<String>,
    pub credentials: Option<serde_json::Value>,
}

impl GooglePayCredentialsNew {
    pub fn build(
        self,
    ) -> error_stack::Result<GooglePayCredentialsNewInternal, common_utils::errors::ParsingError>
    {
        Ok(GooglePayCredentialsNewInternal {
            merchant_id: self.merchant_id,
            connector_name: self.connector_name,
            allowed_payment_methods: serde_json::to_string(&self.allowed_payment_methods)
                .into_report()
                .change_context(common_utils::errors::ParsingError)?,
            allowed_auth: serde_json::to_string(&self.allowed_auth)
                .into_report()
                .change_context(common_utils::errors::ParsingError)?,
            credentials: self.credentials,
        })
    }
}
