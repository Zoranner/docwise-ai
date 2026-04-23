<script setup lang="ts">
import type { Project } from "~/lib/project-prototype";
import Dialog from "~/components/base/Dialog.vue";
import Tag from "~/components/base/Tag.vue";

defineProps<{
  open: boolean;
  projects: Project[];
}>();

defineEmits<{
  (e: "close"): void;
  (e: "unarchive", projectId: string): void;
}>();
</script>

<template>
  <Dialog :open="open" title="已归档项目" width="lg" @close="$emit('close')">
    <div v-if="projects.length" class="archived-project-list">
      <div
        v-for="project in projects"
        :key="project.id"
        class="archived-project-row"
      >
        <div class="min-w-0">
          <div class="project-item-head">
            <p class="project-item-title">{{ project.name }}</p>
            <Tag size="sm">已归档</Tag>
          </div>
          <p class="meta-text copy-offset-xs truncate">{{ project.workspacePath }}</p>
        </div>

        <button
          type="button"
          class="project-item-unarchive"
          @click="$emit('unarchive', project.id)"
        >
          取消归档
        </button>
      </div>
    </div>

    <div v-else class="empty-state">
      当前没有已归档项目。
    </div>

    <template #footer>
      <div class="dialog-actions">
        <button type="button" class="dialog-button dialog-button--ghost" @click="$emit('close')">
          关闭
        </button>
      </div>
    </template>
  </Dialog>
</template>
