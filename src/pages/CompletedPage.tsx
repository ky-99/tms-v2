import { createSignal, createMemo, For, Show, onMount } from "solid-js";
import { Card } from "../components/Card";
import { Input } from "../components/Input";
import { Button } from "../components/Button";
import Pagination from "../components/Pagination";
import { tasksApi } from "../api/tasks";
import type { Task, PaginatedTaskResponse } from "../types/task";
import { useSearchShortcut } from "../hooks/useSearchShortcut";
import { CheckCircle2Icon, SearchIcon } from "../components/icons";

const ITEMS_PER_PAGE = 20;

interface DateGroup {
  date: string;
  tasks: Task[];
}

export function CompletedPage() {
  const [completedTasks, setCompletedTasks] = createSignal<Task[]>([]);
  const [searchQuery, setSearchQuery] = createSignal("");
  const [loading, setLoading] = createSignal(true);
  const [currentPage, setCurrentPage] = createSignal(1);
  const [totalItems, setTotalItems] = createSignal(0);
  const [searchInputRef, setSearchInputRef] = createSignal<HTMLInputElement | undefined>();

  const totalPages = createMemo(() =>
    Math.ceil(totalItems() / ITEMS_PER_PAGE)
  );

  const loadCompletedTasks = async (page: number) => {
    setLoading(true);
    try {
      const offset = (page - 1) * ITEMS_PER_PAGE;
      const query = searchQuery();

      let result: PaginatedTaskResponse;

      if (query.trim()) {
        // Backend search with pagination
        result = await tasksApi.searchPaginated(
          query,
          "completed",
          undefined, // tags
          ITEMS_PER_PAGE,
          offset
        );
      } else {
        // Normal list with pagination (no search)
        result = await tasksApi.listPaginated(
          ["completed"],
          ITEMS_PER_PAGE,
          offset
        );
      }

      setCompletedTasks(result.tasks);
      setTotalItems(result.total);
      setCurrentPage(page);
    } catch (error) {
      console.error("Failed to load completed tasks:", error);
    } finally {
      setLoading(false);
    }
  };

  const handlePageChange = (page: number) => {
    loadCompletedTasks(page);
  };

  const handleSearch = () => {
    setCurrentPage(1);
    loadCompletedTasks(1);
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
    await loadCompletedTasks(1);
  });

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
    return groupTasksByDate(completedTasks());
  });

  return (
    <div class="h-[calc(100vh-3.5rem)] overflow-auto bg-background p-6">
      <div class="mx-auto max-w-4xl">
        <div class="mb-6">
          <div class="mb-4 flex items-center gap-3">
            <div class="text-primary">
              <CheckCircle2Icon />
            </div>
            <h1 class="text-2xl font-semibold text-foreground">
              Completed Tasks
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
                placeholder="Search completed tasks..."
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
              {searchQuery() ? "No tasks found" : "No completed tasks yet"}
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
                          <div class="absolute -left-[27px] top-2 h-3 w-3 rounded-full border-2 border-background bg-primary" />

                          <Card class="border-border bg-card p-4 transition-colors hover:bg-secondary/30">
                            <div class="min-w-0">
                              <div class="flex items-center gap-2 min-w-0">
                                <div class="text-primary shrink-0">
                                  <CheckCircle2Icon />
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
      </div>
    </div>
  );
}
