import { listen } from "@tauri-apps/api/event";
import {
  CHECKPOINT_CHANGED_EVENT,
  type CheckpointChangedEventPayload,
} from "~/types/checkpoint";

/** 订阅检查点 IPC 事件并刷新 ActiveContext（须传入同一 setup 中的 `refresh`）。 */
export function useDocwiseCheckpointEvents(
  refreshActiveContext: () => Promise<void>,
) {
  const lastCheckpointEvent = ref<CheckpointChangedEventPayload | null>(null);
  let unlisten: (() => void) | undefined;

  onMounted(async () => {
    unlisten = await listen<CheckpointChangedEventPayload>(
      CHECKPOINT_CHANGED_EVENT,
      (ev) => {
        lastCheckpointEvent.value = ev.payload;
        void refreshActiveContext();
      },
    );
  });

  onUnmounted(() => {
    unlisten?.();
  });

  return { lastCheckpointEvent };
}
