<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { AgentStreamChannel } from "~/types/planning-stream";

type RenderPreviewResult = {
  snapshotId: string;
  html: string;
  diagnostics: unknown[];
  assetBaseUrl: string;
  themeRevision: string;
};

const previewHtml = ref("");
const errorMessage = ref<string | null>(null);
const loading = ref(true);

const providerOptions = [
  { value: "ollama", label: "Ollama" },
  { value: "openai", label: "OpenAI" },
  { value: "anthropic", label: "Anthropic" },
  { value: "google", label: "Google" },
  { value: "aliyun", label: "Aliyun" },
  { value: "zhipu", label: "Zhipu" },
] as const;

const agentChannel = ref<AgentStreamChannel>("planning");
const agentProvider = ref<string>("ollama");
const agentApiKey = ref("");
const agentBaseUrl = ref("http://localhost:11434/v1");
const agentModel = ref("qwen2.5");
const agentUserText = ref("用一句话说明 Docwise 是做什么的。");
const agentIncludePreview = ref(true);
/** 留空则用后端默认；仅影响当前 model_round 内单次 HTTP 的重试，不会从用户回合开头重跑 */
const agentLlmMaxRetries = ref("");
const agentLlmRetryBaseMs = ref("");
const agentLlmRetryMaxMs = ref("");

const {
  status: agentStatus,
  errorMessage: agentError,
  logLines: agentLogLines,
  assistantDisplay: agentAssistant,
  lastSeq: agentLastSeq,
  start: startAgentStream,
  stopListening: stopAgentListening,
} = useDocwiseAgentStream();

const agentRunning = computed(() => agentStatus.value === "running");

function optU64(s: string): number | undefined {
  const t = s.trim();
  if (!t) return undefined;
  const n = Number(t);
  if (!Number.isFinite(n) || n < 0) return undefined;
  return Math.floor(n);
}

async function runAgentStream() {
  const llmMaxRetries = optU64(agentLlmMaxRetries.value);
  const llmRetryBaseDelayMs = optU64(agentLlmRetryBaseMs.value);
  const llmRetryMaxDelayMs = optU64(agentLlmRetryMaxMs.value);
  await startAgentStream({
    channel: agentChannel.value,
    provider: agentProvider.value,
    apiKey: agentApiKey.value,
    baseUrl: agentBaseUrl.value,
    model: agentModel.value,
    userText: agentUserText.value,
    ...(llmMaxRetries != null ? { llmMaxRetries } : {}),
    ...(llmRetryBaseDelayMs != null ? { llmRetryBaseDelayMs } : {}),
    ...(llmRetryMaxDelayMs != null ? { llmRetryMaxDelayMs } : {}),
    ...(agentChannel.value === "execution"
      ? { includePreviewTool: agentIncludePreview.value }
      : {}),
  });
}

onMounted(async () => {
  try {
    const md = "# Docwise\n\nNuxt 4 + Nuxt UI 已接入。\n";
    const result = await invoke<RenderPreviewResult>("preview_render", {
      content: md,
      snapshotId: null,
    });
    previewHtml.value = result.html;
  } catch (e) {
    errorMessage.value = e instanceof Error ? e.message : String(e);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <UContainer class="py-8">
    <UPageHeader
      title="Docwise"
      description="Markdown 文档体系与智能体协作（设计驱动开发中）"
    />

    <div class="mt-6 grid gap-6 lg:grid-cols-2">
      <UCard class="lg:col-span-2">
        <template #header>
          <div class="flex flex-wrap items-center justify-between gap-3">
            <span class="font-medium">
              智能体流式试跑（<code>planning_agent_turn_stream</code> /
              <code>execution_agent_turn_stream</code>）
            </span>
            <div class="flex flex-wrap items-center gap-2">
              <UBadge
                v-if="agentStatus === 'idle'"
                color="neutral"
                variant="subtle"
              >
                未开始
              </UBadge>
              <UBadge
                v-else-if="agentStatus === 'running'"
                color="info"
                variant="subtle"
              >
                进行中 seq={{ agentLastSeq }}
              </UBadge>
              <UBadge
                v-else-if="agentStatus === 'done'"
                color="success"
                variant="subtle"
              >
                回合结束
              </UBadge>
              <UBadge v-else color="error" variant="subtle">出错</UBadge>
              <UButton
                v-if="agentRunning"
                size="xs"
                color="neutral"
                variant="outline"
                @click="stopAgentListening"
              >
                停止监听
              </UButton>
            </div>
          </div>
        </template>

        <div class="grid gap-4 lg:grid-cols-2">
          <div class="space-y-3">
            <UFormField label="通道">
              <USelect
                v-model="agentChannel"
                :items="[
                  { value: 'planning', label: '规划 docwise:planning-agent' },
                  { value: 'execution', label: '执行 docwise:execution-agent' },
                ]"
                value-key="value"
                label-key="label"
                class="w-full"
              />
            </UFormField>
            <UFormField label="Provider">
              <USelect
                v-model="agentProvider"
                :items="[...providerOptions]"
                value-key="value"
                label-key="label"
                class="w-full"
              />
            </UFormField>
            <UFormField label="Base URL">
              <UInput v-model="agentBaseUrl" class="w-full" />
            </UFormField>
            <UFormField label="Model">
              <UInput v-model="agentModel" class="w-full" />
            </UFormField>
            <UFormField label="API Key（可空，视厂商而定）">
              <UInput
                v-model="agentApiKey"
                type="password"
                autocomplete="off"
                class="w-full"
              />
            </UFormField>
            <UCheckbox
              v-if="agentChannel === 'execution'"
              v-model="agentIncludePreview"
              label="注册内置 preview_render 工具（includePreviewTool）"
            />
            <UFormField label="用户消息">
              <UTextarea v-model="agentUserText" :rows="4" class="w-full" />
            </UFormField>
            <p class="text-muted text-xs">
              LLM 重试（可选）：仅对<strong>当前这一轮模型 HTTP</strong>重试，不撤销已跑过的工具。
            </p>
            <div class="grid gap-2 sm:grid-cols-3">
              <UFormField label="额外重试次数（默认 3）">
                <UInput
                  v-model="agentLlmMaxRetries"
                  placeholder="空=默认"
                  class="w-full"
                />
              </UFormField>
              <UFormField label="退避基数 ms（默认 500）">
                <UInput
                  v-model="agentLlmRetryBaseMs"
                  placeholder="空=默认"
                  class="w-full"
                />
              </UFormField>
              <UFormField label="退避上限 ms（默认 30000）">
                <UInput
                  v-model="agentLlmRetryMaxMs"
                  placeholder="空=默认"
                  class="w-full"
                />
              </UFormField>
            </div>
            <UButton
              :disabled="agentRunning || !agentUserText.trim()"
              @click="runAgentStream"
            >
              启动流式回合
            </UButton>
            <p v-if="agentError" class="text-error text-sm">
              {{ agentError }}
            </p>
          </div>
          <div class="flex min-h-48 flex-col gap-3">
            <p class="text-muted text-xs font-medium uppercase tracking-wide">
              Assistant（累积）
            </p>
            <pre
              class="bg-muted/30 max-h-64 flex-1 overflow-auto rounded-lg p-3 text-xs whitespace-pre-wrap"
              >{{ agentAssistant || "—" }}</pre
            >
            <p class="text-muted text-xs font-medium uppercase tracking-wide">
              事件日志
            </p>
            <pre
              class="bg-muted/30 max-h-48 overflow-auto rounded-lg p-3 text-xs whitespace-pre-wrap"
              >{{ agentLogLines.join("\n") || "—" }}</pre
            >
          </div>
        </div>
      </UCard>

      <UCard>
        <template #header>
          <div class="flex items-center justify-between gap-2">
            <span class="font-medium">预览（Tauri <code>preview_render</code>）</span>
            <UBadge v-if="loading" color="neutral" variant="subtle">加载中</UBadge>
            <UBadge v-else-if="errorMessage" color="error" variant="subtle">失败</UBadge>
            <UBadge v-else color="success" variant="subtle">就绪</UBadge>
          </div>
        </template>
        <p v-if="errorMessage" class="text-error text-sm">
          {{ errorMessage }}
        </p>
        <div v-else class="prose prose-sm dark:prose-invert max-w-none" v-html="previewHtml" />
      </UCard>

      <UCard>
        <template #header>
          <span class="font-medium">模块占位</span>
        </template>
        <ul class="text-muted space-y-2 text-sm">
          <li><code>app/components/project</code> — 蓝图</li>
          <li><code>app/components/board</code> — 任务看板</li>
          <li><code>app/components/editor</code> — 编辑器</li>
          <li><code>app/components/preview</code> — 预览容器</li>
          <li><code>app/components/chat</code> — 统一对话</li>
          <li><code>app/components/timeline</code> — 智能体时间轴</li>
        </ul>
      </UCard>
    </div>
  </UContainer>
</template>
