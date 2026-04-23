import type { WorkspaceTab } from "./project-prototype";

export function getDefaultProjectId(
  projectIds: string[],
  currentProjectId: string | null,
) {
  if (currentProjectId && projectIds.includes(currentProjectId)) {
    return currentProjectId;
  }

  return projectIds[0] ?? null;
}

export function getDefaultBlueprintItemId(
  blueprintItemIds: string[],
  currentBlueprintItemId: string | null,
) {
  if (currentBlueprintItemId && blueprintItemIds.includes(currentBlueprintItemId)) {
    return currentBlueprintItemId;
  }

  return blueprintItemIds[0] ?? null;
}

export function getDefaultTaskId(taskIds: string[], currentTaskId: string | null) {
  if (currentTaskId && taskIds.includes(currentTaskId)) {
    return currentTaskId;
  }

  return taskIds[0] ?? null;
}

export function parseWorkspaceTab(
  tab: string | string[] | null | undefined,
): WorkspaceTab {
  const value = Array.isArray(tab) ? tab[0] : tab;

  if (value === "task") {
    return value;
  }

  return "blueprint";
}
