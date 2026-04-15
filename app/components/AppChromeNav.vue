<script setup lang="ts">
const route = useRoute();

const items = computed(() => {
  const p = route.path;
  return [
    {
      to: "/projects",
      label: "项目",
      active: p === "/projects",
    },
    {
      to: "/projects/overview",
      label: "总览",
      active: p === "/projects/overview",
    },
    {
      to: "/workspace",
      label: "工作台",
      active: p.startsWith("/workspace"),
    },
  ];
});
</script>

<template>
  <header
    class="border-default bg-(--ui-bg-elevated)/80 supports-backdrop-filter:bg-(--ui-bg-elevated)/60 sticky top-0 z-50 border-b backdrop-blur-md"
  >
    <div
      class="mx-auto flex h-14 max-w-[1400px] items-center gap-3 px-4 sm:gap-6 sm:px-6"
    >
      <NuxtLink
        to="/projects"
        class="text-default group flex items-center gap-2 no-underline"
      >
        <span
          class="bg-primary/15 text-primary ring-primary/20 flex h-8 w-8 items-center justify-center rounded-lg text-sm font-bold ring-1"
          >D</span
        >
        <span class="text-lg font-semibold tracking-tight">Docwise</span>
      </NuxtLink>

      <nav class="flex items-center gap-0.5 sm:gap-1" aria-label="主导航">
        <UButton
          v-for="item in items"
          :key="item.to"
          :to="item.to"
          size="sm"
          :variant="item.active ? 'soft' : 'ghost'"
          :color="item.active ? 'primary' : 'neutral'"
          class="rounded-lg"
        >
          {{ item.label }}
        </UButton>
      </nav>
    </div>
  </header>
</template>
