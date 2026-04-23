<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { Project } from "~/lib/project-prototype";
import BaseIcon from "~/components/base/BaseIcon.vue";
import Dialog from "~/components/base/Dialog.vue";

type ProjectEditorPayload = {
  name: string;
  workspacePath: string;
  readablePaths: string[];
};

const props = withDefaults(
  defineProps<{
    open: boolean;
    mode: "create" | "edit";
    project?: Project | null;
  }>(),
  {
    project: null,
  },
);

const emit = defineEmits<{
  (e: "close"): void;
  (e: "submit", payload: ProjectEditorPayload): void;
  (e: "archive", projectId: string): void;
}>();

const name = ref("");
const workspacePath = ref("");
const readablePaths = ref<string[]>([]);
const workspacePickerRef = ref<HTMLInputElement | null>(null);
const readablePathPickerRef = ref<HTMLInputElement | null>(null);

const dialogTitle = computed(() =>
  props.mode === "create" ? "新建项目" : "编辑项目",
);

const submitLabel = computed(() =>
  props.mode === "create" ? "创建项目" : "保存修改",
);

const canSubmit = computed(() =>
  name.value.trim().length > 0 &&
  (props.mode === "edit" || workspacePath.value.trim().length > 0),
);

watch(
  () => [props.open, props.project, props.mode] as const,
  () => {
    if (!props.open) return;

    name.value = props.project?.name ?? "";
    workspacePath.value = props.project?.workspacePath ?? "";
    readablePaths.value = [...(props.project?.readablePaths ?? [])];
  },
  { immediate: true },
);

function buildPayload(): ProjectEditorPayload {
  return {
    name: name.value,
    workspacePath: workspacePath.value,
    readablePaths: [...readablePaths.value],
  };
}

function submit() {
  emit("submit", buildPayload());
}

function archive() {
  if (!props.project) return;
  emit("archive", props.project.id);
}

function extractDirectoryPath(files: FileList | null) {
  const first = files?.[0];
  if (!first) return null;

  const relativePath = first.webkitRelativePath || "";
  const directoryName = relativePath.split("/")[0]?.trim();

  return directoryName || first.name;
}

function openWorkspacePicker() {
  if (props.mode === "edit") return;
  workspacePickerRef.value?.click();
}

function openReadablePathPicker() {
  readablePathPickerRef.value?.click();
}

function handleWorkspaceDirectoryChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const nextPath = extractDirectoryPath(input.files);

  if (nextPath) {
    workspacePath.value = nextPath;
  }

  input.value = "";
}

function handleReadableDirectoryChange(event: Event) {
  const input = event.target as HTMLInputElement;
  const nextPath = extractDirectoryPath(input.files);

  if (nextPath && !readablePaths.value.includes(nextPath)) {
    readablePaths.value = [...readablePaths.value, nextPath];
  }

  input.value = "";
}

function removeReadablePath(index: number) {
  readablePaths.value = readablePaths.value.filter((_, currentIndex) => currentIndex !== index);
}
</script>

<template>
  <Dialog :open="open" :title="dialogTitle" width="lg" @close="$emit('close')">
    <div class="project-dialog-form">
      <label class="project-dialog-field">
        <span class="metric-label">项目名称</span>
        <input
          v-model="name"
          type="text"
          class="project-dialog-input"
          placeholder="输入项目名称"
        />
      </label>

      <div class="project-dialog-field">
        <div class="project-dialog-field__header">
          <span class="metric-label">项目目录</span>
          <button
            type="button"
            class="project-dialog-picker"
            :disabled="mode === 'edit'"
            @click="openWorkspacePicker"
          >
            <BaseIcon name="i-lucide-folder-open" class="project-dialog-picker__icon" aria-hidden="true" />
            <span>浏览</span>
          </button>
        </div>

        <div class="project-dialog-path-row">
          <input
            v-model="workspacePath"
            type="text"
            class="project-dialog-input"
            placeholder="设置项目工作区所在目录"
            :disabled="mode === 'edit'"
          />
        </div>

        <input
          ref="workspacePickerRef"
          type="file"
          class="project-dialog-file-input"
          webkitdirectory
          directory
          @change="handleWorkspaceDirectoryChange"
        />
      </div>

      <div class="project-dialog-field">
        <div class="project-dialog-field__header">
          <span class="metric-label">附加可读目录</span>
          <button
            type="button"
            class="project-dialog-picker"
            @click="openReadablePathPicker"
          >
            <BaseIcon name="i-lucide-plus" class="project-dialog-picker__icon" aria-hidden="true" />
            <span>添加目录</span>
          </button>
        </div>

        <div v-if="readablePaths.length" class="project-dialog-path-list">
          <div
            v-for="(path, index) in readablePaths"
            :key="`${path}-${index}`"
            class="project-dialog-path-item"
          >
            <span class="project-dialog-path-item__text">{{ path }}</span>
            <button
              type="button"
              class="project-dialog-remove"
              aria-label="移除目录"
              @click="removeReadablePath(index)"
            >
              <BaseIcon name="i-lucide-trash-2" class="project-dialog-remove__icon" aria-hidden="true" />
            </button>
          </div>
        </div>

        <div v-else class="project-dialog-path-empty">
          还没有附加可读目录。
        </div>

        <input
          ref="readablePathPickerRef"
          type="file"
          class="project-dialog-file-input"
          webkitdirectory
          directory
          @change="handleReadableDirectoryChange"
        />
      </div>
    </div>

    <template #footer>
      <div class="dialog-actions dialog-actions--split">
        <button
          v-if="mode === 'edit' && project"
          type="button"
          class="dialog-button dialog-button--weak"
          @click="archive"
        >
          归档项目
        </button>

        <div class="dialog-actions">
          <button type="button" class="dialog-button dialog-button--ghost" @click="$emit('close')">
            取消
          </button>
          <button type="button" class="dialog-button dialog-button--primary" :disabled="!canSubmit" @click="submit">
            {{ submitLabel }}
          </button>
        </div>
      </div>
    </template>
  </Dialog>
</template>
