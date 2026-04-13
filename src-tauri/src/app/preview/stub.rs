use comrak::{markdown_to_html, Options};

use super::types::{Diagnostic, RenderPreviewResult};

/// v1 预览后端：Comrak 直出 HTML，与设计文档 `ComrakStubBackend` 对齐。
pub struct ComrakStubBackend;

impl ComrakStubBackend {
    pub fn render(markdown: &str, snapshot_id: impl Into<String>) -> RenderPreviewResult {
        let options = Options::default();
        let html = markdown_to_html(markdown, &options);
        RenderPreviewResult {
            snapshot_id: snapshot_id.into(),
            html,
            diagnostics: Vec::<Diagnostic>::new(),
            asset_base_url: String::new(),
            theme_revision: "stub-v1".to_owned(),
        }
    }
}
