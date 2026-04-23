import { computed } from "vue";
import { useState } from "#imports";
import type { Project } from "~/lib/project-prototype";
import { mockProjects } from "~/lib/project-prototype";

type ProjectDraftInput = {
  name: string;
  workspacePath: string;
  readablePaths: string[];
};

function cloneProjects() {
  return mockProjects.map((project) => ({
    ...project,
    readablePaths: [...project.readablePaths],
    archivedAt: project.archivedAt ?? null,
  }));
}

function formatClockStamp() {
  return new Date()
    .toLocaleTimeString("zh-CN", {
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    })
    .replace(":", ":");
}

function normalizePaths(paths: string[]) {
  return paths
    .map((path) => path.trim())
    .filter(Boolean);
}

function buildProjectId(name: string) {
  return `p-${name.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-")}-${Date.now().toString(36)}`;
}

function buildWorkspaceId(name: string) {
  return `ws-${name.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-")}-${Date.now().toString(36)}`;
}

export function useProjectCatalogState() {
  const allProjects = useState<Project[]>("project-catalog-state", cloneProjects);

  const activeProjects = computed(() =>
    allProjects.value.filter((project) => !project.archivedAt),
  );

  const archivedProjects = computed(() =>
    allProjects.value.filter((project) => Boolean(project.archivedAt)),
  );

  function getProject(projectId: string) {
    return allProjects.value.find((project) => project.id === projectId) ?? null;
  }

  function createProject(input: ProjectDraftInput) {
    const created: Project = {
      id: buildProjectId(input.name),
      workspaceId: buildWorkspaceId(input.name),
      name: input.name.trim(),
      workspacePath: input.workspacePath.trim(),
      readablePaths: normalizePaths(input.readablePaths),
      stage: "等待首轮蓝图",
      status: "planning",
      progress: 0,
      summary: "等待首次蓝图整理与任务拆解。",
      updatedAt: formatClockStamp(),
      archivedAt: null,
    };

    allProjects.value = [created, ...allProjects.value];
    return created;
  }

  function updateProject(
    projectId: string,
    input: Pick<ProjectDraftInput, "name" | "readablePaths">,
  ) {
    allProjects.value = allProjects.value.map((project) =>
      project.id === projectId
        ? {
            ...project,
            name: input.name.trim(),
            readablePaths: normalizePaths(input.readablePaths),
            updatedAt: formatClockStamp(),
          }
        : project,
    );
  }

  function archiveProject(projectId: string) {
    allProjects.value = allProjects.value.map((project) =>
      project.id === projectId
        ? {
            ...project,
            archivedAt: new Date().toISOString(),
            updatedAt: formatClockStamp(),
          }
        : project,
    );
  }

  function unarchiveProject(projectId: string) {
    allProjects.value = allProjects.value.map((project) =>
      project.id === projectId
        ? {
            ...project,
            archivedAt: null,
            updatedAt: formatClockStamp(),
          }
        : project,
    );
  }

  return {
    allProjects,
    activeProjects,
    archivedProjects,
    getProject,
    createProject,
    updateProject,
    archiveProject,
    unarchiveProject,
  };
}
