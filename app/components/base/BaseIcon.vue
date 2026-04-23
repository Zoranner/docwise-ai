<script setup lang="ts">
import { Icon as NuxtIcon } from "#components";

defineProps<{
  name: string;
  size?: string | number;
  mode?: "css" | "svg";
}>();
</script>

<template>
  <NuxtIcon :name="name" :size="size" :mode="mode" v-bind="$attrs" />
</template>
