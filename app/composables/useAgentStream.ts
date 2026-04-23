import { computed, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import {
  type AgentStreamChannel,
  type PlanningStreamEnvelope,
  PLANNING_AGENT_EVENT,
  EXECUTION_AGENT_EVENT,
} from "~/types/planning-stream";

export type StartAgentStreamArgs = {
  channel: AgentStreamChannel;
  provider: string;
  apiKey: string;
  baseUrl: string;
  model: string;
  userText: string;
  timeoutSecs?: number;
  maxToolRounds?: number;
  /** 单次模型 HTTP 的额外重试次数（不含首试）；省略用后端默认 3 */
  llmMaxRetries?: number;
  llmRetryBaseDelayMs?: number;
  llmRetryMaxDelayMs?: number;
  /** 仅 execution；省略时由后端默认 true */
  includePreviewTool?: boolean;
};

export function useAgentStream() {
  const activeRunId = ref<string | null>(null);
  const status = ref<"idle" | "running" | "done" | "error">("idle");
  const errorMessage = ref<string | null>(null);
  const logLines = ref<string[]>([]);
  const assistantBuffer = ref("");
  const assistantRounds = ref<string[]>([]);
  const lastSeq = ref(0);

  let unlisten: UnlistenFn | null = null;

  async function detachListener() {
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
  }

  function pushLog(line: string) {
    logLines.value = [...logLines.value, line];
  }

  function handleEnvelope(runId: string, payload: PlanningStreamEnvelope) {
    if (payload.runId !== runId) return;
    lastSeq.value = payload.seq;
    const ev = payload.event;
    switch (ev.kind) {
      case "turn_started":
        pushLog(
          `[turn_started] agent=${ev.agent} provider=${ev.provider} model=${ev.model} max_tool_rounds=${ev.max_tool_rounds}`,
        );
        break;
      case "model_round":
        pushLog(`[model_round] round_index=${ev.round_index}`);
        break;
      case "llm_request_retry":
        assistantBuffer.value = "";
        pushLog(
          `[llm_request_retry] model_round=${ev.model_round_index} HTTP ${ev.attempt}/${ev.max_attempts} after ${ev.delay_ms}ms — ${ev.reason}`,
        );
        break;
      case "llm_delta":
        assistantBuffer.value += ev.text;
        break;
      case "llm_round_finished": {
        const piece =
          ev.content !== null && ev.content !== undefined && ev.content !== ""
            ? ev.content
            : assistantBuffer.value;
        assistantRounds.value = [...assistantRounds.value, piece];
        assistantBuffer.value = "";
        pushLog(`[llm_round_finished] ${ev.finish_reason}`);
        break;
      }
      case "tool_calls_parsed":
        pushLog(
          `[tool_calls_parsed] ${ev.calls.map((c) => `${c.name}(${c.id})`).join(", ")}`,
        );
        break;
      case "tool_executing":
        pushLog(`[tool_executing] ${ev.name} id=${ev.tool_call_id}`);
        break;
      case "tool_finished":
        pushLog(
          `[tool_finished] ${ev.name} ok=${ev.ok} id=${ev.tool_call_id}`,
        );
        break;
      case "turn_finished":
        pushLog(
          `[turn_finished] tool_rounds=${ev.tool_rounds_completed} messages=${ev.messages.length}`,
        );
        status.value = "done";
        break;
      case "error":
        status.value = "error";
        errorMessage.value = `${ev.code}: ${ev.message}`;
        pushLog(`[error] ${ev.code}: ${ev.message}`);
        break;
    }
  }

  async function start(args: StartAgentStreamArgs) {
    await detachListener();
    const runId = crypto.randomUUID();
    activeRunId.value = runId;
    status.value = "running";
    errorMessage.value = null;
    logLines.value = [];
    assistantBuffer.value = "";
    assistantRounds.value = [];
    lastSeq.value = 0;

    const eventName =
      args.channel === "planning"
        ? PLANNING_AGENT_EVENT
        : EXECUTION_AGENT_EVENT;

    unlisten = await listen<PlanningStreamEnvelope>(eventName, (e) => {
      handleEnvelope(runId, e.payload);
    });

    const baseReq = {
      runId,
      provider: args.provider,
      apiKey: args.apiKey,
      baseUrl: args.baseUrl,
      model: args.model,
      messages: [{ role: "user", content: args.userText }],
      ...(args.timeoutSecs != null ? { timeoutSecs: args.timeoutSecs } : {}),
      ...(args.maxToolRounds != null ? { maxToolRounds: args.maxToolRounds } : {}),
      ...(args.llmMaxRetries != null ? { llmMaxRetries: args.llmMaxRetries } : {}),
      ...(args.llmRetryBaseDelayMs != null
        ? { llmRetryBaseDelayMs: args.llmRetryBaseDelayMs }
        : {}),
      ...(args.llmRetryMaxDelayMs != null
        ? { llmRetryMaxDelayMs: args.llmRetryMaxDelayMs }
        : {}),
    };

    try {
      // Tauri 2：非注入参数按 Rust 形参名组包，此处结构体参数名为 `req`。
      if (args.channel === "planning") {
        await invoke("planning_agent_turn_stream", { req: baseReq });
      } else {
        await invoke("execution_agent_turn_stream", {
          req: {
            ...baseReq,
            ...(args.includePreviewTool !== undefined
              ? { includePreviewTool: args.includePreviewTool }
              : {}),
          },
        });
      }
    } catch (e) {
      status.value = "error";
      errorMessage.value = e instanceof Error ? e.message : String(e);
      await detachListener();
    }
  }

  async function stopListening() {
    await detachListener();
    if (status.value === "running") {
      status.value = "idle";
    }
    activeRunId.value = null;
  }

  onUnmounted(() => {
    void detachListener();
  });

  const assistantDisplay = computed(() => {
    const rounds = assistantRounds.value.join("\n---\n");
    const buf = assistantBuffer.value;
    if (!rounds && !buf) return "";
    if (!rounds) return buf;
    if (!buf) return rounds;
    return `${rounds}\n---\n${buf}`;
  });

  return {
    activeRunId,
    status,
    errorMessage,
    logLines,
    assistantBuffer,
    assistantRounds,
    assistantDisplay,
    lastSeq,
    start,
    stopListening,
  };
}
