/**
 * タスクのステータス
 */
export type TaskStatus = "draft" | "active" | "completed" | "archived";

/**
 * タスクレスポンス（バックエンドからの返却値）
 */
export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  tags: string[];
  parentId?: string;
  parentTitle?: string; // 親タスクのタイトル
  createdAt: string;
  updatedAt: string;
  childrenIds: string[]; // 子タスクのIDリスト
}

/**
 * ページネーション付きタスクリストレスポンス
 */
export interface PaginatedTaskResponse {
  tasks: Task[];
  total: number;
}

/**
 * タスク階層レスポンス（get_hierarchy API用）
 * 子タスクを再帰的に含む階層構造
 */
export interface TaskHierarchy {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  tags: string[];
  parentId?: string;
  createdAt: string;
  updatedAt: string;
  children: TaskHierarchy[]; // 子タスクの配列（再帰的）
}

/**
 * タスク作成リクエスト
 */
export interface CreateTaskRequest {
  title: string;
  description?: string;
  tags?: string[];
  parentId?: string;
}

/**
 * タスク複製リクエスト
 */
export interface DuplicateTaskRequest {
  taskId: string;
  newTitle?: string;
}

/**
 * タスク更新リクエスト
 */
export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  status?: TaskStatus;
  tags?: string[];
  parentId?: string;
}
