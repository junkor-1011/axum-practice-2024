use axum::{extract::Path, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::validation::ValidatedJson;

#[tracing::instrument]
pub async fn handler(
    Path((item, id)): Path<(String, u32)>,
    ValidatedJson(payload): ValidatedJson<PayloadSchema>,
) -> (StatusCode, Json<ResponseSchema>) {
    tracing::debug!("invoke post_json_example handler");

    println!(
        "item: {item}, id: {id}, payload: {}",
        serde_json::json!(payload)
    );

    (
        StatusCode::CREATED,
        Json(ResponseSchema {
            message: "succeeded.".to_string(),
            requested_property: SummaryItem {
                id,
                item_name: item,
                summary_message: payload.summary,
            },
        }),
    )
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
