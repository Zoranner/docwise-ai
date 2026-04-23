import { getDefaultProjectId, parseWorkspaceTab } from "./project-workspace";

type ProjectEntryArgs = {
  projectIds: string[];
  selectedProjectId: string | null;
  tab: string | string[] | null | undefined;
  tabPolicy: "preserve" | "blueprint";
};

export function buildProjectEntryTarget({
  projectIds,
  selectedProjectId,
  tab,
  tabPolicy,
}: ProjectEntryArgs) {
  const targetProjectId = getDefaultProjectId(projectIds, selectedProjectId);
  if (!targetProjectId) return null;

  const targetTab =
    tabPolicy === "blueprint"
      ? "blueprint"
      : parseWorkspaceTab(tab);

  return `/projects/${targetProjectId}?tab=${targetTab}`;
}
