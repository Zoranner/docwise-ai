<script setup lang="ts">
import ShellLayout from "~/components/shell/Shell.vue";
</script>

<template>
  <div class="app-frame">
    <ShellLayout>
      <slot />
    </ShellLayout>
  </div>
</template>
