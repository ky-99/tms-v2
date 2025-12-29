import { createSignal, onMount } from "solid-js";
import { ask, message } from "@tauri-apps/plugin-dialog";
import { taskStore, taskActions } from "../stores/taskStore";
import { queueStore, queueActions } from "../stores/queueStore";
import { tagsApi } from "../api/tags";
import type {
  CreateTaskRequest,
  UpdateTaskRequest,
  TaskHierarchy,
} from "../types/task";
import type { Tag } from "../types/tag";
import { TaskPool } from "../components/TaskPool";
import { QueuePanel } from "../components/QueuePanel";
import { Dialog } from "../components/Dialog";
import { Input } from "../components/Input";
import { Button } from "../components/Button";
import { TagInput } from "../components/TagInput";
import { For } from "solid-js";

export function TaskPage() {
  const [isCreateDialogOpen, setIsCreateDialogOpen] = createSignal(false);
  const [editingTask, setEditingTask] = createSignal<TaskHierarchy | null>(null);
  const [formData, setFormData] = createSignal<CreateTaskRequest>({
    title: "",
    description: "",
    tags: [],
  });
  const [availableTags, setAvailableTags] = createSignal<Tag[]>([]);

  onMount(async () => {
    taskActions.loadHierarchy();
    queueActions.loadQueue();
    await loadTags();
  });

  const loadTags = async () => {
    try {
      const tags = await tagsApi.list();
      setAvailableTags(tags);
    } catch (error) {
      console.error("Failed to load tags:", error);
    }
  };

  const handleCreateTag = async (name: string, color: string): Promise<Tag> => {
    try {
      const newTag = await tagsApi.create({ name, color });
      await loadTags(); // Reload tags to update the list
      return newTag;
    } catch (error) {
      console.error("Failed to create tag:", error);
      throw error;
    }
  };

  const queueTaskIds = () => {
    return new Set(queueStore.queue.map((entry) => entry.taskId));
  };

  const handleCreate = async (e: Event) => {
    e.preventDefault();
    try {
      await taskActions.createTask(formData());
      await taskActions.loadHierarchy();
      setIsCreateDialogOpen(false);
      setFormData({ title: "", description: "", tags: [] });
    } catch (error) {
      console.error("Failed to create task:", error);
      await message(`タスクの作成に失敗しました: ${error}`, {
        title: "Error",
        kind: "error",
      });
    }
  };

  const handleUpdate = async (e: Event) => {
    e.preventDefault();
    const task = editingTask();
    if (!task) return;

    try {
      const updateData: UpdateTaskRequest = {
        title: formData().title,
        description: formData().description,
        tags: formData().tags,
        parentId: formData().parentId,
      };
      await taskActions.updateTask(task.id, updateData);
      await taskActions.loadHierarchy();
      setEditingTask(null);
      setFormData({ title: "", description: "", tags: [] });
    } catch (error) {
      console.error("Failed to update task:", error);
      await message(`タスクの更新に失敗しました: ${error}`, {
        title: "Error",
        kind: "error",
      });
    }
  };

  const handleDelete = async (task: TaskHierarchy) => {
    const confirmed = await ask(`Delete "${task.title}"?`, {
      title: "Confirm Delete",
      kind: "warning",
    });
    if (!confirmed) return;

    try {
      await taskActions.deleteTask(task.id);
      await taskActions.loadHierarchy();
    } catch (error) {
      console.error("Failed to delete task:", error);
      await message(`削除に失敗しました: ${error}`, {
        title: "Error",
        kind: "error",
      });
    }
  };

  const handleMoveToQueue = async (task: TaskHierarchy) => {
    try {
      await queueActions.addToQueue(task.id);
      await taskActions.loadHierarchy();
    } catch (error) {
      console.error("Failed to add to queue:", error);
      await message(`キューへの追加に失敗しました: ${error}`, {
        title: "Error",
        kind: "error",
      });
    }
  };

  const handleEdit = (task: TaskHierarchy) => {
    setEditingTask(task);
    setFormData({
      title: task.title,
      description: task.description || "",
      tags: task.tags,
      parentId: task.parentId,
    });
  };

  const flattenHierarchy = (hierarchy: TaskHierarchy[]): TaskHierarchy[] => {
    if (!hierarchy || hierarchy.length === 0) {
      return [];
    }
    const result: TaskHierarchy[] = [];
    const flatten = (tasks: TaskHierarchy[]) => {
      tasks.forEach((task) => {
        result.push(task);
        if (task.children && task.children.length > 0) {
          flatten(task.children);
        }
      });
    };
    flatten(hierarchy);
    return result;
  };

  return (
    <div class="flex h-screen">
      {/* Task Pool */}
      <TaskPool
        tasks={taskStore.hierarchy}
        onMoveToQueue={handleMoveToQueue}
        onEdit={handleEdit}
        onDelete={handleDelete}
        onCreateTask={() => setIsCreateDialogOpen(true)}
        queueTaskIds={queueTaskIds()}
        availableTags={availableTags()}
      />

      {/* Queue Panel */}
      <QueuePanel />

      {/* Create Dialog */}
      <Dialog
        open={isCreateDialogOpen()}
        onOpenChange={setIsCreateDialogOpen}
        title="新規タスク作成"
      >
        <form onSubmit={handleCreate} class="space-y-4">
          <Input
            label="タイトル"
            value={formData().title}
            onInput={(e) =>
              setFormData({ ...formData(), title: e.currentTarget.value })
            }
            required
          />
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">親タスク</label>
            <select
              class="px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring"
              value={formData().parentId || ""}
              onChange={(e) =>
                setFormData({
                  ...formData(),
                  parentId: e.currentTarget.value || undefined,
                })
              }
            >
              <option value="">なし（ルートタスク）</option>
              <For each={flattenHierarchy(taskStore.hierarchy)}>
                {(task) => <option value={task.id}>{task.title}</option>}
              </For>
            </select>
          </div>
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">説明</label>
            <textarea
              class="px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring"
              rows={4}
              value={formData().description}
              onInput={(e) =>
                setFormData({
                  ...formData(),
                  description: e.currentTarget.value,
                })
              }
            />
          </div>
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">タグ</label>
            <TagInput
              selectedTags={formData().tags || []}
              onTagsChange={(tags) => setFormData({ ...formData(), tags })}
              availableTags={availableTags()}
              onCreateTag={handleCreateTag}
              placeholder="Add tags..."
            />
          </div>
          <div class="flex gap-2 justify-end">
            <Button
              type="button"
              variant="secondary"
              onClick={() => setIsCreateDialogOpen(false)}
            >
              キャンセル
            </Button>
            <Button type="submit">作成</Button>
          </div>
        </form>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog
        open={editingTask() !== null}
        onOpenChange={(open) => !open && setEditingTask(null)}
        title="タスク編集"
      >
        <form onSubmit={handleUpdate} class="space-y-4">
          <Input
            label="タイトル"
            value={formData().title}
            onInput={(e) =>
              setFormData({ ...formData(), title: e.currentTarget.value })
            }
            required
          />
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">親タスク</label>
            <select
              class="px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring"
              value={formData().parentId || ""}
              onChange={(e) =>
                setFormData({
                  ...formData(),
                  parentId: e.currentTarget.value || undefined,
                })
              }
            >
              <option value="">なし（ルートタスク）</option>
              <For
                each={flattenHierarchy(taskStore.hierarchy).filter(
                  (task) => task.id !== editingTask()?.id
                )}
              >
                {(task) => <option value={task.id}>{task.title}</option>}
              </For>
            </select>
          </div>
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">説明</label>
            <textarea
              class="px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-ring"
              rows={4}
              value={formData().description}
              onInput={(e) =>
                setFormData({
                  ...formData(),
                  description: e.currentTarget.value,
                })
              }
            />
          </div>
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">タグ</label>
            <TagInput
              selectedTags={formData().tags || []}
              onTagsChange={(tags) => setFormData({ ...formData(), tags })}
              availableTags={availableTags()}
              onCreateTag={handleCreateTag}
              placeholder="Add tags..."
            />
          </div>
          <div class="flex gap-2 justify-end">
            <Button
              type="button"
              variant="secondary"
              onClick={() => setEditingTask(null)}
            >
              キャンセル
            </Button>
            <Button type="submit">更新</Button>
          </div>
        </form>
      </Dialog>
    </div>
  );
}
