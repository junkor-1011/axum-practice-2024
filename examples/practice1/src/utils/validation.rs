use async_trait::async_trait;
use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("JSON format error: {err}")))?;

        value.validate().map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                format!("Validation error: [{}]", err).replace('\n', ", "),
            )
        })?;

        Ok(ValidatedJson(value))
    }
}
