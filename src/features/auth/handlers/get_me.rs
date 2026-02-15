use axum::{Extension, Json, http::StatusCode, response::IntoResponse};

use crate::{
    ApiResponse, Error, Result,
    features::{auth::UserResponse, shared::AppUser},
};

pub async fn get_me_v1(Extension(user): Extension<Option<AppUser>>) -> Result<impl IntoResponse> {
    if user.is_none() {
        return Err(Error::Unauthorized);
    }

    let user = user.unwrap();

    Ok((
        StatusCode::OK,
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
