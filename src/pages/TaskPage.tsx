import { createSignal, createEffect, onMount } from "solid-js";
import { taskStore, taskActions } from "../stores/taskStore";
import { queueStore, queueActions } from "../stores/queueStore";
import { tagsApi } from "../api/tags";
import { tasksApi } from "../api/tasks";
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
import { Textarea } from "../components/Textarea";
import { Button } from "../components/Button";
import { TagInput } from "../components/TagInput";
import { ParentTaskSelect } from "../components/ParentTaskSelect";
import { For } from "solid-js";
import { useKeyboardShortcuts } from "../hooks/useKeyboardShortcuts";
import { taskSelectionStore, taskSelectionActions } from "../stores/taskSelectionStore";

export function TaskPage() {
  const [isCreateDialogOpen, setIsCreateDialogOpen] = createSignal(false);
  const [editingTask, setEditingTask] = createSignal<TaskHierarchy | null>(null);
  const [formData, setFormData] = createSignal<CreateTaskRequest>({
    title: "",
    description: "",
    tags: [],
  });
  const [availableTags, setAvailableTags] = createSignal<Tag[]>([]);

  const [searchInputRef, setSearchInputRef] = createSignal<HTMLInputElement | undefined>();
  const [createTitleInputRef, setCreateTitleInputRef] = createSignal<HTMLInputElement | undefined>();

  const isAnyDialogOpen = () => {
    return isCreateDialogOpen() || editingTask() !== null;
  };

  // Auto-focus title input when create dialog opens
  createEffect(() => {
    if (isCreateDialogOpen()) {
      const inputElement = createTitleInputRef();
      if (inputElement) {
        // Use setTimeout to ensure dialog is fully rendered
        setTimeout(() => inputElement.focus(), 0);
      }
    }
  });

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
    }
  };

  const handleDelete = async (task: TaskHierarchy) => {
    try {
      await taskActions.deleteTask(task.id);
      await taskActions.loadHierarchy();
    } catch (error) {
      console.error("Failed to delete task:", error);
    }
  };

  const handleMoveToQueue = async (task: TaskHierarchy) => {
    try {
      await queueActions.addToQueue(task.id);
      await taskActions.loadHierarchy();
    } catch (error) {
      console.error("Failed to add to queue:", error);
    }
  };

  /**
   * タスクを複製する
   * - Cmd/Ctrl+D から呼び出される
   * - 親タスクの場合、全ての子タスクも再帰的に複製される
   */
  const handleDuplicate = async (task: TaskHierarchy) => {
    try {
      await tasksApi.duplicate({ taskId: task.id });
      // タスク階層を再読み込み
      await taskActions.loadHierarchy();
    } catch (error) {
      console.error("Failed to duplicate task:", error);
      // エラーはwithErrorHandlingで処理されるため、ここでは何もしない
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

  useKeyboardShortcuts({
    onCreateTask: () => setIsCreateDialogOpen(true),
    onEditTask: handleEdit,
    onArchiveTask: handleDelete,
    onAddToQueue: handleMoveToQueue,
    onDuplicateTask: handleDuplicate,
    isDialogOpen: isAnyDialogOpen,
    getSearchInputRef: searchInputRef,
  });

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
        onTaskSelect={(task) => taskSelectionActions.selectTask(task)}
        selectedTaskId={taskSelectionStore.selectedTaskId}
        queueTaskIds={queueTaskIds()}
        availableTags={availableTags()}
        onSearchInputRef={setSearchInputRef}
      />

      {/* Queue Panel */}
      <QueuePanel />

      {/* Create Dialog */}
      <Dialog
        open={isCreateDialogOpen()}
        onOpenChange={setIsCreateDialogOpen}
        title="Create New Task"
      >
        <form onSubmit={handleCreate} class="space-y-4">
          <Input
            label="Title"
            value={formData().title}
            onInput={(e) =>
              setFormData({ ...formData(), title: e.currentTarget.value })
            }
            ref={setCreateTitleInputRef}
            required
          />
          <ParentTaskSelect
            label="Parent Task"
            value={formData().parentId}
            onChange={(value) => setFormData({ ...formData(), parentId: value })}
            tasks={flattenHierarchy(taskStore.hierarchy)}
          />
          <Textarea
            label="Description"
            rows={4}
            value={formData().description}
            onInput={(e) =>
              setFormData({
                ...formData(),
                description: e.currentTarget.value,
              })
            }
          />
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">Tags</label>
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
              Cancel
            </Button>
            <Button type="submit">Create</Button>
          </div>
        </form>
      </Dialog>

      {/* Edit Dialog */}
      <Dialog
        open={editingTask() !== null}
        onOpenChange={(open) => !open && setEditingTask(null)}
        title="Edit Task"
      >
        <form onSubmit={handleUpdate} class="space-y-4">
          <Input
            label="Title"
            value={formData().title}
            onInput={(e) =>
              setFormData({ ...formData(), title: e.currentTarget.value })
            }
            required
          />
          <ParentTaskSelect
            label="Parent Task"
            value={formData().parentId}
            onChange={(value) => setFormData({ ...formData(), parentId: value })}
            tasks={flattenHierarchy(taskStore.hierarchy)}
            excludeTaskId={editingTask()?.id}
          />
          <Textarea
            label="Description"
            rows={4}
            value={formData().description}
            onInput={(e) =>
              setFormData({
                ...formData(),
                description: e.currentTarget.value,
              })
            }
          />
          <div class="flex flex-col gap-1.5">
            <label class="text-sm font-medium">Tags</label>
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
              Cancel
            </Button>
            <Button type="submit">Update</Button>
          </div>
        </form>
      </Dialog>
    </div>
  );
}
