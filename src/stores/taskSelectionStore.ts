import { createStore } from "solid-js/store";
import type { TaskHierarchy } from "../types/task";

interface TaskSelectionStore {
  selectedTaskId: string | null;
  selectedTask: TaskHierarchy | null;
}

const [taskSelectionStore, setTaskSelectionStore] = createStore<TaskSelectionStore>({
  selectedTaskId: null,
  selectedTask: null,
});

export const taskSelectionActions = {
  selectTask(task: TaskHierarchy | null): void {
    setTaskSelectionStore({
      selectedTaskId: task?.id ?? null,
      selectedTask: task,
    });
  },

  clearSelection(): void {
    setTaskSelectionStore({
      selectedTaskId: null,
      selectedTask: null,
    });
  },
};

export { taskSelectionStore };
