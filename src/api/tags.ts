import { invokeWithTimeout } from "../lib/invoke";
import type { Tag, CreateTagRequest, UpdateTagRequest } from "../types/tag";

/**
 * タグAPI
 */
export const tagsApi = {
  /**
   * 全タグを取得
   */
  async list(): Promise<Tag[]> {
    return await invokeWithTimeout<Tag[]>("list_tags");
  },

  /**
   * タグを作成
   */
  async create(request: CreateTagRequest): Promise<Tag> {
    return await invokeWithTimeout<Tag>("create_tag", { request });
  },

  /**
   * タグを更新
   */
  async update(tagId: string, request: UpdateTagRequest): Promise<Tag> {
    return await invokeWithTimeout<Tag>("update_tag", { tagId, request });
  },

  /**
   * タグを削除
   */
  async delete(tagId: string): Promise<void> {
    return await invokeWithTimeout<void>("delete_tag", { tagId });
  },
};
