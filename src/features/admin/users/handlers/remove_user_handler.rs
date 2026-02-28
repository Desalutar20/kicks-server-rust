use std::sync::Arc;

use axum::{
    Json, debug_handler,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::{
    ApiResponse, Result,
    features::{admin::users::AdminUsersState, auth::UserID},
    validate_and_parse,
};

#[debug_handler()]
pub async fn remove_user_handler_v1(
    State(state): State<Arc<AdminUsersState>>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse> {
    let user_id = validate_and_parse!(
        user_id => UserID::parse(&user_id)
    );

    state.service.remove_user(user_id).await?;

    Ok((StatusCode::OK, Json(ApiResponse { data: "Success" })).into_response())
}
