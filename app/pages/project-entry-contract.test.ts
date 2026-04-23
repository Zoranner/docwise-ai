import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const pagesDir = import.meta.dir;

function read(relativePath: string) {
  return readFileSync(join(pagesDir, relativePath), "utf8");
}

test("entry redirect pages reuse shared project entry helper", () => {
  const rootPage = read("index.vue");
  const projectsIndexPage = read("projects/index.vue");
  const projectsOverviewPage = read("projects/overview.vue");
  const workspacePage = read("workspace/index.vue");

  for (const source of [rootPage, projectsIndexPage, projectsOverviewPage, workspacePage]) {
    expect(source).toContain('import { buildProjectEntryTarget } from "~/lib/project-entry";');
    expect(source).toContain("const target = buildProjectEntryTarget({");
  }

  expect(rootPage).toContain('tabPolicy: "preserve"');
  expect(projectsIndexPage).toContain('tabPolicy: "preserve"');
  expect(projectsOverviewPage).toContain('tabPolicy: "blueprint"');
  expect(workspacePage).toContain('tabPolicy: "blueprint"');
});
