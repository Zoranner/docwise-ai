<script setup lang="ts">
import type { Project, ProjectEvent } from "~/lib/project-prototype";
import Panel from "~/components/base/Panel.vue";
import Tag from "~/components/base/Tag.vue";
import WorkspaceSectionHeader from "~/components/workspace/WorkspaceSectionHeader.vue";
import { getProjectEventKindLabel } from "~/lib/workspace-status";

defineProps<{
  project: Project;
  events: ProjectEvent[];
}>();
</script>

<template>
  <div class="workspace-scroll workspace-scroll--split">
    <Panel class="workspace-pane-panel" padding="none">
      <template #header>
        <WorkspaceSectionHeader title="事件流" :count="events.length" />
      </template>

      <div class="workspace-log-list">
        <div
          v-for="event in events"
          :key="event.id"
          class="workspace-log-row"
        >
          <div class="panel-header-row">
            <div class="min-w-0">
              <div class="workspace-inline-tags">
                <Tag :tone="event.tone">{{ getProjectEventKindLabel(event.kind) }}</Tag>
                <p class="section-heading">{{ event.title }}</p>
              </div>
              <p class="support-text workspace-copy-offset">
                {{ event.detail }}
              </p>
            </div>
            <span class="meta-text shrink-0">{{ event.at }}</span>
          </div>
        </div>
      </div>
    </Panel>
  </div>
</template>
