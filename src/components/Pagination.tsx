import { Component, createSignal } from "solid-js";

interface PaginationProps {
  currentPage: number;
  totalPages: number;
  totalItems: number;
  onPageChange: (page: number) => void;
}

/**
 * ページネーションコンポーネント
 * UI形式: < [number input] >
 * - 前ページボタン (<)
 * - ページ番号入力フィールド (直接ジャンプ可能)
 * - 次ページボタン (>)
 * - 総ページ数表示 (例: "Page 1 of 5")
 */
const Pagination: Component<PaginationProps> = (props) => {
  const [inputValue, setInputValue] = createSignal(props.currentPage.toString());

  // currentPageが変更されたら入力フィールドも更新
  const handlePageChange = (newPage: number) => {
    if (newPage < 1 || newPage > props.totalPages) return;
    setInputValue(newPage.toString());
    props.onPageChange(newPage);
  };

  // 入力フィールドからのページ変更
  const handleInputChange = (e: Event) => {
    const target = e.target as HTMLInputElement;
    setInputValue(target.value);
  };

  const handleInputKeyPress = (e: KeyboardEvent) => {
    if (e.key === "Enter") {
      const page = parseInt(inputValue());
      if (!isNaN(page) && page >= 1 && page <= props.totalPages) {
        props.onPageChange(page);
      } else {
        // 範囲外の場合は現在のページに戻す
        setInputValue(props.currentPage.toString());
      }
    }
  };

  const handleInputBlur = () => {
    const page = parseInt(inputValue());
    if (isNaN(page) || page < 1 || page > props.totalPages) {
      // 無効な値の場合は現在のページに戻す
      setInputValue(props.currentPage.toString());
    } else if (page !== props.currentPage) {
      props.onPageChange(page);
    }
  };

  return (
    <div class="flex items-center justify-center gap-3 mt-8 py-4">
      {/* 前ページボタン */}
      <button
        onClick={() => handlePageChange(props.currentPage - 1)}
        disabled={props.currentPage <= 1}
        class="h-9 px-3 rounded-md border border-border bg-card text-foreground hover:bg-secondary disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        aria-label="Previous page"
      >
        &lt;
      </button>

      {/* ページ番号入力フィールド */}
      <div class="flex items-center gap-2">
        <span class="text-sm text-muted-foreground">Page</span>
        <input
          type="number"
          min="1"
          max={props.totalPages}
          value={inputValue()}
          onInput={handleInputChange}
          onKeyPress={handleInputKeyPress}
          onBlur={handleInputBlur}
          class="w-16 h-9 px-2 text-center border border-border rounded-md bg-card text-foreground focus:outline-none focus:ring-2 focus:ring-primary [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
          aria-label="Page number"
        />
        <span class="text-sm text-muted-foreground">
          of {props.totalPages}
        </span>
      </div>

      {/* 次ページボタン */}
      <button
        onClick={() => handlePageChange(props.currentPage + 1)}
        disabled={props.currentPage >= props.totalPages}
        class="h-9 px-3 rounded-md border border-border bg-card text-foreground hover:bg-secondary disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        aria-label="Next page"
      >
        &gt;
      </button>
    </div>
  );
};

export default Pagination;
