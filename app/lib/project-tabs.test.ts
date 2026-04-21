import { expect, test } from "bun:test";

import { projectTabs } from "./project-tabs";

test("projectTabs returns canonical tab order", () => {
  expect(projectTabs("p-demo")).toEqual([
    { key: "overview", label: "总览", to: "/projects/p-demo?tab=overview" },
    { key: "blueprints", label: "蓝图", to: "/projects/p-demo?tab=blueprints" },
    { key: "tasks", label: "任务", to: "/projects/p-demo?tab=tasks" },
    { key: "reviews", label: "审议", to: "/projects/p-demo?tab=reviews" },
    { key: "outputs", label: "产出", to: "/projects/p-demo?tab=outputs" },
  ]);
});
