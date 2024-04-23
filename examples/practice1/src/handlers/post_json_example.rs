use axum::{
    async_trait,
    extract::{FromRequestParts, Path, State},
    http::{
        header::{AUTHORIZATION, HOST},
        request::Parts,
        HeaderMap, StatusCode,
    },
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::validation::ValidatedJson;

#[tracing::instrument]
pub async fn handler(
    headers: Headers,
    Path((item, id)): Path<(String, u32)>,
    State(client): State<reqwest::Client>,
    ValidatedJson(payload): ValidatedJson<PayloadSchema>,
) -> Result<(StatusCode, Json<ResponseSchema>), (StatusCode, String)> {
    tracing::debug!("invoke post_json_example handler");

    //println!("headers: {headers:#?}");
    if let Some(authorization) = headers.authorization {
        println!("Authorization Header: {authorization}");
    }
    if let Some(host) = headers.host {
        println!("Host Header: {host}");
    }

    println!(
        "item: {item}, id: {id}, payload: {}",
        serde_json::json!(payload)
    );

    let ipv4 = client
        .get("https://checkip.amazonaws.com")
        .send()
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to get ipv4".to_string(),
            )
        })?
        .text()
        .await
        .map_err(|_err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "response is not text".to_string(),
            )
        })?
        .trim()
        .to_string();
    tracing::debug!("ipv4: {ipv4}");

    Ok((
        StatusCode::CREATED,
        Json(ResponseSchema {
            message: "succeeded.".to_string(),
            requested_property: SummaryItem {
                id,
                item_name: item,
                summary_message: payload.summary,
            },
        }),
    ))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PayloadSchema {
    #[validate(range(min = 1, exclusive_max = 1000, message = "within 1-1000"))]
    #[serde(rename = "messageId")]
    message_id: u32,
    #[validate(length(min = 1, max = 20, message = "within 1 - 20"))]
    summary: String,
    #[validate(length(max = 100))]
    #[serde(rename = "messageDetail")]
    message_detail: Option<String>,
    list: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseSchema {
    message: String,
    requested_property: SummaryItem,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SummaryItem {
    id: u32,
    item_name: String,
    summary_message: String,
}

#[derive(Debug)]
pub struct Headers {
    authorization: Option<String>,
    host: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Headers
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                (
                    StatusCode::BAD_REQUEST, // TMP
                    format!("error: {err:#?}"),
                )
            })?;

        let authorization = if headers.contains_key(AUTHORIZATION) {
            let val = match headers[AUTHORIZATION].to_str() {
                Ok(val) => Some(val.to_string()),
                Err(_) => None,
            };
            val
        } else {
            None
        };
        let host = if headers.contains_key(HOST) {
            let val = match headers[HOST].to_str() {
                Ok(val) => Some(val.to_string()),
                Err(_) => None,
            };
            val
        } else {
            None
        };

        Ok(Self {
            authorization,
            host,
        })
    }
}
