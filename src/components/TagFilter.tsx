import { DropdownMenu as KobalteDropdownMenu } from "@kobalte/core/dropdown-menu";
import { For, Show } from "solid-js";
import { cn, truncateText } from "../lib/utils";
import type { Tag } from "../types/tag";
import { PlusIcon, CheckIcon } from "./icons";

export interface TagFilterProps {
  availableTags: Tag[];
  selectedTags: string[]; // 選択中のタグ名リスト
  onTagsChange: (tags: string[]) => void;
}

/**
 * タグフィルターコンポーネント（Kobalte DropdownMenu）
 *
 * - 「+ Tags」ボタンをクリックでメニュー表示
 * - 全タグをメニューアイテムで表示（Selectスタイル）
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
            "z-50 min-w-[12rem] max-h-60 overflow-y-auto rounded-md border border-border bg-card shadow-lg",
            "data-[expanded]:animate-in data-[closed]:animate-out",
            "data-[closed]:fade-out-0 data-[expanded]:fade-in-0",
            "data-[closed]:zoom-out-95 data-[expanded]:zoom-in-95"
          )}
        >
          <Show
            when={props.availableTags.length > 0}
            fallback={
              <div class="px-3 py-6 text-center text-sm text-muted-foreground">
                No tags available
              </div>
            }
          >
            <div class="p-1">
              <For each={props.availableTags}>
                {(tag) => (
                  <KobalteDropdownMenu.Item
                    onSelect={() => toggleTag(tag.name)}
                    closeOnSelect={false}
                    class={cn(
                      "relative flex w-full items-center gap-2 rounded-sm px-3 py-2 text-sm outline-none cursor-pointer transition-colors text-left",
                      "hover:bg-secondary data-[highlighted]:bg-secondary",
                      isTagSelected(tag.name) && "bg-primary/10 text-primary font-medium"
                    )}
                  >
                    <Show when={tag.color}>
                      <div
                        class="h-3 w-3 rounded-full flex-shrink-0"
                        style={{ "background-color": tag.color }}
                      />
                    </Show>
                    <span class="flex-1">{truncateText(tag.name, 30)}</span>
                    <Show when={isTagSelected(tag.name)}>
                      <CheckIcon />
                    </Show>
                  </KobalteDropdownMenu.Item>
                )}
              </For>
            </div>
          </Show>
        </KobalteDropdownMenu.Content>
      </KobalteDropdownMenu.Portal>
    </KobalteDropdownMenu>
  );
}
