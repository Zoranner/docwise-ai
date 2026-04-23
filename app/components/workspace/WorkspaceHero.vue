<script setup lang="ts">
import Tag from "~/components/base/Tag.vue";

type WorkspaceMetric = {
  label: string;
  value: string | number;
  hint?: string;
};

withDefaults(
  defineProps<{
    kicker: string;
    title: string;
    description: string;
    metrics: WorkspaceMetric[];
    statusLabel?: string | null;
    statusTone?: "neutral" | "primary" | "success" | "warning" | "danger";
    meta?: string | null;
  }>(),
  {
    statusLabel: null,
    statusTone: "neutral",
    meta: null,
  },
);
</script>

<template>
  <div class="workspace-hero-shell">
    <div class="workspace-hero">
      <div class="workspace-hero__head">
        <p class="kicker">{{ kicker }}</p>
        <div v-if="statusLabel || meta" class="workspace-hero__meta">
          <p v-if="meta" class="meta-text">{{ meta }}</p>
          <Tag
            v-if="statusLabel"
            :tone="statusTone"
            variant="outline"
          >
            {{ statusLabel }}
          </Tag>
        </div>
      </div>

      <div class="workspace-hero__copy">
        <p class="focus-title">{{ title }}</p>
        <p class="support-text">{{ description }}</p>
      </div>
    </div>

    <div class="workspace-inline-metrics">
      <div
        v-for="metric in metrics"
        :key="metric.label"
        class="workspace-inline-metric"
      >
        <span class="workspace-inline-metric-label">{{ metric.label }}</span>
        <span class="workspace-inline-metric-value">{{ metric.value }}</span>
        <span v-if="metric.hint" class="workspace-inline-metric-hint">{{ metric.hint }}</span>
      </div>
    </div>
  </div>
</template>
