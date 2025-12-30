import { invokeWithTimeout } from "../lib/invoke";
import { withErrorHandling } from "../lib/errorHandler";
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
    return await withErrorHandling(
      () => invokeWithTimeout<QueueEntryWithTask[]>("get_task_queue"),
      "キューの取得に失敗しました"
    );
  },

  /**
   * タスクをキューに追加（ステータスがActiveになる）
   */
  async addToQueue(taskId: string): Promise<QueueEntry> {
    const req: AddToQueueRequest = { taskId };
    return await withErrorHandling(
      () => invokeWithTimeout<QueueEntry>("add_task_to_queue", { req }),
      "タスクのキュー追加に失敗しました"
    );
  },

  /**
   * タスクをキューから削除（ステータスを指定）
   */
  async removeFromQueue(
    taskId: string,
    targetStatus: "draft" | "completed"
  ): Promise<void> {
    const req: RemoveFromQueueRequest = { taskId, targetStatus };
    return await withErrorHandling(
      () => invokeWithTimeout<void>("remove_task_from_queue", { req }),
      "タスクのキュー削除に失敗しました"
    );
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
    return await withErrorHandling(
      () => invokeWithTimeout<void>("clear_task_queue"),
      "キューのクリアに失敗しました"
    );
  },

  /**
   * キュー内の全タスクを完了状態にする
   */
  async completeAll(): Promise<CompleteAllQueueResponse> {
    return await withErrorHandling(
      () => invokeWithTimeout<CompleteAllQueueResponse>("complete_all_queue"),
      "全タスクの完了に失敗しました"
    );
  },

  /**
   * タスクのキュー内位置を更新
   */
  async updateQueuePosition(
    taskId: string,
    newPosition: number
  ): Promise<QueueEntry> {
    const req: UpdateQueueRequest = { taskId, newPosition };
    return await withErrorHandling(
      () => invokeWithTimeout<QueueEntry>("update_queue_position", { req }),
      "キュー位置の更新に失敗しました"
    );
  },

  /**
   * キュー全体を一括で並び替え
   */
  async reorderQueue(taskIds: string[]): Promise<QueueEntry[]> {
    const req: ReorderQueueRequest = { taskIds };
    return await withErrorHandling(
      () => invokeWithTimeout<QueueEntry[]>("reorder_task_queue", { req }),
      "キューの並び替えに失敗しました"
    );
  },
};
