<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    tone?: "default" | "muted";
    padding?: "none" | "sm" | "md";
  }>(),
  {
    tone: "default",
    padding: "md",
  },
);

const bodyClass = computed(() => {
  return [
    "panel__body",
    `panel__body--${props.padding}`,
  ];
});
</script>

<template>
  <section class="panel" :class="[`panel--${tone}`]">
    <header v-if="$slots.header" class="panel__header">
      <slot name="header" />
    </header>

    <div :class="bodyClass">
      <slot />
    </div>

    <footer v-if="$slots.footer" class="panel__footer">
      <slot name="footer" />
    </footer>
  </section>
</template>
