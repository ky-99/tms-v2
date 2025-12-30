import { toastStore, type ToastCategory } from "../stores/toastStore";

/**
 * エラーからカテゴリを判定
 */
function getErrorCategory(error: unknown): ToastCategory {
  if (error instanceof TypeError && error.message.includes("fetch")) {
    return "network";
  }

  if (error instanceof Error) {
    const message = error.message.toLowerCase();
    if (
      message.includes("network") ||
      message.includes("connection") ||
      message.includes("timeout")
    ) {
      return "network";
    }

    if (
      message.includes("validation") ||
      message.includes("invalid") ||
      message.includes("required")
    ) {
      return "validation";
    }
  }

  return "server";
}

/**
 * エラーメッセージを取得
 * サーバーから返ってきたメッセージをそのまま使用
 */
function getErrorMessage(error: unknown): string {
  // Errorオブジェクトの場合
  if (error instanceof Error) {
    return error.message;
  }

  // 文字列の場合（Tauriは文字列でエラーを返すことが多い）
  if (typeof error === "string") {
    return error;
  }

  // オブジェクトでmessageプロパティを持つ場合
  if (error && typeof error === "object" && "message" in error) {
    const msg = (error as { message: unknown }).message;
    if (typeof msg === "string") {
      return msg;
    }
  }

  // その他の場合、可能な限り文字列化
  try {
    return String(error);
  } catch {
    return "予期しないエラーが発生しました";
  }
}

/**
 * API呼び出しをエラーハンドリングでラップ
 */
export async function withErrorHandling<T>(
  fn: () => Promise<T>,
  customMessage?: string
): Promise<T> {
  try {
    return await fn();
  } catch (error) {
    // コンソールに技術的詳細を出力
    console.error("API Error:", error);

    // カテゴリとメッセージを判定
    const category = getErrorCategory(error);
    const message = getErrorMessage(error);

    // Toastを表示（サーバーからのメッセージをそのまま使用）
    toastStore.showError(category, message);

    // エラーを再スロー（呼び出し側でハンドリング可能にする）
    throw error;
  }
}
