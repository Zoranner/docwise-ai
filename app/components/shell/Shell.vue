<script setup lang="ts">
import ProjectRail from "~/components/project/ProjectRail.vue";
import ConversationRail from "~/components/shell/ConversationRail.vue";
import { useShellLayoutState } from "~/composables/useShellLayoutState";

const { projectRailVisible } = useShellLayoutState();
</script>

<template>
  <div
    class="app-shell"
    :class="projectRailVisible ? '' : 'app-shell--rail-hidden'"
  >
    <aside
      class="shell-rail"
      :aria-hidden="projectRailVisible ? 'false' : 'true'"
    >
      <ProjectRail />
    </aside>

    <aside class="shell-chat">
      <ConversationRail />
    </aside>

    <main class="shell-workspace min-w-0">
      <slot />
    </main>
  </div>
</template>
