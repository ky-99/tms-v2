use thiserror::Error;

/// サービス層のエラー型
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    #[error("Parent task not found: {0}")]
    ParentTaskNotFound(String),

    #[error("Tag not found: {0}")]
    TagNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Circular dependency detected: task {0} creates a cycle")]
    CircularDependency(String),

    #[error("Task has children and cannot be deleted: {0}")]
    TaskHasChildren(String),

    #[error("Task is not in draft status and cannot be edited or deleted: {0}")]
    TaskNotDraft(String),

    #[error("Task is not archived and cannot be permanently deleted: {0}")]
    TaskNotArchived(String),

    #[error("孫タスクの作成は禁止されています。階層は最大2レベル（親-子）までです。")]
    GrandchildNotAllowed,

    #[error("Tag is in use and cannot be deleted: {0}")]
    TagInUse(String),

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("Queue entry not found: {0}")]
    QueueEntryNotFound(String),

    #[error("Duplicate queue entry: {0}")]
    DuplicateQueueEntry(String),

    #[error("Invalid task status: {0}")]
    InvalidTaskStatus(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Internal error: {0}")]
    InternalError(String),
}

// Tauri の Result<T, String> へ変換
impl From<ServiceError> for String {
    fn from(err: ServiceError) -> Self {
        err.to_string()
    }
}

// rusqlite::Error からの変換（既存のdb層との互換性）
impl From<rusqlite::Error> for ServiceError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceError::InternalError(format!("SQLite error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ServiceError::TaskNotFound("task-123".to_string());
        assert_eq!(err.to_string(), "Task not found: task-123");
    }

    #[test]
    fn test_error_to_string_conversion() {
        let err = ServiceError::InvalidInput("Title cannot be empty".to_string());
        let err_string: String = err.into();
        assert_eq!(err_string, "Invalid input: Title cannot be empty");
    }

    #[test]
    fn test_circular_dependency_error() {
        let err = ServiceError::CircularDependency("task-456".to_string());
        assert!(err.to_string().contains("cycle"));
    }
}
