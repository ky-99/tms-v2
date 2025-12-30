import { invokeWithTimeout } from "../lib/invoke";
import { withErrorHandling } from "../lib/errorHandler";
import type { Tag, CreateTagRequest, UpdateTagRequest } from "../types/tag";

/**
 * タグAPI
 */
export const tagsApi = {
  /**
   * 全タグを取得
   */
  async list(): Promise<Tag[]> {
    return await withErrorHandling(
      () => invokeWithTimeout<Tag[]>("list_tags"),
      "タグの取得に失敗しました"
    );
  },

  /**
   * タグを作成
   */
  async create(request: CreateTagRequest): Promise<Tag> {
    return await withErrorHandling(
      () => invokeWithTimeout<Tag>("create_tag", { request }),
      "タグの作成に失敗しました"
    );
  },

  /**
   * タグを更新
   */
  async update(tagId: string, request: UpdateTagRequest): Promise<Tag> {
    return await withErrorHandling(
      () => invokeWithTimeout<Tag>("update_tag", { tagId, request }),
      "タグの更新に失敗しました"
    );
  },

  /**
   * タグを削除
   */
  async delete(tagId: string): Promise<void> {
    return await withErrorHandling(
      () => invokeWithTimeout<void>("delete_tag", { tagId }),
      "タグの削除に失敗しました"
    );
  },
};
