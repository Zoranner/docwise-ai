import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const projectDir = import.meta.dir;
const railPath = join(projectDir, "ProjectRail.vue");
const editorDialogPath = join(projectDir, "ProjectEditorDialog.vue");
const archivedDialogPath = join(projectDir, "ArchivedProjectsDialog.vue");

test("project rail exposes create-project action and keeps list compact", () => {
  const rail = readFileSync(railPath, "utf8");

  expect(rail).toContain("新建项目");
  expect(rail).toContain("project-rail-action");
  expect(rail).toContain("project-status-indicator");
  expect(rail).toContain("i-lucide-clock-3");
  expect(rail).toContain("i-lucide-circle-alert");
  expect(rail).not.toContain("tagLabel");
  expect(rail).toContain("project-item-edit");
  expect(rail).toContain("archive-projects-trigger");
  expect(rail).toContain("ProjectEditorDialog");
  expect(rail).toContain("ArchivedProjectsDialog");
  expect(rail).not.toContain("project.progress");
  expect(rail).not.toContain("project.stage");
});

test("project editor dialog owns workspace settings and archive actions", () => {
  const dialog = readFileSync(editorDialogPath, "utf8");

  expect(dialog).toContain("项目目录");
  expect(dialog).toContain("附加可读目录");
  expect(dialog).toContain("浏览");
  expect(dialog).toContain("添加目录");
  expect(dialog).toContain("移除目录");
  expect(dialog).toContain("归档项目");
});

test("archived projects dialog exposes archived list and restore action", () => {
  const dialog = readFileSync(archivedDialogPath, "utf8");

  expect(dialog).toContain("已归档项目");
  expect(dialog).toContain("取消归档");
});
