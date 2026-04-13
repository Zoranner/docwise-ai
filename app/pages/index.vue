<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

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
