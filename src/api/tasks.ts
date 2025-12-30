import { invokeWithTimeout } from "../lib/invoke";
import { withErrorHandling } from "../lib/errorHandler";
import type {
  Task,
  TaskHierarchy,
  CreateTaskRequest,
  DuplicateTaskRequest,
  UpdateTaskRequest,
  PaginatedTaskResponse,
} from "../types/task";

/**
 * タスクAPI
 */
export const tasksApi = {
  /**
   * タスクを作成
   */
  async create(request: CreateTaskRequest): Promise<Task> {
    return await withErrorHandling(
      () => invokeWithTimeout<Task>("create_task", { req: request }),
      "タスクの作成に失敗しました"
    );
  },

  /**
   * タスクを複製
   */
  async duplicate(request: DuplicateTaskRequest): Promise<Task> {
    return await withErrorHandling(
      () => invokeWithTimeout<Task>("duplicate_task", { req: request }),
      "タスクの複製に失敗しました"
    );
  },

  /**
   * タスクを取得
   */
  async get(taskId: string): Promise<Task> {
    return await withErrorHandling(
      () => invokeWithTimeout<Task>("get_task", { taskId })
    );
  },

  /**
   * タスクを更新
   */
  async update(taskId: string, request: UpdateTaskRequest): Promise<Task> {
    return await withErrorHandling(
      () => invokeWithTimeout<Task>("update_task", { taskId, req: request }),
      "タスクの更新に失敗しました"
    );
  },

  /**
   * タスクを削除（論理削除: Draft → Archived）
   */
  async delete(taskId: string): Promise<void> {
    return await withErrorHandling(
      () => invokeWithTimeout<void>("delete_task", { taskId }),
      "タスクの削除に失敗しました"
    );
  },

  /**
   * タスクを完全に削除（物理削除: データベースから削除）
   * ※Archivedステータスのタスクのみ削除可能
   */
  async deletePermanently(taskId: string): Promise<void> {
    return await withErrorHandling(
      () => invokeWithTimeout<void>("delete_task_permanently", { taskId }),
      "タスクの完全削除に失敗しました"
    );
  },

  /**
   * タスクを復元（Archived → Draft）
   */
  async restore(taskId: string): Promise<Task> {
    return await withErrorHandling(
      () => invokeWithTimeout<Task>("restore_task", { taskId }),
      "タスクの復元に失敗しました"
    );
  },

  /**
   * タスク一覧を取得
   */
  async list(): Promise<Task[]> {
    return await withErrorHandling(
      () => invokeWithTimeout<Task[]>("list_tasks")
    );
  },

  /**
   * ステータスでフィルタしてタスク一覧を取得
   */
  async listByStatus(status: string[]): Promise<Task[]> {
    return await withErrorHandling(
      () => invokeWithTimeout<Task[]>("list_tasks", { status })
    );
  },

  /**
   * タスク一覧を取得（ページネーション対応）
   * @param status - ステータスフィルタ（オプション）
   * @param limit - 取得件数（デフォルト: 20）
   * @param offset - オフセット（デフォルト: 0）
   * @returns タスクリストと総件数
   */
  async listPaginated(
    status?: string[],
    limit?: number,
    offset?: number
  ): Promise<PaginatedTaskResponse> {
    return await withErrorHandling(
      () =>
        invokeWithTimeout<PaginatedTaskResponse>("list_tasks_paginated", {
          params: {
            status: status ?? null,
            limit: limit ?? null,
            offset: offset ?? null,
          },
        })
    );
  },

  /**
   * タスク階層を取得（Draft/Active な親 + Draft/Active/Completed な子）
   */
  async getHierarchy(): Promise<TaskHierarchy[]> {
    return await withErrorHandling(
      () => invokeWithTimeout<TaskHierarchy[]>("get_task_hierarchy")
    );
  },

  /**
   * タスクを検索（キーワード、ステータス、タグでフィルタリング）
   * @param q - キーワード（タイトル、説明で検索）
   * @param status - ステータスフィルタ
   * @param tags - タグフィルタ（OR条件）
   * @returns マッチしたタスクリスト
   */
  async search(
    q?: string,
    status?: string,
    tags?: string[]
  ): Promise<Task[]> {
    return await withErrorHandling(
      () =>
        invokeWithTimeout<Task[]>("search_tasks", {
          q: q ?? null,
          status: status ?? null,
          tags: tags ?? null,
        })
    );
  },

  /**
   * タスク検索（ページネーション対応、Backend search）
   * @param q - 検索キーワード（タイトル・説明文）
   * @param status - ステータスフィルタ
   * @param tags - タグフィルタ（OR条件）
   * @param limit - 1ページあたりの件数（デフォルト: 100）
   * @param offset - オフセット（デフォルト: 0）
   * @returns ページネーション対応の検索結果
   */
  async searchPaginated(
    q?: string,
    status?: string,
    tags?: string[],
    limit?: number,
    offset?: number
  ): Promise<PaginatedTaskResponse> {
    return await withErrorHandling(
      () =>
        invokeWithTimeout<PaginatedTaskResponse>("search_tasks", {
          q: q ?? null,
          status: status ?? null,
          tags: tags ?? null,
          limit: limit ?? null,
          offset: offset ?? null,
        })
    );
  },

  /**
   * タスクIDのみを検索（軽量版）
   * タグフィルター用に最適化
   * @param tags - タグフィルタ（OR条件）
   * @param status - ステータスフィルタ（デフォルト: draft + active + completed）
   * @returns マッチしたタスクIDリスト
   */
  async searchIds(tags?: string[], status?: string): Promise<string[]> {
    return await withErrorHandling(
      () =>
        invokeWithTimeout<string[]>("search_task_ids", {
          tags: tags ?? null,
          status: status ?? null,
        })
    );
  },
};
