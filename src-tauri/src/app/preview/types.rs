use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderPreviewResult {
    pub snapshot_id: String,
    pub html: String,
    pub diagnostics: Vec<Diagnostic>,
    pub asset_base_url: String,
    pub theme_revision: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /// `"warning"` | `"error"`，与后续 Mdweave 诊断对齐。
    pub level: String,
    pub message: String,
}
