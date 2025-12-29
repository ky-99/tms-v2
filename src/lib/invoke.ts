import { invoke as tauriInvoke } from '@tauri-apps/api/core';

/**
 * デフォルトタイムアウト（ミリ秒）
 * 5秒でタイムアウトし、エラーを返す
 */
const DEFAULT_TIMEOUT = 5000;

/**
 * タイムアウト機能付きのTauri invokeラッパー
 *
 * @param cmd - 実行するコマンド名
 * @param args - コマンドに渡す引数（オプション）
 * @param timeout - タイムアウト時間（ミリ秒、デフォルト: 5000ms）
 * @returns Promise<T> - コマンドの実行結果
 * @throws Error - タイムアウトまたはコマンド実行エラー
 *
 * @example
 * ```typescript
 * // 基本的な使用方法
 * const task = await invokeWithTimeout<Task>('create_task', { req: createTaskRequest });
 *
 * // カスタムタイムアウト（10秒）
 * const result = await invokeWithTimeout<Task>('long_operation', { id: '123' }, 10000);
 * ```
 */
export async function invokeWithTimeout<T>(
  cmd: string,
  args?: Record<string, unknown>,
  timeout = DEFAULT_TIMEOUT
): Promise<T> {
  return Promise.race([
    tauriInvoke<T>(cmd, args),
    new Promise<never>((_, reject) =>
      setTimeout(
        () => reject(new Error(`操作がタイムアウトしました: ${cmd} (${timeout}ms)`)),
        timeout
      )
    ),
  ]);
}
