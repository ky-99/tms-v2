import { createSignal, For, Show } from "solid-js";
import { Button } from "./Button";
import { Input } from "./Input";
import { TaskHoverPopup } from "./TaskHoverPopup";
import { cn } from "../lib/utils";
import type { TaskHierarchy } from "../types/task";

interface TaskPoolProps {
  tasks: TaskHierarchy[];
  onMoveToQueue: (task: TaskHierarchy) => void;
  onEdit: (task: TaskHierarchy) => void;
  onDelete: (task: TaskHierarchy) => void;
  onCreateTask: () => void;
  queueTaskIds: Set<string>;
}

// Icon components
function SearchIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <circle cx="11" cy="11" r="8" />
      <path d="m21 21-4.35-4.35" />
    </svg>
  );
}

function XIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M18 6 6 18M6 6l12 12" />
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

function ChevronDownIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="m6 9 6 6 6-6" />
    </svg>
  );
}

function ChevronRightIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="m9 18 6-6-6-6" />
    </svg>
  );
}

function CircleIcon(props: { filled?: boolean }) {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill={props.filled ? "currentColor" : "none"} viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10" />
    </svg>
  );
}

function CheckCircle2Icon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10" />
      <path d="m9 12 2 2 4-4" />
    </svg>
  );
}

function ArrowRightIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M5 12h14m-7-7 7 7-7 7" />
    </svg>
  );
}

function PencilIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
    </svg>
  );
}

function Trash2Icon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M3 6h18m-2 0v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6m3 0V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
    </svg>
  );
}

function ProgressCircle(props: { progress: number }) {
  const radius = 8;
  const circumference = 2 * Math.PI * radius;
  const offset = () => circumference - (props.progress / 100) * circumference;

  return (
    <div class="relative flex h-5 w-5 items-center justify-center">
      <svg class="h-5 w-5 -rotate-90" viewBox="0 0 20 20">
        <circle
          cx="10"
          cy="10"
          r={radius}
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          class="text-muted-foreground/30"
        />
        <circle
          cx="10"
          cy="10"
          r={radius}
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-dasharray={circumference}
          stroke-dashoffset={offset()}
          class="text-primary transition-all duration-300"
          stroke-linecap="round"
        />
      </svg>
      <span class="absolute text-[8px] font-semibold text-foreground">{props.progress}</span>
    </div>
  );
}

export function TaskPool(props: TaskPoolProps) {
  const [expandedTasks, setExpandedTasks] = createSignal<Set<string>>(new Set());
  const [searchQuery, setSearchQuery] = createSignal("");
  const [activeFilters, setActiveFilters] = createSignal<Set<string>>(new Set());

  const toggleExpand = (taskId: string) => {
    const newExpanded = new Set(expandedTasks());
    if (newExpanded.has(taskId)) {
      newExpanded.delete(taskId);
    } else {
      newExpanded.add(taskId);
    }
    setExpandedTasks(newExpanded);
  };

  const toggleFilter = (status: string) => {
    const newFilters = new Set(activeFilters());
    if (newFilters.has(status)) {
      newFilters.delete(status);
    } else {
      newFilters.add(status);
    }
    setActiveFilters(newFilters);
  };

  const filteredTasks = () => {
    return props.tasks.filter((task) => {
      const matchesSearch =
        task.title.toLowerCase().includes(searchQuery().toLowerCase()) ||
        task.children?.some((child) => child.title.toLowerCase().includes(searchQuery().toLowerCase()));
      const matchesFilter = activeFilters().size === 0 || activeFilters().has(task.status);
      return matchesSearch && matchesFilter;
    });
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "completed":
        return (
          <div class="text-primary">
            <CheckCircle2Icon />
          </div>
        );
      case "active":
        return (
          <div class="text-primary">
            <CircleIcon filled={true} />
          </div>
        );
      default:
        return (
          <div class="text-muted-foreground">
            <CircleIcon filled={false} />
          </div>
        );
    }
  };

  const calculateProgress = (task: TaskHierarchy): number => {
    if (!task.children || task.children.length === 0) return 0;
    const completedChildren = task.children.filter((child) => child.status === "completed").length;
    return Math.round((completedChildren / task.children.length) * 100);
  };

  return (
    <div class="flex flex-1 flex-col border-r border-border">
      <div class="border-b border-border bg-card px-4 py-3 space-y-3">
        <div class="flex items-center gap-2">
          <div class="relative flex-1">
            <div class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">
              <SearchIcon />
            </div>
            <Input
              type="text"
              placeholder="Search tasks..."
              value={searchQuery()}
              onInput={(e) => setSearchQuery(e.currentTarget.value)}
              class="pl-9 pr-9 bg-background"
            />
            <Show when={searchQuery()}>
              <Button
                variant="ghost"
                onClick={() => setSearchQuery("")}
                class="absolute right-1 top-1/2 h-7 w-7 -translate-y-1/2 p-0"
              >
                <XIcon />
              </Button>
            </Show>
          </div>
          <Button onClick={props.onCreateTask} class="h-10 w-10 p-0 shrink-0" title="New Task">
            <PlusIcon />
          </Button>
        </div>

        <div class="flex flex-wrap gap-2">
          <button
            onClick={() => toggleFilter("draft")}
            class={cn(
              "rounded-full px-3 py-1 text-xs font-medium transition-colors",
              activeFilters().has("draft")
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            )}
          >
            Draft
          </button>
          <button
            onClick={() => toggleFilter("active")}
            class={cn(
              "rounded-full px-3 py-1 text-xs font-medium transition-colors",
              activeFilters().has("active")
                ? "bg-primary text-primary-foreground"
                : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
            )}
          >
            Active
          </button>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto p-4">
        <div class="space-y-2">
          <For each={filteredTasks()}>
            {(task) => (
              <div class="space-y-1">
                <div
                  onClick={(e) => {
                    if (task.children && task.children.length > 0) {
                      e.stopPropagation();
                      toggleExpand(task.id);
                    }
                  }}
                  class={cn(
                    "group flex items-center gap-3 rounded-lg bg-card p-3 transition-colors select-none",
                    task.status === "completed" && "opacity-60",
                    props.queueTaskIds.has(task.id)
                      ? "bg-primary/10 border border-primary/20 hover:bg-primary/5"
                      : "hover:bg-secondary/50",
                    task.children && task.children.length > 0 && "cursor-pointer"
                  )}
                >
                  <Show when={task.children && task.children.length > 0} fallback={<div class="w-4" />}>
                    <div class="text-muted-foreground">
                      <Show when={expandedTasks().has(task.id)} fallback={<ChevronRightIcon />}>
                        <ChevronDownIcon />
                      </Show>
                    </div>
                  </Show>

                  <Show
                    when={task.children && task.children.length > 0}
                    fallback={getStatusIcon(task.status)}
                  >
                    <ProgressCircle progress={calculateProgress(task)} />
                  </Show>

                  <TaskHoverPopup task={task}>
                    <span
                      class={cn(
                        "text-sm font-medium text-foreground cursor-pointer hover:text-primary transition-colors",
                        task.status === "completed" && "line-through"
                      )}
                    >
                      {task.title}
                    </span>
                  </TaskHoverPopup>

                  <div class="flex-1"></div>

                  <div class="flex h-8 items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                    <Show when={task.status === "draft"}>
                      <Button
                        variant="ghost"
                        onClick={(e) => {
                          e.stopPropagation();
                          props.onEdit(task);
                        }}
                        class="h-8 w-8 p-0"
                        title="Edit Task"
                      >
                        <PencilIcon />
                      </Button>
                      <Button
                        variant="ghost"
                        onClick={(e) => {
                          e.stopPropagation();
                          props.onDelete(task);
                        }}
                        class="h-8 w-8 p-0 text-destructive hover:text-destructive"
                        title="Delete Task"
                      >
                        <Trash2Icon />
                      </Button>
                    </Show>
                    <Show when={!(task.children && task.children.length > 0)}>
                      <Button
                        variant="ghost"
                        onClick={(e) => {
                          e.stopPropagation();
                          props.onMoveToQueue(task);
                        }}
                        class="h-8 w-8 p-0"
                        title="Add to Queue"
                        disabled={props.queueTaskIds.has(task.id)}
                      >
                        <ArrowRightIcon />
                      </Button>
                    </Show>
                  </div>
                </div>

              <Show when={task.children && task.children.length > 0 && expandedTasks().has(task.id)}>
                  <div class="ml-6 space-y-1 border-l-2 border-border pl-4">
                    <For each={task.children}>
                      {(child) => (
                        <div
                          class={cn(
                            "group flex items-center gap-3 rounded-lg bg-card p-2.5 transition-colors",
                            child.status === "completed" && "opacity-60",
                            props.queueTaskIds.has(child.id)
                              ? "bg-primary/10 border border-primary/20 hover:bg-primary/5"
                              : "hover:bg-secondary/50"
                          )}
                        >
                          {getStatusIcon(child.status)}
                          <TaskHoverPopup task={child}>
                            <span
                              class={cn(
                                "text-sm text-foreground cursor-pointer hover:text-primary transition-colors",
                                child.status === "completed" && "line-through"
                              )}
                            >
                              {child.title}
                            </span>
                          </TaskHoverPopup>

                          <div class="flex-1"></div>

                          <div class="flex h-8 items-center gap-1 opacity-0 transition-opacity group-hover:opacity-100">
                            <Show when={child.status === "draft"}>
                              <Button
                                variant="ghost"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  props.onEdit(child);
                                }}
                                class="h-8 w-8 p-0"
                                title="Edit Task"
                              >
                                <PencilIcon />
                              </Button>
                              <Button
                                variant="ghost"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  props.onDelete(child);
                                }}
                                class="h-8 w-8 p-0 text-destructive hover:text-destructive"
                                title="Delete Task"
                              >
                                <Trash2Icon />
                              </Button>
                            </Show>
                            <Button
                              variant="ghost"
                              onClick={() => props.onMoveToQueue(child)}
                              class="h-8 w-8 p-0"
                              title="Add to Queue"
                              disabled={props.queueTaskIds.has(child.id)}
                            >
                              <ArrowRightIcon />
                            </Button>
                          </div>
                        </div>
                      )}
                  </For>
                </div>
              </Show>
            </div>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}
