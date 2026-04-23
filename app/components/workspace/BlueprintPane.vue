<script setup lang="ts">
import type { Blueprint, BlueprintItem, Project } from "~/lib/project-prototype";
import Panel from "~/components/base/Panel.vue";
import Tag from "~/components/base/Tag.vue";
import WorkspaceDetailHeader from "~/components/workspace/WorkspaceDetailHeader.vue";
import WorkspaceEmptyState from "~/components/workspace/WorkspaceEmptyState.vue";
import WorkspaceMarkdownView from "~/components/workspace/WorkspaceMarkdownView.vue";
import WorkspaceSectionHeader from "~/components/workspace/WorkspaceSectionHeader.vue";
import {
  blueprintItemTone,
  getBlueprintItemStatusLabel,
} from "~/lib/workspace-status";

defineProps<{
  project: Project;
  blueprint: Blueprint;
  items: BlueprintItem[];
  selectedItem: BlueprintItem | null;
}>();

defineEmits<{
  (e: "select-item", blueprintItemId: string): void;
}>();
</script>

<template>
  <div class="workspace-scroll workspace-scroll--split">
    <div class="workspace-pane-layout workspace-pane-layout--fill">
      <Panel class="workspace-pane-panel" padding="none">
        <template #header>
          <WorkspaceSectionHeader title="蓝图条目" :count="items.length" />
        </template>

        <div class="workspace-list-scroll">
          <div class="stack-list">
            <button
              v-for="item in items"
              :key="item.id"
              type="button"
              class="list-row"
              :class="item.id === selectedItem?.id ? 'is-active' : ''"
              @click="$emit('select-item', item.id)"
            >
              <div class="workspace-list-main">
                <div class="workspace-list-head">
                  <div class="workspace-list-copy">
                    <p class="entity-code">
                      {{ item.code }}
                    </p>
                    <p class="workspace-list-title">{{ item.title }}</p>
                  </div>
                  <span class="workspace-list-progress">{{ item.progress }}%</span>
                </div>
                <div class="item-submeta workspace-submeta-offset">
                  <span class="truncate">{{ item.filePath }}</span>
                  <Tag :tone="blueprintItemTone[item.status]">
                    {{ getBlueprintItemStatusLabel(item.status) }}
                  </Tag>
                </div>
              </div>
            </button>
          </div>
        </div>
      </Panel>

      <Panel class="workspace-pane-panel" padding="none">
        <template #header>
          <WorkspaceDetailHeader
            :kicker="selectedItem?.code ?? '未选中条目'"
            :title="selectedItem?.title ?? '请选择左侧蓝图条目'"
            :status-label="selectedItem ? getBlueprintItemStatusLabel(selectedItem.status) : null"
            :status-tone="selectedItem ? blueprintItemTone[selectedItem.status] : 'neutral'"
          />
        </template>

        <div class="workspace-briefing-scroll">
          <WorkspaceMarkdownView
            v-if="selectedItem"
            :source="selectedItem.briefingMarkdown"
          />

          <WorkspaceEmptyState
            v-else
            message="左侧选择一个蓝图条目查看详情。"
          />
        </div>
      </Panel>
    </div>
  </div>
</template>
