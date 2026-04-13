//! 工具名 → [`agentool::Tool`]，用于模型返回的 `function.name` 分发。

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use agentool::Tool;

/// 同名工具以 **`primary` 为准**；`secondary` 仅补充尚未出现的名字（便于规划智能体挂载 agentool 只读工具）。
pub fn merge_planner_tool_list(
    primary: Vec<Arc<dyn Tool>>,
    secondary: &[Arc<dyn Tool>],
) -> Vec<Arc<dyn Tool>> {
    let mut seen: HashSet<String> = primary.iter().map(|t| t.name().to_string()).collect();
    let mut out = primary;
    for t in secondary {
        let n = t.name().to_string();
        if seen.insert(n) {
            out.push(Arc::clone(t));
        }
    }
    out
}

pub fn tools_by_name(tools: &[Arc<dyn Tool>]) -> HashMap<String, Arc<dyn Tool>> {
    let mut m = HashMap::with_capacity(tools.len());
    for t in tools {
        m.insert(t.name().to_string(), Arc::clone(t));
    }
    m
}
