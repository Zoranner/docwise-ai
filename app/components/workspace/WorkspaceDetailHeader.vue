<script setup lang="ts">
import Tag from "~/components/base/Tag.vue";

withDefaults(
  defineProps<{
    kicker: string;
    title: string;
    statusLabel?: string | null;
    statusTone?: "neutral" | "primary" | "success" | "warning" | "danger";
  }>(),
  {
    statusLabel: null,
    statusTone: "neutral",
  },
);
</script>

<template>
  <div class="panel-header-row">
    <div class="workspace-detail-header__titleline">
      <span class="kicker workspace-detail-header__kicker">{{ kicker }}</span>
      <span class="workspace-detail-header__separator" aria-hidden="true">/</span>
      <h3 class="focus-title workspace-detail-title truncate-heading">{{ title }}</h3>
    </div>
    <Tag
      v-if="statusLabel"
      :tone="statusTone"
    >
      {{ statusLabel }}
    </Tag>
  </div>
</template>
