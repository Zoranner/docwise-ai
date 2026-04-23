import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const shellDir = import.meta.dir;
const railPath = join(shellDir, "ConversationRail.vue");

test("conversation rail uses unified left-aligned blocks for users and agents", () => {
  const rail = readFileSync(railPath, "utf8");

  expect(rail).toContain("message.role === 'user'");
  expect(rail).toContain("message.role === 'agent'");
  expect(rail).toContain("message.agentName");
  expect(rail).toContain("已处理");
  expect(rail).toContain("用户");
  expect(rail).toContain("message.handledDuration");
  expect(rail).toContain("message.presentation.kindLabel");
  expect(rail).toContain("message.activities");
  expect(rail).toContain('class="conversation-message"');
  expect(rail).toContain("conversation-message--user");
  expect(rail).toContain("conversation-message--agent");
  expect(rail).toContain('class="conversation-agent-activity"');
  expect(rail).not.toContain("conversation-message__time");
  expect(rail).not.toContain("{{ message.role }}");
  expect(rail).not.toContain("message.role === 'system'");
});

test("conversation rail keeps agent activities collapsed by default and composer uses tool-first footer", () => {
  const rail = readFileSync(railPath, "utf8");

  expect(rail).toContain("conversation-agent-activity__toggle");
  expect(rail).toContain("conversation-agent-activity__summary");
  expect(rail).toContain("activity.durationLabel");
  expect(rail).toContain("conversation-log-trigger");
  expect(rail).toContain("项目日志");
  expect(rail).not.toContain("conversation-log-trigger__count");
  expect(rail).toContain('attachment-label="添加附件"');
  expect(rail).toContain('submit-label="发送消息"');
  expect(rail).not.toContain("Ctrl/Cmd + Enter");
  expect(rail).not.toContain("conversation-agent-activity__files");
});
