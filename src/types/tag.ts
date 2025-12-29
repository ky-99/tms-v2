/**
 * タグレスポンス（バックエンドからの返却値）
 */
export interface Tag {
  id: string;
  name: string;
  color?: string;
  usageCount: number;
  createdAt: string;
}

/**
 * タグ作成リクエスト
 */
export interface CreateTagRequest {
  name: string;
  color?: string;
}

/**
 * タグ更新リクエスト
 */
export interface UpdateTagRequest {
  name?: string;
  color?: string;
}

/**
 * プリセットカラー定義（Phase 1）
 */
export const PRESET_TAG_COLORS = [
  { name: "Red", value: "#ef4444" },
  { name: "Orange", value: "#f97316" },
  { name: "Yellow", value: "#eab308" },
  { name: "Green", value: "#22c55e" },
  { name: "Blue", value: "#3b82f6" },
  { name: "Indigo", value: "#6366f1" },
  { name: "Purple", value: "#a855f7" },
  { name: "Pink", value: "#ec4899" },
] as const;
