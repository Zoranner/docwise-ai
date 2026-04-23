<script setup lang="ts">
import { computed } from "vue";
import type { BlueprintItem, Project, TaskNode } from "~/lib/project-prototype";
import type { ProjectTaskTreeNode } from "~/composables/useProjectData";
import Panel from "~/components/base/Panel.vue";
import Tag from "~/components/base/Tag.vue";
import WorkspaceDetailHeader from "~/components/workspace/WorkspaceDetailHeader.vue";
import WorkspaceEmptyState from "~/components/workspace/WorkspaceEmptyState.vue";
import WorkspaceMarkdownView from "~/components/workspace/WorkspaceMarkdownView.vue";
import WorkspaceSectionHeader from "~/components/workspace/WorkspaceSectionHeader.vue";
import { getTaskStatusLabel, taskStatusTone } from "~/lib/workspace-status";

const props = defineProps<{
  project: Project;
  blueprintItem: BlueprintItem | null;
  taskTree: ProjectTaskTreeNode[];
  selectedTask: TaskNode | null;
}>();

defineEmits<{
  (e: "select-task", taskId: string): void;
}>();

function flattenTree(nodes: ProjectTaskTreeNode[], depth = 0): Array<ProjectTaskTreeNode & { depth: number }> {
  return nodes.flatMap((node) => [
    { ...node, depth },
    ...flattenTree(node.children, depth + 1),
  ]);
}

const flatTasks = computed(() => flattenTree(props.taskTree));
</script>

<template>
  <div class="workspace-scroll workspace-scroll--split">
    <div class="workspace-pane-layout workspace-pane-layout--fill">
      <Panel class="workspace-pane-panel" padding="none">
        <template #header>
          <WorkspaceSectionHeader title="任务树" :count="flatTasks.length" />
        </template>

        <div class="workspace-list-scroll">
          <div class="stack-list">
            <button
              v-for="task in flatTasks"
              :key="task.id"
              type="button"
              class="list-row"
              :class="task.id === selectedTask?.id ? 'is-active' : ''"
              @click="$emit('select-task', task.id)"
            >
              <div class="workspace-list-main" :style="{ paddingLeft: `${task.depth * 14}px` }">
                <div class="workspace-list-head">
                  <p class="workspace-list-title">{{ task.title }}</p>
                  <Tag :tone="taskStatusTone[task.status]">
                    {{ getTaskStatusLabel(task.status) }}
                  </Tag>
                </div>
                <div class="item-submeta workspace-submeta-offset">
                  <span class="truncate">{{ task.owner }}</span>
                  <span class="tabular-nums">{{ task.progress }}%</span>
                </div>
              </div>
            </button>
          </div>
        </div>
      </Panel>

      <Panel class="workspace-pane-panel" padding="none">
        <template #header>
          <WorkspaceDetailHeader
            kicker="任务节点"
            :title="selectedTask?.title ?? '请选择左侧任务节点'"
            :status-label="selectedTask ? getTaskStatusLabel(selectedTask.status) : null"
            :status-tone="selectedTask ? taskStatusTone[selectedTask.status] : 'neutral'"
          />
        </template>

        <div class="workspace-briefing-scroll">
          <WorkspaceMarkdownView
            v-if="selectedTask"
            :source="selectedTask.briefingMarkdown"
          />

          <WorkspaceEmptyState
            v-else
            message="左侧选择一个任务节点查看详情。"
          />
        </div>
      </Panel>
    </div>
  </div>
</template>
