export interface QueueEntry {
  taskId: string;
  position: number;
  addedAt: string;
}

export interface QueueEntryWithTask {
  taskId: string;
  position: number;
  addedAt: string;
  taskTitle: string;
  taskStatus: string;
  taskDescription?: string;
}

export interface AddToQueueRequest {
  taskId: string;
}

export interface RemoveFromQueueRequest {
  taskId: string;
  targetStatus: "draft" | "completed";
}

export interface UpdateQueueRequest {
  taskId: string;
  newPosition: number;
}

export interface ReorderQueueRequest {
  taskIds: string[];
}
