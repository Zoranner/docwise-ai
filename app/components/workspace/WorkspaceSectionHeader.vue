<script setup lang="ts">
import Tag from "~/components/base/Tag.vue";

defineProps<{
  title: string;
  count: string | number;
}>();
</script>

<template>
  <div class="workspace-section-header">
    <p class="section-heading">{{ title }}</p>
    <Tag>{{ count }}</Tag>
  </div>
</template>
