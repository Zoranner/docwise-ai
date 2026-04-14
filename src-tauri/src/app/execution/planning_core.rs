//! 规划智能体共享：参数解析、工具分发。

use std::collections::HashMap;
use std::sync::Arc;

use agentool::Tool;
use serde_json::{json, Value};

pub(crate) fn parse_tool_arguments(raw: &str) -> Value {
    let t = raw.trim();
    if t.is_empty() {
        return json!({});
    }
    serde_json::from_str(t).unwrap_or_else(|_| json!({ "_raw": raw }))
}

pub(crate) async fn dispatch_tool(
    registry: &HashMap<String, Arc<dyn Tool>>,
    name: &str,
    args: Value,
) -> Value {
    match registry.get(name) {
        Some(tool) => match tool.execute(args).await {
            Ok(v) => v,
            Err(e) => json!({ "error": { "code": e.code, "message": e.message } }),
        },
        None => json!({
            "error": {
                "code": "unknown_tool",
                "message": format!("no tool registered named {name:?}")
            }
        }),
    }
}
