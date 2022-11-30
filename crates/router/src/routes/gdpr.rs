use actix_web::{web, HttpRequest, HttpResponse};
use router_env::{
    tracing::{self, instrument},
    Flow,
};

use super::app::AppState;
use crate::{core::gdpr, services::api, types::api::gdpr as gdpr_api};

#[instrument(skip_all, fields(flow = ?Flow::GdprDataDelete))]
pub async fn delete_customer_data_api(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    let (customer_id, merchant_id) = path.into_inner();
    let payload = web::Json(gdpr_api::GdprDeleteRequest {
        customer_id,
        merchant_id,
    })
    .into_inner();
    api::server_wrap(
        &state,
        &req,
        payload,
        |state, _, del_req| gdpr::delete_customer_data(state, del_req),
        api::MerchantAuthentication::ApiKey,
    )
    .await
}
