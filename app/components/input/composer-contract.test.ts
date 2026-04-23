import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const inputDir = import.meta.dir;
const composerPath = join(inputDir, "Composer.vue");

test("composer exposes attachment tool button and icon-style submit action without footer dividers", () => {
  const source = readFileSync(composerPath, "utf8");

  expect(source).toContain("attachmentLabel");
  expect(source).toContain("composer__tool");
  expect(source).toContain("composer__submit-icon");
  expect(source).toContain("$emit('attach')");
  expect(source).not.toContain("composer__hint");
});
