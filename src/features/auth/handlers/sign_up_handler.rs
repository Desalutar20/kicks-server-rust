use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::WithRejection;
use serde::Deserialize;

use crate::{
    ApiResponse, Error, Result,
    app::AppState,
    features::auth::{
        domain::{EmailAddress, FirstName, LastName, Password, UserGender},
        service::sign_up::SignUpInput,
    },
    validate_and_parse,
};

#[derive(Debug, Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
}

impl TryFrom<SignUpRequest> for SignUpInput {
    type Error = Error;

    fn try_from(value: SignUpRequest) -> std::result::Result<Self, Self::Error> {
        let (email, password, first_name, last_name, gender) = validate_and_parse!(
            email => EmailAddress::parse(value.email),
            password => Password::parse(value.password),
            first_name => value.first_name.map(FirstName::parse).transpose(),
            last_name => value.last_name.map(LastName::parse).transpose(),
            gender => value.gender.map(UserGender::parse).transpose(),
        );

        Ok(SignUpInput {
            email,
            password,
            first_name,
            last_name,
            gender,
        })
    }
}

pub async fn sign_up_v1(
    State(state): State<AppState>,
    WithRejection(Json(data), _): WithRejection<Json<SignUpRequest>, Error>,
) -> Result<impl IntoResponse> {
    state.auth_service.sign_up(data.try_into()?).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            data: "Verification email has been sent. Please check your inbox.",
        }),
    )
        .into_response())
}
