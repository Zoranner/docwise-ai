import { useState } from "#imports";

export function useShellLayoutState() {
  const projectRailVisible = useState<boolean>("shell-project-rail-visible", () => true);

  function toggleProjectRail() {
    projectRailVisible.value = !projectRailVisible.value;
  }

  return {
    projectRailVisible,
    toggleProjectRail,
  };
}
