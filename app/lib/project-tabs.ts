export function projectTabs(projectId: string) {
  return [
    { key: "blueprint", label: "蓝图", to: `/projects/${projectId}?tab=blueprint` },
    { key: "task", label: "任务", to: `/projects/${projectId}?tab=task` },
  ] as const;
}
