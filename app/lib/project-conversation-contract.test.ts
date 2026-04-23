import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const libDir = import.meta.dir;
const prototypePath = join(libDir, "project-prototype.ts");

test("project messages support multiple named agents", () => {
  const prototype = readFileSync(prototypePath, "utf8");

  expect(prototype).toContain('role: "user" | "agent"');
  expect(prototype).toContain("export type ProjectAgentActivity = {");
  expect(prototype).toContain("agentId?: string | null;");
  expect(prototype).toContain("agentName?: string | null;");
  expect(prototype).toContain("agentKind?: ProjectAgentKind | null;");
  expect(prototype).toContain("handledDuration?: string | null;");
  expect(prototype).toContain("activities?: ProjectAgentActivity[];");
});
