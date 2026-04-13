import { invoke } from "@tauri-apps/api/core";

import "./modules";

type RenderPreviewResult = {
  snapshotId: string;
  html: string;
  diagnostics: unknown[];
  assetBaseUrl: string;
  themeRevision: string;
};

async function main(): Promise<void> {
  const el = document.querySelector<HTMLDivElement>("#app");
  if (!el) {
    return;
  }

  try {
    const md = "# Docwise\n\n设计驱动骨架已就绪。\n";
    const result = await invoke<RenderPreviewResult>("preview_render", {
      content: md,
      snapshotId: null,
    });
    el.innerHTML = result.html;
  } catch (e) {
    el.textContent = e instanceof Error ? e.message : String(e);
  }
}

void main();
