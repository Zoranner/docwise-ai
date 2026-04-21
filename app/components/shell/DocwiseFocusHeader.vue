<script setup lang="ts">
const route = useRoute();
const { context, refresh } = useDocwiseActiveContext();

const headerMeta = computed(() => {
  if (route.path === "/projects/overview") {
    return {
      title: "跨项目监看",
      description: "查看当前会话的项目清单、聚合状态与全局执行视角。",
    };
  }
  if (route.path === "/projects") {
    return {
      title: "项目目录",
      description: "维护项目入口并切换右侧监看内容。",
    };
  }
  if (route.path.startsWith("/projects/")) {
    return {
      title: "项目详情",
      description: "在右侧切换总览、蓝图、任务、审议与产出。",
    };
  }
  return {
    title: "监看区",
    description: "右侧承载项目观察与编排结果，不再承担编辑器与预览器职责。",
  };
});

onMounted(refresh);
</script>

<template>
  <header class="border-default bg-(--ui-bg)/90 border-b px-4 py-4 sm:px-6">
    <div class="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
      <div class="min-w-0">
        <p class="text-primary text-xs font-semibold tracking-widest uppercase">
          Monitor
        </p>
        <h1 class="mt-1 text-2xl font-semibold tracking-tight">
          {{ headerMeta.title }}
        </h1>
        <p class="text-muted mt-2 text-sm leading-relaxed">
          {{ headerMeta.description }}
        </p>
      </div>
      <div class="flex flex-wrap gap-2">
        <UBadge color="neutral" variant="subtle">
          工作区 {{ context?.workspaceId || "未打开" }}
        </UBadge>
        <UBadge color="neutral" variant="subtle">
          审议 {{ context?.reviewId || "无" }}
        </UBadge>
        <UBadge color="neutral" variant="subtle">
          产出 {{ context?.outputId || "无" }}
        </UBadge>
      </div>
    </div>
  </header>
</template>
