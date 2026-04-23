<script setup lang="ts">
withDefaults(
  defineProps<{
    tone?: "neutral" | "primary" | "success" | "warning" | "danger";
    variant?: "soft" | "outline" | "solid";
    size?: "sm" | "md";
  }>(),
  {
    tone: "neutral",
    variant: "soft",
    size: "sm",
  },
);
</script>

<template>
  <span
    class="tag"
    :class="[
      `tag--${tone}`,
      `tag--${variant}`,
      `tag--${size}`,
    ]"
  >
    <slot />
  </span>
</template>
