import type { ActiveContext } from "~/types/active-context";

export function emptyFocusContext(workspaceId = ""): ActiveContext {
  return {
    workspaceId,
    projectId: null,
    blueprintId: null,
    taskId: null,
    reviewId: null,
    outputId: null,
  };
}

export function patchFocusContext(
  current: ActiveContext,
  patch: Partial<ActiveContext>,
): ActiveContext {
  const next: ActiveContext = { ...current, ...patch };

  if (patch.workspaceId != null && patch.workspaceId !== current.workspaceId) {
    next.projectId = null;
    next.blueprintId = null;
    next.taskId = null;
    next.reviewId = null;
    next.outputId = null;
  }

  if (patch.projectId != null && patch.projectId !== current.projectId) {
    next.blueprintId = null;
    next.taskId = null;
    next.reviewId = null;
    next.outputId = null;
  }

  if (patch.blueprintId != null && patch.blueprintId !== current.blueprintId) {
    next.taskId = null;
    next.reviewId = null;
    next.outputId = null;
  }

  if (patch.taskId != null && patch.taskId !== current.taskId) {
    next.reviewId = null;
    next.outputId = null;
  }

  return next;
}
