<script setup lang="ts">
const { entries, hydrate } = useDocwiseProjectCatalog();
const { context, refresh } = useDocwiseActiveContext();

const quickLinks = [
  {
    to: "/projects",
    label: "项目清单",
    hint: "登记、打开、管理项目目录",
  },
  {
    to: "/projects/overview",
    label: "监看总览",
    hint: "跨项目观察当前会话状态",
  },
] as const;

onMounted(async () => {
  hydrate();
  await refresh();
});
</script>

<template>
  <div class="flex h-full min-h-0 flex-col gap-4 p-4 sm:p-5">
    <div>
      <p class="text-primary text-xs font-semibold tracking-widest uppercase">
        Command
      </p>
      <h2 class="mt-2 text-xl font-semibold tracking-tight">全局对话</h2>
      <p class="text-muted mt-2 text-sm leading-relaxed">
        这里代表跨所有项目的总指挥席位；右侧只负责监看，不承载文件预览主链。
      </p>
    </div>

    <UCard
      :ui="{
        root: 'ring-default rounded-2xl ring-1',
        body: 'space-y-3 p-4',
      }"
    >
      <div class="flex items-center justify-between gap-3">
        <p class="text-sm font-medium">当前焦点</p>
        <UBadge color="neutral" variant="subtle" size="xs">全局</UBadge>
      </div>
      <div class="space-y-2 text-sm">
        <p class="text-muted">
          工作区：
          <span class="text-default break-all font-mono">
            {{ context?.workspaceId || "未打开" }}
          </span>
        </p>
        <p class="text-muted">
          项目：
          <span class="text-default">{{ context?.projectId || "未聚焦" }}</span>
        </p>
        <p class="text-muted">
          任务：
          <span class="text-default">{{ context?.taskId || "未聚焦" }}</span>
        </p>
      </div>
    </UCard>

    <UCard
      :ui="{
        root: 'ring-default rounded-2xl ring-1',
        body: 'space-y-3 p-4',
      }"
    >
      <div class="flex items-center justify-between gap-3">
        <p class="text-sm font-medium">控制台入口</p>
        <UBadge color="primary" variant="subtle" size="xs">
          {{ entries.length }} 项目
        </UBadge>
      </div>
      <div class="space-y-2">
        <NuxtLink
          v-for="link in quickLinks"
          :key="link.to"
          :to="link.to"
          class="border-default hover:bg-(--ui-bg) block rounded-xl border px-3 py-2 no-underline transition-colors"
        >
          <p class="text-default text-sm font-medium">{{ link.label }}</p>
          <p class="text-muted mt-1 text-xs">{{ link.hint }}</p>
        </NuxtLink>
      </div>
    </UCard>

    <div class="border-default bg-(--ui-bg) mt-auto rounded-2xl border p-4">
      <p class="text-default text-sm font-medium">对话定位</p>
      <p class="text-muted mt-2 text-xs leading-relaxed">
        v0 先打通“项目 → 蓝图 → 任务 → 执行 → 文档落盘 → 人工审议”主闭环；文件查看编辑交给外部工具。
      </p>
    </div>
  </div>
</template>
