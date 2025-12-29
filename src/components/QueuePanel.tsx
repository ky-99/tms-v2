import { For, onMount, Show, createMemo, createSignal } from "solid-js";
import { queueStore, queueActions } from "../stores/queueStore";
import { Button } from "./Button";
import { ConfirmDialog } from "./ConfirmDialog";
import { cn } from "../lib/utils";
import {
  DragDropProvider,
  DragDropSensors,
  SortableProvider,
  createSortable,
  closestCenter,
} from "@thisbeyond/solid-dnd";
import type { DragEvent } from "@thisbeyond/solid-dnd";

// Icon components
function CheckIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path d="M20 6 9 17l-5-5" />
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

function CircleIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10" />
    </svg>
  );
}

function AlertCircleIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="10" />
      <path d="M12 8v4m0 4h.01" />
    </svg>
  );
}

function GripVerticalIcon() {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <circle cx="9" cy="5" r="1" fill="currentColor" />
      <circle cx="9" cy="12" r="1" fill="currentColor" />
      <circle cx="9" cy="19" r="1" fill="currentColor" />
      <circle cx="15" cy="5" r="1" fill="currentColor" />
      <circle cx="15" cy="12" r="1" fill="currentColor" />
      <circle cx="15" cy="19" r="1" fill="currentColor" />
    </svg>
  );
}

interface SortableTaskProps {
  entry: { taskId: string; taskTitle: string };
  index: number;
  onMarkAsCompleted: (taskId: string) => void;
  onReturnToDraft: (taskId: string) => void;
}

function SortableTask(props: SortableTaskProps) {
  const sortable = createSortable(props.entry.taskId);

  return (
    <div
      ref={sortable.ref}
      class={cn(
        "group flex items-center gap-3 rounded-lg border bg-card p-3",
        props.index === 0
          ? "border-primary bg-primary/5 shadow-sm"
          : "border-border hover:bg-secondary/50",
        sortable.isActiveDraggable && "opacity-50"
      )}
      style={{
        transform: `translate3d(0px, ${sortable.transform?.y ?? 0}px, 0)`,
        transition: sortable.isActiveDraggable ? "none" : "transform 0.2s ease",
      }}
    >
      {/* Drag handle */}
      <div
        {...sortable.dragActivators}
        class="shrink-0 cursor-grab text-muted-foreground hover:text-foreground active:cursor-grabbing"
      >
        <GripVerticalIcon />
      </div>

      {/* Status icon */}
      <Show
        when={props.index === 0}
        fallback={
          <div class="shrink-0 text-muted-foreground">
            <CircleIcon />
          </div>
        }
      >
        <div class="shrink-0 text-primary">
          <AlertCircleIcon />
        </div>
      </Show>

      {/* Task title and position */}
      <div class="flex-1 min-w-0">
        <p class="truncate text-sm font-medium text-foreground">
          {props.entry.taskTitle}
        </p>
        <Show
          when={props.index === 0}
          fallback={
            <p class="text-xs text-muted-foreground">
              Waiting ({props.index})
            </p>
          }
        >
          <p class="text-xs text-primary font-medium">In Progress</p>
        </Show>
      </div>

      {/* Action buttons */}
      <div class="flex gap-1 shrink-0">
        <Button
          onClick={() => props.onMarkAsCompleted(props.entry.taskId)}
          class="h-8 w-8 p-0"
          disabled={props.index !== 0}
          title="Complete"
        >
          <CheckIcon />
        </Button>
        <Button
          variant="ghost"
          onClick={() => props.onReturnToDraft(props.entry.taskId)}
          class="h-8 w-8 p-0 text-destructive hover:bg-destructive/10 hover:text-destructive"
          title="Remove"
        >
          <XIcon />
        </Button>
      </div>
    </div>
  );
}

export function QueuePanel() {
  const [confirmDialogOpen, setConfirmDialogOpen] = createSignal(false);
  const [confirmAction, setConfirmAction] = createSignal<"complete" | "clear">("complete");

  onMount(() => {
    queueActions.loadQueue();
  });

  // Memoize task IDs for SortableProvider
  const taskIds = createMemo(() => queueStore.queue.map((e) => e.taskId));

  const handleReturnToDraft = async (taskId: string) => {
    try {
      await queueActions.returnToDraft(taskId);
    } catch (error) {
      console.error("Failed to return to draft:", error);
    }
  };

  const handleMarkAsCompleted = async (taskId: string) => {
    try {
      await queueActions.markAsCompleted(taskId);
    } catch (error) {
      console.error("Failed to mark as completed:", error);
    }
  };

  const handleCompleteAll = () => {
    setConfirmAction("complete");
    setConfirmDialogOpen(true);
  };

  const handleClearAll = () => {
    setConfirmAction("clear");
    setConfirmDialogOpen(true);
  };

  const executeAction = async () => {
    try {
      if (confirmAction() === "complete") {
        await queueActions.completeAll();
      } else {
        await queueActions.clearQueue();
      }
    } catch (error) {
      console.error("Failed to execute action:", error);
    }
  };

  const handleDragEnd = async ({ draggable, droppable }: DragEvent) => {
    if (draggable && droppable) {
      const currentQueue = queueStore.queue;
      const fromIndex = currentQueue.findIndex((e) => e.taskId === draggable.id);
      const toIndex = currentQueue.findIndex((e) => e.taskId === droppable.id);

      if (fromIndex !== -1 && toIndex !== -1 && fromIndex !== toIndex) {
        // Optimistic UI update
        const newQueue = [...currentQueue];
        const [movedItem] = newQueue.splice(fromIndex, 1);
        newQueue.splice(toIndex, 0, movedItem);

        try {
          // Call API to persist the new order
          const taskIds = newQueue.map((e) => e.taskId);
          await queueActions.reorderQueue(taskIds);
        } catch (error) {
          console.error("Failed to reorder queue:", error);
          // Reload queue on error to restore correct state
          await queueActions.loadQueue();
        }
      }
    }
  };

  return (
    <div class="flex w-80 flex-col border-l border-border">
      <div class="border-b border-border bg-card px-6 py-4">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold text-foreground">
            Task Queue ({queueStore.queue.length})
          </h2>
        </div>
        <Show when={queueStore.queue.length > 0}>
          <div class="flex gap-2 mt-3">
            <Button
              onClick={handleCompleteAll}
              class="flex-1 py-1.5 text-xs"
              disabled={queueStore.loading}
            >
              Complete All
            </Button>
            <Button
              onClick={handleClearAll}
              variant="secondary"
              class="flex-1 py-1.5 text-xs"
              disabled={queueStore.loading}
            >
              Clear All
            </Button>
          </div>
        </Show>
      </div>

      <div class="flex-1 overflow-y-auto p-4">
        <Show
          when={queueStore.queue.length > 0}
          fallback={
            <div class="flex min-h-24 items-center justify-center rounded-lg border-2 border-dashed border-border">
              <p class="text-sm text-muted-foreground">Queue is empty</p>
            </div>
          }
        >
          <DragDropProvider onDragEnd={handleDragEnd} collisionDetector={closestCenter}>
            <DragDropSensors />
            <SortableProvider ids={taskIds()}>
              <div class="space-y-2">
                <For each={queueStore.queue}>
                  {(entry, index) => (
                    <SortableTask
                      entry={entry}
                      index={index()}
                      onMarkAsCompleted={handleMarkAsCompleted}
                      onReturnToDraft={handleReturnToDraft}
                    />
                  )}
                </For>
              </div>
            </SortableProvider>
          </DragDropProvider>
        </Show>
      </div>

      <ConfirmDialog
        open={confirmDialogOpen()}
        onOpenChange={setConfirmDialogOpen}
        title={confirmAction() === "complete" ? "Complete All Tasks" : "Clear All Tasks"}
        description={
          confirmAction() === "complete"
            ? `Are you sure you want to mark all ${queueStore.queue.length} tasks in the queue as completed? This action cannot be undone.`
            : `Are you sure you want to clear all ${queueStore.queue.length} tasks from the queue? Tasks will be moved back to draft status.`
        }
        confirmText={confirmAction() === "complete" ? "Complete All" : "Clear All"}
        cancelText="Cancel"
        variant={confirmAction() === "complete" ? "default" : "destructive"}
        onConfirm={executeAction}
      />
    </div>
  );
}
