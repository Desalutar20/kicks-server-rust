use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::{
    ApiResponse, Error, Result,
    app::AppState,
    features::auth::{Password, domain::EmailAddress, service::reset_password::ResetPasswordInput},
    validate_and_parse,
};

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub token: String,
    pub new_password: String,
}

impl TryFrom<ResetPasswordRequest> for ResetPasswordInput {
    type Error = Error;

    fn try_from(value: ResetPasswordRequest) -> std::result::Result<Self, Self::Error> {
        let (email, new_password) = validate_and_parse!(
            email => EmailAddress::parse(value.email),
            new_password => Password::parse(value.new_password),
        );

        Ok(ResetPasswordInput {
            email,
            new_password,
            token: value.token,
        })
    }
}

pub async fn reset_password_v1(
    State(state): State<AppState>,
    WithRejection(Json(data), _): WithRejection<Json<ResetPasswordRequest>, Error>,
) -> Result<impl IntoResponse> {
    state.auth_service.reset_password(data.try_into()?).await?;

    Ok((StatusCode::OK, Json(ApiResponse { data: "Success" })).into_response())
}
