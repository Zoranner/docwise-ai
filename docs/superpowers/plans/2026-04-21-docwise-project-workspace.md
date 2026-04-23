# Docwise Project Workspace Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将当前 Docwise 前端原型重构为“项目列表 + 项目对话 + 项目工作区”的三列本地软件界面，并用 mock 数据跑通蓝图、任务、日志三个标签。

**Architecture:** 先重建 prototype domain 和前端状态，让数据关系贴合 `Project -> Blueprint -> BlueprintItem -> TaskTree -> EventLog`；再重做 shell 和页面，把顶部导航、右侧头部、五标签结构整体移除；最后补充系统化主题配置并跑完整前端验证。

**Tech Stack:** Nuxt 4, Vue 3, Nuxt UI 4, Bun, TypeScript

---

### Task 1: 重建原型领域模型与状态

**Files:**
- Modify: `app/lib/docwise-prototype.ts`
- Modify: `app/composables/useDocwisePrototypeData.ts`
- Modify: `app/composables/useDocwisePrototypeState.ts`
- Modify: `app/lib/project-tabs.ts`
- Modify: `app/lib/project-tabs.test.ts`
- Create: `app/lib/project-workspace.test.ts`
- Create: `app/lib/project-workspace.ts`

- [ ] **Step 1: 先写失败测试，锁定新标签与新选择规则**

```ts
import { expect, test } from "bun:test";

import {
  getDefaultBlueprintItemId,
  getDefaultProjectId,
  getDefaultTaskId,
} from "./project-workspace";
import { projectTabs } from "./project-tabs";

test("projectTabs returns blueprint task log order", () => {
  expect(projectTabs("p-demo")).toEqual([
    { key: "blueprint", label: "蓝图", to: "/projects/p-demo?tab=blueprint" },
    { key: "task", label: "任务", to: "/projects/p-demo?tab=task" },
    { key: "log", label: "日志", to: "/projects/p-demo?tab=log" },
  ]);
});

test("workspace helpers resolve default selections from project data", () => {
  expect(getDefaultProjectId(["p-alpha", "p-beta"], "p-beta")).toBe("p-beta");
  expect(getDefaultProjectId(["p-alpha", "p-beta"], null)).toBe("p-alpha");
  expect(getDefaultBlueprintItemId(["bi-1", "bi-2"], null)).toBe("bi-1");
  expect(getDefaultTaskId(["task-1", "task-2"], null)).toBe("task-1");
});
```

- [ ] **Step 2: 跑测试，确认它先失败**

Run: `bun test app/lib/project-tabs.test.ts app/lib/project-workspace.test.ts`

Expected: `project-tabs.test.ts` 因旧标签顺序失败，`project-workspace.test.ts` 因文件不存在失败。

- [ ] **Step 3: 写最小实现，替换旧 prototype 数据结构**

```ts
export type DocwiseWorkspaceTab = "blueprint" | "task" | "log";

export function projectTabs(projectId: string) {
  return [
    { key: "blueprint", label: "蓝图", to: `/projects/${projectId}?tab=blueprint` },
    { key: "task", label: "任务", to: `/projects/${projectId}?tab=task` },
    { key: "log", label: "日志", to: `/projects/${projectId}?tab=log` },
  ] as const;
}
```

```ts
export function getDefaultProjectId(projectIds: string[], currentProjectId: string | null) {
  return currentProjectId && projectIds.includes(currentProjectId)
    ? currentProjectId
    : (projectIds[0] ?? null);
}

export function getDefaultBlueprintItemId(
  blueprintItemIds: string[],
  currentBlueprintItemId: string | null,
) {
  return currentBlueprintItemId && blueprintItemIds.includes(currentBlueprintItemId)
    ? currentBlueprintItemId
    : (blueprintItemIds[0] ?? null);
}

export function getDefaultTaskId(taskIds: string[], currentTaskId: string | null) {
  return currentTaskId && taskIds.includes(currentTaskId)
    ? currentTaskId
    : (taskIds[0] ?? null);
}
```

- [ ] **Step 4: 把 mock 数据收敛到项目化工作区模型**

关键要求：

- `DocwisePrototypeConversation.projectId` 不再允许 `null`
- 为每个项目补 `progress`、`conversation`、`blueprintItems`、`taskTree`、`eventLogs`
- `Task` 数据通过 `blueprintItemId` 归属，不再只靠 `blueprintId`
- `useDocwisePrototypeState` 新增 `selectedBlueprintItemId` 与 `currentTab: DocwiseWorkspaceTab`

- [ ] **Step 5: 重新跑测试，确认新模型基础通过**

Run: `bun test app/lib/project-tabs.test.ts app/lib/project-workspace.test.ts`

Expected: 2 个测试文件全部通过。

- [ ] **Step 6: Commit**

```bash
git add app/lib/docwise-prototype.ts app/composables/useDocwisePrototypeData.ts app/composables/useDocwisePrototypeState.ts app/lib/project-tabs.ts app/lib/project-tabs.test.ts app/lib/project-workspace.ts app/lib/project-workspace.test.ts
git commit -m "feat: reshape docwise project workspace prototype state"
```

### Task 2: 重做三列壳层与主题系统

**Files:**
- Modify: `app/layouts/default.vue`
- Modify: `app/components/shell/DocwiseShell.vue`
- Modify: `app/components/shell/DocwiseConversationRail.vue`
- Delete: `app/components/AppChromeNav.vue`
- Delete: `app/components/shell/DocwiseFocusHeader.vue`
- Create: `app/components/project/DocwiseProjectRail.vue`
- Create: `app/components/workspace/DocwiseWorkspacePanel.vue`
- Create: `app/components/workspace/DocwiseBlueprintPane.vue`
- Create: `app/components/workspace/DocwiseTaskPane.vue`
- Create: `app/components/workspace/DocwiseLogPane.vue`
- Create: `app.config.ts`
- Modify: `app/assets/css/main.css`

- [ ] **Step 1: 先写失败测试，锁定主题配置与三标签文案**

```ts
import { expect, test } from "bun:test";

import { projectTabs } from "./project-tabs";

test("project workspace labels are stable for the shell", () => {
  expect(projectTabs("p-alpha").map((item) => item.label)).toEqual(["蓝图", "任务", "日志"]);
});
```

- [ ] **Step 2: 跑测试，确认当前实现还没满足新壳层依赖**

Run: `bun test app/lib/project-tabs.test.ts`

Expected: 若 Task 1 未完成则失败；若已完成则可直接继续进入组件实现。

- [ ] **Step 3: 写系统化主题配置**

```ts
export default defineAppConfig({
  ui: {
    colors: {
      primary: "amber",
      neutral: "stone",
    },
    card: {
      slots: {
        base: "rounded-[24px] shadow-none ring-1 ring-inset ring-(--ui-border)",
      },
    },
    button: {
      slots: {
        base: "rounded-full font-medium",
      },
    },
    tabs: {
      slots: {
        root: "w-full",
      },
    },
  },
});
```

- [ ] **Step 4: 用 Nuxt UI 组件重做三列壳层**

关键要求：

- `default.vue` 移除 `AppChromeNav`
- `DocwiseShell.vue` 改为三列 grid
- 第一列专职项目列表
- 第二列专职项目会话
- 第三列专职项目工作区
- 不保留顶部 title/header bar

- [ ] **Step 5: 在工作区里拆三个职责单一的 pane 组件**

关键要求：

- `DocwiseWorkspacePanel.vue` 负责 tabs 切换
- `DocwiseBlueprintPane.vue` 负责蓝图目录与条目详情
- `DocwiseTaskPane.vue` 负责当前条目任务树
- `DocwiseLogPane.vue` 负责事件流

- [ ] **Step 6: 跑 lint，确认组件和样式层没有静态错误**

Run: `bun run lint`

Expected: exit code 0。

- [ ] **Step 7: Commit**

```bash
git add app/layouts/default.vue app/components/shell/DocwiseShell.vue app/components/shell/DocwiseConversationRail.vue app/components/project/DocwiseProjectRail.vue app/components/workspace/DocwiseWorkspacePanel.vue app/components/workspace/DocwiseBlueprintPane.vue app/components/workspace/DocwiseTaskPane.vue app/components/workspace/DocwiseLogPane.vue app/assets/css/main.css app.config.ts
git rm app/components/AppChromeNav.vue app/components/shell/DocwiseFocusHeader.vue
git commit -m "feat: rebuild docwise shell as project workspace"
```

### Task 3: 重写页面入口并完成验证

**Files:**
- Modify: `app/pages/index.vue`
- Modify: `app/pages/projects/index.vue`
- Modify: `app/pages/projects/overview.vue`
- Modify: `app/pages/projects/[id].vue`

- [ ] **Step 1: 先写失败测试，锁定项目页只认三标签**

```ts
import { expect, test } from "bun:test";

import { projectTabs } from "~/lib/project-tabs";

test("project route only exposes blueprint task log tabs", () => {
  expect(projectTabs("p-alpha").map((item) => item.key)).toEqual([
    "blueprint",
    "task",
    "log",
  ]);
});
```

- [ ] **Step 2: 跑测试，确认标签约束生效**

Run: `bun test app/lib/project-tabs.test.ts`

Expected: PASS。

- [ ] **Step 3: 页面入口改成项目工作区心智**

关键要求：

- `/` 直接跳到当前项目或第一个项目
- `/projects` 直接跳到当前项目或第一个项目
- `/projects/overview` 退役为跳转，不再承载单独总览页
- `/projects/[id].vue` 只渲染第三列工作区内容

- [ ] **Step 4: 跑完整前端验证**

Run: `bun test`

Expected: 全部 bun 测试通过。

Run: `bun run lint`

Expected: ESLint 0 error。

Run: `bun run build`

Expected: Nuxt build 成功，无类型或模板错误。

- [ ] **Step 5: 用本地服务做一次轻量联机确认**

Run: `Invoke-WebRequest -UseBasicParsing http://localhost:3000/ | Select-Object -ExpandProperty StatusCode`

Expected: `200`

- [ ] **Step 6: Commit**

```bash
git add app/pages/index.vue app/pages/projects/index.vue app/pages/projects/overview.vue app/pages/projects/[id].vue
git commit -m "feat: ship docwise project workspace frontend prototype"
```
