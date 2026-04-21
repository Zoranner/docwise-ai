import { expect, test } from "bun:test";

import { patchFocusContext } from "./focus-context";

test("patchFocusContext clears lower-level focus when project changes", () => {
  const next = patchFocusContext(
    {
      workspaceId: "E:/demo",
      projectId: "p1",
      blueprintId: "b1",
      taskId: "t1",
      reviewId: "r1",
      outputId: "o1",
    },
    { projectId: "p2" },
  );

  expect(next.projectId).toBe("p2");
  expect(next.blueprintId).toBeNull();
  expect(next.taskId).toBeNull();
  expect(next.reviewId).toBeNull();
  expect(next.outputId).toBeNull();
});
