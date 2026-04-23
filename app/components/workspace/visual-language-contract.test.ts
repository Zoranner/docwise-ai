import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const workspaceDir = import.meta.dir;
const cssPath = join(workspaceDir, "..", "..", "assets", "css", "main.css");
const heroPath = join(workspaceDir, "WorkspaceHero.vue");

test("workspace visual language stays dense and non-card-like", () => {
  const css = readFileSync(cssPath, "utf8");
  const hero = readFileSync(heroPath, "utf8");

  expect(css).toContain("--size-rail-width: 260px;");
  expect(css).toContain("--size-chat-width: 450px;");
  expect(css).toContain("--radius-panel: 4px;");
  expect(css).toContain("--radius-control: 4px;");
  expect(css).toContain("--font-size-title: 13px;");
  expect(css).toContain("--font-size-section: 13px;");
  expect(css).toContain("--font-size-body: 13px;");
  expect(css).toContain(".workspace-hero-shell");
  expect(css).toContain(".workspace-inline-metrics");

  expect(css).not.toContain(".workspace-metric-card");
  expect(css).not.toContain("font-size: 28px;");
  expect(css).not.toContain("var(--shadow-panel),");
  expect(hero).not.toContain('import Panel from "~/components/base/Panel.vue";');
});
