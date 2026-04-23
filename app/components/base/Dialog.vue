<script setup lang="ts">
import BaseIcon from "~/components/base/BaseIcon.vue";

withDefaults(
  defineProps<{
    open: boolean;
    title: string;
    width?: "md" | "lg";
  }>(),
  {
    width: "md",
  },
);

const emit = defineEmits<{
  (e: "close"): void;
}>();
</script>

<template>
  <Teleport to="body">
    <div
      v-if="open"
      class="dialog-backdrop"
      @click.self="emit('close')"
    >
      <div class="dialog-shell" :class="`dialog-shell--${width}`">
        <div class="dialog-header">
          <h2 class="section-title">{{ title }}</h2>
          <button
            type="button"
            class="dialog-close"
            aria-label="关闭弹窗"
            @click="emit('close')"
          >
            <BaseIcon name="i-lucide-x" class="dialog-close__icon" aria-hidden="true" />
          </button>
        </div>

        <div class="dialog-body">
          <slot />
        </div>

        <div v-if="$slots.footer" class="dialog-footer">
          <slot name="footer" />
        </div>
      </div>
    </div>
  </Teleport>
</template>
