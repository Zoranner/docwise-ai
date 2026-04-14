//! LLM HTTP/SSE 可重试错误的退避（供流式与非流式共用）。

use std::time::Duration;

use tokio::time::sleep;

/// `retry_index`：第几次重试的 0 起下标（0 = 首次重试前等待，对应指数 2^0）。
pub(crate) fn backoff_delay_ms(retry_index: u32, base_ms: u64, max_ms: u64) -> u64 {
    let pow = retry_index.min(16);
    let exp = base_ms.saturating_mul(1u64 << pow);
    exp.min(max_ms).max(base_ms)
}

pub(crate) async fn sleep_delay_ms(ms: u64) {
    sleep(Duration::from_millis(ms.max(1))).await;
}
