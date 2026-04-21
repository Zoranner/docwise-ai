<script setup lang="ts">
definePageMeta({ layout: false });
await navigateTo("/projects/overview", { replace: true });
</script>

<template>
  <div
    class="text-muted flex min-h-dvh items-center justify-center bg-(--ui-bg) text-sm"
  >
    工作台已并入项目总览，正在跳转…
  </div>
</template>
