use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use lmkit::{create_chat_provider, Provider, ProviderConfig};
use serde::Deserialize;
use tauri::async_runtime;
use tauri::{AppHandle, Emitter, State};
use uuid::Uuid;

use crate::app::checkpoint::CheckpointBridge;
use crate::app::execution::{
    agentool_bundles, run_execution_turn_stream, wire_to_chat, ChatMessageWire,
    ExecutionStreamTurn, PlanningLoopConfig, PlanningStreamEnvelope, PlanningStreamEvent,
    EXECUTION_AGENT_EVENT,
};
use crate::app::state::{SharedActiveContext, SharedProject};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionAgentStreamRequest {
    #[serde(default)]
    pub run_id: Option<String>,
    pub provider: String,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    pub messages: Vec<ChatMessageWire>,
    #[serde(default)]
    pub max_tool_rounds: Option<u32>,
    #[serde(default)]
    pub llm_max_retries: Option<u32>,
    #[serde(default)]
    pub llm_retry_base_delay_ms: Option<u64>,
    #[serde(default)]
    pub llm_retry_max_delay_ms: Option<u64>,
    /// 是否注册内置 `preview_render`（默认 true，与设计文档 v1 可选 preview 一致）。
    #[serde(default = "default_include_preview")]
    pub include_preview_tool: bool,
}

fn default_include_preview() -> bool {
    true
}

/// 启动文档执行智能体流式回合：事件名为 [`EXECUTION_AGENT_EVENT`]，载荷同规划侧 [`PlanningStreamEnvelope`]。
#[tauri::command]
pub fn execution_agent_turn_stream(
    app: AppHandle,
    state: State<'_, SharedProject>,
    active: State<'_, SharedActiveContext>,
    req: ExecutionAgentStreamRequest,
) -> Result<(), String> {
    let run_id = req.run_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let provider = Provider::from_str(&req.provider).map_err(|e| e.to_string())?;
    let cfg = ProviderConfig {
        provider,
        api_key: req.api_key,
        base_url: req.base_url,
        model: req.model,
        dimension: None,
        timeout: req.timeout_secs.map(Duration::from_secs),
        max_concurrent: None,
    };

    let messages: Vec<_> = req
        .messages
        .into_iter()
        .map(wire_to_chat)
        .collect::<Result<_, String>>()?;

    let shared = state.deref().clone();
    let checkpoint_bridge = Arc::new(CheckpointBridge::new(app.clone(), active.deref().clone()));
    let config = PlanningLoopConfig::for_agent_command(
        req.max_tool_rounds,
        req.llm_max_retries,
        req.llm_retry_base_delay_ms,
        req.llm_retry_max_delay_ms,
    );
    let include_preview_tool = req.include_preview_tool;

    let run_id_emit = run_id.clone();
    let app = app.clone();

    async_runtime::spawn(async move {
        let run_id_for_stream = run_id_emit.clone();
        let emit = move |envelope: PlanningStreamEnvelope| {
            let _ = app.emit(EXECUTION_AGENT_EVENT, &envelope);
        };

        let chat = match create_chat_provider(&cfg) {
            Ok(c) => c,
            Err(e) => {
                emit(PlanningStreamEnvelope {
                    run_id: run_id_for_stream.clone(),
                    seq: 1,
                    event: PlanningStreamEvent::Error {
                        code: "provider_init".into(),
                        message: e.format_diagnostic(),
                        partial_messages: None,
                    },
                });
                return;
            }
        };

        let provider_label = cfg.provider.to_string();
        let model_label = cfg.model.clone();

        let additional_tools = match shared.0.lock().await.focused_context() {
            None => Vec::new(),
            Some(ctx) => {
                match agentool_bundles::workspace_tools_executor(ctx.workspace_root().to_path_buf())
                {
                    Ok(v) => v,
                    Err(msg) => {
                        emit(PlanningStreamEnvelope {
                            run_id: run_id_for_stream.clone(),
                            seq: 1,
                            event: PlanningStreamEvent::Error {
                                code: "agentool_init".into(),
                                message: msg,
                                partial_messages: None,
                            },
                        });
                        return;
                    }
                }
            }
        };

        let turn = ExecutionStreamTurn {
            provider: chat.as_ref(),
            shared,
            messages,
            additional_tools,
            config,
            run_id: &run_id_for_stream,
            provider_label: &provider_label,
            model_label: &model_label,
            include_preview_tool,
            checkpoint_bridge: Some(checkpoint_bridge),
        };
        let _ = run_execution_turn_stream(turn, emit).await;
    });

    Ok(())
}
