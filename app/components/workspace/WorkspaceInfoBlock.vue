<script setup lang="ts">
import Panel from "~/components/base/Panel.vue";

withDefaults(
  defineProps<{
    label: string;
    tone?: "default" | "muted";
  }>(),
  {
    tone: "default",
  },
);
</script>

<template>
  <Panel :tone="tone">
    <p class="metric-label">{{ label }}</p>
    <div class="workspace-info-block__content">
      <slot />
    </div>
  </Panel>
</template>
