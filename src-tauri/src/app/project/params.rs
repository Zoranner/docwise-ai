use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintItemAddParams {
    pub blueprint_id: String,
    pub file_path: String,
    pub title: String,
    pub audience: Option<String>,
    pub goal: Option<String>,
    pub must_cover: Option<String>,
    pub constraints: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintItemUpdateParams {
    pub id: String,
    pub seq: Option<i32>,
    pub file_path: Option<String>,
    pub title: Option<String>,
    pub audience: Option<String>,
    pub goal: Option<String>,
    pub must_cover: Option<String>,
    pub constraints: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdateParams {
    pub id: String,
    pub title: Option<String>,
    pub goal: Option<String>,
    pub acceptance: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub blocked_reason: Option<String>,
    pub tags: Option<String>,
    pub conversation_ref: Option<String>,
}
