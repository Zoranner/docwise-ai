<script setup lang="ts">
import { definePageMeta, navigateTo } from "#imports";
import { buildProjectEntryTarget } from "~/lib/project-entry";
import { useProjectData } from "~/composables/useProjectData";
import { useProjectWorkspaceState } from "~/composables/useProjectWorkspaceState";

definePageMeta({ layout: false });

const { projects } = useProjectData();
const { selectedProjectId } = useProjectWorkspaceState();

const target = buildProjectEntryTarget({
  projectIds: projects.value.map((project) => project.id),
  selectedProjectId: selectedProjectId.value,
  tab: null,
  tabPolicy: "blueprint",
});

if (target) {
  await navigateTo(target, { replace: true });
}
</script>

<template>
  <div class="loading-state">
    总览已并入项目常驻列表，正在进入当前项目…
  </div>
</template>
