import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const workspaceDir = import.meta.dir;
const taskPanePath = join(workspaceDir, "TaskPane.vue");
const logPanePath = join(workspaceDir, "LogPane.vue");

test("task pane uses the same fill layout and markdown briefing pattern as blueprint pane", () => {
  const source = readFileSync(taskPanePath, "utf8");

  expect(source).toContain('class="workspace-scroll workspace-scroll--split"');
  expect(source).toContain('class="workspace-pane-layout workspace-pane-layout--fill"');
  expect(source).toContain('class="workspace-list-scroll"');
  expect(source).toContain('class="workspace-briefing-scroll"');
  expect(source).toContain("selectedTask.briefingMarkdown");
  expect(source).toContain("getTaskStatusLabel(task.status)");
  expect(source).not.toContain("WorkspaceHero");
  expect(source).not.toContain("WorkspaceInfoBlock");
  expect(source).not.toContain("负责人");
  expect(source).not.toContain("局部完成度");
  expect(source).not.toContain("已完成");
  expect(source).not.toContain("执行摘要");
  expect(source).not.toContain("归属项目");
});

test("log pane removes hero summary and keeps only the full-height event stream", () => {
  const source = readFileSync(logPanePath, "utf8");

  expect(source).toContain('class="workspace-scroll workspace-scroll--split"');
  expect(source).toContain('class="workspace-pane-panel"');
  expect(source).toContain('class="workspace-log-list"');
  expect(source).toContain("getProjectEventKindLabel(event.kind)");
  expect(source).not.toContain("WorkspaceHero");
  expect(source).not.toContain("workspace-pane-offset");
});
