/** 项目中心清单条目（仅存浏览器 localStorage，与宿主内已打开会话分离）。 */
export type DocwiseProjectCatalogEntry = {
  id: string;
  name: string;
  workspacePath: string;
  group?: string;
  updatedAt?: string;
};
