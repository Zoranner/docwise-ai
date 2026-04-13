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
    pub level: DiagnosticLevel,
    pub message: String,
}

/// 预留与 `MdweaveBackend` 诊断对齐；`ComrakStubBackend` 当前不产出诊断。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum DiagnosticLevel {
    Warning,
    Error,
}
