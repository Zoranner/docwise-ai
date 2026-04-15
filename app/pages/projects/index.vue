<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import type { DocwiseProjectCatalogEntry } from "~/types/project-catalog";

const { entries, hydrate, add, remove, touchWorkspacePath } =
  useDocwiseProjectCatalog();

const newName = ref("");
const newPath = ref("");
const newGroup = ref("");
const formError = ref<string | null>(null);
const openError = ref<string | null>(null);

const grouped = computed(() => {
  const map = new Map<string, DocwiseProjectCatalogEntry[]>();
  for (const e of entries.value) {
    const g = e.group?.trim() || "未分组";
    if (!map.has(g)) map.set(g, []);
    map.get(g)!.push(e);
  }
  return [...map.entries()].sort(([a], [b]) => {
    if (a === "未分组") return 1;
    if (b === "未分组") return -1;
    return a.localeCompare(b, "zh-CN");
  });
});

onMounted(() => {
  hydrate();
});

function resetForm() {
  newName.value = "";
  newPath.value = "";
  newGroup.value = "";
  formError.value = null;
}

function onAddProject() {
  const name = newName.value.trim();
  const workspacePath = newPath.value.trim();
  if (!name || !workspacePath) {
    formError.value = "请填写名称与工作区目录";
    return;
  }
  formError.value = null;
  add({ name, workspacePath, group: newGroup.value.trim() || undefined });
  resetForm();
}

async function openCatalogEntry(path: string) {
  openError.value = null;
  try {
    await invoke("workspace_open", { path });
    touchWorkspacePath(path);
    await navigateTo("/workspace");
  } catch (e) {
    openError.value = e instanceof Error ? e.message : String(e);
  }
}
</script>

<template>
  <div class="docwise-page">
    <header class="mb-10 sm:mb-12">
      <p
        class="text-primary mb-2 text-xs font-semibold tracking-widest uppercase"
      >
        Workspace
      </p>
      <h1 class="text-3xl font-semibold tracking-tight sm:text-4xl">项目中心</h1>
      <p class="docwise-section-desc mt-3">
        登记常用本地目录，一键打开并进入工作台。清单保存在本机；后端会话内可同时挂载多个工作区并在工作台切换前台。
      </p>
    </header>

    <div class="grid gap-10 lg:grid-cols-[minmax(0,22rem)_1fr] lg:gap-12 xl:grid-cols-[minmax(0,24rem)_1fr]">
      <aside class="lg:sticky lg:top-20 lg:self-start">
        <UCard
          :ui="{
            root: 'ring-default overflow-hidden rounded-2xl shadow-sm ring-1',
            header: 'border-default border-b px-5 py-4',
            body: 'p-5',
          }"
        >
          <template #header>
            <div>
              <h2 class="docwise-section-title">登记项目</h2>
              <p class="text-muted mt-0.5 text-xs">加入清单，便于重复打开</p>
            </div>
          </template>
          <div class="space-y-4">
            <UFormField label="显示名称" required>
              <UInput
                v-model="newName"
                size="md"
                placeholder="例如 客户 A 交付"
                class="w-full"
              />
            </UFormField>
            <UFormField label="工作区根目录" required>
              <UInput
                v-model="newPath"
                size="md"
                placeholder="绝对路径，例如 E:\work\project-a"
                class="w-full font-mono text-sm"
              />
            </UFormField>
            <UFormField label="分组（可选）">
              <UInput
                v-model="newGroup"
                size="md"
                placeholder="客户 / 内部 / 实验…"
                class="w-full"
              />
            </UFormField>
            <UAlert
              v-if="formError"
              color="error"
              variant="subtle"
              :title="formError"
            />
            <UButton block size="md" @click="onAddProject">加入清单</UButton>
          </div>
        </UCard>

        <UCard
          class="mt-4"
          :ui="{
            root: 'ring-default rounded-2xl ring-1',
            body: 'text-muted space-y-2 p-4 text-xs leading-relaxed',
          }"
        >
          <p>
            <strong class="text-default">总览</strong>
            页的跨项目指标依赖后端聚合，当前为占位。
          </p>
        </UCard>
      </aside>

      <section>
        <div class="mb-4 flex flex-wrap items-end justify-between gap-3">
          <div>
            <h2 class="docwise-section-title">已登记</h2>
            <p class="text-muted mt-1 text-sm">
              共 <span class="text-default font-medium">{{ entries.length }}</span>
              条
            </p>
          </div>
        </div>

        <UAlert
          v-if="openError"
          class="mb-4"
          color="error"
          variant="subtle"
          :title="openError"
        />

        <div
          v-if="entries.length === 0"
          class="border-default bg-(--ui-bg-elevated)/50 flex flex-col items-center justify-center rounded-2xl border border-dashed py-16 px-6 text-center"
        >
          <p class="text-muted text-sm">暂无项目</p>
          <p class="text-muted mt-2 max-w-sm text-xs">
            在左侧填写名称与目录，加入清单后即可在此打开。
          </p>
        </div>

        <div v-else class="space-y-8">
          <div v-for="[groupName, list] in grouped" :key="groupName">
            <h3
              class="text-muted mb-3 flex items-center gap-2 text-xs font-semibold tracking-wider uppercase"
            >
              {{ groupName }}
              <span
                class="bg-(--ui-bg-elevated) text-muted rounded-full px-2 py-0.5 text-[10px] font-normal tabular-nums"
                >{{ list.length }}</span
              >
            </h3>
            <ul
              class="border-default divide-default divide-y overflow-hidden rounded-2xl border bg-(--ui-bg-elevated) shadow-sm"
              role="list"
            >
              <li
                v-for="e in list"
                :key="e.id"
                class="hover:bg-(--ui-bg)/80 flex flex-col gap-3 p-4 transition-colors sm:flex-row sm:items-center sm:justify-between sm:gap-4"
              >
                <div class="min-w-0 flex-1">
                  <p class="text-default font-medium">{{ e.name }}</p>
                  <p
                    class="text-muted mt-1 break-all font-mono text-xs leading-snug"
                  >
                    {{ e.workspacePath }}
                  </p>
                </div>
                <div class="flex shrink-0 flex-wrap gap-2">
                  <UButton
                    size="sm"
                    @click="openCatalogEntry(e.workspacePath)"
                  >
                    打开
                  </UButton>
                  <UButton
                    size="sm"
                    color="neutral"
                    variant="ghost"
                    @click="remove(e.id)"
                  >
                    移除
                  </UButton>
                </div>
              </li>
            </ul>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>
