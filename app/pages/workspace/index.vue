<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { AgentStreamChannel } from "~/types/planning-stream";
import type { WorkspaceSessionDto } from "~/types/workspace-session";

type DeskTab = "session" | "editor" | "agent" | "dev";

const deskTab = ref<DeskTab>("session");

const workspacePathInput = ref("");
const workspaceOpenError = ref<string | null>(null);
const workspaceResolvedPath = ref<string | null>(null);

const openSessions = ref<WorkspaceSessionDto[]>([]);
const selectedSessionId = ref<string | null>(null);

const sessionSelectItems = computed(() =>
  openSessions.value.map((s) => ({
    value: s.workspaceId,
    label: s.path.length > 64 ? `${s.path.slice(0, 61)}…` : s.path,
  })),
);

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

const editorRelPath = ref("");
const editorContent = ref("");
const editorError = ref<string | null>(null);
const editorLoading = ref(false);

function relPath(s: string): string {
  return s.trim().replace(/\\/g, "/");
}

async function refreshOpenSessions() {
  try {
    openSessions.value = await invoke<WorkspaceSessionDto[]>(
      "workspace_list_open",
    );
    const focused = openSessions.value.find((s) => s.focused);
    selectedSessionId.value =
      focused?.workspaceId ?? openSessions.value[0]?.workspaceId ?? null;
  } catch {
    openSessions.value = [];
    selectedSessionId.value = null;
  }
}

async function onSessionSelect(id: string | null) {
  if (!id) return;
  selectedSessionId.value = id;
  const cur = openSessions.value.find((s) => s.workspaceId === id);
  if (cur?.focused) return;
  workspaceOpenError.value = null;
  try {
    await invoke("workspace_focus", { workspace_id: id });
    await refreshActiveContext();
    workspaceResolvedPath.value = await invoke<string | null>(
      "workspace_get_path",
    );
    await refreshOpenSessions();
  } catch (e) {
    workspaceOpenError.value = e instanceof Error ? e.message : String(e);
  }
}

async function closeSelectedSession() {
  const id = selectedSessionId.value;
  if (!id) return;
  workspaceOpenError.value = null;
  try {
    await invoke("workspace_close", { workspace_id: id });
    await refreshOpenSessions();
    workspaceResolvedPath.value = await invoke<string | null>(
      "workspace_get_path",
    );
    await refreshActiveContext();
  } catch (e) {
    workspaceOpenError.value = e instanceof Error ? e.message : String(e);
  }
}

async function loadEditorFile() {
  const rel = relPath(editorRelPath.value);
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
  const rel = relPath(editorRelPath.value);
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
    const snap = relPath(editorRelPath.value);
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

const tabButtons: { id: DeskTab; label: string; hint: string }[] = [
  { id: "session", label: "连接", hint: "打开目录与上下文" },
  { id: "editor", label: "编辑", hint: "文件与预览" },
  { id: "agent", label: "智能体", hint: "流式试跑" },
  { id: "dev", label: "调试", hint: "JSON 与占位" },
];

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
    await refreshOpenSessions();
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
  await refreshOpenSessions();
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
  <div class="docwise-page">
    <UCard
      class="mb-8"
      :ui="{
        root: 'ring-default overflow-hidden rounded-2xl shadow-sm ring-1',
        body: 'p-5 sm:p-6',
      }"
    >
      <div
        class="flex flex-col gap-5 lg:flex-row lg:items-start lg:justify-between"
      >
        <div class="min-w-0 flex-1">
          <p
            class="text-primary mb-1 text-xs font-semibold tracking-widest uppercase"
          >
            Desk
          </p>
          <h1 class="text-2xl font-semibold tracking-tight sm:text-3xl">
            工作台
          </h1>
          <p class="text-muted mt-2 max-w-xl text-sm leading-relaxed">
            会话内可挂载多个工作区；切换前台后，文件读写与
            <code class="text-default bg-(--ui-bg) rounded px-1 py-px text-xs"
              >project.db</code
            >
            均指向当前前台根目录。
          </p>
        </div>
        <div
          class="border-default flex w-full min-w-0 flex-col gap-3 rounded-xl border bg-(--ui-bg)/60 p-4 sm:max-w-md lg:w-96"
        >
          <div class="flex flex-wrap items-center gap-2">
            <span class="text-muted shrink-0 text-xs font-medium">前台</span>
            <USelect
              v-if="openSessions.length"
              :model-value="selectedSessionId"
              :items="sessionSelectItems"
              value-key="value"
              label-key="label"
              size="sm"
              class="min-w-0 flex-1"
              placeholder="选择工作区"
              @update:model-value="onSessionSelect"
            />
            <span v-else class="text-muted text-xs">尚未打开工作区</span>
          </div>
          <div class="flex flex-wrap gap-2">
            <UButton
              size="sm"
              color="neutral"
              variant="outline"
              :disabled="!selectedSessionId"
              @click="closeSelectedSession"
            >
              从会话移除
            </UButton>
          </div>
          <UAlert
            v-if="workspaceOpenError"
            color="error"
            variant="subtle"
            :title="workspaceOpenError"
          />
        </div>
      </div>
    </UCard>

    <div
      class="bg-(--ui-bg-elevated) ring-default mb-8 inline-flex flex-wrap gap-1 rounded-xl p-1 ring-1"
      role="tablist"
      aria-label="工作台分区"
    >
      <UButton
        v-for="t in tabButtons"
        :key="t.id"
        size="sm"
        :variant="deskTab === t.id ? 'solid' : 'ghost'"
        :color="deskTab === t.id ? 'primary' : 'neutral'"
        class="rounded-lg"
        @click="deskTab = t.id"
      >
        <span>{{ t.label }}</span>
        <span class="text-muted hidden font-normal sm:inline">· {{ t.hint }}</span>
      </UButton>
    </div>

    <!-- 连接 -->
    <div v-show="deskTab === 'session'" class="space-y-6">
      <UCard
        :ui="{
          root: 'ring-default rounded-2xl ring-1',
          body: 'p-5 sm:p-6',
        }"
      >
        <h2 class="docwise-section-title">打开工作区</h2>
        <p class="docwise-section-desc">
          路径已在本会话打开过时，将只切换到前台而不重复初始化数据库。
        </p>
        <div class="mt-5 flex flex-col gap-3 sm:flex-row sm:items-end">
          <UFormField label="目录绝对路径" class="min-w-0 flex-1">
            <UInput
              v-model="workspacePathInput"
              size="md"
              placeholder="例如 E:\path\to\workspace"
              class="w-full font-mono text-sm"
            />
          </UFormField>
          <UButton size="md" class="shrink-0" @click="openWorkspace"
            >打开</UButton
          >
        </div>
        <p v-if="workspaceResolvedPath" class="text-muted mt-4 text-xs">
          当前根：
          <span class="text-default font-mono">{{ workspaceResolvedPath }}</span>
        </p>
        <UAlert
          v-if="activeContextError"
          class="mt-4"
          color="error"
          variant="subtle"
          :title="activeContextError"
        />
      </UCard>

      <details
        class="border-default group open:bg-(--ui-bg-elevated)/40 rounded-xl border"
      >
        <summary
          class="text-muted hover:text-default cursor-pointer list-none px-4 py-3 text-sm font-medium marker:hidden [&::-webkit-details-marker]:hidden"
        >
          <span class="inline-flex items-center gap-2">
            ActiveContext（开发）
            <span class="text-muted text-xs font-normal group-open:hidden"
              >展开</span
            >
            <span class="text-muted hidden text-xs font-normal group-open:inline"
              >收起</span
            >
          </span>
        </summary>
        <pre class="docwise-code-panel mx-4 mb-4">{{
          activeContext ? JSON.stringify(activeContext, null, 2) : "—"
        }}</pre>
      </details>

      <details
        class="border-default group open:bg-(--ui-bg-elevated)/40 rounded-xl border"
      >
        <summary
          class="text-muted hover:text-default cursor-pointer list-none px-4 py-3 text-sm font-medium marker:hidden [&::-webkit-details-marker]:hidden"
        >
          <span class="inline-flex items-center gap-2">
            最近检查点事件
            <span class="text-muted text-xs font-normal group-open:hidden"
              >展开</span
            >
            <span class="text-muted hidden text-xs font-normal group-open:inline"
              >收起</span
            >
          </span>
        </summary>
        <pre class="docwise-code-panel mx-4 mb-4">{{
          lastCheckpointEvent
            ? JSON.stringify(lastCheckpointEvent, null, 2)
            : "—"
        }}</pre>
      </details>
    </div>

    <!-- 编辑 -->
    <div v-show="deskTab === 'editor'" class="space-y-6">
      <UCard
        :ui="{
          root: 'ring-default rounded-2xl ring-1',
          body: 'p-5 sm:p-6',
        }"
      >
        <h2 class="docwise-section-title">文件</h2>
        <p class="docwise-section-desc">相对当前前台根路径读写 UTF-8 文本。</p>
        <div
          class="mt-5 flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-end"
        >
          <UFormField label="相对路径" class="min-w-0 flex-1 sm:min-w-[16rem]">
            <UInput
              v-model="editorRelPath"
              size="md"
              placeholder="docs/readme.md"
              class="w-full font-mono text-sm"
              :disabled="editorLoading"
            />
          </UFormField>
          <div class="flex flex-wrap gap-2">
            <UButton
              size="md"
              :disabled="editorLoading || !workspaceResolvedPath"
              @click="loadEditorFile"
            >
              读取
            </UButton>
            <UButton
              size="md"
              color="primary"
              :disabled="editorLoading || !workspaceResolvedPath"
              @click="saveEditorFile"
            >
              保存
            </UButton>
            <UButton
              size="md"
              variant="outline"
              :disabled="!editorContent.trim()"
              @click="previewFromEditor"
            >
              刷新预览
            </UButton>
          </div>
        </div>
        <p v-if="editorError" class="text-error mt-3 text-sm">{{ editorError }}</p>
      </UCard>

      <div class="grid gap-6 xl:grid-cols-2">
        <UCard
          :ui="{
            root: 'ring-default flex min-h-[22rem] flex-col rounded-2xl ring-1',
            body: 'flex flex-1 flex-col p-4 sm:p-5',
          }"
        >
          <p class="text-muted mb-2 text-xs font-medium uppercase">编辑器</p>
          <UTextarea
            v-model="editorContent"
            :rows="18"
            class="font-mono min-h-[18rem] w-full flex-1 text-sm"
            placeholder="读取文件后在此编辑…"
          />
        </UCard>
        <UCard
          :ui="{
            root: 'ring-default flex min-h-[22rem] flex-col rounded-2xl ring-1',
            body: 'flex flex-1 flex-col p-4 sm:p-5',
          }"
        >
          <div class="mb-2 flex items-center justify-between gap-2">
            <p class="text-muted text-xs font-medium uppercase">预览</p>
            <UBadge v-if="loading" color="neutral" variant="subtle" size="xs"
              >加载中</UBadge
            >
            <UBadge
              v-else-if="errorMessage"
              color="error"
              variant="subtle"
              size="xs"
              >失败</UBadge
            >
            <UBadge v-else color="success" variant="subtle" size="xs"
              >就绪</UBadge
            >
          </div>
          <p v-if="errorMessage" class="text-error text-sm">{{ errorMessage }}</p>
          <div
            v-else
            class="prose prose-sm dark:prose-invert max-h-[min(32rem,55vh)] flex-1 overflow-auto"
            v-html="previewHtml"
          />
        </UCard>
      </div>
    </div>

    <!-- 智能体 -->
    <div v-show="deskTab === 'agent'" class="space-y-6">
      <UCard
        :ui="{
          root: 'ring-default rounded-2xl ring-1',
          body: 'p-5 sm:p-6',
        }"
      >
        <div class="flex flex-wrap items-start justify-between gap-4">
          <div>
            <h2 class="docwise-section-title">流式回合</h2>
            <p class="docwise-section-desc">
              规划 / 执行通道试跑；状态与日志见右栏。
            </p>
          </div>
          <div class="flex flex-wrap items-center gap-2">
            <UBadge
              v-if="agentStatus === 'idle'"
              color="neutral"
              variant="subtle"
              >未开始</UBadge
            >
            <UBadge
              v-else-if="agentStatus === 'running'"
              color="info"
              variant="subtle"
              >进行中 · {{ agentLastSeq }}</UBadge
            >
            <UBadge
              v-else-if="agentStatus === 'done'"
              color="success"
              variant="subtle"
              >结束</UBadge
            >
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

        <div class="mt-6 grid gap-6 lg:grid-cols-2">
          <div class="space-y-4">
            <UFormField label="通道">
              <USelect
                v-model="agentChannel"
                :items="[
                  { value: 'planning', label: '规划' },
                  { value: 'execution', label: '执行' },
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
            <UFormField label="API Key">
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
              label="注册 preview_render 工具"
            />
            <UFormField label="用户消息">
              <UTextarea v-model="agentUserText" :rows="4" class="w-full" />
            </UFormField>
            <p class="text-muted text-xs">
              LLM 重试仅作用于当前轮 HTTP，不撤销已执行工具。
            </p>
            <div class="grid gap-2 sm:grid-cols-3">
              <UFormField label="重试次数">
                <UInput v-model="agentLlmMaxRetries" placeholder="默认" />
              </UFormField>
              <UFormField label="退避基数 ms">
                <UInput v-model="agentLlmRetryBaseMs" placeholder="默认" />
              </UFormField>
              <UFormField label="退避上限 ms">
                <UInput v-model="agentLlmRetryMaxMs" placeholder="默认" />
              </UFormField>
            </div>
            <UButton
              :disabled="agentRunning || !agentUserText.trim()"
              @click="runAgentStream"
            >
              启动回合
            </UButton>
            <p v-if="agentError" class="text-error text-sm">{{ agentError }}</p>
          </div>
          <div class="flex min-h-64 flex-col gap-4">
            <div>
              <p class="text-muted mb-1 text-xs font-medium uppercase">
                Assistant
              </p>
              <pre class="docwise-code-panel min-h-48">{{
                agentAssistant || "—"
              }}</pre>
            </div>
            <div>
              <p class="text-muted mb-1 text-xs font-medium uppercase">日志</p>
              <pre class="docwise-code-panel min-h-40">{{
                agentLogLines.join("\n") || "—"
              }}</pre>
            </div>
          </div>
        </div>
      </UCard>
    </div>

    <!-- 调试 / 占位 -->
    <div v-show="deskTab === 'dev'" class="space-y-6">
      <UCard
        :ui="{
          root: 'ring-default rounded-2xl ring-1',
          body: 'p-5 sm:p-6',
        }"
      >
        <h2 class="docwise-section-title">模块路线</h2>
        <ul class="text-muted mt-4 list-inside list-disc space-y-2 text-sm">
          <li><code class="text-default">project</code> — 蓝图</li>
          <li><code class="text-default">board</code> — 任务</li>
          <li><code class="text-default">editor</code> / <code>preview</code></li>
          <li><code class="text-default">chat</code> — 统一对话</li>
          <li><code class="text-default">timeline</code></li>
        </ul>
      </UCard>
    </div>
  </div>
</template>
