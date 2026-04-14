//! 工作区路径解析与只读访问（相对工作区根，禁止 `..` 越界）。
//!
//! 与 `agentool::fs` 的沙箱语义对齐：相对路径、词法规范化、解析结果必须落在已打开工作区的规范根之下。

use std::path::{Component, Path, PathBuf};

/// 将 `user` 接到 `root` 上并折叠 `.` / `..`（不访问文件系统）。
fn combine_and_normalize(root: &Path, user: &Path) -> PathBuf {
    let combined = if user.is_absolute() {
        user.to_path_buf()
    } else {
        root.join(user)
    };
    let mut out = PathBuf::new();
    for component in combined.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => out.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = out.pop();
            }
            Component::Normal(part) => out.push(part),
        }
    }
    out
}

fn is_descendant(root: &Path, path: &Path) -> bool {
    let r: Vec<_> = root.components().collect();
    let p: Vec<_> = path.components().collect();
    if p.len() < r.len() {
        return false;
    }
    r.iter().zip(p.iter()).all(|(a, b)| a == b)
}

/// 在已打开工作区内解析 `rel`（相对路径片段，空串表示根目录）。若路径已存在则返回 [`canonicalize`](Path::canonicalize) 结果。
pub fn resolve_workspace_path(root: &Path, rel: &str) -> Result<PathBuf, String> {
    let root_canon = root
        .canonicalize()
        .map_err(|e| format!("workspace root: {e}"))?;
    let s = rel.trim();
    let user_path = if s.is_empty() {
        return Ok(root_canon);
    } else {
        Path::new(s)
    };
    if user_path.is_absolute() {
        return Err("path must be relative to workspace root".into());
    }
    let logical = combine_and_normalize(&root_canon, user_path);
    if !is_descendant(&root_canon, &logical) {
        return Err("path escapes workspace root".into());
    }
    if logical.exists() {
        let c = logical
            .canonicalize()
            .map_err(|e| format!("canonicalize: {e}"))?;
        if !is_descendant(&root_canon, &c) {
            return Err("resolved path escapes workspace root".into());
        }
        return Ok(c);
    }
    Ok(logical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn rejects_escape_via_dotdot() {
        let root = fs::canonicalize(std::env::temp_dir()).expect("tmp");
        let err = resolve_workspace_path(&root, "..").expect_err("escape");
        assert!(
            err.contains("escapes") || err.contains("workspace"),
            "{err}"
        );
    }
}
