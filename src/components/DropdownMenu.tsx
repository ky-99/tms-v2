import { DropdownMenu as KobalteDropdownMenu } from "@kobalte/core/dropdown-menu";
import { JSX, For } from "solid-js";
import { cn } from "../lib/utils";

// 3点リーダー縦並びアイコン
function MoreVerticalIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-4 w-4"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      stroke-width="2"
    >
      <circle cx="12" cy="5" r="1" fill="currentColor" />
      <circle cx="12" cy="12" r="1" fill="currentColor" />
      <circle cx="12" cy="19" r="1" fill="currentColor" />
    </svg>
  );
}

export interface DropdownMenuItem {
  label: string;
  onClick: () => void;
  variant?: "default" | "destructive";
}

export interface DropdownMenuProps {
  items: DropdownMenuItem[];
  triggerClassName?: string;
}

export function DropdownMenu(props: DropdownMenuProps) {
  return (
    <KobalteDropdownMenu>
      <KobalteDropdownMenu.Trigger
        class={cn(
          "h-8 w-8 inline-flex items-center justify-center rounded-md",
          "text-foreground hover:bg-secondary",
          "focus:outline-none",
          "transition-colors",
          props.triggerClassName
        )}
      >
        <MoreVerticalIcon />
      </KobalteDropdownMenu.Trigger>
      <KobalteDropdownMenu.Portal>
        <KobalteDropdownMenu.Content
          class={cn(
            "z-50 min-w-[8rem] overflow-hidden rounded-md border border-border bg-card p-1 shadow-lg",
            "data-[expanded]:animate-in data-[closed]:animate-out",
            "data-[closed]:fade-out-0 data-[expanded]:fade-in-0",
            "data-[closed]:zoom-out-95 data-[expanded]:zoom-in-95"
          )}
        >
          <For each={props.items}>
            {(item) => (
              <KobalteDropdownMenu.Item
                class={cn(
                  "relative flex cursor-pointer select-none items-center rounded-sm px-3 py-2 text-sm outline-none transition-colors",
                  "focus:bg-secondary",
                  item.variant === "destructive"
                    ? "text-destructive hover:bg-destructive/10 focus:bg-destructive/10"
                    : "text-foreground hover:bg-secondary"
                )}
                onSelect={item.onClick}
              >
                {item.label}
              </KobalteDropdownMenu.Item>
            )}
          </For>
        </KobalteDropdownMenu.Content>
      </KobalteDropdownMenu.Portal>
    </KobalteDropdownMenu>
  );
}
