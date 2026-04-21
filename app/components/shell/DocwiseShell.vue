<template>
  <div
    class="docwise-shell-grid min-h-[calc(100dvh-3.5rem)] grid lg:grid-cols-[320px_minmax(0,1fr)]"
  >
    <aside
      class="border-default bg-(--ui-bg-elevated) min-h-0 border-b lg:border-r lg:border-b-0"
    >
      <DocwiseConversationRail />
    </aside>
    <section class="min-h-0 flex min-w-0 flex-col">
      <DocwiseFocusHeader />
      <div class="min-h-0 flex-1 overflow-auto">
        <slot />
      </div>
    </section>
  </div>
</template>
