<template>
  <div
    class="text-default bg-(--ui-bg) docwise-shell flex min-h-dvh flex-col antialiased"
  >
    <AppChromeNav />
    <div class="min-h-0 flex-1">
      <slot />
    </div>
  </div>
</template>
