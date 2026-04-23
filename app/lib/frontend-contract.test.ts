import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const rootDir = join(import.meta.dir, "..");

const manualImportTargets = [
  "composables/useActiveContext.ts",
  "composables/useProjectCatalog.ts",
  "composables/useProjectData.ts",
  "composables/useProjectWorkspaceState.ts",
  "composables/useCheckpointEvents.ts",
  "composables/useAgentStream.ts",
  "pages/index.vue",
  "pages/projects/index.vue",
  "pages/projects/overview.vue",
  "pages/projects/[id].vue",
  "pages/workspace/index.vue",
  "components/project/ProjectRail.vue",
  "components/shell/ConversationRail.vue",
  "components/workspace/WorkspacePanel.vue",
  "components/workspace/BlueprintPane.vue",
  "components/workspace/TaskPane.vue",
  "components/workspace/LogPane.vue",
  "components/base/Panel.vue",
] as const;

const namingTargets = [
  "lib/project-prototype.ts",
  "composables/useProjectData.ts",
  "composables/useProjectWorkspaceState.ts",
  "composables/useActiveContext.ts",
  "composables/useProjectCatalog.ts",
  "composables/useCheckpointEvents.ts",
  "composables/useAgentStream.ts",
  "components/project/ProjectRail.vue",
  "components/shell/ConversationRail.vue",
  "components/workspace/WorkspacePanel.vue",
  "components/workspace/BlueprintPane.vue",
  "components/workspace/TaskPane.vue",
  "components/workspace/LogPane.vue",
  "pages/index.vue",
  "pages/projects/index.vue",
  "pages/projects/overview.vue",
  "pages/projects/[id].vue",
] as const;

const legacyNamingTargets = [
  "lib/docwise-prototype.ts",
  "composables/useDocwisePrototypeData.ts",
  "composables/useDocwisePrototypeState.ts",
  "composables/useDocwiseActiveContext.ts",
  "composables/useDocwiseProjectCatalog.ts",
  "composables/useDocwiseCheckpointEvents.ts",
  "composables/useDocwiseAgentStream.ts",
] as const;

const vueHelpers = ["ref", "computed", "watch", "onMounted", "onUnmounted"] as const;
const nuxtHelpers = ["useState", "navigateTo", "useRoute", "definePageMeta"] as const;

function read(relativePath: string) {
  return readFileSync(join(rootDir, relativePath), "utf8");
}

function usesHelper(source: string, helper: string) {
  return new RegExp(`\\b${helper}(?:<[^\\n]+?>)?\\s*\\(`).test(source);
}

function hasNamedImport(source: string, moduleName: string, helper: string) {
  return new RegExp(
    `import\\s*\\{[^}]*\\b${helper}\\b[^}]*\\}\\s*from\\s*["']${moduleName}["']`,
    "m",
  ).test(source);
}

test("frontend shell files explicitly import used vue and nuxt helpers", () => {
  const failures: string[] = [];

  for (const relativePath of manualImportTargets) {
    const source = read(relativePath);

    for (const helper of vueHelpers) {
      if (usesHelper(source, helper) && !hasNamedImport(source, "vue", helper)) {
        failures.push(`${relativePath} missing vue import for ${helper}`);
      }
    }

    for (const helper of nuxtHelpers) {
      if (usesHelper(source, helper) && !hasNamedImport(source, "#imports", helper)) {
        failures.push(`${relativePath} missing #imports import for ${helper}`);
      }
    }
  }

  expect(failures).toEqual([]);
});

test("project workspace frontend slice does not retain Docwise-prefixed public names", () => {
  const failures: string[] = [];

  for (const relativePath of legacyNamingTargets) {
    if (existsSync(join(rootDir, relativePath))) {
      failures.push(`${relativePath} should be removed`);
    }
  }

  for (const relativePath of namingTargets) {
    const source = read(relativePath);
    if (/\bDocwise[A-Z]/.test(source) || /useDocwise[A-Z]/.test(source)) {
      failures.push(relativePath);
    }
  }

  expect(failures).toEqual([]);
});
