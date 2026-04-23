import type {
  BlueprintItemStatus,
  BlueprintStatus,
  ProjectEvent,
  TaskStatus,
} from "~/lib/project-prototype";

export const blueprintStatusTone = {
  draft: "warning",
  approved: "success",
  active: "primary",
  revised: "warning",
} as const;

export const blueprintItemTone = {
  planned: "neutral",
  active: "primary",
  done: "success",
} as const;

export const taskStatusTone = {
  backlog: "neutral",
  running: "warning",
  waiting_checkpoint: "danger",
  blocked: "danger",
  done: "success",
} as const;

export function getBlueprintStatusLabel(status: BlueprintStatus) {
  switch (status) {
    case "draft":
      return "草拟中";
    case "approved":
      return "已确认";
    case "active":
      return "推进中";
    case "revised":
      return "待修订";
  }
}

export function getBlueprintItemStatusLabel(status: BlueprintItemStatus) {
  switch (status) {
    case "planned":
      return "待开始";
    case "active":
      return "进行中";
    case "done":
      return "已完成";
  }
}

export function getTaskStatusLabel(status: TaskStatus) {
  switch (status) {
    case "backlog":
      return "待开始";
    case "running":
      return "进行中";
    case "waiting_checkpoint":
      return "待确认";
    case "blocked":
      return "阻塞";
    case "done":
      return "已完成";
  }
}

export function getProjectEventKindLabel(kind: ProjectEvent["kind"]) {
  switch (kind) {
    case "blueprint":
      return "蓝图";
    case "task":
      return "任务";
    case "decision":
      return "决策";
    case "output":
      return "产出";
    case "project":
      return "项目";
  }
}
