<script setup lang="ts">
import { useSlots } from "vue";
import BaseIcon from "~/components/base/BaseIcon.vue";

const slots = useSlots();

const props = withDefaults(
  defineProps<{
    modelValue: string;
    placeholder?: string;
    rows?: number;
    submitLabel?: string;
    attachmentLabel?: string;
  }>(),
  {
    placeholder: "",
    rows: 2,
    submitLabel: "发送",
    attachmentLabel: "添加附件",
  },
);

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
  (e: "submit" | "attach"): void;
}>();

function updateValue(event: Event) {
  emit("update:modelValue", (event.target as HTMLTextAreaElement).value);
}

function handleKeydown(event: KeyboardEvent) {
  if ((event.ctrlKey || event.metaKey) && event.key === "Enter") {
    event.preventDefault();
    emit("submit");
  }
}
</script>

<template>
  <div class="composer">
    <textarea
      class="composer__input"
      :value="modelValue"
      :rows="rows"
      :placeholder="placeholder"
      @input="updateValue"
      @keydown="handleKeydown"
    />

    <div class="composer__footer">
      <div class="composer__tools">
        <button
          type="button"
          class="composer__tool"
          :aria-label="props.attachmentLabel"
          @click="$emit('attach')"
        >
          <BaseIcon name="i-lucide-plus" class="composer__tool-icon" aria-hidden="true" />
        </button>

        <p v-if="slots.default" class="composer__meta">
          <slot />
        </p>
      </div>

      <button
        type="button"
        class="composer__submit"
        :aria-label="props.submitLabel"
        @click="$emit('submit')"
      >
        <BaseIcon name="i-lucide-arrow-up" class="composer__submit-icon" aria-hidden="true" />
      </button>
    </div>
  </div>
</template>
