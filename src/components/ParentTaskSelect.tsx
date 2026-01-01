import { Select as KobalteSelect } from "@kobalte/core/select";
import { createSignal, For, Show } from "solid-js";
import { cn, truncateText } from "../lib/utils";
import type { TaskHierarchy } from "../types/task";
import { ChevronDownIcon, CheckIcon } from "./icons";

interface ParentTaskSelectProps {
  value: string | undefined;
  onChange: (value: string | undefined) => void;
  tasks: TaskHierarchy[];
  excludeTaskId?: string; // For edit mode, exclude the task being edited
  label?: string;
}

export function ParentTaskSelect(props: ParentTaskSelectProps) {
  const [searchValue, setSearchValue] = createSignal("");

  // Filter tasks based on search and exclude current task (for edit mode)
  const filteredTasks = () => {
    const search = searchValue().toLowerCase().trim();
    let tasks = props.tasks;

    // Only show root tasks (tasks without parents)
    tasks = tasks.filter((task) => !task.parentId);

    // Exclude current task if editing
    if (props.excludeTaskId) {
      tasks = tasks.filter((task) => task.id !== props.excludeTaskId);
    }

    // Filter by search
    if (search) {
      tasks = tasks.filter((task) =>
        task.title.toLowerCase().includes(search)
      );
    }

    return tasks;
  };

  // Get display value (truncated)
  const displayValue = () => {
    if (!props.value) return "None (Root Task)";
    const task = props.tasks.find((t) => t.id === props.value);
    return task ? truncateText(task.title, 50) : "None (Root Task)";
  };

  return (
    <div class="flex flex-col gap-1.5">
      {props.label && (
        <label class="text-sm font-medium">{props.label}</label>
      )}
      <KobalteSelect
        value={props.value || ""}
        onChange={(value) => props.onChange(value || undefined)}
        options={["", ...filteredTasks().map((t) => t.id)]}
        placeholder="Select parent task..."
        itemComponent={(itemProps) => {
          const task = props.tasks.find((t) => t.id === itemProps.item.rawValue);
          return (
            <KobalteSelect.Item
              item={itemProps.item}
              class="relative flex w-full items-center gap-2 px-3 py-2 text-sm outline-none cursor-pointer rounded-sm hover:bg-secondary data-[highlighted]:bg-secondary data-[selected]:bg-primary/10 data-[selected]:text-primary data-[selected]:font-medium data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed transition-colors text-left"
            >
              <KobalteSelect.ItemLabel class="flex-1">
                {itemProps.item.rawValue === "" ? "None (Root Task)" : truncateText(task?.title || "Unknown", 50)}
              </KobalteSelect.ItemLabel>
              <KobalteSelect.ItemIndicator>
                <CheckIcon />
              </KobalteSelect.ItemIndicator>
            </KobalteSelect.Item>
          );
        }}
      >
        <KobalteSelect.Trigger
          class={cn(
            "flex items-center justify-between w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background",
            "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring/30 focus-visible:ring-offset-2",
            "disabled:cursor-not-allowed disabled:opacity-50",
            "[&_[data-placeholder-shown]]:text-muted-foreground"
          )}
        >
          <KobalteSelect.Value<string>>
            {(state) => (
              <span class="flex-1 text-left">
                {!state.selectedOption() ? "Select parent task..." : displayValue()}
              </span>
            )}
          </KobalteSelect.Value>
          <KobalteSelect.Icon>
            <ChevronDownIcon />
          </KobalteSelect.Icon>
        </KobalteSelect.Trigger>
        <KobalteSelect.Portal>
          <KobalteSelect.Content class="absolute z-50 min-w-[var(--kb-popper-anchor-width)] rounded-md border border-border bg-card shadow-lg data-[expanded]:animate-in data-[closed]:animate-out data-[closed]:fade-out-0 data-[expanded]:fade-in-0 data-[closed]:zoom-out-95 data-[expanded]:zoom-in-95">
            <KobalteSelect.Listbox class="max-h-60 overflow-y-auto p-1">
              <Show
                when={filteredTasks().length > 0 || !searchValue()}
                fallback={
                  <div class="px-3 py-6 text-center text-sm text-muted-foreground">
                    No tasks found
                  </div>
                }
              >
                {/* Always show "None" option */}
                <KobalteSelect.Item
                  item={{ rawValue: "", label: "None (Root Task)" }}
                  class="relative flex w-full items-center gap-2 px-3 py-2 text-sm outline-none cursor-pointer rounded-sm hover:bg-secondary data-[highlighted]:bg-secondary data-[selected]:bg-primary/10 data-[selected]:text-primary data-[selected]:font-medium transition-colors text-left"
                >
                  <KobalteSelect.ItemLabel class="flex-1">None (Root Task)</KobalteSelect.ItemLabel>
                  <KobalteSelect.ItemIndicator>
                    <CheckIcon />
                  </KobalteSelect.ItemIndicator>
                </KobalteSelect.Item>

                {/* Show filtered tasks */}
                <For each={filteredTasks()}>
                  {(task) => (
                    <KobalteSelect.Item
                      item={{ rawValue: task.id, label: task.title }}
                      class="relative flex w-full items-center gap-2 px-3 py-2 text-sm outline-none cursor-pointer rounded-sm hover:bg-secondary data-[highlighted]:bg-secondary data-[selected]:bg-primary/10 data-[selected]:text-primary data-[selected]:font-medium transition-colors text-left"
                    >
                      <KobalteSelect.ItemLabel class="flex-1">{truncateText(task.title, 50)}</KobalteSelect.ItemLabel>
                      <KobalteSelect.ItemIndicator>
                        <CheckIcon />
                      </KobalteSelect.ItemIndicator>
                    </KobalteSelect.Item>
                  )}
                </For>
              </Show>
            </KobalteSelect.Listbox>
          </KobalteSelect.Content>
        </KobalteSelect.Portal>
      </KobalteSelect>
    </div>
  );
}
