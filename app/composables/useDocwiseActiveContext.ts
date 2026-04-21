import { invoke } from "@tauri-apps/api/core";
import { patchFocusContext } from "~/lib/focus-context";
import type { ActiveContext } from "~/types/active-context";

export function useDocwiseActiveContext() {
  const context = ref<ActiveContext | null>(null);
  const errorMessage = ref<string | null>(null);

  async function refresh() {
    errorMessage.value = null;
    try {
      context.value = await invoke<ActiveContext>("active_context_get");
    } catch (e) {
      errorMessage.value = e instanceof Error ? e.message : String(e);
    }
  }

  /** 整份替换；已打开工作区时 `workspaceId` 须与当前根路径一致。 */
  async function replace(ctx: ActiveContext) {
    errorMessage.value = null;
    try {
      await invoke("active_context_replace", { ctx });
      await refresh();
    } catch (e) {
      errorMessage.value = e instanceof Error ? e.message : String(e);
      throw e;
    }
  }

  /** 在已有上下文上合并字段后写回。 */
  async function patch(partial: Partial<ActiveContext>) {
    if (!context.value) await refresh();
    const cur = context.value;
    if (!cur) return;
    await replace(patchFocusContext(cur, partial));
  }

  return {
    context,
    errorMessage,
    refresh,
    replace,
    patch,
  };
}
