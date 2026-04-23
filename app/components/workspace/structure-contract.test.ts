import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

function readPane(fileName: string) {
  return readFileSync(join(import.meta.dir, fileName), "utf8");
}

test("workspace panel removes blueprint task tabs and centers the workspace around content plus execution", () => {
  const panel = readPane("WorkspacePanel.vue");

  expect(panel).toContain('class="workspace-context-shell-header"');
  expect(panel).toContain("工作区");
  expect(panel).toContain("内容结构");
  expect(panel).toContain("当前内容");
  expect(panel).toContain("执行任务");
  expect(panel).toContain("workspace-content-tree");
  expect(panel).toContain("workspace-task-list");
  expect(panel).not.toContain('import Tabs from "~/components/navigation/Tabs.vue";');
  expect(panel).not.toContain("projectTabs(");
  expect(panel).not.toContain("currentTab === 'task'");
  expect(panel).not.toContain("currentTab === 'blueprint'");
});

test("workspace panel keeps directory and file context in one surface instead of separate blueprint task panes", () => {
  const panel = readPane("WorkspacePanel.vue");

  expect(panel).toContain("selectedItem?.briefingMarkdown");
  expect(panel).toContain("select-content-path");
  expect(panel).toContain("select-task");
  expect(panel).not.toContain("{{ node.path }}");
  expect(panel).not.toContain('{{ node.kind === "directory" ? "目录" : "文件" }}');
  expect(panel).not.toContain("workspace-context-bar__path");
  expect(panel).not.toContain("workspace-directory-summary");
  expect(panel).not.toContain("个文件");
  expect(panel).not.toContain("个任务");
  expect(panel).toContain("workspace-tree-toggle");
  expect(panel).toContain("toggleDirectory");
});
