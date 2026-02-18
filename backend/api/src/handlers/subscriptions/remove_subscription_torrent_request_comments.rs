use crate::{
    handlers::subscriptions::create_subscription_torrent_request_comments::AddSubscriptionTorrentRequestCommentsQuery,
    middlewares::auth_middleware::Authdata, Arcadia,
};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;

pub type RemoveSubscriptionTorrentRequestCommentsQuery = AddSubscriptionTorrentRequestCommentsQuery;

#[utoipa::path(
    delete,
    operation_id = "Remove torrent request comments subscription",
    tag = "Subscription",
    path = "/api/subscriptions/torrent-request-comments",
    params (RemoveSubscriptionTorrentRequestCommentsQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully unsubscribed to the item"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<RemoveSubscriptionTorrentRequestCommentsQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool
        .delete_subscription_torrent_request_comments(query.torrent_request_id, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
