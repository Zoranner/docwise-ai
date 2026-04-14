//! 按产品设计中「智能体角色与工具权限」组装 [`agentool`] 工具（工作区根目录与 `ProjectContext` 一致）。
//!
//! - **规划对话智能体**：`file_read`、`directory_list`；`web_*`；`md_*`；`memory_*`。
//! - **文档执行智能体**：`fs` 全量；`find`；`web`；`git`；`md`；`memory`；`todo_add` / `todo_list` / `todo_update`（不含 `todo_remove`，与设计权限表一致）。

use std::path::PathBuf;
use std::sync::Arc;

use agentool::Tool;

const PLANNER_FS_NAMES: &[&str] = &["file_read", "directory_list"];

const EXECUTOR_TODO_NAMES: &[&str] = &["todo_add", "todo_list", "todo_update"];

/// 规划对话智能体用的 agentool 工具（需已打开工作区以解析路径）。
pub fn workspace_tools_planner(root: PathBuf) -> Result<Vec<Arc<dyn Tool>>, String> {
    let root_opt = Some(root);
    let mut out: Vec<Arc<dyn Tool>> = Vec::new();

    let fs_ctx =
        Arc::new(agentool::fs::FsContext::new(root_opt.clone(), false).map_err(|e| e.to_string())?);
    out.extend(
        agentool::fs::all_tools(fs_ctx)
            .into_iter()
            .filter(|t| PLANNER_FS_NAMES.contains(&t.name())),
    );

    let web_ctx =
        Arc::new(agentool::web::WebContext::new().map_err(|e| format!("web_context: {e}"))?);
    out.extend(agentool::web::all_tools(web_ctx));

    let md_ctx =
        Arc::new(agentool::md::MdContext::new(root_opt.clone(), false).map_err(|e| e.to_string())?);
    out.extend(agentool::md::all_tools(md_ctx));

    let mem_ctx =
        Arc::new(agentool::memory::MemoryContext::new(root_opt, false).map_err(|e| e.to_string())?);
    out.extend(agentool::memory::all_tools(mem_ctx));

    Ok(out)
}

/// 文档执行智能体用的 agentool 工具。
pub fn workspace_tools_executor(root: PathBuf) -> Result<Vec<Arc<dyn Tool>>, String> {
    let root_opt = Some(root);
    let mut out: Vec<Arc<dyn Tool>> = Vec::new();

    let fs_ctx =
        Arc::new(agentool::fs::FsContext::new(root_opt.clone(), false).map_err(|e| e.to_string())?);
    out.extend(agentool::fs::all_tools(fs_ctx));

    let find_ctx =
        Arc::new(agentool::find::FindContext::new(root_opt.clone()).map_err(|e| e.to_string())?);
    out.extend(agentool::find::all_tools(find_ctx));

    let web_ctx =
        Arc::new(agentool::web::WebContext::new().map_err(|e| format!("web_context: {e}"))?);
    out.extend(agentool::web::all_tools(web_ctx));

    let git_ctx =
        Arc::new(agentool::git::GitContext::new(root_opt.clone()).map_err(|e| e.to_string())?);
    out.extend(agentool::git::all_tools(git_ctx));

    let md_ctx =
        Arc::new(agentool::md::MdContext::new(root_opt.clone(), false).map_err(|e| e.to_string())?);
    out.extend(agentool::md::all_tools(md_ctx));

    let mem_ctx = Arc::new(
        agentool::memory::MemoryContext::new(root_opt.clone(), false).map_err(|e| e.to_string())?,
    );
    out.extend(agentool::memory::all_tools(mem_ctx));

    let todo_ctx =
        Arc::new(agentool::todo::TodoContext::new(root_opt, false).map_err(|e| e.to_string())?);
    out.extend(
        agentool::todo::all_tools(todo_ctx)
            .into_iter()
            .filter(|t| EXECUTOR_TODO_NAMES.contains(&t.name())),
    );

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planner_fs_subset() {
        let dir =
            std::env::temp_dir().join(format!("docwise_planner_tools_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let tools = workspace_tools_planner(dir.clone()).expect("tools");
        let names: Vec<_> = tools.iter().map(|t| t.name().to_string()).collect();
        assert!(names.contains(&"file_read".into()));
        assert!(names.contains(&"directory_list".into()));
        assert!(!names.iter().any(|n| n == "file_write"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn executor_includes_find_git_and_limited_todo() {
        let dir = std::env::temp_dir().join(format!("docwise_exec_tools_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let tools = workspace_tools_executor(dir.clone()).expect("tools");
        let names: Vec<_> = tools.iter().map(|t| t.name().to_string()).collect();
        assert!(names.contains(&"grep_search".into()));
        assert!(names.contains(&"git_status".into()));
        assert!(names.contains(&"todo_add".into()));
        assert!(!names.contains(&"todo_remove".into()));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
