use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::app::AppState;
use crate::features::auth::domain::EmailAddress;
use crate::features::auth::service::verify_account::VerifyAccountInput;
use crate::{ApiResponse, Error, Result, validate_and_parse};

#[derive(Debug, Deserialize)]
pub struct VerifyAccountRequest {
    pub email: String,
    pub token: String,
}

impl TryInto<VerifyAccountInput> for VerifyAccountRequest {
    type Error = Error;

    fn try_into(self) -> std::result::Result<VerifyAccountInput, Self::Error> {
        let email = validate_and_parse!(email => EmailAddress::parse(self.email));

        Ok(VerifyAccountInput {
            email,
            token: self.token,
        })
    }
}

pub async fn verify_account_v1(
    State(state): State<AppState>,
    WithRejection(Json(data), _): WithRejection<Json<VerifyAccountRequest>, Error>,
) -> Result<impl IntoResponse> {
    state.auth_service.verify_account(data.try_into()?).await?;

    Ok((
              StatusCode::OK,
              Json(ApiResponse {
                             data: "Your account has been successfully verified! Please log in now using your credentials.",
              }),
          )
              .into_response())
}
