import { invokeWithTimeout } from "../lib/invoke";
import type {
  AddToQueueRequest,
  CompleteAllQueueResponse,
  QueueEntry,
  QueueEntryWithTask,
  RemoveFromQueueRequest,
  ReorderQueueRequest,
  UpdateQueueRequest,
} from "../types/queue";

export const queueApi = {
  /**
   * キュー全体を取得（タスク情報含む）
   */
  async getQueue(): Promise<QueueEntryWithTask[]> {
    return await invokeWithTimeout<QueueEntryWithTask[]>("get_task_queue");
  },

  /**
   * タスクをキューに追加（ステータスがActiveになる）
   */
  async addToQueue(taskId: string): Promise<QueueEntry> {
    const req: AddToQueueRequest = { taskId };
    return await invokeWithTimeout<QueueEntry>("add_task_to_queue", { req });
  },

  /**
   * タスクをキューから削除（ステータスを指定）
   */
  async removeFromQueue(
    taskId: string,
    targetStatus: "draft" | "completed"
  ): Promise<void> {
    const req: RemoveFromQueueRequest = { taskId, targetStatus };
    return await invokeWithTimeout<void>("remove_task_from_queue", { req });
  },

  /**
   * タスクをdraftに戻す（タスクプールに戻る）
   */
  async returnToDraft(taskId: string): Promise<void> {
    return this.removeFromQueue(taskId, "draft");
  },

  /**
   * タスクを完了としてマーク
   */
  async markAsCompleted(taskId: string): Promise<void> {
    return this.removeFromQueue(taskId, "completed");
  },

  /**
   * キュー全体をクリア
   */
  async clearQueue(): Promise<void> {
    return await invokeWithTimeout<void>("clear_task_queue");
  },

  /**
   * キュー内の全タスクを完了状態にする
   */
  async completeAll(): Promise<CompleteAllQueueResponse> {
    return await invokeWithTimeout<CompleteAllQueueResponse>("complete_all_queue");
  },

  /**
   * タスクのキュー内位置を更新
   */
  async updateQueuePosition(
    taskId: string,
    newPosition: number
  ): Promise<QueueEntry> {
    const req: UpdateQueueRequest = { taskId, newPosition };
    return await invokeWithTimeout<QueueEntry>("update_queue_position", { req });
  },

  /**
   * キュー全体を一括で並び替え
   */
  async reorderQueue(taskIds: string[]): Promise<QueueEntry[]> {
    const req: ReorderQueueRequest = { taskIds };
    return await invokeWithTimeout<QueueEntry[]>("reorder_task_queue", { req });
  },
};
