import { onMount, onCleanup } from "solid-js";
import { taskSelectionStore, taskSelectionActions } from "../stores/taskSelectionStore";
import type { TaskHierarchy } from "../types/task";

interface KeyboardShortcutsConfig {
  onCreateTask: () => void;
  onEditTask: (task: TaskHierarchy) => void;
  onArchiveTask: (task: TaskHierarchy) => void;
  onAddToQueue: (task: TaskHierarchy) => void;
  onDuplicateTask: (task: TaskHierarchy) => void;
  isDialogOpen: () => boolean;
  getSearchInputRef: () => HTMLInputElement | undefined;
}

export function useKeyboardShortcuts(config: KeyboardShortcutsConfig) {
  const handleKeyDown = (event: KeyboardEvent) => {
    // 1. 入力欄フォーカス中はスキップ
    if (isInputFocused()) {
      return;
    }

    // 2. ダイアログ表示中はスキップ（Escapeを除く）
    if (config.isDialogOpen() && event.key !== "Escape") {
      return;
    }

    // 3. Cmd/Ctrl判定
    const isMac = navigator.platform.toLowerCase().includes("mac");
    const isModifier = isMac ? event.metaKey : event.ctrlKey;

    // 4. 各ショートカットのハンドリング
    if (isModifier) {
      switch (event.key.toLowerCase()) {
        case "n":
          event.preventDefault();
          config.onCreateTask();
          break;

        case "e":
          event.preventDefault();
          handleEdit();
          break;

        case "a":
          event.preventDefault();
          handleArchive();
          break;

        case "q":
          event.preventDefault();
          handleAddToQueue();
          break;

        case "d":
          event.preventDefault();
          handleDuplicate();
          break;

        case "f":
          event.preventDefault();
          focusSearchBar();
          break;

        default:
          // 他のショートカットはブラウザデフォルト
          break;
      }
    }
  };

  const isInputFocused = (): boolean => {
    const activeElement = document.activeElement;
    if (!activeElement) return false;

    const tagName = activeElement.tagName.toLowerCase();
    return (
      tagName === "input" ||
      tagName === "textarea" ||
      tagName === "select" ||
      activeElement.getAttribute("contenteditable") === "true"
    );
  };

  const handleEdit = () => {
    const selectedTask = taskSelectionStore.selectedTask;
    if (!selectedTask) return;
    if (selectedTask.status !== "draft") return;

    config.onEditTask(selectedTask);
    taskSelectionActions.clearSelection();
  };

  const handleArchive = () => {
    const selectedTask = taskSelectionStore.selectedTask;
    if (!selectedTask) return;
    if (selectedTask.status !== "draft") return;

    config.onArchiveTask(selectedTask);
    taskSelectionActions.clearSelection();
  };

  const handleAddToQueue = () => {
    const selectedTask = taskSelectionStore.selectedTask;
    if (!selectedTask) return;
    if (selectedTask.children && selectedTask.children.length > 0) return;

    config.onAddToQueue(selectedTask);
    taskSelectionActions.clearSelection();
  };

  const handleDuplicate = () => {
    const selectedTask = taskSelectionStore.selectedTask;
    if (!selectedTask) return;

    config.onDuplicateTask(selectedTask);
    taskSelectionActions.clearSelection();
  };

  const focusSearchBar = () => {
    const searchInputRef = config.getSearchInputRef();
    if (searchInputRef) {
      searchInputRef.focus();
    }
  };

  onMount(() => {
    document.addEventListener("keydown", handleKeyDown);
  });

  onCleanup(() => {
    document.removeEventListener("keydown", handleKeyDown);
  });
}
