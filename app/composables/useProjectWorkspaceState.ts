import { useState } from "#imports";
import type {
  ProjectMessage,
  WorkspaceTab,
} from "~/lib/project-prototype";
import {
  mockBlueprintItems,
  mockConversations,
  mockMessages,
  mockTaskNodes,
} from "~/lib/project-prototype";
import {
  buildProjectContentTree,
  getDefaultContentPath,
  getScopedBlueprintItems,
  getScopedTaskNodes,
} from "~/lib/project-content";
import {
  getDefaultBlueprintItemId,
  getDefaultProjectId,
  getDefaultTaskId,
  parseWorkspaceTab,
} from "~/lib/project-workspace";
import { useProjectCatalogState } from "~/composables/useProjectCatalogState";

export function useProjectWorkspaceState() {
  const { activeProjects, getProject } = useProjectCatalogState();
  const currentWorkspaceId = useState<string>("project-workspace-id", () => "ws-alpha");
  const selectedProjectId = useState<string | null>("project-id", () => "p-alpha");
  const selectedBlueprintItemId = useState<string | null>(
    "project-blueprint-item-id",
    () => "bi-alpha-1",
  );
  const selectedContentPath = useState<string | null>("project-content-path", () => "docs");
  const selectedTaskId = useState<string | null>("project-task-id", () => "task-alpha-1");
  const currentTab = useState<WorkspaceTab>("project-tab", () => "blueprint");
  const conversationDraft = useState<string>("project-conversation-draft", () => "");
  const messages = useState<ProjectMessage[]>("project-messages", () =>
    mockMessages.map((message) => ({ ...message })),
  );

  function syncSelections(projectId: string | null, workspaceId?: string | null) {
    const nextActiveProjects = activeProjects.value;
    const nextProjectId = getDefaultProjectId(
      nextActiveProjects.map((project) => project.id),
      projectId,
    );

    selectedProjectId.value = nextProjectId;

    const nextProject = nextProjectId ? getProject(nextProjectId) : null;
    if (workspaceId) {
      currentWorkspaceId.value = workspaceId;
    } else if (nextProject) {
      currentWorkspaceId.value = nextProject.workspaceId;
    }

    const projectItems = mockBlueprintItems.filter((item) => item.projectId === nextProjectId);
    const contentTree = buildProjectContentTree(projectItems);
    const nextContentPath = getDefaultContentPath(contentTree, selectedContentPath.value);
    const scopedItems = getScopedBlueprintItems(projectItems, nextContentPath);

    selectedContentPath.value = nextContentPath;
    selectedBlueprintItemId.value = getDefaultBlueprintItemId(
      scopedItems.map((item) => item.id),
      selectedBlueprintItemId.value,
    );
    selectedTaskId.value = getDefaultTaskId(
      getScopedTaskNodes(
        projectItems,
        mockTaskNodes.filter((task) => task.projectId === nextProjectId),
        nextContentPath,
      ).map((task) => task.id),
      selectedTaskId.value,
    );
  }

  function selectProject(projectId: string | null, workspaceId?: string | null) {
    syncSelections(projectId, workspaceId);
  }

  function selectBlueprintItem(blueprintItemId: string | null) {
    const projectId = selectedProjectId.value;
    const nextItem = blueprintItemId
      ? mockBlueprintItems.find((item) => item.id === blueprintItemId && item.projectId === projectId)
      : null;

    selectedContentPath.value = nextItem?.filePath ?? null;
    selectedBlueprintItemId.value = blueprintItemId;

    selectedTaskId.value = getDefaultTaskId(
      mockTaskNodes
        .filter((task) => task.projectId === projectId && task.blueprintItemId === blueprintItemId)
        .map((task) => task.id),
      selectedTaskId.value,
    );
  }

  function selectContentPath(contentPath: string | null) {
    const projectId = selectedProjectId.value;
    const projectItems = mockBlueprintItems.filter((item) => item.projectId === projectId);
    const scopedItems = getScopedBlueprintItems(projectItems, contentPath);

    selectedContentPath.value = contentPath;
    selectedBlueprintItemId.value = getDefaultBlueprintItemId(
      scopedItems.map((item) => item.id),
      selectedBlueprintItemId.value,
    );
    selectedTaskId.value = getDefaultTaskId(
      getScopedTaskNodes(
        projectItems,
        mockTaskNodes.filter((task) => task.projectId === projectId),
        contentPath,
      ).map((task) => task.id),
      selectedTaskId.value,
    );
  }

  function selectTask(taskId: string | null) {
    selectedTaskId.value = taskId;
  }

  function selectTab(tab: string) {
    currentTab.value = parseWorkspaceTab(tab);
  }

  function setConversationDraft(value: string) {
    conversationDraft.value = value;
  }

  function sendConversationMessage() {
    const content = conversationDraft.value.trim();
    const projectId = selectedProjectId.value;
    if (!content || !projectId) return;

    const conversation = mockConversations.find((item) => item.projectId === projectId);
    const projectItems = mockBlueprintItems.filter((item) => item.projectId === projectId);
    const targetItem = projectItems.find((item) => item.id === selectedBlueprintItemId.value) ?? null;
    const targetTitle = targetItem?.title ?? selectedContentPath.value ?? "当前工作区";
    const stamp = Date.now();

    messages.value = [
      ...messages.value,
      {
        id: `msg-user-${stamp}`,
        projectId,
        role: "user",
        content,
        at: "刚刚",
      },
      {
        id: `msg-agent-planner-${stamp}`,
        projectId,
        role: "agent",
        agentId: "agent-planner",
        agentName: "规划智能体",
        agentKind: "planner",
        handledDuration: "00:16",
        content: `${conversation?.title ?? "项目对话"}已收到。我会先围绕${targetTitle ?? "当前项目"}拆出目标、约束和下一步动作，再把需要你拍板的点抬出来。`,
        at: "刚刚",
        activities: [
          {
            kind: "thought",
            label: "思考结束",
            durationLabel: "1s",
            body: `正在梳理 ${targetTitle} 的关键约束，并同步给右侧工作区。`,
          },
          {
            kind: "tool",
            label: "查看文件",
            durationLabel: "2s",
            body: "已核对当前内容树、执行任务列表与项目栏配置，准备继续推进。",
            relatedFiles: [
              "app/components/workspace/WorkspacePanel.vue",
              "app/components/project/ProjectRail.vue",
            ],
          },
        ],
      },
      {
        id: `msg-agent-executor-${stamp}`,
        projectId,
        role: "agent",
        agentId: "agent-executor",
        agentName: "执行智能体",
        agentKind: "executor",
        handledDuration: "00:22",
        content: `我会继续推进 ${targetTitle} 的具体写作和结构整理；右侧工作区会围绕当前目录或文件展示要求与执行，不再拆成蓝图和任务两个 tab。`,
        at: "刚刚",
      },
    ];

    conversationDraft.value = "";
  }

  syncSelections(selectedProjectId.value, currentWorkspaceId.value);

  return {
    currentWorkspaceId,
    selectedProjectId,
    selectedBlueprintItemId,
    selectedContentPath,
    selectedTaskId,
    currentTab,
    conversationDraft,
    messages,
    selectProject,
    selectBlueprintItem,
    selectContentPath,
    selectTask,
    selectTab,
    setConversationDraft,
    sendConversationMessage,
  };
}
