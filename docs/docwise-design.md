# Docwise 设计方案

| 属性 | 说明 |
|------|------|
| 状态 | 定稿 |
| 文档类型 | 产品与应用架构设计 |
| 关联文档 | [`mdweave-design.md`](./mdweave-design.md)：文档预览与导出工具链 |
| 底层依赖 | `src-tauri/crates/agent-tools`（工具集）、`src-tauri/crates/lmkit-rs`（模型调用）；任务模型在 Docwise 内部规划，成熟后抽象回 agentool；`project.db` 通过 `sea-orm` 访问，迁移用 `sea-orm-migration` |

---

## Docwise 是什么

Docwise 是一个桌面应用，帮助用户建立和维护 Markdown 文档体系。它的核心思路是：**用户定义目标，智能体负责执行**。

用户不需要逐字逐句地写文档，而是先描述"我需要哪些文档、每篇文档要达到什么目标"，然后让智能体自主规划、拆解任务、撰写内容、持续更新。用户的主要工作是管理这个过程——设定优先级、查看进度、在关键节点做决策。

**用户扮演两个角色：**
- 架构师：定义文档集的结构、目标读者、质量要求、写作约束
- 管理者：设定优先级、查看进度、处理风险、在检查点做方向决策

**v1 的核心能力：**
- 用户可以建立文档蓝图，定义文档集包含哪些文档及其要求
- 智能体基于蓝图自主拆解并执行写作任务，任务结构为树状
- 用户通过统一对话框与智能体团队协作，实时看到执行过程
- 预览与导出由 `mdweave` 统一处理

---

## 场景一：用户通过对话建立文档蓝图

### 这个场景做什么

用户在对话框中描述需求，规划对话智能体理解需求后，通过调用 project 工具集中的蓝图工具将需求结构化为蓝图。整个过程是一个来回沟通的对话，蓝图在对话中逐步成形，而不是用户填写表单。

### 涉及的概念

**Workspace（工作区）**
用户在本地文件系统中选择的一个目录，所有文档文件和 Docwise 的数据库都存放在这里。Docwise 的数据存在工作区下的 `.agent/` 目录中，文档文件直接存在工作区根目录或用户指定的子目录中。

**Blueprint（蓝图）**
描述"需要建立什么样的文档集"的结构化对象。一个蓝图包含：
- 文档集的整体目标
- 目标读者
- 文档清单（每篇文档对应一个 `BlueprintItem`）
- 风格、术语、引用与格式约束

蓝图是智能体的行动边界——智能体只能在蓝图批准的范围内写文件，超出范围必须先修改蓝图。

**BlueprintItem（蓝图条目）**
蓝图中的一篇具体文档的要求，包含：
- 目标文件路径（相对工作区根）
- 这篇文档面向谁
- 这篇文档要达到什么目标
- 必须覆盖的主题列表
- 禁止偏离的边界列表

### 流程

蓝图有两种修改入口，最终效果一致：

**入口一：对话框驱动（主要方式）**

```
用户在对话框中描述需求
  │
  ├─ 规划对话智能体理解需求
  │   ├─ 调用 blueprint_create 创建草稿
  │   └─ 调用 blueprint_item_add 逐条添加文档要求
  │
  ├─ 智能体在对话框中展示蓝图摘要，询问是否有遗漏或需要调整
  │
  ├─ 用户反馈（可多轮）
  │   └─ 智能体调用 blueprint_update / blueprint_item_update 修改蓝图
  │
  └─ 用户确认蓝图内容
      └─ 智能体调用 blueprint_set_status(approve)
         蓝图进入 approved 状态，等待任务生成
```

**入口二：蓝图视图直接编辑**

用户也可以在蓝图视图中直接修改蓝图内容（编辑条目、调整顺序、修改约束等）。编辑保存后，系统自动向对话框发送一条系统消息：

```
[系统] 用户修改了蓝图：更新了 deployment.md 的约束条件，请注意查看。
```

规划对话智能体收到通知后，可以主动确认变更是否影响已有任务，或询问用户是否需要重新规划。

若蓝图处于 `active` 状态（任务已在执行中），直接编辑会将蓝图状态置为 `revised`，系统消息中会额外提示：

```
[系统] 用户修改了蓝图（当前有任务正在执行），蓝图已进入 revised 状态，请确认是否需要调整任务计划。
```

典型对话示例：

```
用户：我需要一套面向工程团队的产品文档，包括架构说明、部署指南和 FAQ

规划智能体：好的，我来整理一下。你们的部署环境是什么？K8s 还是裸机？

用户：K8s，用 Helm Chart

规划智能体：明白了。我规划了以下文档：
  - architecture.md：系统架构说明，面向工程团队
  - deployment.md：K8s + Helm Chart 部署指南，含回滚和故障排查
  - faq.md：常见问题，面向工程团队和运维
  有需要调整的吗？

用户：deployment.md 还需要包含环境变量说明

规划智能体：已更新，deployment.md 新增"环境变量"章节要求。确认后我就开始拆解任务。

用户：确认

规划智能体：蓝图已批准，开始生成任务...
```

### 蓝图状态机

```
draft ──approve──→ approved ──开始生成任务──→ active
  │                                              │
  └──supersede──→ superseded         管理者调整──→ revised ──re-approve──→ approved
                                                 │
                                        项目结束──→ archived
```

| 状态 | 含义 |
|------|------|
| `draft` | 草稿，智能体正在与用户沟通完善中 |
| `approved` | 已批准，等待任务生成 |
| `active` | 任务已生成，正在执行 |
| `revised` | 执行中用户调整了蓝图，需要重新批准 |
| `superseded` | 被新蓝图替代 |
| `archived` | 项目结束，归档 |

### 数据存储

蓝图存储在工作区 `.agent/project.db`，由 `src-tauri/src/app/project/` 模块维护。该模块同时实现 project 工具集中的蓝图工具，注册给规划对话智能体调用（详见"Docwise 内置工具集"章节）。`agentool` 不包含蓝图表。

```sql
CREATE TABLE IF NOT EXISTS blueprints (
    id          TEXT PRIMARY KEY,
    title       TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'draft',
    goal        TEXT NOT NULL DEFAULT '',
    audience    TEXT NOT NULL DEFAULT '',
    style_guide TEXT NOT NULL DEFAULT '',   -- JSON：风格、术语、引用、格式约束
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS blueprint_items (
    id           TEXT PRIMARY KEY,
    blueprint_id TEXT NOT NULL REFERENCES blueprints(id) ON DELETE CASCADE,
    seq          INTEGER NOT NULL,
    file_path    TEXT NOT NULL,             -- 相对工作区根的目标文件路径
    title        TEXT NOT NULL,
    audience     TEXT NOT NULL DEFAULT '',
    goal         TEXT NOT NULL DEFAULT '',
    must_cover   TEXT NOT NULL DEFAULT '[]',    -- JSON 数组
    constraints  TEXT NOT NULL DEFAULT '[]',    -- JSON 数组
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL
);
```

---

## 场景二：智能体执行写作任务

### 这个场景做什么

蓝图批准后，系统将蓝图拆解为一组任务，智能体按优先级顺序执行这些任务，将内容写入工作区的 Markdown 文件。

### 涉及的概念

**Task（任务）**
智能体需要完成的一项具体工作。任务结构为**树状**：一个父任务可以拆解为多个子任务，子任务可以继续拆解。兄弟任务可以并行执行，互不阻塞。任务有明确的目标（`goal`）和验收标准（`acceptance`）。

> 任务存储在 Docwise `project/` 模块内实现，支持 `parent_id` 树状结构和 `blueprint_item_id` 关联。详见"项目模型说明"章节。

**TaskStep（任务步骤）**
任务内部的线性执行步骤，例如"收集资料 → 起草大纲 → 撰写正文 → 校对"。v1 不支持步骤间的依赖图，只支持顺序执行。

**TaskRun（执行实例）**
任务的一次具体执行记录，包含开始时间、结束时间、执行状态、错误信息和摘要。一个任务可以有多次 run（例如失败后重试）。

**Snapshot（文件历史快照）**
`agentool::fs` 在每次写操作（`file_write`、`file_edit`、`file_delete`、`file_move`、`file_copy`）执行前自动创建的不可变快照，按文件内容 SHA-256 去重存储。智能体可通过 `fs.snapshot_get` / `fs.snapshot_list` 查阅历史内容或在出错时恢复，无需手动触发。

**PathLock（路径写锁）**
任务执行时对目标文件加的写锁，防止两个任务同时写同一个文件。v1 规定同一文件在任意时刻最多只有一个处于 `running` 状态的写任务。

**Artifact（产出物）**
任务执行完成后记录的产出，可以是文件路径、摘要文本或引用资料。

**FileBuffer（编辑缓冲区）**
用户在编辑器中打开文件后，未保存的修改内容。这是纯内存态对象，不持久化。当用户正在编辑某个文件时，该文件上的自治写任务不得抢占写入，任务进入 `blocked` 状态等待用户保存或放弃修改。

### 执行循环

智能体执行一个任务的完整过程：

```
1. 系统检查前置条件
   ├── 目标文件是否有 dirty FileBuffer？→ 有则任务 blocked，等待
   ├── 目标文件是否已被其他任务锁定？→ 有则任务 blocked，等待
   └── 条件满足 → 继续

2. 任务开始执行（创建 TaskRun，状态 running）
   └── 对目标文件加 PathLock

3. 智能体执行循环（agent loop）
   ├── 构建 ChatRequest（系统提示 + 任务目标 + 工具列表）
   ├── 调用 lmkit-rs ChatProvider::complete_stream
   ├── 处理 ChatEvent 流
   │   ├── Delta → 累积文本
   │   ├── ToolCallDelta → 合并为 ToolCall（merge_tool_call_deltas）
   │   └── Finish → 判断结束原因
   ├── 执行工具调用（agentool 工具集）
   │   ├── 写文件（`file_write`/`file_edit` 等，写前自动快照）
   │   ├── 读文件、搜索（find/fs 工具）
   │   └── 任务状态更新（task 工具）
   └── 循环直到 finish_reason = Stop 且无待处理工具调用

4. 任务完成
   ├── 释放 PathLock
   ├── 记录 Artifact（产出文件路径 + 摘要）
   ├── 结束 TaskRun（状态 done）
   └── 更新 Task 状态为 done
```

### 任务状态机

```
backlog ──调度──→ ready ──开始执行──→ running
                                        │
                          ┌─────────────┼─────────────┐
                          ↓             ↓             ↓
                      done          failed      waiting_checkpoint
                                      │                │
                                   重试→ready    用户处理→running
                                        │
                              blocked（冲突/等待）
                                        │
                                  解除→恢复前状态
```

| 状态 | 含义 |
|------|------|
| `backlog` | 已创建，尚未进入执行队列 |
| `ready` | 满足执行条件，等待执行 |
| `running` | 当前正在执行 |
| `waiting_checkpoint` | 等待用户处理检查点 |
| `blocked` | 等待用户输入、dirty buffer 释放或冲突解决 |
| `failed` | 执行失败，可重试 |
| `done` | 任务完成 |
| `cancelled` | 任务被取消 |

### 崩溃恢复

应用启动时，`project/` 模块的 `ProjectContext::new` 会自动执行恢复：
- 遗留 `running` 状态的任务 → 改为 `blocked`，`blocked_reason = process_restart`
- 遗留 `running` 状态的 run → 改为 `failed`，`error = process_restart`
- 删除已过期的 `path_locks` 行

### 数据存储

任务数据存储在工作区 `.agent/project.db`，由 Docwise `src-tauri/src/app/project/` 模块维护。该模块同时实现 project 工具集中的任务工具，注册给规划对话智能体和文档执行智能体调用（详见"Docwise 内置工具集"章节）。

核心表结构（完整迁移见 `src-tauri/src/app/project/migration/`）：

| 表 | 用途 |
|----|------|
| `blueprints` | 蓝图主行：目标、受众、风格约束、状态等 |
| `blueprint_items` | 蓝图条目：目标文件路径、要求、约束 |
| `tasks` | 任务主行：目标、状态、优先级、`parent_id`、`blueprint_item_id` 等 |
| `task_steps` | 线性步骤（`seq` + `title` + `status`） |
| `task_runs` | 执行实例：开始/结束时间、状态、摘要、错误 |
| `path_locks` | 路径写锁：`path` UNIQUE，可选 `expires_at` |
| `checkpoints` | 检查点（见场景三） |
| `artifacts` | 产出物：类型、路径、可选内容 |

`tasks.blueprint_item_id` 外键引用 `blueprint_items(id)`，通过 join 可推导出所属蓝图。`parent_id IS NULL` 的根任务直接对应一个蓝图条目；子任务通过树状结构隐式归属。

Snapshot 存储在独立的 `.agent/snapshots.db`（元数据）和 `.agent/snapshots/<sha256>.md`（内容文件），由 `agentool::fs` 在每次写操作（`file_write`、`file_edit`、`file_delete`、`file_move`、`file_copy`）执行前自动创建，无需智能体显式调用：

```sql
CREATE TABLE IF NOT EXISTS snapshots (
    id          TEXT PRIMARY KEY,   -- SHA-256 of content
    task_id     TEXT,
    run_id      TEXT,
    file_path   TEXT NOT NULL,
    content_sha TEXT NOT NULL,
    created_at  TEXT NOT NULL
);
```

### 可用工具集

`agentool` 提供多个通用工具集，各工具集通过独立的 Context 初始化，按需组合注册给不同角色的智能体。完整的工具权限分配见"智能体角色与工具权限"章节。

**fs**（`feature = "fs"`）**——文件操作**

`file_read`、`file_write`、`file_edit`（精确替换）、`directory_create`、`directory_list`、`file_delete`、`file_move`、`file_copy`、`snapshot_get`、`snapshot_list`（后两者待实现）

**find**（`feature = "find"`）**——搜索**

| 工具 | 说明 |
|------|------|
| `grep_search` | 正则搜索文件内容，支持 glob 过滤和大小写忽略 |
| `glob_search` | 按 glob 模式列出匹配文件路径 |

**web**（`feature = "web"`）**——网络**

| 工具 | 说明 |
|------|------|
| `web_search` | 网页搜索，返回标题、URL、摘要（默认 DuckDuckGo，可替换后端） |
| `web_fetch` | 抓取网页并转换为 Markdown 文本 |

**git**（`feature = "git"`）**——版本管理**

| 工具 | 说明 |
|------|------|
| `git_status` | 列出工作区变更（新增/修改/删除/未跟踪） |
| `git_diff` | 查看未暂存或已暂存的 diff |
| `git_commit` | 暂存并提交（可指定文件列表或全量提交） |
| `git_log` | 查看最近提交历史 |
| `worktree_add` | 创建新的关联 worktree，分支不存在时从 HEAD 创建 |
| `worktree_list` | 列出所有关联 worktree 及其路径和锁定状态 |
| `worktree_remove` | 删除关联 worktree，`force: true` 可强制删除已锁定的 |
| `worktree_lock` | 锁定 worktree，防止意外删除 |
| `worktree_unlock` | 解锁已锁定的 worktree |

**md**（`feature = "md"`）**——Markdown 分析**

| 工具 | 说明 |
|------|------|
| `extract_toc` | 提取文档标题大纲（ATX 风格，忽略代码块内的标题） |
| `markdown_stats` | 统计字符数、段落数、标题数、行数（排除代码块） |

**memory**（`feature = "memory"`）**——跨任务记忆**

存储路径：`.agent/memory/`（按日期分文件）和 `.agent/memory/MEMORY.md`（汇总）

| 工具 | 说明 |
|------|------|
| `memory_write` | 写入新记忆块（key 全局唯一） |
| `memory_update` | 更新已有记忆块 |
| `memory_read` | 读取指定 key 的记忆块 |
| `memory_search` | 按关键词/标签搜索记忆 |

典型用途：蓝图规划智能体将用户偏好的写作风格、确认的术语表写入 memory；文档执行智能体在开始新任务前先搜索相关背景；检查点决策结果写入 memory 供后续任务参考。

**todo**（`feature = "todo"`）**——轻量待办**

存储路径：`.agent/todos.json`

`todo_add`、`todo_list`、`todo_update`、`todo_remove`

典型用途：子智能体拆分子任务时记录待办项，或向主智能体汇报当前进度。**不**替代 `task` 作为调度主存储。

---

## 场景三：智能体协作与用户介入

### 这个场景做什么

智能体在执行过程中需要与其他智能体沟通、向用户提问、或请求方向性决策。所有这些交互都发生在统一对话框中，用户可以实时看到智能体团队的协作过程。

### 涉及的概念

**统一对话框**
所有智能体和用户共享的消息空间。规划对话智能体的消息、执行智能体的工作汇报、智能体之间的协作沟通，都在同一个消息流里。用户可以随时发言，也可以 @ 特定智能体。

**消息路由层**
对话框背后的轻量分类层，不是独立智能体。负责判断用户消息应该路由给哪个智能体：
- 有 @ → 判断是否符合目标智能体职责，不符合则静默路由到正确智能体
- 无 @ → 默认给规划对话智能体
- 判断不确定 → 兜底给规划对话智能体
- v1 用规则判断，v2 升级为模型分类

**Checkpoint（检查点）**
某个任务节点上的"等待答复"标记。执行智能体在对话框中 @ 用户时，若同时标记"需要答复才能继续"，对应任务节点自动置为 `waiting_checkpoint`。用户在对话框回复后，节点自动解除等待继续执行。

树状任务结构下，检查点只暂停被标记的节点，兄弟节点和其他分支照常运行。用户不需要感知"检查点"这个概念——它就是对话框里一条带等待标记的消息。

**智能体时间轴**
每个执行智能体的个人页面，以时间轴形式展示该智能体的具体动作序列：工具调用、文件读写、git 提交等。对话框展示"为什么"和"谁说了什么"，时间轴展示"做了什么"。

### 典型协作流程

```
规划智能体：蓝图已拆解为 5 个任务
            @执行智能体A 请处理 architecture.md，目标是面向工程团队的架构说明

执行智能体A：收到，我看了现有文件结构
            @规划智能体 这篇文档需要包含部署章节吗？

规划智能体：需要
            @用户 你们的部署环境是 K8s 还是裸机？[等待答复 → architecture.md 节点 waiting_checkpoint]

用户：K8s，用 Helm Chart 部署

规划智能体：@执行智能体A 包含 K8s + Helm Chart 部署章节
            [architecture.md 节点恢复 running]

执行智能体A：明白，开始写作
            [时间轴：file_write（自动快照）→ git_commit]
```

### 触发检查点的条件

执行智能体在以下情况下应在对话框中 @ 用户并标记"需要答复才能继续"：
- 任务目标与蓝图约束冲突，需要方向决策
- 缺少关键信息无法继续（如用户特定的技术细节）
- 执行持续失败，需要人工介入
- 完成阶段性成果，需要确认是否继续下一阶段

以下情况可以 @ 规划智能体而不阻塞任务：
- 需要澄清蓝图中某个条目的要求
- 发现蓝图范围可能需要调整

### 检查点状态机

检查点状态仍存储在 `project.db` 的 `checkpoints` 表，但触发和关闭都通过对话框消息驱动：

```
open（智能体 @ 用户，标记等待）
  │
  ├── 用户在对话框回复 → resolved → closed（任务节点恢复 running）
  └── 用户选择暂停/取消 → closed（任务节点置为 blocked/cancelled）
```

新增字段 `conversation_ref`：记录触发该检查点的对话消息 ID，用于从任务视图跳转到对应对话上下文。

---

## 场景四：预览文档

### 这个场景做什么

用户在编辑器中打开一个 Markdown 文件，或者智能体完成写作后，系统将文件内容渲染为 HTML 展示在预览区。

### 涉及的概念

**FileBuffer（编辑缓冲区）**
用户在编辑器中对文件的未保存修改。预览区可以实时跟随 FileBuffer 内容更新（编辑时预览），也可以基于已保存的文件内容渲染。

**PreviewBackend（预览后端）**
Docwise 定义的 trait，抽象了"将 Markdown 渲染为 HTML"这个操作。v1 有两个实现：
- `ComrakStubBackend`：直接用 `comrak` 渲染，不依赖 `mdweave`，用于 v1 开发阶段
- `MdweaveBackend`：接入真实 `mdweave` crate，v1 后期或 v2 替换

两个实现的返回类型完全相同，切换时只改注册代码，不改调用方。

### 流程

```
1. 用户打开预览，或编辑器内容变化触发预览刷新
2. 系统确定渲染基线：
   ├── 编辑时预览 → 使用当前 FileBuffer 内容
   └── 任务产出预览 → 使用对应 Snapshot 内容
3. 调用 PreviewBackend::render
4. 返回 RenderPreviewResult，前端展示 HTML
```

### 接口定义

```rust
// Tauri command 返回给前端的结构
pub struct RenderPreviewResult {
    pub snapshot_id: String,   // 对应的 Snapshot id（编辑时预览填临时 id）
    pub html: String,
    pub diagnostics: Vec<Diagnostic>,
    pub asset_base_url: String,
    pub theme_revision: String,
}

pub struct Diagnostic {
    pub level: DiagnosticLevel,  // Warning | Error
    pub message: String,
}
```

`ComrakStubBackend` 中：`diagnostics` 为空数组，`theme_revision` 填 `"stub-v1"`。

### v1 导出

`export_docx` 在 v1 阶段返回 `not_implemented` 错误，不影响核心流程。导出功能等待 `mdweave` crate 就绪后接入。

---

## 模块地图

`src-tauri/src/app/` 下各模块的职责与依赖：

| 模块 | 职责 | 读写 | 依赖 |
|------|------|------|------|
| `workspace/` | 打开工作区、读写文件、管理 FileBuffer | 工作区文件系统 | — |
| `project/` | 蓝图与任务 CRUD、状态机；实现并注册 project 工具集供两个智能体调用 | `.agent/project.db` | `sea-orm` |
| `execution/` | 执行循环：调用模型、分发工具调用、触发检查点 | — | `lmkit-rs`、`agentool`、`project/` |
| `checkpoint/` | 检查点状态机、前端通知 | — | `project/` |
| `preview/` | 调用 PreviewBackend 渲染 HTML | 工作区文件（只读） | `comrak`（v1）、`mdweave`（v2）、`agentool::fs` |
| `state/` | 管理 `ActiveContext`（当前工作区/文件/蓝图/任务上下文） | 内存态 | — |

### ActiveContext

主工作区、侧栏、看板、检查点共享同一组上下文字段，通过 Tauri 状态管理或事件广播同步到前端：

```typescript
type ActiveContext = {
  workspaceId: string
  filePath: string | null
  blueprintId: string | null
  taskId: string | null
  runId: string | null
  checkpointId: string | null
}
```

- 主工作区跟随 `filePath`
- 侧栏对话默认绑定当前 `blueprintId` 或 `taskId`
- 看板点击可切换 `taskId`，并在必要时联动 `filePath`
- 进入检查点时同时设置 `checkpointId` 与关联的 `taskId`、`blueprintId`

### 前端模块

`src/modules/` 下各模块对应界面区域：

| 模块 | 界面区域 | 主要功能 |
|------|----------|----------|
| `project/` | 蓝图视图 | 规划视角：定义文档集结构、管理蓝图条目与约束、查看蓝图状态；不展示任务执行细节 |
| `board/` | 任务看板 | 执行视角：以树状/列表形式展示任务运行状态、进度、风险；不涉及蓝图编辑 |
| `editor/` | 主工作区 | 浏览文件、手工修订 |
| `preview/` | 预览容器 | 展示渲染后的 HTML |
| `chat/` | 统一对话框 | 用户与智能体团队的协作空间，支持 @ 路由 |
| `timeline/` | 智能体时间轴 | 单个执行智能体的动作序列（工具调用、文件读写等） |

### 目录结构

```
docwise/
├── src/                          # 前端（Tauri WebView）
│   └── modules/
│       ├── project/              # 蓝图视图（规划视角）
│       ├── board/                # 任务看板（执行视角）
│       ├── editor/               # 主工作区
│       ├── preview/              # 预览容器
│       ├── chat/                 # 统一对话框
│       └── timeline/             # 智能体时间轴
└── src-tauri/
    ├── src/
    │   └── app/
    │       ├── workspace/        # 工作区管理、FileBuffer
    │       ├── project/          # 蓝图与任务（sea-orm 实体 + 工具集）
    │       │   ├── entity/       # sea-orm 生成的实体（blueprints、tasks 等）
    │       │   ├── migration/    # sea-orm-migration 迁移文件
    │       │   ├── context.rs    # ProjectContext（初始化、崩溃恢复）
    │       │   └── tools.rs      # project 工具集注册
    │       ├── execution/        # 执行循环（agent loop）
    │       ├── checkpoint/       # 检查点状态机与前端通知
    │       ├── preview/          # PreviewBackend trait 与实现
    │       └── state/            # ActiveContext 管理
    └── crates/
        ├── agent-tools/          # agentool 工具库（fs/find/web/git/md/memory/todo）
        └── lmkit-rs/             # 多厂商 LLM 客户端
```

---

## 上游库使用说明

### lmkit-rs

提供多厂商模型调用能力。Docwise 使用的核心接口：

```rust
// 创建 ChatProvider
let provider = lmkit_rs::chat::create(&ProviderConfig {
    provider: Provider::Anthropic,
    api_key: "...",
    base_url: "...",
    model: "claude-opus-4-6",
    timeout: Some(Duration::from_secs(120)),
    max_concurrent: Some(2),  // 提示值，Docwise 应用层自行限流
    ..
})?;

// 流式调用
let stream = provider.complete_stream(&ChatRequest {
    messages: vec![...],
    tools: Some(vec![...]),
    preset: Some(RequestPreset::Execution),  // 低 temperature，追求确定性
    ..
}).await?;

// 处理流式事件
while let Some(event) = stream.next().await {
    match event? {
        ChatEvent::Delta(text) => { /* 累积文本 */ }
        ChatEvent::ToolCallDelta(deltas) => { /* 累积工具调用增量 */ }
        ChatEvent::Finish(reason) => { /* 判断结束原因 */ }
    }
}

// 合并工具调用增量
let tool_calls = merge_tool_call_deltas(&accumulated_deltas);
```

**Docwise 应用层负责**（lmkit-rs 不提供）：
- 取消令牌（中断流式请求）
- 按 `max_concurrent` 提示做宿主侧限流
- 业务级重试策略
- 检查点升级策略

### agentool

提供任务存储和多个工具集，按需启用 Cargo feature 并组合注册给不同角色的智能体。

```rust
// 初始化各 Context（应用启动时）
let fs_ctx   = Arc::new(FsContext::new(workspace_root.clone(), false));
let find_ctx = Arc::new(FindContext::new(workspace_root.clone()));
let web_ctx  = Arc::new(WebContext::default());
let git_ctx  = Arc::new(GitContext::new(workspace_root.clone()));
let md_ctx   = Arc::new(MdContext::new(workspace_root.clone(), false));
let mem_ctx  = Arc::new(MemoryContext::new(workspace_root.clone()));
let todo_ctx = Arc::new(TodoContext::new(workspace_root.clone()));

// 按角色组合工具集（示例：文档执行智能体）
let tools: Vec<Arc<dyn Tool>> = [
    agentool::fs::all_tools(fs_ctx.clone()),
    agentool::find::all_tools(find_ctx.clone()),
    agentool::web::all_tools(web_ctx.clone()),
    agentool::git::all_tools(git_ctx.clone()),
    agentool::md::all_tools(md_ctx.clone()),
    agentool::memory::all_tools(mem_ctx.clone()),
].concat();
```

各工具集的详细说明见场景二"可用工具集"。工具权限分配策略见"智能体角色与工具权限"章节。

---

## 智能体角色与工具权限

Docwise v1 有两个智能体角色。每个角色只拿到完成自身职责所需的工具子集。

### 规划对话智能体

**职责**：用户的唯一交互入口。负责理解用户需求、生成和调整蓝图、将蓝图拆解为任务、在对话框中分配任务给执行智能体、回答用户关于进度和风险的问题、接收并路由用户对执行智能体的 @ 消息。

这个角色由原来的"对话智能体"和"蓝图规划智能体"合并而来——两者的职责本质上是同一个连续对话过程，拆开反而增加不必要的切换。

| 工具集 | 工具 |
|--------|------|
| project（Docwise 内置，蓝图） | `blueprint_create`、`blueprint_get`、`blueprint_list`、`blueprint_update`、`blueprint_set_status`、`blueprint_item_add`、`blueprint_item_update`、`blueprint_item_remove` |
| project（Docwise 内置，任务） | `task_create`、`task_list`、`task_get`、`task_get_tree`、`task_update` |
| fs | `file_read`、`directory_list` |
| web | `web_search`、`web_fetch` |
| md | `extract_toc`、`markdown_stats` |
| memory | 全部 |

不给：`file_write`（规划阶段不写正文）、`task_acquire_lock`、执行控制类工具。

### 文档执行智能体

**职责**：按任务目标撰写文档，写入工作区文件，记录产出物。在对话框中汇报进度、向规划智能体或用户提问。遇到需要用户决策的情况时，在对话框中 @ 用户并标记"需要答复才能继续"，触发对应任务节点的检查点。

可以有多个执行智能体实例并行运行，各自处理不同的任务节点。

| 工具集 | 工具 |
|--------|------|
| preview（Docwise 内置，v1 可选） | `preview_render` |
| project（Docwise 内置，任务） | `task_get`、`task_update`、`task_start_run`、`task_end_run`、`task_append_step`、`task_update_step`、`task_open_checkpoint`、`task_close_checkpoint`、`task_acquire_lock`、`task_release_lock`、`task_add_artifact` |
| fs | 全部 |
| find | `grep_search`、`glob_search` |
| web | `web_search`、`web_fetch` |
| git | `git_status`、`git_diff`、`git_commit`、`git_log` |
| md | `extract_toc`、`markdown_stats` |
| memory | 全部 |
| todo | `todo_add`、`todo_list`、`todo_update` |

不给：`task_create`（任务由规划智能体创建）、`task_delete`、蓝图工具。

### Docwise 内置工具集

这两套工具在 `src-tauri/src/app/project/` 内实现，不进 `agentool`：

**project 工具集**

蓝图工具（规划对话智能体专用）：

| 工具 | 说明 |
|------|------|
| `blueprint_create` | 创建蓝图草稿 |
| `blueprint_get` | 获取蓝图详情（含所有条目） |
| `blueprint_list` | 列出蓝图，支持按状态过滤 |
| `blueprint_update` | 更新蓝图字段（目标、受众、风格约束等） |
| `blueprint_set_status` | 状态转移（approve / archive / supersede） |
| `blueprint_item_add` | 添加蓝图条目（一篇文档的要求） |
| `blueprint_item_update` | 更新条目字段 |
| `blueprint_item_remove` | 删除条目 |

任务工具（两个智能体按权限使用）：

| 工具 | 说明 |
|------|------|
| `task_create` | 创建任务，支持 `parent_id` 和 `blueprint_item_id` 参数，默认状态 `backlog` |
| `task_list` | 列出任务，支持按状态/标签/`parent_id`/`blueprint_item_id` 过滤 |
| `task_get` | 获取单个任务详情 |
| `task_get_tree` | 获取以某任务为根的完整子树（含状态汇总） |
| `task_update` | 更新任务字段（状态、优先级、目标等） |
| `task_delete` | 删除任务 |
| `task_start_run` | 开始一次执行实例（TaskRun） |
| `task_end_run` | 结束执行实例，记录状态和摘要 |
| `task_append_step` | 追加线性步骤 |
| `task_update_step` | 更新步骤状态 |
| `task_open_checkpoint` | 开启检查点，任务进入 `waiting_checkpoint` |
| `task_close_checkpoint` | 关闭检查点 |
| `task_acquire_lock` | 对文件路径加写锁 |
| `task_release_lock` | 释放写锁 |
| `task_add_artifact` | 记录产出物（`kind`：`file`、`summary`、`report`、`reference` 等） |

**preview 工具集**（v1 可选）

| 工具 | 说明 |
|------|------|
| `preview_render` | 渲染指定文件为 HTML，返回诊断信息；执行智能体完成写作后可主动调用自检 |

### 工具权限汇总

| 工具集 | 规划对话智能体 | 文档执行智能体 |
|--------|---------------|---------------|
| project（内置，蓝图） | 全部 | — |
| project（内置，任务读） | `task_list`、`task_get`、`task_get_tree` | `task_get`、`task_get_tree` |
| project（内置，任务写） | `task_create`、`task_update`、`task_delete` | `task_update`、run/step/checkpoint/lock/artifact |
| preview（内置，v1 可选） | — | `preview_render` |
| fs（读） | `file_read`、`directory_list` | 全部 |
| fs（写） | — | `file_write`、`file_edit`、`directory_create`、`file_delete`、`file_move`、`file_copy` |
| fs（快照） | — | `snapshot_get`、`snapshot_list` |
| find | — | `grep_search`、`glob_search` |
| web | 全部 | 全部 |
| git | — | 全部 |
| md | 全部 | 全部 |
| memory | 全部 | 全部 |
| todo | — | `todo_add`、`todo_list`、`todo_update` |

---

## 项目模型说明

project 工具集在 Docwise `src-tauri/src/app/project/` 内实现，不依赖 `agentool`。蓝图与任务共用同一个 `.agent/project.db`，外键约束在库内完整生效。

| 字段 | 位置 | 说明 |
|------|------|------|
| `tasks.blueprint_item_id` | `tasks` 表 | 关联蓝图条目，根任务（`parent_id IS NULL`）直接对应一个条目；子任务通过树状结构隐式归属 |
| `tasks.parent_id` | `tasks` 表 | 支持树状结构，`NULL` 表示根任务；级联删除子任务 |
| `tasks.conversation_ref` | `tasks` 表 | 关联对话框消息 ID，从任务视图可跳转到对话上下文 |
| `checkpoints.conversation_ref` | `checkpoints` 表 | 关联触发该检查点的对话消息，用户回复后自动关闭检查点 |

若未来 `agentool` 补充树状结构和对话引用支持，可将 `project/` 中的任务部分迁移回 `agentool::task`，条件：
- 树状结构在 Docwise 中稳定运行
- `conversation_ref` 语义足够通用（不绑定 Docwise 特有概念）
- `parent_id` 的级联删除和状态传播规则经过验证

## v1 边界与演进方向

**v1 明确不做：**
- 多人实时协作
- 多任务并发写同一文件
- 复杂任务依赖图
- 自动 merge
- PDF 导出
- 蓝图模板库

**v1 完成后，以下条件稳定才考虑演进：**
- `Blueprint` 真相来源稳定
- `Checkpoint` 治理闭环稳定
- `project/` 模块表结构与启动恢复语义稳定
- `mdweave` 预览/导出闭环稳定

---

## 修订记录

| 日期 | 说明 |
|------|------|
| 2026-04-09 | 初版定稿：Blueprint + Task + Checkpoint 模型；对齐 agentool 与 lmkit-rs 实际 API；按场景主线重组，补充概念解释与流程串联 |
| 2026-04-09 | 补充：全部工具集说明、智能体角色与工具权限分配表、git worktree 缺口说明；Docwise 内置工具集（blueprint/preview）；任务模型规划（树状结构、conversation_ref、抽象回 agentool 的条件） |
| 2026-04-09 | 重构：对话+规划智能体合并为规划对话智能体；检查点改为对话框驱动；加入统一对话框与消息路由设计；蓝图由对话驱动生成；蓝图支持用户直接编辑并触发系统通知 |
| 2026-04-13 | 修正：blueprint 工具集明确由 `blueprint/` 模块实现并注册给智能体；快照能力下沉至 `agentool::fs`（写操作前自动触发），snapshot 工具集移除，读取接口（`snapshot_get`/`snapshot_list`）归入 fs 工具集；task 工具集从 `agentool` 移出，改为 Docwise 内置（`tasks/` 模块），补充树状结构（`parent_id`）、`conversation_ref`、`task_get_tree` 等能力；fs 工具命名统一为名词前置（`file_read`、`file_write` 等）；git 工具集补充 worktree 管理工具（`worktree_add/list/remove/lock/unlock`） |
| 2026-04-13 | 重构：`blueprint/` 与 `tasks/` 模块合并为 `project/`；blueprint 工具集与 task 工具集合并为 project 工具集；`.agent/blueprints.db` 与 `.agent/tasks.db` 合并为 `.agent/project.db`；`tasks` 表新增 `blueprint_item_id` 外键，外键约束在同库内完整生效 |
| 2026-04-13 | 补充：完整目录结构（`src-tauri/src/app/` 与 `src/modules/`）；修正文档执行智能体标题缺失；`find` 工具集补充实际工具名（`grep_search`/`glob_search`）并加入执行智能体工具表与权限汇总表；`snapshot_get`/`snapshot_list` 标注待实现；DDL 路径改为 `project/migration/`；agentool 使用示例补充 `find_ctx` |
