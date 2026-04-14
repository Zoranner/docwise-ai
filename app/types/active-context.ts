/** 与后端 `ActiveContext`（camelCase）一致，见 `docwise-design.md` ActiveContext。 */
export type ActiveContext = {
  workspaceId: string;
  filePath: string | null;
  blueprintId: string | null;
  taskId: string | null;
  runId: string | null;
  checkpointId: string | null;
};
