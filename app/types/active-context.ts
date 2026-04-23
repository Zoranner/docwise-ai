/** 与后端 `ActiveContext`（camelCase）一致，见 `product-design.md` ActiveContext。 */
export type ActiveContext = {
  workspaceId: string;
  projectId: string | null;
  blueprintId: string | null;
  taskId: string | null;
  reviewId: string | null;
  outputId: string | null;
};
