/** 与后端 `WorkspaceSessionDto`（camelCase）一致。 */
export type WorkspaceSessionDto = {
  workspaceId: string;
  path: string;
  focused: boolean;
};
