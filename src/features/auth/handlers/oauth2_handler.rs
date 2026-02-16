use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::{
    SignedCookieJar,
    cookie::{Cookie, Expiration, SameSite},
};
use serde::Deserialize;
use time::{Duration, OffsetDateTime};

use crate::{
    Error, Result,
    app::AppState,
    configuration::app_config::ApplicationConfig,
    features::auth::{
        OAuth2Code, OAuth2State, generate_session_cookie,
        service::oauth2::{OAuth2Provider, OAuth2SignInInput},
    },
    validate_and_parse,
};

#[derive(Debug, Deserialize)]
pub struct OAuth2RedirectUrlRequestQuery {
    pub redirect_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2SignInRequestQuery {
    state: String,
    code: String,
}

pub async fn get_google_redirect_url_v1(
    State(state): State<AppState>,
    Query(query): Query<OAuth2RedirectUrlRequestQuery>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse> {
    let (url, oauth_state) = state
        .auth_service
        .generate_oauth2_redirect_url(OAuth2Provider::Google, query.redirect_path)?;

    Ok((
        jar.add(generate_oauth_state_cookie(oauth_state, &state.config)),
        Redirect::to(url.as_str()),
    )
        .into_response())
}

pub async fn google_sign_in_v1(
    State(state): State<AppState>,
    Query(query): Query<OAuth2SignInRequestQuery>,
    jar: SignedCookieJar,
) -> Result<Response> {
    let parsed = parse_oauth2_request(query, &jar, &state.config)?;

    let (session_id, redirect_path) = state
        .auth_service
        .oauth2_sign_in(OAuth2Provider::Google, parsed)
        .await?;

    let cookie = jar
        .add(generate_session_cookie(session_id, &state.config))
        .remove(Cookie::from(
            state.config.oauth_state_cookie_name.to_string(),
        ));

    match redirect_path {
        Some(path) => Ok((
            cookie,
            Redirect::to(&format!("{}{}", state.config.client_url, path)),
        )
            .into_response()),
        None => Ok((cookie, Redirect::to(&state.config.client_url)).into_response()),
    }
}

fn generate_oauth_state_cookie<'a>(state: OAuth2State, config: &ApplicationConfig) -> Cookie<'a> {
    let expires =
        OffsetDateTime::now_utc() + Duration::minutes(config.oauth_state_ttl_minutes as i64);
    Cookie::build((
        config.oauth_state_cookie_name.to_string(),
        state.to_string(),
    ))
    .same_site(SameSite::Lax)
    .expires(Expiration::from(expires))
    .secure(config.cookie_secure)
    .path("/")
    .http_only(true)
    .build()
}

fn parse_oauth2_request(
    query: OAuth2SignInRequestQuery,
    cookie: &SignedCookieJar,
    config: &ApplicationConfig,
) -> Result<OAuth2SignInInput> {
    let cookie_state = cookie
        .get(&config.oauth_state_cookie_name)
        .ok_or(Error::Conflict("Invalit state".into()))?;

    let (st, cookie_state, code) = validate_and_parse!(
        state => OAuth2State::parse(query.state),
        cookie_state => OAuth2State::parse(cookie_state.value().to_string()),
        code => OAuth2Code::parse(query.code)
    );

    Ok(OAuth2SignInInput {
        state: st,
        cookie_state,
        code,
    })
}
