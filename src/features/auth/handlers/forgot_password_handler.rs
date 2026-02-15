use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::{
    ApiResponse, Error, Result,
    app::AppState,
    features::auth::{domain::EmailAddress, service::forgot_password::ForgotPasswordInput},
    validate_and_parse,
};

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

impl TryFrom<ForgotPasswordRequest> for ForgotPasswordInput {
    type Error = Error;

    fn try_from(value: ForgotPasswordRequest) -> std::result::Result<Self, Self::Error> {
        let email = validate_and_parse!(
            email => EmailAddress::parse(value.email),
        );

        Ok(ForgotPasswordInput { email })
    }
}

pub async fn forgot_password_v1(
    State(state): State<AppState>,
    WithRejection(Json(data), _): WithRejection<Json<ForgotPasswordRequest>, Error>,
) -> Result<impl IntoResponse> {
    state.auth_service.forgot_password(data.try_into()?).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            data: "If the email address is registered, a verification email has been sent.",
        }),
    )
        .into_response())
}
