<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { AgentStreamChannel } from "~/types/planning-stream";

const workspacePathInput = ref("");
const workspaceOpenError = ref<string | null>(null);
const workspaceResolvedPath = ref<string | null>(null);

const {
  context: activeContext,
  errorMessage: activeContextError,
  refresh: refreshActiveContext,
  patch: patchActiveContext,
} = useDocwiseActiveContext();

const { lastCheckpointEvent } = useDocwiseCheckpointEvents(refreshActiveContext);

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

/** 相对工作区根，统一用 / */
const editorRelPath = ref("");
const editorContent = ref("");
const editorError = ref<string | null>(null);
const editorLoading = ref(false);

function normalizeWorkspaceRelPath(s: string): string {
  return s.trim().replace(/\\/g, "/");
}

async function loadEditorFile() {
  const rel = normalizeWorkspaceRelPath(editorRelPath.value);
  if (!rel) {
    editorError.value = "请填写相对路径";
    return;
  }
  editorError.value = null;
  editorLoading.value = true;
  try {
    editorContent.value = await invoke<string>("workspace_read_text_file", {
      path: rel,
    });
    editorRelPath.value = rel;
    await patchActiveContext({ filePath: rel });
  } catch (e) {
    editorError.value = e instanceof Error ? e.message : String(e);
  } finally {
    editorLoading.value = false;
  }
}

async function saveEditorFile() {
  const rel = normalizeWorkspaceRelPath(editorRelPath.value);
  if (!rel) {
    editorError.value = "请填写相对路径";
    return;
  }
  editorError.value = null;
  editorLoading.value = true;
  try {
    await invoke("workspace_write_text_file", {
      path: rel,
      content: editorContent.value,
    });
    editorRelPath.value = rel;
    await patchActiveContext({ filePath: rel });
  } catch (e) {
    editorError.value = e instanceof Error ? e.message : String(e);
  } finally {
    editorLoading.value = false;
  }
}

async function previewFromEditor() {
  const md = editorContent.value;
  if (!md.trim()) {
    editorError.value = "编辑器为空";
    return;
  }
  editorError.value = null;
  try {
    const snap = normalizeWorkspaceRelPath(editorRelPath.value);
    const result = await invoke<RenderPreviewResult>("preview_render", {
      content: md,
      snapshotId: snap || null,
    });
    previewHtml.value = result.html;
    errorMessage.value = null;
  } catch (e) {
    errorMessage.value = e instanceof Error ? e.message : String(e);
  }
}

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

async function openWorkspace() {
  const p = workspacePathInput.value.trim();
  if (!p) return;
  workspaceOpenError.value = null;
  try {
    await invoke("workspace_open", { path: p });
    workspaceResolvedPath.value = await invoke<string | null>(
      "workspace_get_path",
    );
    await refreshActiveContext();
  } catch (e) {
    workspaceOpenError.value = e instanceof Error ? e.message : String(e);
  }
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
  await refreshActiveContext();
  if (activeContext.value?.filePath) {
    editorRelPath.value = activeContext.value.filePath;
  }
  try {
    workspaceResolvedPath.value = await invoke<string | null>(
      "workspace_get_path",
    );
  } catch {
    workspaceResolvedPath.value = null;
  }
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
          <span class="font-medium">工作区与 ActiveContext</span>
        </template>
        <p class="text-muted mb-3 text-sm">
          打开本地目录后写入 <code>.agent/project.db</code>，并同步设计文档中的
          <code>ActiveContext</code>（供侧栏 / 看板 / 对话共用，当前为内存态）。
        </p>
        <div class="flex flex-wrap items-end gap-2">
          <UFormField label="工作区目录路径" class="min-w-48 flex-1">
            <UInput
              v-model="workspacePathInput"
              placeholder="例如 C:\path\to\workspace"
              class="w-full font-mono text-sm"
            />
          </UFormField>
          <UButton @click="openWorkspace">打开</UButton>
        </div>
        <p v-if="workspaceOpenError" class="text-error mt-2 text-sm">
          {{ workspaceOpenError }}
        </p>
        <p v-if="workspaceResolvedPath" class="text-muted mt-2 text-xs">
          已打开：<span class="font-mono">{{ workspaceResolvedPath }}</span>
        </p>
        <p v-if="activeContextError" class="text-error mt-2 text-sm">
          {{ activeContextError }}
        </p>
        <p class="text-muted mt-3 text-xs font-medium uppercase tracking-wide">
          ActiveContext（JSON）
        </p>
        <pre
          class="bg-muted/30 mt-1 max-h-40 overflow-auto rounded-lg p-3 text-xs whitespace-pre-wrap"
          >{{ activeContext ? JSON.stringify(activeContext, null, 2) : "—" }}</pre
        >
        <p class="text-muted mt-3 text-xs font-medium uppercase tracking-wide">
          最近检查点事件（<code>docwise:checkpoint-changed</code>）
        </p>
        <pre
          class="bg-muted/30 mt-1 max-h-32 overflow-auto rounded-lg p-3 text-xs whitespace-pre-wrap"
          >{{
            lastCheckpointEvent
              ? JSON.stringify(lastCheckpointEvent, null, 2)
              : "—"
          }}</pre
        >
      </UCard>

      <UCard class="lg:col-span-2">
        <template #header>
          <span class="font-medium">简易编辑器（FileBuffer 雏形）</span>
        </template>
        <p class="text-muted mb-3 text-sm">
          相对工作区根路径读写 UTF-8；打开/保存时会 <code>active_context_patch</code> 更新
          <code>filePath</code>，与设计文档
          <code>ActiveContext</code> 对齐。
        </p>
        <div class="flex flex-wrap items-end gap-2">
          <UFormField label="相对路径" class="min-w-64 flex-1">
            <UInput
              v-model="editorRelPath"
              placeholder="例如 docs/readme.md"
              class="w-full font-mono text-sm"
              :disabled="editorLoading"
            />
          </UFormField>
          <UButton
            :disabled="editorLoading || !workspaceResolvedPath"
            @click="loadEditorFile"
          >
            打开
          </UButton>
          <UButton
            :disabled="editorLoading || !workspaceResolvedPath"
            color="primary"
            @click="saveEditorFile"
          >
            保存
          </UButton>
          <UButton
            variant="outline"
            :disabled="!editorContent.trim()"
            @click="previewFromEditor"
          >
            预览编辑器内容
          </UButton>
        </div>
        <p v-if="editorError" class="text-error mt-2 text-sm">
          {{ editorError }}
        </p>
        <UTextarea
          v-model="editorContent"
          :rows="14"
          class="font-mono mt-3 w-full text-sm"
          placeholder="打开文件后在此编辑…"
        />
      </UCard>

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
