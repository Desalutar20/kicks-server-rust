use axum::{Extension, Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::SignedCookieJar;

use crate::{ApiResponse, Error, Result, app::AppState, features::shared::AppUser};

pub async fn logout_v1(
    State(state): State<AppState>,
    Extension(user): Extension<Option<AppUser>>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse> {
    if user.is_none() {
        return Err(Error::Unauthorized);
    }

    let session_id = jar
        .get(&state.config.session_cookie_name)
        .ok_or_else(|| Error::Unauthorized)?;

    state.auth_service.logout(session_id.value()).await?;

    let jar = jar.remove(state.config.session_cookie_name.clone());

    Ok((StatusCode::OK, jar, Json(ApiResponse { data: "Success" })).into_response())
}
