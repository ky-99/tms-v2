import { DropdownMenu as KobalteDropdownMenu } from "@kobalte/core/dropdown-menu";
import { For, Show } from "solid-js";
import { cn } from "../lib/utils";
import type { Tag } from "../types/tag";

// Plus アイコン
function PlusIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-4 w-4"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      stroke-width="2"
    >
      <path d="M5 12h14M12 5v14" />
    </svg>
  );
}

export interface TagFilterProps {
  availableTags: Tag[];
  selectedTags: string[]; // 選択中のタグ名リスト
  onTagsChange: (tags: string[]) => void;
}

/**
 * タグフィルターコンポーネント
 *
 * - 「+ Tags」ボタンをクリックでドロップダウンメニュー表示
 * - 全タグをチェックボックスリストで表示
 * - 複数選択可能（OR条件）
 * - 選択中のタグ数をボタンに表示
 */
export function TagFilter(props: TagFilterProps) {
  const toggleTag = (tagName: string) => {
    if (props.selectedTags.includes(tagName)) {
      // 選択解除
      props.onTagsChange(props.selectedTags.filter((t) => t !== tagName));
    } else {
      // 選択追加
      props.onTagsChange([...props.selectedTags, tagName]);
    }
  };

  const isTagSelected = (tagName: string) => {
    return props.selectedTags.includes(tagName);
  };

  const selectedCount = () => props.selectedTags.length;

  return (
    <KobalteDropdownMenu>
      <KobalteDropdownMenu.Trigger
        class={cn(
          "rounded-full px-3 py-1 text-xs font-medium transition-colors inline-flex items-center gap-1",
          selectedCount() > 0
            ? "bg-primary text-primary-foreground"
            : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
        )}
      >
        <PlusIcon />
        <span>Tags</span>
        <Show when={selectedCount() > 0}>
          <span>({selectedCount()})</span>
        </Show>
      </KobalteDropdownMenu.Trigger>
      <KobalteDropdownMenu.Portal>
        <KobalteDropdownMenu.Content
          class={cn(
            "z-50 min-w-[12rem] max-h-[20rem] overflow-y-auto rounded-md border border-border bg-card p-2 shadow-lg",
            "data-[expanded]:animate-in data-[closed]:animate-out",
            "data-[closed]:fade-out-0 data-[expanded]:fade-in-0",
            "data-[closed]:zoom-out-95 data-[expanded]:zoom-in-95"
          )}
        >
          <Show
            when={props.availableTags.length > 0}
            fallback={
              <div class="px-3 py-2 text-sm text-muted-foreground">
                No tags available
              </div>
            }
          >
            <For each={props.availableTags}>
              {(tag) => (
                <label
                  class="flex items-center gap-2 rounded-sm px-3 py-2 text-sm cursor-pointer hover:bg-secondary transition-colors"
                  onClick={(e) => {
                    e.preventDefault();
                    toggleTag(tag.name);
                  }}
                >
                  <input
                    type="checkbox"
                    checked={isTagSelected(tag.name)}
                    onChange={() => toggleTag(tag.name)}
                    class="h-4 w-4 rounded border-border text-primary focus:ring-1 focus:ring-primary cursor-pointer"
                  />
                  <Show when={tag.color}>
                    <div
                      class="h-3 w-3 rounded-full flex-shrink-0"
                      style={{ "background-color": tag.color }}
                    />
                  </Show>
                  <span class="flex-1">{tag.name}</span>
                  <span class="text-xs text-muted-foreground">
                    ({tag.usageCount})
                  </span>
                </label>
              )}
            </For>
          </Show>
        </KobalteDropdownMenu.Content>
      </KobalteDropdownMenu.Portal>
    </KobalteDropdownMenu>
  );
}
