use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use crate::schema::google_pay_credentials;
use common_utils;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Identifiable, Queryable)]
#[diesel(table_name = google_pay_credentials)]
pub struct GooglePayCredentials {
    #[serde(skip_serializing)]
    pub id: i32,
    pub merchant_id: String,
    pub connector_name: String,
    pub allowed_payment_methods: Vec<String>,
    pub allowed_auth: Vec<String>,
    pub credentials: serde_json::Value
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = google_pay_credentials)]
#[serde(deny_unknown_fields)]
pub struct GooglePayCredentialsNew {
    pub merchant_id: String,
    pub connector_name: String,
    #[diesel(serialize_as = String)]
    pub allowed_payment_methods: Vec<String>,
    #[diesel(serialize_as = String)]
    pub allowed_auth: Vec<String>,
    pub credentials: Option<serde_json::Value>
}
