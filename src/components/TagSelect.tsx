import { Popover as KobaltePopover } from "@kobalte/core/popover";
import { createSignal, For, Show } from "solid-js";
import { cn, truncateText } from "../lib/utils";
import type { Tag } from "../types/tag";

interface TagSelectProps {
  selectedTags: string[];
  availableTags: Tag[];
  onToggleTag: (tagName: string) => void;
  onOpenCreateMode?: () => void;
  placeholder?: string;
}

// Icon components
function ChevronDownIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-4 w-4"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      stroke-width="2"
    >
      <path d="M19 9l-7 7-7-7" />
    </svg>
  );
}

function CheckIcon() {
  return (
    <svg
      class="h-4 w-4 text-primary"
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      stroke-width="2"
    >
      <path d="M5 13l4 4L19 7" />
    </svg>
  );
}

function PlusIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M5 12h14M12 5v14" />
    </svg>
  );
}

export function TagSelect(props: TagSelectProps) {
  const [open, setOpen] = createSignal(false);

  const isTagSelected = (tagName: string) => {
    return props.selectedTags.includes(tagName);
  };

  const handleToggleTag = (tagName: string) => {
    props.onToggleTag(tagName);
    // Close popover after selection
    setOpen(false);
  };

  const handleOpenCreateMode = () => {
    if (props.onOpenCreateMode) {
      props.onOpenCreateMode();
      // Close popover when opening create mode
      setOpen(false);
    }
  };

  return (
    <KobaltePopover open={open()} onOpenChange={setOpen}>
      <KobaltePopover.Trigger
        class={cn(
          "flex items-center justify-between w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background",
          "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring/30 focus-visible:ring-offset-2",
          "disabled:cursor-not-allowed disabled:opacity-50"
        )}
      >
        <span class="flex-1 text-left text-muted-foreground">
          {props.placeholder || "Select tags..."}
        </span>
        <ChevronDownIcon />
      </KobaltePopover.Trigger>
      <KobaltePopover.Portal>
        <KobaltePopover.Content class="z-50 min-w-[var(--kb-popper-anchor-width)] rounded-md border border-border bg-card shadow-lg data-[expanded]:animate-in data-[closed]:animate-out data-[closed]:fade-out-0 data-[expanded]:fade-in-0 data-[closed]:zoom-out-95 data-[expanded]:zoom-in-95">
          <div class="max-h-60 overflow-y-auto p-1">
            <Show
              when={props.availableTags.length > 0}
              fallback={
                <div class="px-3 py-6 text-center text-sm text-muted-foreground">
                  No tags available
                </div>
              }
            >
              {/* Create new tag button - at top */}
              <Show when={props.onOpenCreateMode}>
                <button
                  type="button"
                  onClick={handleOpenCreateMode}
                  class="flex w-full items-center gap-2 rounded-sm px-3 py-2 text-sm hover:bg-secondary transition-colors text-left border-b border-border mb-1"
                >
                  <PlusIcon />
                  <span>Create new tag</span>
                </button>
              </Show>

              <For each={props.availableTags}>
                {(tag) => {
                  const isSelected = isTagSelected(tag.name);
                  return (
                    <button
                      type="button"
                      onClick={() => handleToggleTag(tag.name)}
                      class={cn(
                        "flex w-full items-center gap-2 rounded-sm px-3 py-2 text-sm transition-colors text-left",
                        isSelected
                          ? "bg-primary/10 text-primary font-medium"
                          : "hover:bg-secondary"
                      )}
                    >
                      <Show when={tag.color}>
                        <div
                          class="h-3 w-3 rounded-full flex-shrink-0"
                          style={{ "background-color": tag.color }}
                        />
                      </Show>
                      <span class="flex-1">{truncateText(tag.name, 40)}</span>
                      <Show when={isSelected}>
                        <CheckIcon />
                      </Show>
                    </button>
                  );
                }}
              </For>
            </Show>
          </div>
        </KobaltePopover.Content>
      </KobaltePopover.Portal>
    </KobaltePopover>
  );
}
