use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::SignedCookieJar;

use crate::{Error, Result, app::AppState, features::auth::generate_session_cookie};

pub async fn authenticate(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    mut req: Request,
    next: Next,
) -> Result<(SignedCookieJar, Response)> {
    let session_id = jar
        .get(&state.config.session_cookie_name)
        .ok_or(Error::Unauthorized)?;

    let user = state.auth_service.authenticate(session_id.value()).await?;

    req.extensions_mut().insert(Some(user));
    let jar = jar.add(generate_session_cookie(
        session_id.value().to_string(),
        &state.config,
    ));

    let response = next.run(req).await;

    Ok((jar, response))
}
