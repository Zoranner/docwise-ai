import type { DocwiseProjectCatalogEntry } from "~/types/project-catalog";

const STORAGE_KEY = "docwise.projectCatalog.v1";

function readStorage(): DocwiseProjectCatalogEntry[] {
  if (!import.meta.client) return [];
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    const v = JSON.parse(raw) as DocwiseProjectCatalogEntry[];
    return Array.isArray(v) ? v : [];
  } catch {
    return [];
  }
}

export function useDocwiseProjectCatalog() {
  const entries = useState<DocwiseProjectCatalogEntry[]>(
    "docwise-project-catalog",
    () => [],
  );

  function persist(list: DocwiseProjectCatalogEntry[]) {
    if (import.meta.client) {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(list));
    }
    entries.value = list;
  }

  function hydrate() {
    entries.value = readStorage();
  }

  function add(
    entry: Omit<DocwiseProjectCatalogEntry, "id" | "updatedAt"> & {
      id?: string;
    },
  ) {
    const list = readStorage();
    const id = entry.id ?? crypto.randomUUID();
    list.push({
      ...entry,
      id,
      updatedAt: new Date().toISOString(),
    });
    persist(list);
  }

  function remove(id: string) {
    persist(readStorage().filter((e) => e.id !== id));
  }

  function touchWorkspacePath(path: string) {
    const list = readStorage();
    const norm = path.trim().replace(/\\/g, "/");
    const i = list.findIndex(
      (e) => e.workspacePath.trim().replace(/\\/g, "/") === norm,
    );
    if (i >= 0) {
      list[i] = {
        ...list[i],
        updatedAt: new Date().toISOString(),
      };
      persist(list);
    }
  }

  return { entries, hydrate, add, remove, persist, touchWorkspacePath };
}
