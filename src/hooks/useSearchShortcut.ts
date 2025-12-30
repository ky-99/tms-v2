import { onMount, onCleanup } from "solid-js";

interface SearchShortcutConfig {
  getSearchInputRef: () => HTMLInputElement | undefined;
}

/**
 * シンプルな検索フォーカス用のキーボードショートカット
 * Cmd/Ctrl + F で検索バーにフォーカス
 */
export function useSearchShortcut(config: SearchShortcutConfig) {
  const handleKeyDown = (event: KeyboardEvent) => {
    // 入力欄フォーカス中はスキップ
    if (isInputFocused()) {
      return;
    }

    // Cmd/Ctrl判定
    const isMac = navigator.platform.toLowerCase().includes("mac");
    const isModifier = isMac ? event.metaKey : event.ctrlKey;

    // Cmd/Ctrl + F
    if (isModifier && event.key.toLowerCase() === "f") {
      event.preventDefault();
      focusSearchBar();
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
