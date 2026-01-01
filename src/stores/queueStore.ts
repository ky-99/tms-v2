import { createStore } from "solid-js/store";
import type { QueueEntryWithTask } from "../types/queue";
import { queueApi } from "../api/queue";

interface QueueStore {
  queue: QueueEntryWithTask[];
  loading: boolean;
  error: string | null;
}

const [queueStore, setQueueStore] = createStore<QueueStore>({
  queue: [],
  loading: false,
  error: null,
});

export const queueActions = {
  /**
   * キュー全体を取得
   */
  async loadQueue(): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      const queue = await queueApi.getQueue();
      setQueueStore("queue", queue);
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * タスクをキューに追加
   */
  async addToQueue(taskId: string): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      await queueApi.addToQueue(taskId);
      // キューを再読み込み
      await queueActions.loadQueue();
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * タスクをdraftに戻す（タスクプールに戻る）
   */
  async returnToDraft(taskId: string): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      await queueApi.returnToDraft(taskId);
      // キューを再読み込み
      await queueActions.loadQueue();
      // Note: 呼び出し側でloadHierarchy()を実行すること
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * タスクを完了としてマーク
   */
  async markAsCompleted(taskId: string): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      await queueApi.markAsCompleted(taskId);
      // キューを再読み込み
      await queueActions.loadQueue();
      // Note: 呼び出し側でloadHierarchy()を実行すること
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * キュー全体をクリア
   */
  async clearQueue(): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      await queueApi.clearQueue();
      // キューを再読み込み
      await queueActions.loadQueue();
      // Note: 呼び出し側でloadHierarchy()を実行すること
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * キュー内の全タスクを完了状態にする
   */
  async completeAll(): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      await queueApi.completeAll();
      // キューを再読み込み（空になるはず）
      await queueActions.loadQueue();
      // Note: 呼び出し側でloadHierarchy()を実行すること
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * タスクのキュー内位置を更新
   */
  async updateQueuePosition(
    taskId: string,
    newPosition: number
  ): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      await queueApi.updateQueuePosition(taskId, newPosition);
      // キューを再読み込み
      await queueActions.loadQueue();
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * キュー全体を並び替え
   */
  async reorderQueue(taskIds: string[]): Promise<void> {
    setQueueStore("loading", true);
    setQueueStore("error", null);
    try {
      await queueApi.reorderQueue(taskIds);
      // キューを再読み込み
      await queueActions.loadQueue();
    } catch (error) {
      setQueueStore(
        "error",
        error instanceof Error ? error.message : String(error)
      );
      throw error;
    } finally {
      setQueueStore("loading", false);
    }
  },

  /**
   * エラーをクリア
   */
  clearError(): void {
    setQueueStore("error", null);
  },
};

export { queueStore };
