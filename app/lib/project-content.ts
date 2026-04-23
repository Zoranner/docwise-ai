import type { BlueprintItem, TaskNode } from "./project-prototype";

export type ProjectContentNode = {
  path: string;
  name: string;
  kind: "directory" | "file";
  itemId?: string;
  children: ProjectContentNode[];
};

function sortNodes(nodes: ProjectContentNode[]) {
  return [...nodes].sort((left, right) => {
    if (left.kind !== right.kind) {
      return left.kind === "directory" ? -1 : 1;
    }

    return left.name.localeCompare(right.name, "zh-CN");
  });
}

export function buildProjectContentTree(items: BlueprintItem[]) {
  const nodeMap = new Map<string, ProjectContentNode>();
  const rootNodes = new Map<string, ProjectContentNode>();

  for (const item of items) {
    const segments = item.filePath.split("/").filter(Boolean);
    if (!segments.length) continue;

    let parentPath: string | null = null;

    for (const [index, segment] of segments.entries()) {
      const path = parentPath ? `${parentPath}/${segment}` : segment;
      const isFile = index === segments.length - 1;

      let node = nodeMap.get(path);
      if (!node) {
        node = {
          path,
          name: segment,
          kind: isFile ? "file" : "directory",
          itemId: isFile ? item.id : undefined,
          children: [],
        };
        nodeMap.set(path, node);

        if (parentPath) {
          const parent = nodeMap.get(parentPath);
          parent?.children.push(node);
        } else {
          rootNodes.set(path, node);
        }
      } else if (isFile) {
        node.kind = "file";
        node.itemId = item.id;
      }

      parentPath = path;
    }
  }

  function sortTree(nodes: ProjectContentNode[]): ProjectContentNode[] {
    return sortNodes(nodes).map((node) => ({
      ...node,
      children: sortTree(node.children),
    }));
  }

  return sortTree([...rootNodes.values()]);
}

export function flattenProjectContentTree(
  nodes: ProjectContentNode[],
  depth = 0,
): Array<ProjectContentNode & { depth: number }> {
  return nodes.flatMap((node) => [
    { ...node, depth },
    ...flattenProjectContentTree(node.children, depth + 1),
  ]);
}

export function findProjectContentNode(nodes: ProjectContentNode[], path: string | null) {
  if (!path) return null;

  for (const node of nodes) {
    if (node.path === path) return node;

    const childMatch = findProjectContentNode(node.children, path);
    if (childMatch) return childMatch;
  }

  return null;
}

export function getDefaultContentPath(
  nodes: ProjectContentNode[],
  currentPath: string | null,
) {
  if (currentPath && findProjectContentNode(nodes, currentPath)) {
    return currentPath;
  }

  return nodes[0]?.path ?? null;
}

export function getScopedBlueprintItems(items: BlueprintItem[], path: string | null) {
  if (!path) return [...items];

  const exactItem = items.find((item) => item.filePath === path);
  if (exactItem) return [exactItem];

  const directoryPrefix = `${path}/`;
  return items.filter((item) => item.filePath.startsWith(directoryPrefix));
}

export function getScopedTaskNodes(
  items: BlueprintItem[],
  tasks: TaskNode[],
  path: string | null,
) {
  const itemIds = new Set(getScopedBlueprintItems(items, path).map((item) => item.id));
  return tasks.filter((task) => itemIds.has(task.blueprintItemId));
}
