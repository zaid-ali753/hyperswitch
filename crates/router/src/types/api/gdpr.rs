use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GdprDeleteRequest {
    pub customer_id: String,
    pub merchant_id: String,
}
