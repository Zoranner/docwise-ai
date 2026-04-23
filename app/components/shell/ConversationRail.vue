<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import BaseIcon from "~/components/base/BaseIcon.vue";
import Tag from "~/components/base/Tag.vue";
import Composer from "~/components/input/Composer.vue";
import { useProjectData } from "~/composables/useProjectData";
import { useShellLayoutState } from "~/composables/useShellLayoutState";
import { useProjectWorkspaceState } from "~/composables/useProjectWorkspaceState";
import { getConversationPresentation } from "~/lib/project-conversation";
import { getProjectEventKindLabel } from "~/lib/workspace-status";

const { getProject, getProjectEvents } = useProjectData();
const { projectRailVisible, toggleProjectRail } = useShellLayoutState();
const {
  selectedProjectId,
  conversationDraft,
  messages,
  sendConversationMessage,
} = useProjectWorkspaceState();
const logPanelOpen = ref(false);
const logPanelRef = ref<HTMLElement | null>(null);

const currentProject = computed(() =>
  selectedProjectId.value ? getProject(selectedProjectId.value) : null,
);
const projectEvents = computed(() =>
  selectedProjectId.value ? getProjectEvents(selectedProjectId.value) : [],
);

const projectMessages = computed(() =>
  messages.value.filter((message) => message.projectId === selectedProjectId.value),
);

const conversationMessages = computed(() =>
  projectMessages.value.map((message) => ({
    ...message,
    presentation: getConversationPresentation(message),
  })),
);

function submitMessage() {
  sendConversationMessage();
}

function attachToConversation() {
  return;
}

function toggleLogPanel() {
  logPanelOpen.value = !logPanelOpen.value;
}

function handlePointerDown(event: PointerEvent) {
  const target = event.target;

  if (!logPanelOpen.value || !(target instanceof Node)) return;
  if (logPanelRef.value?.contains(target)) return;

  logPanelOpen.value = false;
}

onMounted(() => {
  document.addEventListener("pointerdown", handlePointerDown);
});

onBeforeUnmount(() => {
  document.removeEventListener("pointerdown", handlePointerDown);
});
</script>

<template>
  <div class="flex h-full min-h-0 flex-col">
    <div class="rail-header">
      <div class="conversation-project-header">
        <div class="conversation-project-header__main">
          <button
            type="button"
            class="shell-rail-toggle"
            :aria-label="projectRailVisible ? '隐藏项目栏' : '显示项目栏'"
            @click="toggleProjectRail"
          >
            <BaseIcon
              :name="projectRailVisible ? 'i-lucide-panel-left-close' : 'i-lucide-panel-left-open'"
              class="shell-rail-toggle__icon"
              aria-hidden="true"
            />
          </button>

          <p class="section-title truncate-heading">
            {{ currentProject?.name ?? "未选择项目" }}
          </p>
        </div>

        <div ref="logPanelRef" class="conversation-header-actions">
          <button
            type="button"
            class="conversation-log-trigger"
            aria-label="切换项目日志面板"
            :aria-expanded="logPanelOpen ? 'true' : 'false'"
            @click="toggleLogPanel"
          >
            <BaseIcon name="i-lucide-history" class="conversation-log-trigger__icon" aria-hidden="true" />
          </button>

          <div v-if="logPanelOpen" class="conversation-log-panel">
            <div class="conversation-log-panel__header">
              <p class="section-title">项目日志</p>
              <Tag size="sm">
                {{ projectEvents.length }}
              </Tag>
            </div>

            <div v-if="projectEvents.length" class="conversation-log-panel__list">
              <div
                v-for="event in projectEvents"
                :key="event.id"
                class="conversation-log-panel__item"
              >
                <div class="conversation-log-panel__row">
                  <div class="min-w-0">
                    <div class="workspace-inline-tags">
                      <Tag :tone="event.tone" size="sm">{{ getProjectEventKindLabel(event.kind) }}</Tag>
                      <p class="section-heading">{{ event.title }}</p>
                    </div>
                    <p class="support-text copy-offset-xs">
                      {{ event.detail }}
                    </p>
                  </div>
                  <span class="meta-text shrink-0">{{ event.at }}</span>
                </div>
              </div>
            </div>

            <div v-else class="empty-state conversation-log-panel__empty">
              当前还没有项目日志。
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="conversation-summary-banner">
      <p
        class="support-text conversation-summary-banner__text"
        :title="
          currentProject?.summary ??
            '这里显示当前项目的短状态总结，不显示总体进度、时间或单个当前条目。'
        "
      >
        <Tag tone="primary" size="sm">
          状态摘要
        </Tag>
        <span class="conversation-summary-banner__copy">
        {{
          currentProject?.summary ??
            "这里显示当前项目的短状态总结，不显示总体进度、时间或单个当前条目。"
        }}
        </span>
      </p>
    </div>

    <div class="rail-scroll">
      <div v-if="conversationMessages.length" class="conversation-stream">
        <template
          v-for="message in conversationMessages"
          :key="message.id"
        >
          <article
            class="conversation-message"
            :class="
              message.role === 'user'
                ? 'conversation-message--user'
                : 'conversation-message--agent'
            "
            :data-agent-kind="message.presentation.kindLabel ?? undefined"
          >
            <div
              v-if="message.presentation.showHeader"
              class="conversation-message__header"
            >
              <div class="conversation-message__identity">
                <span class="conversation-message__speaker">
                  {{
                    message.role === 'user'
                      ? "用户"
                      : (message.agentName ?? message.presentation.speaker)
                  }}
                </span>
                <span
                  v-if="message.handledDuration"
                  class="conversation-message__status"
                >
                  已处理 {{ message.handledDuration }}
                </span>
              </div>
            </div>

            <div
              class="conversation-bubble"
              :class="[
                message.role === 'user'
                  ? 'conversation-bubble--user'
                  : 'conversation-bubble--agent',
                message.role === 'agent'
                  ? `conversation-bubble--${message.presentation.tone}`
                  : null,
              ]"
            >
              <p class="support-text message-text">{{ message.content }}</p>
            </div>
            <div
              v-if="message.activities?.length"
              class="conversation-agent-activity"
            >
              <details
                v-for="activity in message.activities"
                :key="`${message.id}-${activity.label}`"
                class="conversation-agent-activity__item"
              >
                <summary class="conversation-agent-activity__toggle">
                  <div class="conversation-agent-activity__summary">
                    <div class="conversation-agent-activity__identity">
                      <p class="conversation-agent-activity__label">
                        {{ activity.label }}
                      </p>
                    </div>

                    <span class="conversation-agent-activity__duration">
                      {{ activity.durationLabel }}
                    </span>
                  </div>
                </summary>

                <div class="conversation-agent-activity__content">
                  <p
                    v-if="activity.body"
                    class="support-text conversation-agent-activity__body"
                  >
                    {{ activity.body }}
                  </p>

                  <p
                    v-for="file in activity.relatedFiles ?? []"
                    :key="file"
                    class="conversation-agent-activity__file"
                  >
                    {{ file }}
                  </p>
                </div>
              </details>
            </div>
          </article>
        </template>
      </div>

      <div
        v-else
        class="empty-state"
      >
        选择项目后，从这里持续聊下去。
      </div>
    </div>

    <div class="rail-footer">
      <Composer
        v-model="conversationDraft"
        :rows="3"
        placeholder="继续围绕当前项目说下去。对话只属于当前项目。"
        attachment-label="添加附件"
        submit-label="发送消息"
        @attach="attachToConversation"
        @submit="submitMessage"
      />
    </div>
  </div>
</template>
