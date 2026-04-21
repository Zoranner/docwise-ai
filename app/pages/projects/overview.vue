<script setup lang="ts">
const { entries, hydrate } = useDocwiseProjectCatalog();

onMounted(() => {
  hydrate();
});
</script>

<template>
  <div class="docwise-page">
    <header class="mb-10">
      <p
        class="text-primary mb-2 text-xs font-semibold tracking-widest uppercase"
      >
        Overview
      </p>
      <h1 class="text-3xl font-semibold tracking-tight sm:text-4xl">跨项目总览</h1>
      <p class="docwise-section-desc mt-3">
        任务漏斗、审议队列、蓝图状态等需后端跨多个
        <code class="text-default bg-(--ui-bg-elevated) rounded px-1 py-0.5 text-xs"
          >project.db</code
        >
        聚合；当前仅展示清单规模占位。
      </p>
    </header>

    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <UCard
        :ui="{
          root: 'ring-default rounded-2xl ring-1',
          body: 'p-6',
        }"
      >
        <p class="text-muted text-xs font-medium uppercase">清单条目</p>
        <p class="text-default mt-2 text-3xl font-semibold tabular-nums">
          {{ entries.length }}
        </p>
        <p class="text-muted mt-2 text-xs">本机登记的快捷打开项</p>
      </UCard>
      <UCard
        :ui="{
          root: 'ring-default rounded-2xl ring-1',
          body: 'p-6',
        }"
      >
        <p class="text-muted text-xs font-medium uppercase">聚合状态</p>
        <p class="text-muted mt-2 text-sm leading-relaxed">待接入后端查询</p>
      </UCard>
      <UCard
        class="sm:col-span-2 lg:col-span-1"
        :ui="{
          root: 'ring-default rounded-2xl ring-1',
          body: 'p-6',
        }"
      >
        <p class="text-muted text-xs font-medium uppercase">会话内工作区</p>
        <p class="text-muted mt-2 text-sm leading-relaxed">
          打开项目后，在左侧全局对话指挥，在右侧监看区观察执行与结果。
        </p>
      </UCard>
    </div>
  </div>
</template>
