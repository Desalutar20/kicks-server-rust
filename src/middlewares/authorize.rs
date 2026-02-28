use axum::{
    Extension,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    Error, Result,
    features::{auth::UserRole, shared::AppUser},
};

pub async fn authorize(
    State(roles): State<Vec<UserRole>>,
    Extension(user): Extension<Option<AppUser>>,
    req: Request,
    next: Next,
) -> Result<Response> {
    if user.is_none() {
        return Err(Error::Unauthorized);
    }

    let user = user.unwrap();
    if !roles.contains(&user.role) {
        return Err(Error::Forbidden);
    }

    let response = next.run(req).await;

    Ok(response)
}
