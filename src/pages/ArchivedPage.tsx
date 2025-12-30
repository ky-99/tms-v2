import { createSignal, createMemo, For, Show, onMount } from "solid-js";
import { Card } from "../components/Card";
import { Input } from "../components/Input";
import { Button } from "../components/Button";
import { DropdownMenu } from "../components/DropdownMenu";
import { ConfirmDialog } from "../components/ConfirmDialog";
import Pagination from "../components/Pagination";
import { tasksApi } from "../api/tasks";
import type { Task, PaginatedTaskResponse } from "../types/task";
import { truncateText } from "../lib/utils";
import { useSearchShortcut } from "../hooks/useSearchShortcut";

const ITEMS_PER_PAGE = 20;

// Icon components
function ArchiveIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <rect x="2" y="4" width="20" height="5" rx="1" />
      <path d="M4 9v9a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9" />
      <path d="M10 13h4" />
    </svg>
  );
}

function SearchIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <circle cx="11" cy="11" r="8" />
      <path d="m21 21-4.35-4.35" />
    </svg>
  );
}

function RotateCcwIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
      <path d="M3 3v5h5" />
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

interface DateGroup {
  date: string;
  tasks: Task[];
}

export function ArchivedPage() {
  const [archivedTasks, setArchivedTasks] = createSignal<Task[]>([]);
  const [searchQuery, setSearchQuery] = createSignal("");
  const [loading, setLoading] = createSignal(true);
  const [currentPage, setCurrentPage] = createSignal(1);
  const [totalItems, setTotalItems] = createSignal(0);
  const [deleteDialogOpen, setDeleteDialogOpen] = createSignal(false);
  const [taskToDelete, setTaskToDelete] = createSignal<Task | null>(null);
  const [searchInputRef, setSearchInputRef] = createSignal<HTMLInputElement | undefined>();

  const totalPages = createMemo(() =>
    Math.ceil(totalItems() / ITEMS_PER_PAGE)
  );

  const loadArchivedTasks = async (page: number) => {
    setLoading(true);
    try {
      const offset = (page - 1) * ITEMS_PER_PAGE;
      const query = searchQuery();

      let result: PaginatedTaskResponse;

      if (query.trim()) {
        // Backend search with pagination
        result = await tasksApi.searchPaginated(
          query,
          "archived",
          undefined, // tags
          ITEMS_PER_PAGE,
          offset
        );
      } else {
        // Normal list with pagination (no search)
        result = await tasksApi.listPaginated(
          ["archived"],
          ITEMS_PER_PAGE,
          offset
        );
      }

      setArchivedTasks(result.tasks);
      setTotalItems(result.total);
      setCurrentPage(page);
    } catch (error) {
      console.error("Failed to load archived tasks:", error);
    } finally {
      setLoading(false);
    }
  };

  const handlePageChange = (page: number) => {
    loadArchivedTasks(page);
  };

  const handleSearch = () => {
    setCurrentPage(1);
    loadArchivedTasks(1);
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      handleSearch();
    }
  };

  useSearchShortcut({
    getSearchInputRef: searchInputRef,
  });

  onMount(async () => {
    await loadArchivedTasks(1);
  });

  const handleRestore = async (taskId: string) => {
    try {
      await tasksApi.restore(taskId);
      // 現在のページをリロード
      await loadArchivedTasks(currentPage());
    } catch (error) {
      console.error("Failed to restore task:", error);
    }
  };

  const handleDeletePermanently = (task: Task) => {
    setTaskToDelete(task);
    setDeleteDialogOpen(true);
  };

  const confirmDeletePermanently = async () => {
    const task = taskToDelete();
    if (!task) return;

    try {
      await tasksApi.deletePermanently(task.id);
      // 現在のページをリロード
      await loadArchivedTasks(currentPage());
    } catch (error) {
      console.error("Failed to delete task permanently:", error);
    }
  };

  const groupTasksByDate = (tasks: Task[]): DateGroup[] => {
    const groups = new Map<string, Task[]>();

    tasks.forEach((task) => {
      const date = new Date(task.updatedAt).toDateString();
      if (!groups.has(date)) {
        groups.set(date, []);
      }
      groups.get(date)!.push(task);
    });

    return Array.from(groups.entries())
      .map(([date, tasks]) => ({
        date,
        tasks: tasks.sort(
          (a, b) =>
            new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
        ),
      }))
      .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
  };

  const filteredAndGroupedTasks = createMemo(() => {
    // Backend already filtered, just group by date
    return groupTasksByDate(archivedTasks());
  });

  return (
    <div class="h-[calc(100vh-3.5rem)] overflow-auto bg-background p-6">
      <div class="mx-auto max-w-4xl">
        <div class="mb-6">
          <div class="mb-4 flex items-center gap-3">
            <div class="text-primary">
              <ArchiveIcon />
            </div>
            <h1 class="text-2xl font-semibold text-foreground">
              Archived Tasks
            </h1>
          </div>

          <div class="flex gap-2">
            <div class="relative flex-1">
              <div class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground">
                <SearchIcon />
              </div>
              <Input
                ref={setSearchInputRef}
                type="text"
                placeholder="Search archived tasks..."
                value={searchQuery()}
                onInput={(e) => setSearchQuery(e.currentTarget.value)}
                onKeyDown={handleKeyDown}
                class="pl-9"
              />
            </div>
            <Button onClick={handleSearch}>
              <SearchIcon />
            </Button>
          </div>
        </div>

        <Show when={!loading() && filteredAndGroupedTasks().length === 0}>
          <Card class="p-8 text-center">
            <p class="text-muted-foreground">
              {searchQuery() ? "No tasks found" : "No archived tasks"}
            </p>
          </Card>
        </Show>

        <Show when={!loading() && filteredAndGroupedTasks().length > 0}>
          <div class="space-y-8">
            <For each={filteredAndGroupedTasks()}>
              {({ date, tasks }) => (
                <div class="relative">
                  <div class="mb-4 flex items-center gap-3">
                    <div class="rounded-md bg-secondary px-3 py-1">
                      <time class="text-sm font-medium text-foreground">
                        {new Date(date).toLocaleDateString("en-US", {
                          month: "short",
                          day: "numeric",
                          year: "numeric",
                        })}
                      </time>
                    </div>
                    <div class="h-px flex-1 bg-border" />
                  </div>

                  <div class="space-y-3 border-l-2 border-border pl-6">
                    <For each={tasks}>
                      {(task) => (
                        <div class="relative">
                          <div class="absolute -left-[27px] top-2 h-3 w-3 rounded-full border-2 border-background bg-muted-foreground" />

                          <Card class="border-border bg-card p-4 transition-colors hover:bg-secondary/30">
                            <div class="grid grid-cols-[1fr_auto] gap-4 items-start">
                              <div class="min-w-0">
                                <div class="flex items-center gap-2 min-w-0">
                                  <div class="text-muted-foreground shrink-0">
                                    <ArchiveIcon />
                                  </div>
                                  <h3 class="font-medium text-foreground truncate min-w-0">
                                    {task.parentTitle ? `@${task.parentTitle}/${task.title}` : task.title}
                                  </h3>
                                </div>
                                <Show when={task.description}>
                                  <p class="mt-1 text-sm text-muted-foreground break-words">
                                    {task.description}
                                  </p>
                                </Show>
                                <time class="mt-2 block text-xs text-muted-foreground">
                                  {new Date(task.updatedAt).toLocaleTimeString(
                                    "en-US",
                                    {
                                      hour: "2-digit",
                                      minute: "2-digit",
                                    }
                                  )}
                                </time>
                              </div>
                              <DropdownMenu
                                items={[
                                  {
                                    label: "Restore",
                                    onClick: () => handleRestore(task.id),
                                    variant: "default"
                                  },
                                  {
                                    label: "Delete permanently",
                                    onClick: () => handleDeletePermanently(task),
                                    variant: "destructive"
                                  }
                                ]}
                              />
                            </div>
                          </Card>
                        </div>
                      )}
                    </For>
                  </div>
                </div>
              )}
            </For>
          </div>

          <Pagination
            currentPage={currentPage()}
            totalPages={totalPages()}
            totalItems={totalItems()}
            onPageChange={handlePageChange}
          />
        </Show>

        {/* Delete Confirmation Dialog */}
        <ConfirmDialog
          open={deleteDialogOpen()}
          onOpenChange={setDeleteDialogOpen}
          title="Permanently Delete Task"
          description={`This action cannot be undone. To confirm, please type the task title: "${truncateText(taskToDelete()?.title || '', 50)}"`}
          confirmText="Delete Permanently"
          cancelText="Cancel"
          variant="destructive"
          onConfirm={confirmDeletePermanently}
          requireVerification={true}
          verificationText={taskToDelete()?.title || ""}
          verificationLabel="Task title:"
          verificationPlaceholder="Type the task title to confirm"
        />
      </div>
    </div>
  );
}
