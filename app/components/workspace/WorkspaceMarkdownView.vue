<script setup lang="ts">
import { computed } from "vue";

type MarkdownBlock =
  | { type: "heading"; text: string }
  | { type: "paragraph"; text: string }
  | { type: "list"; items: string[] };

const props = defineProps<{
  source: string;
}>();

const blocks = computed<MarkdownBlock[]>(() => {
  const lines = props.source.split(/\r?\n/);
  const nextBlocks: MarkdownBlock[] = [];
  let paragraph: string[] = [];
  let listItems: string[] = [];

  function flushParagraph() {
    if (!paragraph.length) return;
    nextBlocks.push({
      type: "paragraph",
      text: paragraph.join(" "),
    });
    paragraph = [];
  }

  function flushList() {
    if (!listItems.length) return;
    nextBlocks.push({
      type: "list",
      items: [...listItems],
    });
    listItems = [];
  }

  for (const rawLine of lines) {
    const line = rawLine.trim();

    if (!line) {
      flushParagraph();
      flushList();
      continue;
    }

    if (line.startsWith("# ")) {
      flushParagraph();
      flushList();
      nextBlocks.push({ type: "heading", text: line.slice(2).trim() });
      continue;
    }

    if (line.startsWith("## ")) {
      flushParagraph();
      flushList();
      nextBlocks.push({ type: "heading", text: line.slice(3).trim() });
      continue;
    }

    if (line.startsWith("- ")) {
      flushParagraph();
      listItems.push(line.slice(2).trim());
      continue;
    }

    flushList();
    paragraph.push(line);
  }

  flushParagraph();
  flushList();

  return nextBlocks;
});
</script>

<template>
  <div class="workspace-markdown-surface">
    <template v-for="(block, index) in blocks" :key="`${block.type}-${index}`">
      <h4 v-if="block.type === 'heading'" class="workspace-markdown-surface__heading">
        {{ block.text }}
      </h4>

      <p v-else-if="block.type === 'paragraph'" class="workspace-markdown-surface__paragraph">
        {{ block.text }}
      </p>

      <ul v-else class="workspace-markdown-surface__list">
        <li
          v-for="item in block.items"
          :key="item"
          class="workspace-markdown-surface__list-item"
        >
          {{ item }}
        </li>
      </ul>
    </template>
  </div>
</template>
