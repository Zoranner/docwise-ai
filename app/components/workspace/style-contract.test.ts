import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

function readPane(fileName: string) {
  return readFileSync(join(import.meta.dir, fileName), "utf8");
}

test("workspace panes use shared semantic layout classes instead of local spacing utilities", () => {
  const blueprintPane = readPane("BlueprintPane.vue");
  const taskPane = readPane("TaskPane.vue");
  const logPane = readPane("LogPane.vue");

  expect(blueprintPane).toContain('class="workspace-pane-layout workspace-pane-layout--fill"');
  expect(taskPane).toContain('class="workspace-pane-layout workspace-pane-layout--fill"');
  expect(logPane).toContain('class="workspace-scroll workspace-scroll--split"');

  expect(blueprintPane).toContain('class="workspace-briefing-scroll"');
  expect(taskPane).toContain('class="workspace-briefing-scroll"');
  expect(logPane).toContain('class="workspace-log-list"');
  expect(logPane).toContain('class="workspace-log-row"');

  expect(blueprintPane).not.toContain("mt-3");
  expect(taskPane).not.toContain("mt-3");
  expect(logPane).not.toContain("mt-3");

  expect(blueprintPane).not.toContain("space-y-3");
  expect(taskPane).not.toContain("space-y-3");
  expect(logPane).not.toContain("px-4");
  expect(logPane).not.toContain("py-3");
  expect(logPane).not.toContain("p-0");
});
