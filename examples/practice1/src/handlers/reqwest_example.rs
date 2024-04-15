use axum::{http::StatusCode, response::IntoResponse};

pub async fn get_ipv4() -> Result<String, CustomError> {
    let res = reqwest::get("https://checkip.amazonaws.com").await?;

    tracing::info!("Response: {:?} {}", res.version(), res.status());
    tracing::info!("Headers: {:#?}", res.headers());

    let ipv4 = res.text().await?.trim().to_string();

    Ok(ipv4)
}

pub enum CustomError {
    InternalServerError(anyhow::Error),
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CustomError::InternalServerError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("something went wrong: {err}"),
            )
                .into_response(),
        }
    }
}

impl<E> From<E> for CustomError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::InternalServerError(err.into())
    }
}
