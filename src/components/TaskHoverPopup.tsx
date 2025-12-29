import { Popover as KobaltePopover } from "@kobalte/core/popover";
import { JSX, createSignal, onCleanup, Show, For } from "solid-js";
import { cn } from "../lib/utils";
import type { Task } from "../types/task";

interface TaskHoverPopupProps {
  task: Task;
  children: JSX.Element;
}

/**
 * タスクホバー詳細ポップアップコンポーネント
 *
 * informationアイコン上で1000ms以上ホバーすると詳細ポップアップを表示します。
 * - description、tagsを表示（コンパクト版）
 */
export function TaskHoverPopup(props: TaskHoverPopupProps) {
  const [isOpen, setIsOpen] = createSignal(false);
  let hoverTimer: number | undefined;

  const handleMouseEnter = () => {
    // 1000ms後にポップアップを表示
    hoverTimer = window.setTimeout(() => {
      setIsOpen(true);
    }, 1000);
  };

  const handleMouseLeave = () => {
    // タイマーをクリアしてポップアップを閉じる
    if (hoverTimer) {
      clearTimeout(hoverTimer);
      hoverTimer = undefined;
    }
    setIsOpen(false);
  };

  // コンポーネントのクリーンアップ時にタイマーをクリア
  onCleanup(() => {
    if (hoverTimer) {
      clearTimeout(hoverTimer);
    }
  });


  return (
    <KobaltePopover open={isOpen()} onOpenChange={setIsOpen}>
      <KobaltePopover.Trigger
        class="w-full outline-none focus:outline-none"
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
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
            <p class="text-sm text-foreground whitespace-pre-wrap">
              {props.task.description || "No description"}
            </p>
          </div>

          {/* Tags */}
          <Show when={props.task.tags && props.task.tags.length > 0}>
            <div>
              <p class="text-xs font-medium text-muted-foreground mb-1">Tags:</p>
              <div class="flex flex-wrap gap-1">
                <For each={props.task.tags}>
                  {(tag) => (
                    <span class="px-2 py-1 text-xs rounded-md bg-primary/10 text-primary">
                      {tag}
                    </span>
                  )}
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
