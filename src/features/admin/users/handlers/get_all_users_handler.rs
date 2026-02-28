use std::sync::Arc;

use axum::{
    Json, debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    Error, PaginatedResponse, Result,
    features::{
        admin::users::{
            AdminUsersState,
            domain::{GetAllUsersPagination, GetAllUsersSearch},
            handlers::AdminUserResponse,
            service::get_all_users::GetAllUsersInput,
        },
        auth::UserGender,
    },
    validate_and_parse,
};

#[derive(Debug, Deserialize)]
pub struct GetAllUsersRequestQuery {
    pub is_banned: Option<bool>,
    pub is_verified: Option<bool>,
    pub start_date: Option<OffsetDateTime>,
    pub end_date: Option<OffsetDateTime>,
    pub search: Option<String>,
    pub gender: Option<String>,
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

impl TryFrom<GetAllUsersRequestQuery> for GetAllUsersInput {
    type Error = Error;

    fn try_from(value: GetAllUsersRequestQuery) -> std::result::Result<Self, Self::Error> {
        let (search, gender, pagination) = validate_and_parse!(
            search => value.search.map(GetAllUsersSearch::parse).transpose(),
            gender => value.gender.map(UserGender::parse).transpose(),
            pagination => GetAllUsersPagination::parse(value.limit, value.page)
        );

        Ok(GetAllUsersInput {
            is_banned: value.is_banned,
            is_verified: value.is_verified,
            start_date: value.start_date,
            end_date: value.end_date,
            search,
            gender,
            pagination,
        })
    }
}

#[debug_handler()]
pub async fn get_all_users_handler_v1(
    State(state): State<Arc<AdminUsersState>>,
    WithRejection(Query(data), _): WithRejection<Query<GetAllUsersRequestQuery>, Error>,
) -> Result<impl IntoResponse> {
    let (admin_users, metadata) = state.service.get_all_users(data.try_into()?).await?;

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::new(admin_users, metadata, |data| {
            data.into_iter().map(AdminUserResponse::from).collect()
        })),
    )
        .into_response())
}
