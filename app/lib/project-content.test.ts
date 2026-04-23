import { expect, test } from "bun:test";

import type { BlueprintItem, TaskNode } from "./project-prototype";
import {
  buildProjectContentTree,
  findProjectContentNode,
  flattenProjectContentTree,
  getDefaultContentPath,
  getScopedBlueprintItems,
  getScopedTaskNodes,
} from "./project-content";

const items: BlueprintItem[] = [
  {
    id: "bi-1",
    projectId: "p-demo",
    blueprintId: "bp-demo",
    code: "A01",
    filePath: "delivery/outline.md",
    title: "交付总览",
    goal: "说明交付目录。",
    summary: "交付入口。",
    briefingMarkdown: "# 交付总览",
    status: "active",
    progress: 80,
  },
  {
    id: "bi-2",
    projectId: "p-demo",
    blueprintId: "bp-demo",
    code: "A02",
    filePath: "delivery/deployment/install.md",
    title: "安装说明",
    goal: "说明安装步骤。",
    summary: "安装章节。",
    briefingMarkdown: "# 安装说明",
    status: "planned",
    progress: 24,
  },
  {
    id: "bi-3",
    projectId: "p-demo",
    blueprintId: "bp-demo",
    code: "A03",
    filePath: "design/api.md",
    title: "接口设计",
    goal: "说明接口约束。",
    summary: "接口文档。",
    briefingMarkdown: "# 接口设计",
    status: "done",
    progress: 100,
  },
];

const tasks: TaskNode[] = [
  {
    id: "task-1",
    projectId: "p-demo",
    blueprintItemId: "bi-1",
    parentId: null,
    title: "整理交付总览",
    summary: "完成交付入口。",
    briefingMarkdown: "# task-1",
    owner: "Executor",
    status: "running",
    progress: 70,
  },
  {
    id: "task-2",
    projectId: "p-demo",
    blueprintItemId: "bi-2",
    parentId: null,
    title: "补齐安装说明",
    summary: "完成部署步骤。",
    briefingMarkdown: "# task-2",
    owner: "Planner",
    status: "backlog",
    progress: 0,
  },
];

test("project content tree exposes directories and files as a single navigable structure", () => {
  const tree = buildProjectContentTree(items);
  const flat = flattenProjectContentTree(tree);

  expect(flat.map((node) => `${node.kind}:${node.path}`)).toEqual([
    "directory:delivery",
    "directory:delivery/deployment",
    "file:delivery/deployment/install.md",
    "file:delivery/outline.md",
    "directory:design",
    "file:design/api.md",
  ]);

  expect(findProjectContentNode(tree, "delivery/deployment")?.kind).toBe("directory");
  expect(findProjectContentNode(tree, "delivery/outline.md")?.kind).toBe("file");
  expect(getDefaultContentPath(tree, null)).toBe("delivery");
});

test("content scope returns exact file requirements or subtree execution scope", () => {
  expect(getScopedBlueprintItems(items, "delivery/outline.md").map((item) => item.id)).toEqual(["bi-1"]);
  expect(getScopedBlueprintItems(items, "delivery").map((item) => item.id)).toEqual(["bi-1", "bi-2"]);
  expect(getScopedTaskNodes(items, tasks, "delivery").map((task) => task.id)).toEqual([
    "task-1",
    "task-2",
  ]);
  expect(getScopedTaskNodes(items, tasks, "design").map((task) => task.id)).toEqual([]);
});
