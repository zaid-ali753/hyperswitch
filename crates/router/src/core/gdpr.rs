use crate::{core::errors::RouterResponse, routes::AppState, services, types::api::gdpr};

pub async fn delete_customer_data(
    state: &AppState,
    req: gdpr::GdprDeleteRequest,
) -> RouterResponse<()> {
    Ok(services::BachResponse::Json(()))
}
