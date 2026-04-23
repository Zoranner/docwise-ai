<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { BlueprintItem, Project, TaskNode } from "~/lib/project-prototype";
import type { ProjectContentNode } from "~/lib/project-content";
import BaseIcon from "~/components/base/BaseIcon.vue";
import Panel from "~/components/base/Panel.vue";
import Tag from "~/components/base/Tag.vue";
import WorkspaceDetailHeader from "~/components/workspace/WorkspaceDetailHeader.vue";
import WorkspaceEmptyState from "~/components/workspace/WorkspaceEmptyState.vue";
import WorkspaceMarkdownView from "~/components/workspace/WorkspaceMarkdownView.vue";
import WorkspaceSectionHeader from "~/components/workspace/WorkspaceSectionHeader.vue";
import {
  blueprintItemTone,
  getBlueprintItemStatusLabel,
  getTaskStatusLabel,
  taskStatusTone,
} from "~/lib/workspace-status";

const props = defineProps<{
  project: Project;
  contentTree: ProjectContentNode[];
  selectedNode: ProjectContentNode | null;
  selectedItem: BlueprintItem | null;
  scopedItems: BlueprintItem[];
  scopedTasks: TaskNode[];
  selectedTask: TaskNode | null;
}>();

const emit = defineEmits<{
  (e: "select-content-path" | "select-task", value: string): void;
}>();

const expandedDirectories = ref<string[]>([]);

function collectTopLevelDirectoryPaths(nodes: ProjectContentNode[]) {
  return nodes.filter((node) => node.kind === "directory").map((node) => node.path);
}

function collectAncestorDirectoryPaths(node: ProjectContentNode | null) {
  if (!node) return [];

  const segments = node.path.split("/").filter(Boolean);
  const directorySegments = node.kind === "directory" ? segments : segments.slice(0, -1);

  return directorySegments.map((_, index) => directorySegments.slice(0, index + 1).join("/"));
}

function isDirectoryExpanded(path: string) {
  return expandedDirectories.value.includes(path);
}

function toggleDirectory(path: string) {
  expandedDirectories.value = isDirectoryExpanded(path)
    ? expandedDirectories.value.filter((item) => item !== path)
    : [...expandedDirectories.value, path];
}

function flattenVisibleContentTree(
  nodes: ProjectContentNode[],
  expandedPaths: string[],
  depth = 0,
): Array<ProjectContentNode & { depth: number }> {
  return nodes.flatMap((node) => {
    const entry = [{ ...node, depth }];

    if (node.kind !== "directory" || !expandedPaths.includes(node.path)) {
      return entry;
    }

    return [
      ...entry,
      ...flattenVisibleContentTree(node.children, expandedPaths, depth + 1),
    ];
  });
}

watch(
  () => props.contentTree,
  (tree) => {
    if (!expandedDirectories.value.length) {
      expandedDirectories.value = collectTopLevelDirectoryPaths(tree);
    }
  },
  { immediate: true },
);

watch(
  () => props.selectedNode,
  (node) => {
    const next = new Set(expandedDirectories.value);

    for (const path of collectAncestorDirectoryPaths(node)) {
      next.add(path);
    }

    expandedDirectories.value = [...next];
  },
  { immediate: true },
);

const flatContentNodes = computed(() =>
  flattenVisibleContentTree(props.contentTree, expandedDirectories.value),
);

const selectedNodeKicker = computed(() => {
  if (!props.selectedNode) return "当前内容";
  return props.selectedNode.kind === "directory" ? "目录" : "文件";
});

const selectedNodeTitle = computed(() => {
  if (!props.selectedNode) return "请选择左侧内容节点";
  return props.selectedNode.name;
});

function buildScopedTaskTree(nodes: TaskNode[], parentId: string | null = null): Array<TaskNode & { depth: number }> {
  return nodes.flatMap((node) => {
    if (node.parentId !== parentId) return [];

    return [
      { ...node, depth: 0 },
      ...buildScopedTaskTree(nodes, node.id).map((child) => ({
        ...child,
        depth: child.depth + 1,
      })),
    ];
  });
}

const flatScopedTasks = computed(() => buildScopedTaskTree(props.scopedTasks));

const currentTaskFileTitle = computed(() => {
  if (!props.selectedTask) return null;

  return props.scopedItems.find((item) => item.id === props.selectedTask?.blueprintItemId)?.title ?? null;
});
</script>

<template>
  <div class="flex h-full min-h-0 flex-col overflow-hidden">
    <div class="workspace-context-shell-header">
      <p class="section-title">工作区</p>
    </div>

    <div class="workspace-scroll workspace-scroll--split">
      <div class="workspace-pane-layout workspace-pane-layout--fill">
        <Panel class="workspace-pane-panel" padding="none">
          <template #header>
            <WorkspaceSectionHeader title="内容结构" :count="flatContentNodes.length" />
          </template>

          <div class="workspace-tree-scroll">
            <div class="stack-list workspace-content-tree">
              <div
                v-for="node in flatContentNodes"
                :key="node.path"
                class="workspace-tree-entry"
                :class="node.path === selectedNode?.path ? 'is-active' : ''"
                :style="{ paddingLeft: `${node.depth * 14}px` }"
              >
                <button
                  v-if="node.kind === 'directory'"
                  type="button"
                  class="workspace-tree-toggle"
                  :aria-label="isDirectoryExpanded(node.path) ? '收起目录' : '展开目录'"
                  @click="toggleDirectory(node.path)"
                >
                  <BaseIcon
                    :name="isDirectoryExpanded(node.path) ? 'i-lucide-chevron-down' : 'i-lucide-chevron-right'"
                    class="workspace-tree-toggle__icon"
                    aria-hidden="true"
                  />
                </button>
                <span v-else class="workspace-tree-spacer" aria-hidden="true" />

                <button
                  type="button"
                  class="workspace-tree-button"
                  @click="emit('select-content-path', node.path)"
                >
                  <BaseIcon
                    :name="node.kind === 'directory' ? 'i-lucide-folder-open' : 'i-lucide-file-text'"
                    class="workspace-tree-icon"
                    aria-hidden="true"
                  />
                  <span class="workspace-tree-label">{{ node.name }}</span>
                </button>
              </div>
            </div>
          </div>
        </Panel>

        <Panel class="workspace-pane-panel" padding="none">
          <template #header>
            <div class="workspace-context-bar">
              <div class="workspace-context-bar__copy">
                <WorkspaceDetailHeader
                  :kicker="selectedNodeKicker"
                  :title="selectedNodeTitle"
                  :status-label="selectedItem ? getBlueprintItemStatusLabel(selectedItem.status) : null"
                  :status-tone="selectedItem ? blueprintItemTone[selectedItem.status] : 'neutral'"
                />
              </div>
            </div>
          </template>

          <div class="workspace-context-scroll">
            <section v-if="selectedItem" class="workspace-context-section">
              <WorkspaceSectionHeader title="当前内容" />
              <div class="workspace-context-copy">
                <WorkspaceMarkdownView :source="selectedItem?.briefingMarkdown ?? ''" />
              </div>
            </section>

            <section class="workspace-context-section workspace-context-section--tasks">
              <WorkspaceSectionHeader title="执行任务" :count="flatScopedTasks.length" />

              <div v-if="flatScopedTasks.length" class="workspace-task-shell">
                <div class="workspace-task-list">
                  <button
                    v-for="task in flatScopedTasks"
                    :key="task.id"
                    type="button"
                    class="list-row workspace-task-row"
                    :class="task.id === selectedTask?.id ? 'is-active' : ''"
                    @click="emit('select-task', task.id)"
                  >
                    <div class="workspace-list-main" :style="{ paddingLeft: `${task.depth * 14}px` }">
                      <div class="workspace-list-head">
                        <p class="workspace-list-title">{{ task.title }}</p>
                        <Tag :tone="taskStatusTone[task.status]" size="sm">
                          {{ getTaskStatusLabel(task.status) }}
                        </Tag>
                      </div>
                      <div class="item-submeta workspace-submeta-offset">
                        <span class="truncate">
                          {{ props.scopedItems.find((item) => item.id === task.blueprintItemId)?.title ?? task.owner }}
                        </span>
                        <span class="tabular-nums">{{ task.progress }}%</span>
                      </div>
                    </div>
                  </button>
                </div>

                <div class="workspace-task-detail">
                  <div v-if="selectedTask" class="workspace-task-detail__content">
                    <WorkspaceDetailHeader
                      kicker="任务"
                      :title="selectedTask.title"
                      :status-label="getTaskStatusLabel(selectedTask.status)"
                      :status-tone="taskStatusTone[selectedTask.status]"
                    />
                    <p v-if="currentTaskFileTitle" class="panel-meta copy-offset-xs">
                      {{ currentTaskFileTitle }}
                    </p>
                    <div class="workspace-context-copy copy-offset-sm">
                      <WorkspaceMarkdownView :source="selectedTask.briefingMarkdown" />
                    </div>
                  </div>

                  <WorkspaceEmptyState
                    v-else
                    message="当前范围还没有选中具体任务。"
                  />
                </div>
              </div>

              <WorkspaceEmptyState
                v-else
                message="当前范围还没有执行任务。"
              />
            </section>
          </div>
        </Panel>
      </div>
    </div>
  </div>
</template>
