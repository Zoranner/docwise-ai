import { expect, test } from "bun:test";

import {
  getDefaultBlueprintItemId,
  getDefaultProjectId,
  getDefaultTaskId,
  parseWorkspaceTab,
} from "./project-workspace";

test("workspace helpers resolve default selections from project data", () => {
  expect(getDefaultProjectId(["p-alpha", "p-beta"], "p-beta")).toBe("p-beta");
  expect(getDefaultProjectId(["p-alpha", "p-beta"], null)).toBe("p-alpha");
  expect(getDefaultProjectId([], null)).toBeNull();

  expect(getDefaultBlueprintItemId(["bi-1", "bi-2"], null)).toBe("bi-1");
  expect(getDefaultBlueprintItemId(["bi-1", "bi-2"], "bi-2")).toBe("bi-2");
  expect(getDefaultBlueprintItemId([], null)).toBeNull();

  expect(getDefaultTaskId(["task-1", "task-2"], null)).toBe("task-1");
  expect(getDefaultTaskId(["task-1", "task-2"], "task-2")).toBe("task-2");
  expect(getDefaultTaskId([], null)).toBeNull();
});

test("parseWorkspaceTab keeps only blueprint task", () => {
  expect(parseWorkspaceTab("blueprint")).toBe("blueprint");
  expect(parseWorkspaceTab("task")).toBe("task");
  expect(parseWorkspaceTab("log")).toBe("blueprint");
  expect(parseWorkspaceTab(["task", "log"])).toBe("task");
  expect(parseWorkspaceTab("overview")).toBe("blueprint");
  expect(parseWorkspaceTab(null)).toBe("blueprint");
});
