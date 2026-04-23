import { expect, test } from "bun:test";

import { projectTabs } from "./project-tabs";

test("projectTabs returns blueprint task order", () => {
  expect(projectTabs("p-demo")).toEqual([
    { key: "blueprint", label: "蓝图", to: "/projects/p-demo?tab=blueprint" },
    { key: "task", label: "任务", to: "/projects/p-demo?tab=task" },
  ]);
});
