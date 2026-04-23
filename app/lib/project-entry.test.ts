import { expect, test } from "bun:test";

import { buildProjectEntryTarget } from "./project-entry";

test("buildProjectEntryTarget preserves selected tab for root and project entry redirects", () => {
  expect(
    buildProjectEntryTarget({
      projectIds: ["p-alpha", "p-beta"],
      selectedProjectId: "p-beta",
      tab: "task",
      tabPolicy: "preserve",
    }),
  ).toBe("/projects/p-beta?tab=task");

  expect(
    buildProjectEntryTarget({
      projectIds: ["p-alpha", "p-beta"],
      selectedProjectId: null,
      tab: "log",
      tabPolicy: "preserve",
    }),
  ).toBe("/projects/p-alpha?tab=blueprint");
});

test("buildProjectEntryTarget collapses compatibility routes to blueprint tab", () => {
  expect(
    buildProjectEntryTarget({
      projectIds: ["p-alpha", "p-beta"],
      selectedProjectId: "p-beta",
      tab: "task",
      tabPolicy: "blueprint",
    }),
  ).toBe("/projects/p-beta?tab=blueprint");

  expect(
    buildProjectEntryTarget({
      projectIds: [],
      selectedProjectId: null,
      tab: "task",
      tabPolicy: "blueprint",
    }),
  ).toBeNull();
});
