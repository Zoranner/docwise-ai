<script setup lang="ts">
import { computed, ref } from "vue";
import { navigateTo } from "#imports";
import type { ProjectStatus } from "~/lib/project-prototype";
import BaseIcon from "~/components/base/BaseIcon.vue";
import Tag from "~/components/base/Tag.vue";
import ArchivedProjectsDialog from "~/components/project/ArchivedProjectsDialog.vue";
import ProjectEditorDialog from "~/components/project/ProjectEditorDialog.vue";
import { useProjectCatalogState } from "~/composables/useProjectCatalogState";
import { useProjectWorkspaceState } from "~/composables/useProjectWorkspaceState";

const {
  activeProjects,
  archivedProjects,
  getProject,
  createProject,
  updateProject,
  archiveProject,
  unarchiveProject,
} = useProjectCatalogState();
const { currentTab, selectedProjectId, selectProject } = useProjectWorkspaceState();
const createDialogOpen = ref(false);
const editDialogProjectId = ref<string | null>(null);
const archivedDialogOpen = ref(false);

const editingProject = computed(() =>
  editDialogProjectId.value ? getProject(editDialogProjectId.value) : null,
);

const projectStatusMeta: Record<
  ProjectStatus,
  {
    icon: string;
    indicatorClass: string;
  }
> = {
  active: {
    icon: "i-lucide-loader-circle",
    indicatorClass: "project-status-indicator--active",
  },
  planning: {
    icon: "i-lucide-clock-3",
    indicatorClass: "project-status-indicator--planning",
  },
  blocked: {
    icon: "i-lucide-circle-alert",
    indicatorClass: "project-status-indicator--blocked",
  },
  done: {
    icon: "i-lucide-check",
    indicatorClass: "project-status-indicator--done",
  },
} as const;

async function openProject(projectId: string, workspaceId: string) {
  selectProject(projectId, workspaceId);
  await navigateTo({
    path: `/projects/${projectId}`,
    query: { tab: currentTab.value },
  });
}

function openCreateDialog() {
  createDialogOpen.value = true;
}

function openEditDialog(projectId: string) {
  editDialogProjectId.value = projectId;
}

function closeEditDialog() {
  editDialogProjectId.value = null;
}

async function handleCreateProject(payload: {
  name: string;
  workspacePath: string;
  readablePaths: string[];
}) {
  const created = createProject(payload);
  createDialogOpen.value = false;
  selectProject(created.id, created.workspaceId);
  await navigateTo({
    path: `/projects/${created.id}`,
    query: { tab: currentTab.value },
  });
}

function handleUpdateProject(payload: {
  name: string;
  workspacePath: string;
  readablePaths: string[];
}) {
  if (!editingProject.value) return;
  updateProject(editingProject.value.id, payload);
  closeEditDialog();
}

async function handleArchiveProject(projectId: string) {
  archiveProject(projectId);
  closeEditDialog();

  if (selectedProjectId.value !== projectId) return;

  const nextActiveProject = activeProjects.value.find((project) => project.id !== projectId) ?? null;
  if (!nextActiveProject) {
    selectProject(null);
    await navigateTo("/projects");
    return;
  }

  selectProject(nextActiveProject.id, nextActiveProject.workspaceId);
  await navigateTo({
    path: `/projects/${nextActiveProject.id}`,
    query: { tab: currentTab.value },
  });
}

function handleUnarchiveProject(projectId: string) {
  unarchiveProject(projectId);
}
</script>

<template>
  <div class="flex h-full min-h-0 flex-col">
    <div class="rail-header">
      <div class="project-rail-header">
        <div class="project-rail-header__title">
          <p class="section-title">项目</p>
          <Tag>
            {{ activeProjects.length }}
          </Tag>
        </div>

        <button
          type="button"
          class="project-rail-action"
          @click="openCreateDialog"
        >
          新建项目
        </button>
      </div>
    </div>

    <div class="rail-scroll">
      <div class="stack-list">
        <div
          v-for="project in activeProjects"
          :key="project.id"
          class="project-item-shell"
        >
          <button
            type="button"
            class="side-item"
            :class="project.id === selectedProjectId ? 'is-active' : ''"
            @click="openProject(project.id, project.workspaceId)"
          >
            <div class="project-item-head">
              <BaseIcon
                :name="projectStatusMeta[project.status].icon"
                class="project-status-indicator"
                :class="projectStatusMeta[project.status].indicatorClass"
                aria-hidden="true"
              />
              <p class="project-item-title">{{ project.name }}</p>
            </div>
          </button>

          <button
            type="button"
            class="project-item-edit"
            aria-label="编辑项目"
            @click="openEditDialog(project.id)"
          >
            <BaseIcon name="i-lucide-pencil" class="project-item-edit__icon" aria-hidden="true" />
          </button>
        </div>
      </div>
    </div>

    <div class="rail-footer rail-footer--ghost">
      <button
        type="button"
        class="archive-projects-trigger"
        @click="archivedDialogOpen = true"
      >
        已归档
        <Tag
          v-if="archivedProjects.length"
          size="sm"
        >
          {{ archivedProjects.length }}
        </Tag>
      </button>
    </div>

    <ProjectEditorDialog
      :open="createDialogOpen"
      mode="create"
      @close="createDialogOpen = false"
      @submit="handleCreateProject"
    />

    <ProjectEditorDialog
      :open="Boolean(editingProject)"
      mode="edit"
      :project="editingProject"
      @close="closeEditDialog"
      @submit="handleUpdateProject"
      @archive="handleArchiveProject"
    />

    <ArchivedProjectsDialog
      :open="archivedDialogOpen"
      :projects="archivedProjects"
      @close="archivedDialogOpen = false"
      @unarchive="handleUnarchiveProject"
    />
  </div>
</template>
