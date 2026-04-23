import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const workspaceDir = import.meta.dir;
const blueprintPanePath = join(workspaceDir, "BlueprintPane.vue");
const detailHeaderPath = join(workspaceDir, "WorkspaceDetailHeader.vue");

test("blueprint pane uses fill layout with independent list and briefing scroll areas", () => {
  const source = readFileSync(blueprintPanePath, "utf8");

  expect(source).toContain('class="workspace-scroll workspace-scroll--split"');
  expect(source).toContain('class="workspace-pane-layout workspace-pane-layout--fill"');
  expect(source).toContain('class="workspace-list-scroll"');
  expect(source).toContain('class="workspace-briefing-scroll"');
  expect(source).toContain("selectedItem.briefingMarkdown");
  expect(source).not.toContain("WorkspaceHero");
  expect(source).not.toContain("WorkspaceInfoBlock");
  expect(source).not.toContain("文件路径");
  expect(source).not.toContain("完成度");
  expect(source).not.toContain("条目目标");
  expect(source).not.toContain("当前进展");
  expect(source).not.toContain("蓝图约束");
});

test("blueprint pane and detail header use localized status labels in compact headers", () => {
  const blueprintPane = readFileSync(blueprintPanePath, "utf8");
  const detailHeader = readFileSync(detailHeaderPath, "utf8");

  expect(blueprintPane).toContain("getBlueprintItemStatusLabel(item.status)");
  expect(blueprintPane).not.toContain("{{ item.status }}");
  expect(blueprintPane).not.toContain(":status-label=\"selectedItem?.status ?? null\"");
  expect(detailHeader).toContain('class="workspace-detail-header__titleline"');
  expect(detailHeader).toContain("workspace-detail-header__kicker");
});
