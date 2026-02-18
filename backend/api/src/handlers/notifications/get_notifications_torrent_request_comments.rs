use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::notification::NotificationTorrentRequestComment, redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetNotificationsTorrentRequestCommentsQuery {
    pub include_read: bool,
}

#[utoipa::path(
    get,
    operation_id = "Get notifications for torrent request comments",
    tag = "Notification",
    path = "/api/notifications/torrent-request-comments",
    params (GetNotificationsTorrentRequestCommentsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the notifications", body = Vec<NotificationTorrentRequestComment>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetNotificationsTorrentRequestCommentsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let notifications = arc
        .pool
        .find_notifications_torrent_request_comments(user.sub, query.include_read)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!(notifications)))
}
