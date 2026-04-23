<script setup lang="ts">
type TabItem = {
  key: string;
  label: string;
};

const props = defineProps<{
  items: readonly TabItem[];
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

function selectTab(key: string) {
  if (key === props.modelValue) return;
  emit("update:modelValue", key);
}
</script>

<template>
  <nav class="tabs" aria-label="workspace tabs">
    <button
      v-for="item in items"
      :key="item.key"
      type="button"
      class="tabs__item"
      :class="{ 'is-active': item.key === modelValue }"
      @click="selectTab(item.key)"
    >
      {{ item.label }}
    </button>
  </nav>
</template>
