<script setup lang="ts">
definePageMeta({ layout: false });
await navigateTo("/projects", { replace: true });
</script>

<template>
  <div
    class="text-muted flex min-h-dvh items-center justify-center bg-(--ui-bg) text-sm"
  >
    正在进入项目中心…
  </div>
</template>
