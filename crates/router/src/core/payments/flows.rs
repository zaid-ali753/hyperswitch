mod authorize_flow;
mod cancel_flow;
mod capture_flow;
mod psync_flow;
mod session_flow;

use async_trait::async_trait;
use error_stack::{report, IntoReport, ResultExt};

use super::PaymentData;
use crate::{
    core::{
        errors::{self, RouterResult},
        payments,
    },
    routes::AppState,
    services,
    types::{self, api, storage},
};

pub trait Flow<F, Req, Res>: Send + std::fmt::Debug {
    fn to_construct_r_d(
        &self,
    ) -> RouterResult<&(dyn ConstructFlowSpecificData<F, Req, Res> + Send + Sync)> {
        Err(report!(errors::ApiErrorResponse::InternalServerError)).attach_printable_lazy(|| {
            format!("construct routerdata interface not found for {self:?}")
        })
    }
}

pub trait DecideFlow<F, Req, Res>: Send + std::fmt::Debug {
    fn to_decide_flows(&self) -> RouterResult<&(dyn Feature<F, Req, Res> + Send + Sync)> {
        Err(report!(errors::ApiErrorResponse::InternalServerError)).attach_printable_lazy(|| {
            format!("construct routerdata interface not found for {self:?}")
        })
    }
}

#[async_trait]
pub trait ConstructFlowSpecificData<F, Req, Res> {
    async fn construct_r_d<'a>(
        &self,
        state: &AppState,
        connector_id: &str,
        merchant_account: &storage::MerchantAccount,
    ) -> RouterResult<types::RouterData<F, Req, Res>>;
}

#[async_trait]
pub trait Feature<F, T, Res> {
    async fn decide_flows<'a>(
        &self,
        state: &AppState,
        connector: api::ConnectorData,
        maybe_customer: &Option<api::CustomerResponse>,
        payment_data: PaymentData<F>,
        call_connector_action: payments::CallConnectorAction,
    ) -> (RouterResult<types::RouterData<F, T, Res>>, PaymentData<F>)
    where
        // Self: std::marker::Sized,
        F: Clone,
        dyn api::Connector: services::ConnectorIntegration<F, T, Res>;
}
