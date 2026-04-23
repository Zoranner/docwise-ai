<script setup lang="ts">
import { computed, watch } from "vue";
import { navigateTo, useRoute } from "#imports";
import WorkspacePanel from "~/components/workspace/WorkspacePanel.vue";
import {
  buildProjectContentTree,
  findProjectContentNode,
  getDefaultContentPath,
  getScopedBlueprintItems,
  getScopedTaskNodes,
} from "~/lib/project-content";
import { getDefaultProjectId, parseWorkspaceTab } from "~/lib/project-workspace";
import { useProjectData } from "~/composables/useProjectData";
import { useProjectWorkspaceState } from "~/composables/useProjectWorkspaceState";

const route = useRoute();
const { projects, getBlueprintItems, getProject, getTaskById, getTaskNodes } =
  useProjectData();
const {
  selectedContentPath,
  selectedTaskId,
  selectContentPath,
  selectProject,
  selectTask,
  selectTab,
} = useProjectWorkspaceState();

const projectId = computed(() => String(route.params.id || ""));
const project = computed(() => getProject(projectId.value));
const items = computed(() => (project.value ? getBlueprintItems(project.value.id) : []));
const contentTree = computed(() => buildProjectContentTree(items.value));
const selectedNode = computed(() =>
  findProjectContentNode(contentTree.value, selectedContentPath.value),
);
const selectedItem = computed(() =>
  items.value.find((item) => item.filePath === selectedContentPath.value) ?? null,
);
const scopedItems = computed(() =>
  getScopedBlueprintItems(items.value, selectedContentPath.value),
);
const scopedTasks = computed(() =>
  project.value
    ? getScopedTaskNodes(
        items.value,
        getTaskNodes(project.value.id),
        selectedContentPath.value,
      )
    : [],
);
const selectedTask = computed(() =>
  selectedTaskId.value ? getTaskById(selectedTaskId.value) : null,
);

watch(
  project,
  async (value) => {
    if (value) {
      selectProject(value.id, value.workspaceId);
      return;
    }

    const fallbackProjectId = getDefaultProjectId(
      projects.value.map((item) => item.id),
      null,
    );

    if (fallbackProjectId && fallbackProjectId !== projectId.value) {
      await navigateTo(`/projects/${fallbackProjectId}?tab=${parseWorkspaceTab(route.query.tab)}`, {
        replace: true,
      });
    }
  },
  { immediate: true },
);

watch(
  () => route.query.tab,
  (tab) => {
    selectTab(parseWorkspaceTab(tab));
  },
  { immediate: true },
);

watch(
  items,
  () => {
    if (!contentTree.value.length) return;

    const nextContentPath = getDefaultContentPath(
      contentTree.value,
      selectedContentPath.value,
    );

    if (nextContentPath !== selectedContentPath.value) {
      selectContentPath(nextContentPath);
    }
  },
  { immediate: true },
);
</script>

<template>
  <WorkspacePanel
    v-if="project"
    :project="project"
    :content-tree="contentTree"
    :selected-node="selectedNode"
    :selected-item="selectedItem"
    :scoped-items="scopedItems"
    :scoped-tasks="scopedTasks"
    :selected-task="selectedTask"
    @select-content-path="selectContentPath"
    @select-task="selectTask"
  />

  <div
    v-else
    class="loading-state"
  >
    正在装载项目工作区…
  </div>
</template>
