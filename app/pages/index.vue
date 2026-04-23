<script setup lang="ts">
import { definePageMeta, navigateTo, useRoute } from "#imports";
import { buildProjectEntryTarget } from "~/lib/project-entry";
import { useProjectData } from "~/composables/useProjectData";
import { useProjectWorkspaceState } from "~/composables/useProjectWorkspaceState";

definePageMeta({ layout: false });

const route = useRoute();
const { projects } = useProjectData();
const { selectedProjectId } = useProjectWorkspaceState();

const target = buildProjectEntryTarget({
  projectIds: projects.value.map((project) => project.id),
  selectedProjectId: selectedProjectId.value,
  tab: route.query.tab,
  tabPolicy: "preserve",
});

if (target) {
  await navigateTo(target, { replace: true });
}
</script>

<template>
  <div class="loading-state">
    正在进入项目工作区…
  </div>
</template>
