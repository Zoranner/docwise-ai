/** 与 `planning_events.rs` 中 Serde 形状对齐（envelope 与 wire 类型 camelCase；enum 载荷字段为 snake_case）。 */

export const PLANNING_AGENT_EVENT = "docwise:planning-agent";
export const EXECUTION_AGENT_EVENT = "docwise:execution-agent";

export type AgentStreamChannel = "planning" | "execution";

export type ToolCallWire = {
  id: string;
  name: string;
  arguments: string;
};

export type ChatMessageWire = {
  role: string;
  content?: string;
  toolCalls?: ToolCallWire[];
  toolCallId?: string;
  name?: string;
};

export type PlanningStreamEvent =
  | {
      kind: "turn_started";
      agent: string;
      provider: string;
      model: string;
      max_tool_rounds: number;
    }
  | { kind: "model_round"; round_index: number }
  | {
      kind: "llm_request_retry";
      /** 与 model_round.round_index 相同：仅重试该次模型 HTTP，非整轮从头 */
      model_round_index: number;
      attempt: number;
      max_attempts: number;
      delay_ms: number;
      reason: string;
    }
  | { kind: "llm_delta"; text: string }
  | {
      kind: "llm_round_finished";
      finish_reason: string;
      content: string | null;
    }
  | { kind: "tool_calls_parsed"; calls: ToolCallWire[] }
  | { kind: "tool_executing"; tool_call_id: string; name: string }
  | {
      kind: "tool_finished";
      tool_call_id: string;
      name: string;
      ok: boolean;
      result: unknown;
    }
  | {
      kind: "turn_finished";
      last_assistant_text: string | null;
      tool_rounds_completed: number;
      messages: ChatMessageWire[];
    }
  | {
      kind: "error";
      code: string;
      message: string;
      partial_messages?: ChatMessageWire[];
    };

export type PlanningStreamEnvelope = {
  runId: string;
  seq: number;
  event: PlanningStreamEvent;
};
