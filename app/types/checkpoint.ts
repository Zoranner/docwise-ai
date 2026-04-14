/** 与后端 `CHECKPOINT_CHANGED_EVENT` 一致。 */
export const CHECKPOINT_CHANGED_EVENT = "docwise:checkpoint-changed" as const;

export type CheckpointDtoWire = {
  id: string;
  taskId: string;
  status: string;
  conversationRef: string;
  createdAt: string;
  updatedAt: string;
};

export type CheckpointChangedEventPayload = {
  action: "opened" | "closed";
  checkpoint: CheckpointDtoWire;
  taskStatusAfter: string | null;
};
