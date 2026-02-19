use actix_multipart::form::{bytes::Bytes, MultipartForm};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::redis::RedisPoolInterface;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{middlewares::auth_middleware::Authdata, services::image_host_service, Arcadia};

#[derive(Debug, MultipartForm, ToSchema)]
pub struct UploadImageForm {
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub image: Bytes,
}

#[derive(Serialize, ToSchema)]
pub struct UploadImageResponse {
    pub url: String,
}

#[utoipa::path(
    post,
    operation_id = "Upload image",
    tag = "Image Host",
    path = "/api/image-host/upload",
    request_body(content = UploadImageForm, content_type = "multipart/form-data"),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully uploaded the image", body = UploadImageResponse),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: MultipartForm<UploadImageForm>,
    arc: Data<Arcadia<R>>,
    _user: Authdata,
) -> Result<HttpResponse> {
    let (Some(api_url), Some(api_key)) = (
        &arc.image_host.chevereto_api_url,
        &arc.image_host.chevereto_api_key,
    ) else {
        return Err(Error::ImageHostNotConfigured);
    };

    let url =
        image_host_service::upload_image_to_chevereto(api_url, api_key, &form.image.data).await?;

    Ok(HttpResponse::Ok().json(UploadImageResponse { url }))
}
