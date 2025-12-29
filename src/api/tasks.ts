import { invokeWithTimeout } from "../lib/invoke";
import type {
  Task,
  TaskHierarchy,
  CreateTaskRequest,
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
    return await invokeWithTimeout<Task>("create_task", { req: request });
  },

  /**
   * タスクを取得
   */
  async get(taskId: string): Promise<Task> {
    return await invokeWithTimeout<Task>("get_task", { taskId });
  },

  /**
   * タスクを更新
   */
  async update(taskId: string, request: UpdateTaskRequest): Promise<Task> {
    return await invokeWithTimeout<Task>("update_task", { taskId, req: request });
  },

  /**
   * タスクを削除（論理削除: Draft → Archived）
   */
  async delete(taskId: string): Promise<void> {
    return await invokeWithTimeout<void>("delete_task", { taskId });
  },

  /**
   * タスクを完全に削除（物理削除: データベースから削除）
   * ※Archivedステータスのタスクのみ削除可能
   */
  async deletePermanently(taskId: string): Promise<void> {
    return await invokeWithTimeout<void>("delete_task_permanently", { taskId });
  },

  /**
   * タスクを復元（Archived → Draft）
   */
  async restore(taskId: string): Promise<Task> {
    return await invokeWithTimeout<Task>("restore_task", { taskId });
  },

  /**
   * タスク一覧を取得
   */
  async list(): Promise<Task[]> {
    return await invokeWithTimeout<Task[]>("list_tasks");
  },

  /**
   * ステータスでフィルタしてタスク一覧を取得
   */
  async listByStatus(status: string[]): Promise<Task[]> {
    return await invokeWithTimeout<Task[]>("list_tasks", { status });
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
    return await invokeWithTimeout<PaginatedTaskResponse>(
      "list_tasks_paginated",
      {
        params: {
          status: status ?? null,
          limit: limit ?? null,
          offset: offset ?? null,
        },
      }
    );
  },

  /**
   * タスク階層を取得（Draft/Active な親 + Draft/Active/Completed な子）
   */
  async getHierarchy(): Promise<TaskHierarchy[]> {
    return await invokeWithTimeout<TaskHierarchy[]>("get_task_hierarchy");
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
    return await invokeWithTimeout<Task[]>("search_tasks", {
      q: q ?? null,
      status: status ?? null,
      tags: tags ?? null,
    });
  },
};
