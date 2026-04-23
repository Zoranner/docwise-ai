import { computed } from "vue";
import type { TaskNode } from "~/lib/project-prototype";
import {
  mockBlueprintItems,
  mockBlueprints,
  mockConversations,
  mockMessages,
  mockProjectEvents,
  mockTaskNodes,
} from "~/lib/project-prototype";
import { useProjectCatalogState } from "~/composables/useProjectCatalogState";

export type ProjectTaskTreeNode = TaskNode & {
  children: ProjectTaskTreeNode[];
};

function buildTaskTree(nodes: TaskNode[], parentId: string | null = null) {
  return nodes
    .filter((node) => node.parentId === parentId)
    .map((node) => ({
      ...node,
      children: buildTaskTree(nodes, node.id),
    }));
}

export function useProjectData() {
  const { activeProjects, allProjects, getProject } = useProjectCatalogState();
  const projects = computed(() => activeProjects.value);

  function getBlueprint(projectId: string) {
    return mockBlueprints.find((blueprint) => blueprint.projectId === projectId) ?? null;
  }

  function getBlueprintItems(projectId: string) {
    return mockBlueprintItems.filter((item) => item.projectId === projectId);
  }

  function getBlueprintItem(blueprintItemId: string) {
    return mockBlueprintItems.find((item) => item.id === blueprintItemId) ?? null;
  }

  function getTaskNodes(projectId: string) {
    return mockTaskNodes.filter((node) => node.projectId === projectId);
  }

  function getTaskNodesForBlueprintItem(projectId: string, blueprintItemId: string) {
    return mockTaskNodes.filter(
      (node) => node.projectId === projectId && node.blueprintItemId === blueprintItemId,
    );
  }

  function getTaskTree(projectId: string, blueprintItemId: string) {
    return buildTaskTree(getTaskNodesForBlueprintItem(projectId, blueprintItemId));
  }

  function getTaskById(taskId: string) {
    return mockTaskNodes.find((node) => node.id === taskId) ?? null;
  }

  function getProjectEvents(projectId: string) {
    return mockProjectEvents.filter((event) => event.projectId === projectId);
  }

  function getConversation(projectId: string) {
    return mockConversations.find((conversation) => conversation.projectId === projectId) ?? null;
  }

  function getMessages(projectId: string) {
    return mockMessages.filter((message) => message.projectId === projectId);
  }

  function getProjectMetrics(projectId: string) {
    const tasks = getTaskNodes(projectId);
    const events = getProjectEvents(projectId);

    return {
      taskCount: tasks.length,
      runningCount: tasks.filter((task) => task.status === "running").length,
      blockedCount: tasks.filter(
        (task) => task.status === "blocked" || task.status === "waiting_checkpoint",
      ).length,
      eventCount: events.length,
    };
  }

  return {
    projects,
    allProjects: computed(() => allProjects.value),
    getProject,
    getBlueprint,
    getBlueprintItems,
    getBlueprintItem,
    getTaskNodes,
    getTaskNodesForBlueprintItem,
    getTaskTree,
    getTaskById,
    getProjectEvents,
    getConversation,
    getMessages,
    getProjectMetrics,
  };
}
