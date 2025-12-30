import { Popover as KobaltePopover } from "@kobalte/core/popover";
import { JSX, Show, For } from "solid-js";
import { cn } from "../lib/utils";
import type { Task } from "../types/task";
import type { Tag } from "../types/tag";

interface TaskHoverPopupProps {
  task: Task;
  availableTags: Tag[];
  children: JSX.Element;
}

/**
 * タスク詳細ポップアップコンポーネント
 *
 * タイトルクリックで詳細ポップアップを表示します。
 * - description、tagsを表示（コンパクト版）
 */
export function TaskHoverPopup(props: TaskHoverPopupProps) {
  // Get tag color from available tags
  const getTagColor = (tagName: string): string | undefined => {
    return props.availableTags.find((t) => t.name === tagName)?.color;
  };

  return (
    <KobaltePopover placement="top">
      <KobaltePopover.Trigger
        class="outline-none focus:outline-none inline-block max-w-full"
        onClick={(e: MouseEvent) => e.stopPropagation()}
      >
        {props.children}
      </KobaltePopover.Trigger>
      <KobaltePopover.Portal>
        <KobaltePopover.Content
          class={cn(
            "z-50 w-64 rounded-lg border border-border bg-card p-3 shadow-lg outline-none focus:outline-none",
            "data-[expanded]:animate-in data-[closed]:animate-out",
            "data-[closed]:fade-out-0 data-[expanded]:fade-in-0",
            "data-[closed]:zoom-out-95 data-[expanded]:zoom-in-95"
          )}
        >
          {/* Description */}
          <div class="mb-2">
            <p class="text-xs font-medium text-muted-foreground mb-1">Description:</p>
            <p class="text-sm text-foreground whitespace-pre-wrap max-h-40 overflow-y-auto">
              {props.task.description || "No description"}
            </p>
          </div>

          {/* Tags */}
          <Show when={props.task.tags && props.task.tags.length > 0}>
            <div>
              <p class="text-xs font-medium text-muted-foreground mb-1">Tags:</p>
              <div class="flex flex-wrap gap-1">
                <For each={props.task.tags}>
                  {(tagName) => {
                    const color = getTagColor(tagName);
                    return (
                      <span
                        class="px-2 py-1 text-xs rounded-md font-medium"
                        style={{
                          "background-color": color ? `${color}20` : "#e5e7eb",
                          color: color || "#374151",
                        }}
                      >
                        {tagName}
                      </span>
                    );
                  }}
                </For>
              </div>
            </div>
          </Show>

          <KobaltePopover.Arrow />
        </KobaltePopover.Content>
      </KobaltePopover.Portal>
    </KobaltePopover>
  );
}
