# Docwise Agent-First MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a testable Docwise v0 that centers on global conversation, project/blueprint/task orchestration, human reviews, and file outputs without relying on in-app Markdown viewing or preview.

**Architecture:** Keep the Rust/Tauri backend as the orchestration core, rename the canonical domain to `Project / Blueprint / Task / Review / Output`, and reshape the Nuxt shell into a left conversation rail plus right monitoring workspace. Move document editing and preview out of the primary product path; the app should track what was written, where it was written, and why execution is blocked, while VSCode or similar tools handle file reading and manual editing.

**Tech Stack:** Nuxt 4, Vue 3, Nuxt UI, TypeScript, Bun, Tauri 2, Rust, SeaORM, SQLite

---

## 路线图

### 阶段一

统一领域模型与命名，移除 `Checkpoint / Artifact` 作为核心术语，建立 `Review / Output` 的 canonical backend model，并把 `ActiveContext` 从文件焦点改成监看焦点。

### 阶段二

重做应用壳层，形成“顶部全局导航 + 左侧常驻对话 + 右侧监看区”的结构；让 `/workspace` 不再承担编辑器和预览器职责。

### 阶段三

补齐项目详情 read model 和右侧 Tabs：`总览 / 蓝图 / 任务 / 审议 / 产出`。先把观测和编排台搭起来，再考虑后续任何可视化。

### 阶段四

把智能体会话、任务阻塞、人工审议与产出记录真正串起来，确保“项目 -> 蓝图 -> 任务 -> 执行 -> 文档落盘 -> 人工介入”能完整跑通。

### 阶段五

清理旧入口和旧心智，补齐验证命令、文档与回归测试，形成可持续推进的 v0 基线。

## 文件结构

### 后端领域与命名

- Modify: `docwise/src-tauri/src/app/project/entity/mod.rs`
- Replace: `docwise/src-tauri/src/app/project/entity/checkpoint.rs` -> `docwise/src-tauri/src/app/project/entity/review.rs`
- Replace: `docwise/src-tauri/src/app/project/entity/artifact.rs` -> `docwise/src-tauri/src/app/project/entity/output.rs`
- Modify: `docwise/src-tauri/src/app/project/mod.rs`
- Modify: `docwise/src-tauri/src/app/project/dto.rs`
- Modify: `docwise/src-tauri/src/app/project/ops.rs`
- Modify: `docwise/src-tauri/src/app/project/params.rs`
- Modify: `docwise/src-tauri/src/app/project/tools.rs`
- Modify: `docwise/src-tauri/src/app/project/context.rs`
- Modify: `docwise/src-tauri/migration/src/m20260413120000_init_project.rs`

### 后端状态、命令与聚合

- Modify: `docwise/src-tauri/src/app/state.rs`
- Modify: `docwise/src-tauri/src/app/commands/active_context_cmd.rs`
- Modify: `docwise/src-tauri/src/app/commands/project_cmds.rs`
- Modify: `docwise/src-tauri/src/app/commands/overview_cmd.rs`
- Modify: `docwise/src-tauri/src/app/commands/workspace.rs`
- Modify: `docwise/src-tauri/src/app/commands/mod.rs`
- Modify: `docwise/src-tauri/src/lib.rs`

### 前端壳层与类型

- Modify: `docwise/app/layouts/default.vue`
- Modify: `docwise/app/components/AppChromeNav.vue`
- Modify: `docwise/app/pages/index.vue`
- Modify: `docwise/app/pages/projects/index.vue`
- Modify: `docwise/app/pages/projects/overview.vue`
- Replace: `docwise/app/pages/workspace/index.vue`
- Add: `docwise/app/pages/projects/[projectId].vue`
- Add: `docwise/app/components/shell/DocwiseShell.vue`
- Add: `docwise/app/components/shell/DocwiseConversationRail.vue`
- Add: `docwise/app/components/shell/DocwiseFocusHeader.vue`
- Add: `docwise/app/components/project/ProjectOverviewPanel.vue`
- Add: `docwise/app/components/project/ProjectBlueprintPanel.vue`
- Add: `docwise/app/components/project/ProjectTaskPanel.vue`
- Add: `docwise/app/components/project/ProjectReviewPanel.vue`
- Add: `docwise/app/components/project/ProjectOutputPanel.vue`
- Modify: `docwise/app/composables/useDocwiseActiveContext.ts`
- Modify: `docwise/app/composables/useDocwiseAgentStream.ts`
- Replace: `docwise/app/composables/useDocwiseCheckpointEvents.ts` -> `docwise/app/composables/useDocwiseReviewEvents.ts`
- Add: `docwise/app/types/review.ts`
- Add: `docwise/app/types/output.ts`
- Modify: `docwise/app/types/active-context.ts`
- Replace: `docwise/app/types/checkpoint.ts` -> `docwise/app/types/review.ts`

### 前端纯逻辑与测试

- Add: `docwise/app/lib/focus-context.ts`
- Add: `docwise/app/lib/focus-context.test.ts`
- Add: `docwise/app/lib/project-tabs.ts`
- Add: `docwise/app/lib/project-tabs.test.ts`
- Add: `docwise/app/lib/chrome-nav.ts`
- Add: `docwise/app/lib/chrome-nav.test.ts`
- Modify: `docwise/package.json`

### 文档

- Modify: `docwise/docs/docwise-design.md`
- Modify: `docwise/docs/docwise-ui-agent-first-design.md`
- Modify: `docwise/docs/docwise-ui-shell-design.md`

## Task 1: 统一领域命名与数据库真相

**Files:**
- Modify: `docwise/src-tauri/migration/src/m20260413120000_init_project.rs`
- Modify: `docwise/src-tauri/src/app/project/entity/mod.rs`
- Replace: `docwise/src-tauri/src/app/project/entity/checkpoint.rs` -> `docwise/src-tauri/src/app/project/entity/review.rs`
- Replace: `docwise/src-tauri/src/app/project/entity/artifact.rs` -> `docwise/src-tauri/src/app/project/entity/output.rs`
- Modify: `docwise/src-tauri/src/app/project/mod.rs`
- Modify: `docwise/src-tauri/src/app/project/dto.rs`
- Modify: `docwise/src-tauri/src/app/project/ops.rs`
- Modify: `docwise/src-tauri/src/app/project/params.rs`
- Modify: `docwise/src-tauri/src/app/project/tools.rs`
- Test: `docwise/src-tauri/src/app/project/mod.rs`

- [ ] **Step 1: 写一个失败的 schema 测试，锁定 `reviews` / `outputs` canonical 名称**

```rust
#[cfg(test)]
mod tests {
    use super::context::ProjectContext;
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    use tempfile::tempdir;

    #[tokio::test]
    async fn project_context_creates_review_and_output_tables() {
        let dir = tempdir().expect("tempdir");
        let ctx = ProjectContext::open(dir.path().to_path_buf())
            .await
            .expect("open project context");

        let rows = ctx
            .db
            .query_all(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name".to_owned(),
            ))
            .await
            .expect("query sqlite_master");

        let names: Vec<String> = rows
            .iter()
            .filter_map(|row| row.try_get::<String>("", "name").ok())
            .collect();

        assert!(names.iter().any(|name| name == "reviews"));
        assert!(names.iter().any(|name| name == "outputs"));
        assert!(!names.iter().any(|name| name == "checkpoints"));
        assert!(!names.iter().any(|name| name == "artifacts"));
    }
}
```

- [ ] **Step 2: 运行测试，确认现状失败**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri project_context_creates_review_and_output_tables -- --exact
```

Expected: FAIL，因为当前初始迁移仍创建 `checkpoints` / `artifacts`。

- [ ] **Step 3: 最小实现 canonical rename，不保留双轨命名**

```rust
// migration/src/m20260413120000_init_project.rs
Table::create()
    .table(Reviews::Table)
    .if_not_exists()
    .col(ColumnDef::new(Reviews::Id).string().not_null().primary_key())
    .col(ColumnDef::new(Reviews::TaskId).string().not_null())
    .col(ColumnDef::new(Reviews::RunId).string())
    .col(ColumnDef::new(Reviews::Status).string().not_null())
    .col(ColumnDef::new(Reviews::Reason).string().not_null())
    .col(ColumnDef::new(Reviews::CreatedAt).string().not_null())
    .col(ColumnDef::new(Reviews::ResolvedAt).string())
    .to_owned();

Table::create()
    .table(Outputs::Table)
    .if_not_exists()
    .col(ColumnDef::new(Outputs::Id).string().not_null().primary_key())
    .col(ColumnDef::new(Outputs::TaskId).string().not_null())
    .col(ColumnDef::new(Outputs::RunId).string())
    .col(ColumnDef::new(Outputs::Kind).string().not_null())
    .col(ColumnDef::new(Outputs::Path).string().not_null())
    .col(ColumnDef::new(Outputs::Summary).string())
    .col(ColumnDef::new(Outputs::CreatedAt).string().not_null())
    .to_owned();
```

```rust
// project/entity/mod.rs
pub mod review;
pub mod output;
pub mod blueprint;
pub mod blueprint_item;
pub mod path_lock;
pub mod task;
pub mod task_run;
pub mod task_step;
```

- [ ] **Step 4: 运行测试与基础静态检查**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri project_context_creates_review_and_output_tables -- --exact
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Expected: test PASS，`fmt` 和 `clippy` 通过。

- [ ] **Step 5: 提交命名收敛**

```bash
cd docwise
git add src-tauri/migration/src/m20260413120000_init_project.rs src-tauri/src/app/project
git commit -m "Refactor project domain naming for reviews and outputs"
```

## Task 2: 把 ActiveContext 改成监看焦点模型

**Files:**
- Modify: `docwise/src-tauri/src/app/state.rs`
- Modify: `docwise/src-tauri/src/app/commands/active_context_cmd.rs`
- Modify: `docwise/src-tauri/src/app/commands/workspace.rs`
- Modify: `docwise/app/types/active-context.ts`
- Modify: `docwise/app/composables/useDocwiseActiveContext.ts`
- Add: `docwise/app/lib/focus-context.ts`
- Add: `docwise/app/lib/focus-context.test.ts`
- Test: `docwise/src-tauri/src/app/state.rs`
- Test: `docwise/app/lib/focus-context.test.ts`
- Modify: `docwise/package.json`

- [ ] **Step 1: 写 Rust 与 TS 的失败测试，锁定新焦点字段**

```rust
#[cfg(test)]
mod tests {
    use super::ActiveContext;
    use std::path::Path;

    #[test]
    fn reset_for_workspace_root_clears_focus_fields_but_keeps_workspace() {
        let ctx = ActiveContext::reset_for_workspace_root(Path::new("E:/demo"));
        assert_eq!(ctx.workspace_id, "E:/demo");
        assert_eq!(ctx.project_id, None);
        assert_eq!(ctx.blueprint_id, None);
        assert_eq!(ctx.task_id, None);
        assert_eq!(ctx.review_id, None);
        assert_eq!(ctx.output_id, None);
    }
}
```

```ts
// app/lib/focus-context.test.ts
import { expect, test } from "bun:test";
import { emptyFocusContext, patchFocusContext } from "./focus-context";

test("patchFocusContext clears lower-level focus when project changes", () => {
  const next = patchFocusContext(
    {
      workspaceId: "E:/demo",
      projectId: "p1",
      blueprintId: "b1",
      taskId: "t1",
      reviewId: "r1",
      outputId: "o1",
    },
    { projectId: "p2" },
  );

  expect(next.projectId).toBe("p2");
  expect(next.blueprintId).toBeNull();
  expect(next.taskId).toBeNull();
  expect(next.reviewId).toBeNull();
  expect(next.outputId).toBeNull();
});
```

- [ ] **Step 2: 运行测试，确认现状失败**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri reset_for_workspace_root_clears_focus_fields_but_keeps_workspace -- --exact
cd ..
bun test app/lib/focus-context.test.ts
```

Expected: FAIL，因为当前 `ActiveContext` 仍使用 `filePath / runId / checkpointId`。

- [ ] **Step 3: 实现新的焦点类型与 patch 规则**

```rust
// src-tauri/src/app/state.rs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveContext {
    pub workspace_id: String,
    pub project_id: Option<String>,
    pub blueprint_id: Option<String>,
    pub task_id: Option<String>,
    pub review_id: Option<String>,
    pub output_id: Option<String>,
}
```

```ts
// app/lib/focus-context.ts
import type { ActiveContext } from "~/types/active-context";

export function emptyFocusContext(workspaceId = ""): ActiveContext {
  return {
    workspaceId,
    projectId: null,
    blueprintId: null,
    taskId: null,
    reviewId: null,
    outputId: null,
  };
}

export function patchFocusContext(
  current: ActiveContext,
  patch: Partial<ActiveContext>,
): ActiveContext {
  const next = { ...current, ...patch };
  if (patch.projectId && patch.projectId !== current.projectId) {
    next.blueprintId = null;
    next.taskId = null;
    next.reviewId = null;
    next.outputId = null;
  }
  if (patch.blueprintId && patch.blueprintId !== current.blueprintId) {
    next.taskId = null;
    next.reviewId = null;
    next.outputId = null;
  }
  if (patch.taskId && patch.taskId !== current.taskId) {
    next.reviewId = null;
    next.outputId = null;
  }
  return next;
}
```

```json
// package.json
{
  "scripts": {
    "test": "bun test"
  }
}
```

- [ ] **Step 4: 运行新旧两侧验证**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri reset_for_workspace_root_clears_focus_fields_but_keeps_workspace -- --exact
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cd ..
bun test app/lib/focus-context.test.ts
bun run lint
```

Expected: all PASS。

- [ ] **Step 5: 提交上下文重构**

```bash
cd docwise
git add src-tauri/src/app/state.rs src-tauri/src/app/commands/active_context_cmd.rs src-tauri/src/app/commands/workspace.rs app/types/active-context.ts app/composables/useDocwiseActiveContext.ts app/lib/focus-context.ts app/lib/focus-context.test.ts package.json
git commit -m "Refactor active context around project focus"
```

## Task 3: 建立新的全局壳层与导航结构

**Files:**
- Modify: `docwise/app/layouts/default.vue`
- Modify: `docwise/app/components/AppChromeNav.vue`
- Modify: `docwise/app/pages/index.vue`
- Replace: `docwise/app/pages/workspace/index.vue`
- Add: `docwise/app/components/shell/DocwiseShell.vue`
- Add: `docwise/app/components/shell/DocwiseConversationRail.vue`
- Add: `docwise/app/components/shell/DocwiseFocusHeader.vue`
- Add: `docwise/app/lib/project-tabs.ts`
- Add: `docwise/app/lib/project-tabs.test.ts`
- Test: `docwise/app/lib/project-tabs.test.ts`

- [ ] **Step 1: 写导航配置失败测试，锁定右侧 Tabs**

```ts
// app/lib/project-tabs.test.ts
import { expect, test } from "bun:test";
import { projectTabs } from "./project-tabs";

test("projectTabs returns canonical tab order", () => {
  expect(projectTabs("p-demo")).toEqual([
    { key: "overview", label: "总览", to: "/projects/p-demo?tab=overview" },
    { key: "blueprints", label: "蓝图", to: "/projects/p-demo?tab=blueprints" },
    { key: "tasks", label: "任务", to: "/projects/p-demo?tab=tasks" },
    { key: "reviews", label: "审议", to: "/projects/p-demo?tab=reviews" },
    { key: "outputs", label: "产出", to: "/projects/p-demo?tab=outputs" },
  ]);
});
```

- [ ] **Step 2: 运行测试，确认现状失败**

Run:

```bash
cd docwise
bun test app/lib/project-tabs.test.ts
```

Expected: FAIL，因为 `project-tabs.ts` 尚不存在。

- [ ] **Step 3: 实现壳层纯逻辑与基础布局**

```ts
// app/lib/project-tabs.ts
export function projectTabs(projectId: string) {
  return [
    { key: "overview", label: "总览", to: `/projects/${projectId}?tab=overview` },
    { key: "blueprints", label: "蓝图", to: `/projects/${projectId}?tab=blueprints` },
    { key: "tasks", label: "任务", to: `/projects/${projectId}?tab=tasks` },
    { key: "reviews", label: "审议", to: `/projects/${projectId}?tab=reviews` },
    { key: "outputs", label: "产出", to: `/projects/${projectId}?tab=outputs` },
  ] as const;
}
```

```vue
<!-- app/components/shell/DocwiseShell.vue -->
<template>
  <div class="docwise-shell-grid min-h-[calc(100dvh-3.5rem)] grid lg:grid-cols-[360px_minmax(0,1fr)]">
    <aside class="border-default bg-(--ui-bg-elevated) min-h-0 border-r">
      <DocwiseConversationRail />
    </aside>
    <section class="min-h-0 flex flex-col">
      <DocwiseFocusHeader />
      <div class="min-h-0 flex-1 overflow-auto">
        <slot />
      </div>
    </section>
  </div>
</template>
```

```vue
<!-- app/layouts/default.vue -->
<template>
  <div class="text-default bg-(--ui-bg) min-h-dvh">
    <AppChromeNav />
    <DocwiseShell>
      <slot />
    </DocwiseShell>
  </div>
</template>
```

- [ ] **Step 4: 运行前端静态验证**

Run:

```bash
cd docwise
bun test app/lib/project-tabs.test.ts
bun run lint
bun run build
```

Expected: all PASS。

- [ ] **Step 5: 提交新壳层**

```bash
cd docwise
git add app/layouts/default.vue app/components/AppChromeNav.vue app/components/shell app/pages/index.vue app/pages/workspace/index.vue app/lib/project-tabs.ts app/lib/project-tabs.test.ts
git commit -m "Refactor Docwise shell around global conversation"
```

## Task 4: 落地项目详情 read model 与右侧 Tabs

**Files:**
- Add: `docwise/app/pages/projects/[projectId].vue`
- Add: `docwise/app/components/project/ProjectOverviewPanel.vue`
- Add: `docwise/app/components/project/ProjectBlueprintPanel.vue`
- Add: `docwise/app/components/project/ProjectTaskPanel.vue`
- Add: `docwise/app/components/project/ProjectReviewPanel.vue`
- Add: `docwise/app/components/project/ProjectOutputPanel.vue`
- Modify: `docwise/src-tauri/src/app/commands/overview_cmd.rs`
- Modify: `docwise/src-tauri/src/app/commands/project_cmds.rs`
- Modify: `docwise/src-tauri/src/app/commands/mod.rs`
- Modify: `docwise/src-tauri/src/lib.rs`
- Add: `docwise/app/types/review.ts`
- Add: `docwise/app/types/output.ts`
- Test: `docwise/src-tauri/src/app/commands/overview_cmd.rs`

- [ ] **Step 1: 写后端失败测试，锁定项目详情 read model 需要返回审议与产出计数**

```rust
#[cfg(test)]
mod tests {
    use super::project_overview_counts;

    #[test]
    fn project_overview_counts_includes_reviews_and_outputs() {
        let counts = project_overview_counts(3, 5, 2, 4);
        assert_eq!(counts.review_count, 2);
        assert_eq!(counts.output_count, 4);
    }
}
```

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectOverviewCounts {
    pub blueprint_count: u32,
    pub task_count: u32,
    pub review_count: u32,
    pub output_count: u32,
}

pub fn project_overview_counts(
    blueprint_count: u32,
    task_count: u32,
    review_count: u32,
    output_count: u32,
) -> ProjectOverviewCounts {
    ProjectOverviewCounts {
        blueprint_count,
        task_count,
        review_count,
        output_count,
    }
}
```

- [ ] **Step 2: 运行测试，确认现状失败**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri project_overview_counts_includes_reviews_and_outputs -- --exact
```

Expected: FAIL，因为聚合 helper 和新 DTO 尚不存在。

- [ ] **Step 3: 实现项目详情读模型与前端分页容器**

```ts
// app/pages/projects/[projectId].vue
const route = useRoute();
const projectId = computed(() => route.params.projectId as string);
const activeTab = computed(() => String(route.query.tab ?? "overview"));
```

```vue
<template>
  <div class="docwise-page space-y-6">
    <div class="flex flex-wrap gap-2">
      <UButton
        v-for="tab in tabs"
        :key="tab.key"
        :to="tab.to"
        :variant="activeTab === tab.key ? 'solid' : 'ghost'"
      >
        {{ tab.label }}
      </UButton>
    </div>
    <ProjectOverviewPanel v-if="activeTab === 'overview'" :project-id="projectId" />
    <ProjectBlueprintPanel v-else-if="activeTab === 'blueprints'" :project-id="projectId" />
    <ProjectTaskPanel v-else-if="activeTab === 'tasks'" :project-id="projectId" />
    <ProjectReviewPanel v-else-if="activeTab === 'reviews'" :project-id="projectId" />
    <ProjectOutputPanel v-else :project-id="projectId" />
  </div>
</template>
```

- [ ] **Step 4: 运行整体验证**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri project_overview_counts_includes_reviews_and_outputs -- --exact
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cd ..
bun run lint
bun run build
```

Expected: PASS。

- [ ] **Step 5: 提交 read model 与项目页**

```bash
cd docwise
git add src-tauri/src/app/commands/overview_cmd.rs src-tauri/src/app/commands/project_cmds.rs src-tauri/src/app/commands/mod.rs src-tauri/src/lib.rs app/pages/projects/[projectId].vue app/components/project app/types/review.ts app/types/output.ts
git commit -m "Add project monitoring views for blueprints tasks reviews and outputs"
```

## Task 5: 把智能体流与审议对象串起来

**Files:**
- Replace: `docwise/src-tauri/src/app/checkpoint/mod.rs` -> `docwise/src-tauri/src/app/review/mod.rs`
- Modify: `docwise/src-tauri/src/app/execution/execution_stream.rs`
- Modify: `docwise/src-tauri/src/app/execution/planning.rs`
- Modify: `docwise/src-tauri/src/app/project/tools.rs`
- Modify: `docwise/src-tauri/src/app/commands/execution_cmd.rs`
- Modify: `docwise/src-tauri/src/app/commands/planning_cmd.rs`
- Modify: `docwise/app/composables/useDocwiseAgentStream.ts`
- Replace: `docwise/app/composables/useDocwiseCheckpointEvents.ts` -> `docwise/app/composables/useDocwiseReviewEvents.ts`
- Modify: `docwise/app/types/planning-stream.ts`
- Modify: `docwise/app/types/review.ts`
- Test: `docwise/src-tauri/src/app/project/tools.rs`

- [ ] **Step 1: 写工具定义失败测试，锁定 review tool 名称**

```rust
#[cfg(test)]
mod tests {
    use super::{definitions_for_lmkit, executor_project_tools};
    use crate::app::state::SharedProject;

    #[test]
    fn executor_tool_definitions_use_review_names() {
        let tools = executor_project_tools(SharedProject::default(), None);
        let defs = definitions_for_lmkit(&tools);
        let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();

        assert!(names.contains(&"review_open"));
        assert!(names.contains(&"review_resolve"));
        assert!(!names.contains(&"task_open_checkpoint"));
        assert!(!names.contains(&"task_close_checkpoint"));
    }
}
```

- [ ] **Step 2: 运行测试，确认现状失败**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri executor_tool_definitions_use_review_names -- --exact
```

Expected: FAIL，因为当前工具仍暴露 checkpoint 名称。

- [ ] **Step 3: 实现 review-centered agent loop 接口**

```rust
// project/tools.rs
register_tool("review_open", review_open_tool(ctx.clone()));
register_tool("review_resolve", review_resolve_tool(ctx.clone()));

// execution stream semantics
emit(PlanningStreamEnvelope {
    event: PlanningStreamEvent::tool_finished {
        name: "review_open".to_owned(),
        ok: true,
        tool_call_id,
    },
    ..
});
```

```ts
// useDocwiseReviewEvents.ts
export function useDocwiseReviewEvents(refresh: () => Promise<void>) {
  const lastReviewEvent = ref<ReviewEvent | null>(null);
  // listen("docwise://review-changed", ...)
  return { lastReviewEvent };
}
```

- [ ] **Step 4: 运行 agent-facing 验证**

Run:

```bash
cd docwise/src-tauri
cargo test -p docwise-tauri executor_tool_definitions_use_review_names -- --exact
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cd ..
bun run lint
```

Expected: PASS。

- [ ] **Step 5: 提交执行与审议整合**

```bash
cd docwise
git add src-tauri/src/app/review/mod.rs src-tauri/src/app/execution src-tauri/src/app/project/tools.rs src-tauri/src/app/commands/execution_cmd.rs src-tauri/src/app/commands/planning_cmd.rs app/composables/useDocwiseAgentStream.ts app/composables/useDocwiseReviewEvents.ts app/types/planning-stream.ts app/types/review.ts
git commit -m "Integrate review flow into agent execution"
```

## Task 6: 移除编辑预览主路径并补齐文档与回归

**Files:**
- Modify: `docwise/app/pages/projects/overview.vue`
- Modify: `docwise/app/pages/projects/index.vue`
- Modify: `docwise/app/pages/workspace/index.vue`
- Modify: `docwise/docs/docwise-design.md`
- Modify: `docwise/docs/docwise-ui-agent-first-design.md`
- Modify: `docwise/docs/docwise-ui-shell-design.md`
- Test: `docwise/app/lib/project-tabs.test.ts`
- Test: `docwise/app/lib/focus-context.test.ts`

- [ ] **Step 1: 写一个失败的纯前端测试，锁定不再暴露旧“工作台”入口**

```ts
import { expect, test } from "bun:test";
import { chromeNavItems } from "./chrome-nav";

test("chrome nav no longer exposes workspace as a primary editor route", () => {
  expect(chromeNavItems.map((item) => item.to)).not.toContain("/workspace");
});
```

```ts
// app/lib/chrome-nav.ts
export const chromeNavItems = [
  { to: "/projects", label: "项目" },
  { to: "/projects/overview", label: "总览" },
];
```

- [ ] **Step 2: 运行测试，确认现状失败**

Run:

```bash
cd docwise
bun test app/lib/project-tabs.test.ts app/lib/focus-context.test.ts app/lib/chrome-nav.test.ts
```

Expected: FAIL，因为旧导航仍包含 `/workspace` 作为主要入口。

- [ ] **Step 3: 实现入口清理与文档同步**

```vue
<!-- AppChromeNav.vue -->
<script setup lang="ts">
import { chromeNavItems } from "~/lib/chrome-nav";
</script>
```

```md
<!-- docs/docwise-design.md -->
- v0 不再以内置编辑器/预览器为产品核心
- 右侧监看区以项目、蓝图、任务、审议、产出为主
- 文件阅读与手动编辑由外部工具承担
```

- [ ] **Step 4: 运行最终全量验证**

Run:

```bash
cd docwise
bun test
bun run lint
bun run build
cd src-tauri
cargo test -p docwise-tauri
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Expected: all PASS。

- [ ] **Step 5: 提交 MVP 收口**

```bash
cd docwise
git add app/pages/projects/overview.vue app/pages/projects/index.vue app/pages/workspace/index.vue app/components/AppChromeNav.vue app/lib/chrome-nav.ts app/lib/chrome-nav.test.ts docs/docwise-design.md docs/docwise-ui-agent-first-design.md docs/docwise-ui-shell-design.md
git commit -m "Finalize agent-first MVP product shell for Docwise"
```

## 计划自检

### 覆盖检查

- spec 的领域术语收敛：Task 1、Task 2、Task 5
- 左侧全局对话 + 右侧监看壳层：Task 3、Task 4
- 项目详情 Tabs：Task 4
- 移除内置编辑与预览主路径：Task 3、Task 6
- “真实落盘而不依赖预览”的 MVP：Task 4、Task 5、Task 6

### 占位检查

- 没有使用 `TBD` / `TODO` / “后续补上” 作为计划步骤
- 每个任务都给了实际文件路径、最小测试代码、验证命令和提交动作

### 类型一致性检查

- 计划统一使用 `Project / Blueprint / Task / Review / Output`
- `blocked` 只作为任务状态
- 不再把 `Checkpoint` / `Artifact` 作为 canonical 业务对象
