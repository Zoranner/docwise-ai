import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const shellDir = import.meta.dir;
const shellRootPath = join(shellDir, "Shell.vue");
const shellPath = join(shellDir, "ConversationRail.vue");
const cssPath = join(shellDir, "..", "..", "assets", "css", "main.css");
const workspacePanelPath = join(shellDir, "..", "workspace", "WorkspacePanel.vue");

test("conversation rail header uses project summary instead of progress and item focus badges", () => {
  const rail = readFileSync(shellPath, "utf8");
  const css = readFileSync(cssPath, "utf8");

  expect(rail).toContain("currentProject?.summary");
  expect(rail).toContain("conversation-summary-banner");
  expect(rail).toContain("状态摘要");
  expect(rail).toContain("Tag");
  expect(rail).toContain("conversation-summary-banner__copy");
  expect(rail).toContain(':title="');
  expect(rail).not.toContain("currentProject.progress");
  expect(rail).not.toContain("currentBlueprintItem");
  expect(rail).not.toContain("currentConversation.updatedAt");
  expect(rail).not.toContain("conversation-project-stage");
  expect(rail).not.toContain("conversation-project-summary");
  expect(rail).toContain("conversation-log-trigger");
  expect(rail).toContain("conversation-log-panel");
  expect(css).toContain(".conversation-summary-banner");
  expect(css).toContain(".conversation-log-panel");
  expect(css).toContain("background: rgba(137, 160, 190, 0.08);");
  expect(css).toContain("-webkit-line-clamp: 3;");
});

test("shell hides workspace column on narrow widths so chat can expand", () => {
  const css = readFileSync(cssPath, "utf8");

  expect(css).toContain("@media (max-width: 1040px)");
  expect(css).toContain("@media (max-width: 780px)");
  expect(css).toContain(".shell-workspace");
  expect(css).toContain("display: none;");
});

test("shell exposes project rail toggle from chat header and supports collapsed rail layout", () => {
  const shell = readFileSync(shellRootPath, "utf8");
  const rail = readFileSync(shellPath, "utf8");
  const css = readFileSync(cssPath, "utf8");

  expect(shell).toContain("projectRailVisible");
  expect(shell).toContain("app-shell--rail-hidden");
  expect(rail).toContain("shell-rail-toggle");
  expect(rail).toContain("toggleProjectRail");
  expect(css).toContain("grid-template-columns: var(--size-chat-width) minmax(0, 1fr);");
});

test("rail headers and workspace tabs share a consistent shell header height", () => {
  const css = readFileSync(cssPath, "utf8");
  const workspacePanel = readFileSync(workspacePanelPath, "utf8");

  expect(css).toContain("--size-shell-header-height");
  expect(css).toContain("min-height: var(--size-shell-header-height);");
  expect(workspacePanel).toContain("workspace-context-shell-header");
  expect(css).not.toContain(".workspace-tabs {");
});
