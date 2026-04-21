export function projectTabs(projectId: string) {
  return [
    { key: "overview", label: "总览", to: `/projects/${projectId}?tab=overview` },
    { key: "blueprints", label: "蓝图", to: `/projects/${projectId}?tab=blueprints` },
    { key: "tasks", label: "任务", to: `/projects/${projectId}?tab=tasks` },
    { key: "reviews", label: "审议", to: `/projects/${projectId}?tab=reviews` },
    { key: "outputs", label: "产出", to: `/projects/${projectId}?tab=outputs` },
  ] as const;
}
