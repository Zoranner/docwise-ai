import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const workspaceFilesUsingProps = [
  "TaskPane.vue",
  "WorkspacePanel.vue",
];

for (const fileName of workspaceFilesUsingProps) {
  test(`${fileName} binds defineProps when script uses props`, () => {
    const filePath = resolve(import.meta.dir, fileName);
    const source = readFileSync(filePath, "utf8");

    expect(source.includes("props.")).toBe(true);
    expect(source.includes("const props = defineProps<")).toBe(true);
  });
}
