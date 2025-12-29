import { createStore } from "solid-js/store";
import type {
  Task,
  TaskHierarchy,
  CreateTaskRequest,
  UpdateTaskRequest,
} from "../types/task";
import { tasksApi } from "../api/tasks";

interface TaskStore {
  tasks: Task[];
  hierarchy: TaskHierarchy[]; // タスク階層構造
  loading: boolean;
  error: string | null;
}

const [taskStore, setTaskStore] = createStore<TaskStore>({
  tasks: [],
  hierarchy: [],
  loading: false,
  error: null,
});

/**
 * タスクストアのアクション
 */
export const taskActions = {
  /**
   * 全タスクを読み込み
   */
  async loadTasks(): Promise<void> {
    setTaskStore("loading", true);
    setTaskStore("error", null);
    try {
      const tasks = await tasksApi.list();
      setTaskStore("tasks", tasks);
    } catch (error) {
      setTaskStore("error", error instanceof Error ? error.message : String(error));
      throw error;
    } finally {
      setTaskStore("loading", false);
    }
  },

  /**
   * タスク階層を読み込み（Draft/Active な親 + Draft/Active/Completed な子）
   */
  async loadHierarchy(): Promise<void> {
    setTaskStore("loading", true);
    setTaskStore("error", null);
    try {
      const hierarchy = await tasksApi.getHierarchy();
      setTaskStore("hierarchy", hierarchy);
    } catch (error) {
      setTaskStore("error", error instanceof Error ? error.message : String(error));
      throw error;
    } finally {
      setTaskStore("loading", false);
    }
  },

  /**
   * タスクを作成
   */
  async createTask(request: CreateTaskRequest): Promise<void> {
    setTaskStore("loading", true);
    setTaskStore("error", null);
    try {
      await tasksApi.create(request);
      // Note: タスク一覧は loadHierarchy() で再読み込みする
    } catch (error) {
      setTaskStore("error", error instanceof Error ? error.message : String(error));
      throw error;
    } finally {
      setTaskStore("loading", false);
    }
  },

  /**
   * タスクを取得
   */
  async getTask(taskId: string): Promise<Task> {
    setTaskStore("loading", true);
    setTaskStore("error", null);
    try {
      const task = await tasksApi.get(taskId);
      // Note: タスク一覧は loadHierarchy() で再読み込みする
      return task;
    } catch (error) {
      setTaskStore("error", error instanceof Error ? error.message : String(error));
      throw error;
    } finally {
      setTaskStore("loading", false);
    }
  },

  /**
   * タスクを更新
   */
  async updateTask(taskId: string, request: UpdateTaskRequest): Promise<void> {
    setTaskStore("loading", true);
    setTaskStore("error", null);
    try {
      await tasksApi.update(taskId, request);
      // Note: タスク一覧は loadHierarchy() で再読み込みする
    } catch (error) {
      setTaskStore("error", error instanceof Error ? error.message : String(error));
      throw error;
    } finally {
      setTaskStore("loading", false);
    }
  },

  /**
   * タスクを削除
   */
  async deleteTask(taskId: string): Promise<void> {
    setTaskStore("loading", true);
    setTaskStore("error", null);
    try {
      await tasksApi.delete(taskId);
      // Note: タスク一覧は loadHierarchy() で再読み込みする
    } catch (error) {
      setTaskStore("error", error instanceof Error ? error.message : String(error));
      throw error;
    } finally {
      setTaskStore("loading", false);
    }
  },

  /**
   * エラーをクリア
   */
  clearError(): void {
    setTaskStore("error", null);
  },
};

export { taskStore };
