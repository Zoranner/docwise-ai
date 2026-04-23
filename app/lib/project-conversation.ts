import type {
  ProjectAgentKind,
  ProjectMessage,
} from "~/lib/project-prototype";

export type ConversationPresentation = {
  lane: "user" | "agent";
  showHeader: boolean;
  speaker: string | null;
  statusLabel: string | null;
  kindLabel: string | null;
  tone: ProjectAgentKind | "user";
};

const agentKindLabelMap: Record<ProjectAgentKind, string> = {
  planner: "规划",
  executor: "执行",
  reviewer: "审阅",
  coordinator: "协调",
};

export function getAgentKindLabel(kind: ProjectAgentKind | null | undefined) {
  if (!kind) return null;
  return agentKindLabelMap[kind];
}

export function getConversationPresentation(
  message: ProjectMessage,
): ConversationPresentation {
  if (message.role === "user") {
    return {
      lane: "user",
      showHeader: true,
      speaker: "用户",
      statusLabel: null,
      kindLabel: null,
      tone: "user",
    };
  }

  return {
    lane: "agent",
    showHeader: true,
    speaker: message.agentName ?? "智能体",
    statusLabel: message.handledDuration
      ? `已处理 ${message.handledDuration}`
      : null,
    kindLabel: getAgentKindLabel(message.agentKind),
    tone: message.agentKind ?? "coordinator",
  };
}
