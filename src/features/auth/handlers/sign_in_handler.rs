use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
    SignedCookieJar, WithRejection,
    cookie::{Cookie, Expiration},
};
use serde::Deserialize;
use time::{Duration, OffsetDateTime};

use crate::{
    ApiResponse, Error, Result,
    app::AppState,
    configuration::app_config::ApplicationConfig,
    features::auth::{
        domain::{EmailAddress, Password},
        handlers::UserResponse,
        service::sign_in::SignInInput,
    },
    validate_and_parse,
};

#[derive(Debug, Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

impl TryFrom<SignInRequest> for SignInInput {
    type Error = Error;

    fn try_from(value: SignInRequest) -> std::result::Result<Self, Self::Error> {
        let (email, password) = validate_and_parse!(
            email => EmailAddress::parse(value.email),
            password => Password::parse(value.password),
        );

        Ok(SignInInput { email, password })
    }
}

pub async fn sign_in_v1(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    WithRejection(Json(data), _): WithRejection<Json<SignInRequest>, Error>,
) -> Result<impl IntoResponse> {
    let (user, session_id) = state.auth_service.sign_in(data.try_into()?).await?;
    let cookie = generate_session_cookie(session_id, &state.config);

    Ok((
        StatusCode::OK,
        jar.add(cookie),
        Json(ApiResponse {
            data: UserResponse {
                email: user.email.to_string(),
                gender: user.gender,
                last_name: user.last_name.map(|x| x.to_string()),
                first_name: user.first_name.map(|x| x.to_string()),
                role: user.role,
            },
        }),
    )
        .into_response())
}

pub fn generate_session_cookie<'a>(session_id: String, config: &ApplicationConfig) -> Cookie<'a> {
    let expires = OffsetDateTime::now_utc() + Duration::minutes(config.session_ttl_minutes as i64);

    Cookie::build((config.session_cookie_name.to_string(), session_id))
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .expires(Expiration::from(expires))
        .secure(config.cookie_secure)
        .path("/")
        .http_only(true)
        .build()
}
