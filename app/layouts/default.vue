<template>
  <div
    class="text-default bg-(--ui-bg) docwise-shell flex min-h-dvh flex-col antialiased"
  >
    <AppChromeNav />
    <DocwiseShell>
      <slot />
    </DocwiseShell>
  </div>
</template>
