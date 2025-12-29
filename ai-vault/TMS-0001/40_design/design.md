# Design: Task Management System v2 (TMS-v2)

> Confidentiality: Internal
> Repo: tms-v2
> Ticket: TMS-0001
> Branch: feature/tms-v2-poc
> Owner: Developer
> Created: 2025-12-21
> Last Updated: 2025-12-28

---

## 0. Purpose
- Define component responsibilities, internal interfaces, flows, error handling, and event publication/consumption implications.
- This document MUST remain consistent with:
  - Requirements (REQ)
  - Contracts (operationId/messageId)
  - Architecture constraints (NFRs)

References:
- Requirements: `10_prd/requirements.md`
- Domain: `20_domain/domain.md`
- Glossary: `20_domain/glossary.md`
- OpenAPI: `30_contract/openapi.yaml`
- AsyncAPI: `30_contract/asyncapi.yaml`
- Architecture: `40_design/architecture.md`
- Decisions: `40_design/decisions.md`
- Tasks: `40_design/tasks.md`
- Traceability: `90_review/traceability.md`

---

## 1. Design Constraints (Inherited)
- NFR constraints from architecture.md:
  - Performance: UI操作100ms以内、SQLite ACID保証
  - Consistency: Strong consistency（トランザクション境界）
  - Security: 不要（個人使用）
  - Observability: 基本的なログとメトリクス
- “Do not do” list:
  - クラウドサービスとの連携を実装しない
  - 複数ユーザー対応を実装しない
  - 非同期イベント駆動アーキテクチャを採用しない
  - 外部APIとの連携を実装しない

---

## 2. Component Model (C4 L3)
> ここで定義した“コンポーネント名”が traceability の Components 列の正になる。

### 2.1 Components Overview
| Component | Responsibility | Inputs | Outputs | Notes |
|---|---|---|---|---|
| TaskService | タスクCRUD操作、検索・フィルタリング、階層管理、親子ステータス自動同期 | HTTP operationIds (createTask, updateTask, etc.) | DB writes (tasks table) | Rustバックエンド |
| QueueService | 日次タスクキューの管理、親タスクのキュー登録制限 | HTTP operationIds (addToQueue, removeFromQueue, etc.) | DB writes (task_queue table) | Rustバックエンド |
| TagService | タグCRUD操作、管理 | HTTP operationIds (createTag, updateTag, etc.) | DB writes (tags table) | Rustバックエンド |
| DatabaseManager | SQLiteデータベース操作 | SQL queries | Database results | Rustバックエンド |
| IPCRouter | IPC通信のルーティング | Frontend requests | Backend responses | Rustバックエンド |
| FrontendUI | UIコンポーネントのレンダリング、ユーザー操作の処理、状態管理、検索・フィルター機能、タスク詳細表示 | User interactions | IPC calls | SolidJSフロントエンド |

### 2.2 Component Contracts (per component)
#### Component: TaskService
- **Responsibility**:
  - タスクの作成、読み取り、更新、削除（論理削除・物理削除）
  - タスクの階層構造管理（親子関係）
  - タスクの検索とフィルタリング（statusパラメータ対応）
  - ページネーション付きタスク一覧取得（総件数付き）
  - 親子ステータスの自動同期（子タスク変更時にリアルタイム実行）
  - Draft状態タスクのみ編集・削除可能制限
  - Archivedタスクの物理削除とrestore機能
- **Implements REQs**: REQ-0002, REQ-0003, REQ-0004, REQ-0005, REQ-0008, REQ-0010, REQ-0011, REQ-0012, REQ-0016, REQ-0017, REQ-0018, REQ-0019, REQ-0022, REQ-0024
- **HTTP operations**: createTask, getTask, updateTask, deleteTask, deleteTaskPermanently, restoreTask, getTaskHierarchy, listTasks, listTasksPaginated, searchTasks
- **Events published**: N/A
- **Events consumed**: N/A
- **Dependencies**:
  - Internal: DatabaseManager
  - External: N/A
- **State/Data**:
  - Owned data: N/A（ステートレス）
  - Reads data: tasks, task_tags
- **Observability**:
  - Required logs/metrics: 操作成功/失敗カウンター
- **Failure handling**:
  - DBエラー時は適切なHTTPエラーレスポンスを返す

#### Component: QueueService
- **Responsibility**:
  - 日次タスクキューの管理
  - タスクのキューへの追加・削除（完了時のupdated_at更新含む）
  - キュー全体のクリア
  - キュー順序の更新と一括並び替え（D&D機能のサポート）
  - 親タスクのキュー登録制限（子タスクを持つタスクは登録不可）
- **Implements REQs**: REQ-0006, REQ-0007, REQ-0009, REQ-0023, REQ-0028
- **HTTP operations**: getTaskQueue, addTaskToQueue, removeTaskFromQueue, clearTaskQueue, updateQueuePosition, reorderTaskQueue
- **Events published**: N/A
- **Events consumed**: N/A
- **Dependencies**:
  - Internal: DatabaseManager, TaskService
  - External: N/A
- **State/Data**:
  - Owned data: N/A（ステートレス）
  - Reads data: task_queue, tasks
- **Observability**:
  - Required logs/metrics: キュー操作カウンター
- **Failure handling**:
  - 重複追加時は適切なエラーレスポンス

#### Component: TagService
- **Responsibility**:
  - タグの作成、更新、削除
  - タグ使用状況の管理
- **Implements REQs**: REQ-0005
- **HTTP operations**: listTags, createTag, updateTag, deleteTag
- **Events published**: N/A
- **Events consumed**: N/A
- **Dependencies**:
  - Internal: DatabaseManager
  - External: N/A
- **State/Data**:
  - Owned data: N/A（ステートレス）
  - Reads data: tags, task_tags
- **Observability**:
  - Required logs/metrics: タグ操作カウンター
- **Failure handling**:
  - 使用中のタグ削除時は409エラー

#### Component: DatabaseManager
- **Responsibility**:
  - SQLiteデータベース接続管理
  - SQLクエリの実行
  - トランザクション管理
- **Implements REQs**: N/A（インフラコンポーネント）
- **HTTP operations**: N/A
- **Events published**: N/A
- **Events consumed**: N/A
- **Dependencies**:
  - Internal: N/A
  - External: SQLiteライブラリ
- **State/Data**:
  - Owned data: DB接続プール
  - Reads data: 全テーブル
- **Observability**:
  - Required logs/metrics: DB接続状態、クエリ実行時間
- **Failure handling**:
  - 接続エラー時は自動リトライ

#### Component: IPCRouter
- **Responsibility**:
  - IPCメッセージのルーティング
  - リクエストの適切なサービスへの転送
  - レスポンスの整形
- **Implements REQs**: N/A（インフラコンポーネント）
- **HTTP operations**: 全operationId（IPC経由）
- **Events published**: N/A
- **Events consumed**: N/A
- **Dependencies**:
  - Internal: TaskService, QueueService, TagService
  - External: Tauri IPC
- **State/Data**:
  - Owned data: N/A（ステートレス）
  - Reads data: N/A
- **Observability**:
  - Required logs/metrics: IPCリクエストカウンター
- **Failure handling**:
  - 不正リクエスト時は適切なエラーレスポンス

#### Component: FrontendUI
- **Responsibility**:
  - UIコンポーネントのレンダリング
  - ユーザー操作の処理
  - 状態管理
  - 検索バーとフィルター機能の提供（Draft/Activeのみ）
  - タスク詳細ポップアップ表示
  - Completed/Archivedタスク確認画面の提供（ページネーション付き）
  - ページネーションコントロールUI（前/次ページ、ページ番号入力）
  - Draft状態タスクのみ編集・削除ボタン表示
  - タイトルspanサイズ調整（テキスト幅のみに制限）
  - TaskQueue空時UI改善（領域全体に点線枠表示）
  - Archivedページでのrestore機能と物理削除機能（3点リーダーメニュー）
  - QueuePanelでのドラッグ&ドロップによる順序変更機能
- **Implements REQs**: REQ-0004, REQ-0007, REQ-0013, REQ-0014, REQ-0015, REQ-0020, REQ-0021, REQ-0025, REQ-0026, REQ-0027, REQ-0028
- **HTTP operations**: N/A（IPC呼び出し）
- **Events published**: N/A
- **Events consumed**: N/A
- **Dependencies**:
  - Internal: IPCRouter（間接的に）
  - External: SolidJS, Tauri API, Kobalte, Tailwind CSS, @dnd-kit/core
- **State/Data**:
  - Owned data: UI状態（SolidJS Store）
  - Reads data: IPC経由でバックエンドデータ
- **Observability**:
  - Required logs/metrics: UI操作カウンター
- **Failure handling**:
  - IPCエラー時はユーザー通知

---

## 3. End-to-end Flows (Sequence, text)
> “ユーザー行動/HTTP”と“イベント通知”を分けて書く。

### 3.1 HTTP Flow: createTask
- operationId: createTask
- Maps to REQs: REQ-0002
- Preconditions: 有効なCreateTaskRequestデータ
- Steps:
  1. FrontendUIがユーザー入力を受け取りIPC呼び出し
  2. IPCRouterがリクエストをTaskServiceへ転送
  3. TaskServiceがデータをバリデーション
  4. DatabaseManager経由でtasksテーブルにINSERT
  5. 作成されたTaskエンティティを返却
- Data changes: tasksテーブルに新規レコード追加
- Event publication: N/A
- Responses:
  - Success: 201 + Taskオブジェクト
  - Error: 400（バリデーションエラー）

### 3.2 HTTP Flow: addTaskToQueue
- operationId: addTaskToQueue
- Maps to REQs: REQ-0006, REQ-0009
- Preconditions: 存在するタスクID、タスクがActive状態、子タスクを持たない
- Steps:
  1. FrontendUIからタスク選択とキュー追加操作
  2. IPCRouterがリクエストをQueueServiceへ転送
  3. QueueServiceがタスク存在チェック、親タスクチェック（子タスクを持つ場合は拒否）、重複チェック
  4. DatabaseManager経由でtask_queueテーブルにINSERT
  5. 成功レスポンスを返却
- Data changes: task_queueテーブルに新規レコード追加
- Event publication: N/A
- Responses:
  - Success: 200
  - Error: 400（重複、親タスク）、404（タスク不存在）

### 3.3 HTTP Flow: listTasks
- operationId: listTasks
- Maps to REQs: REQ-0004, REQ-0005, REQ-0010
- Preconditions: なし
- Steps:
  1. FrontendUIがタスクプール画面表示時にIPC呼び出し
  2. IPCRouterがリクエストをTaskServiceへ転送
  3. TaskServiceがクエリパラメータ（status, tags）でフィルタ（デフォルトでDraft + Activeを表示）
  4. DatabaseManager経由でtasksテーブルをSELECT
  5. 結果をTask配列として返却
- Data changes: N/A（読み取り専用）
- Event publication: N/A
- Responses:
  - Success: 200 + Task配列
  - Error: 500（DBエラー）

### 3.4 HTTP Flow: searchTasks (拡張版)
- operationId: searchTasks
- Maps to REQs: REQ-0005, REQ-0011, REQ-0012
- Preconditions: 少なくとも1つのパラメータ（q, status, tags）が提供される
- Steps:
  1. FrontendUIが検索バー/フィルターからパラメータを収集
  2. IPCRouterがリクエストをTaskServiceへ転送
  3. TaskServiceがキーワード検索（q）とフィルタリング（status, tags）を実行
  4. DatabaseManager経由でtasksテーブルをSELECT（複合条件）
  5. 結果をTask配列として返却
- Data changes: N/A（読み取り専用）
- Event publication: N/A
- Responses:
  - Success: 200 + Task配列
  - Error: 400（パラメータ不足）、500（DBエラー）

### 3.5 Internal Flow: 親子ステータス自動同期
- Trigger: 子タスクのステータス変更時（update_task, delete_task, queue操作）
- Maps to REQs: REQ-0008
- Preconditions: 親タスクが存在する子タスクのステータスが変更される
- Constraint: BR-016により階層は最大2レベル（親-子）のため、孫タスクは存在しない
- Steps:
  1. TaskService/QueueServiceが子タスクのステータスを変更
  2. 親タスクIDを取得
  3. 全ての子タスクのステータスを集計
  4. BR-013ルールに基づき親タスクのステータスを計算
     - 全子がDraft → 親もDraft
     - 1つでもActive → 親もActive
     - 全子がCompleted → 親もCompleted
     - 全子がArchived OR Completed → 親はArchived可能
  5. 親タスクのステータスを更新
  6. **注**: BR-016により孫タスクは存在しないため、再帰的更新は1階層のみ
- Data changes: tasksテーブルの親タスクのstatusカラム更新
- Event publication: N/A
- Observability: 親ステータス更新のログ記録

### 3.6 HTTP Flow: deleteTaskPermanently
- operationId: deleteTaskPermanently
- Maps to REQs: REQ-0018
- Preconditions: タスクがArchived状態、子タスクを持たない
- Steps:
  1. FrontendUI（ArchivedPage）が削除ボタンクリックでIPC呼び出し
  2. IPCRouterがリクエストをTaskServiceへ転送
  3. TaskServiceがタスク存在チェック、Archived状態チェック、子タスクチェック
  4. DatabaseManager経由でtasksテーブルからDELETE（物理削除）
  5. 関連レコード（task_tags, task_queue）もCASCADE削除
  6. 成功レスポンスを返却
- Data changes: tasksテーブルからレコード完全削除
- Event publication: N/A
- Responses:
  - Success: 204
  - Error: 400（Archived以外、子タスク存在）、404（タスク不存在）

### 3.7 HTTP Flow: restoreTask
- operationId: restoreTask
- Maps to REQs: REQ-0022
- Preconditions: タスクがArchived状態
- Steps:
  1. FrontendUI（ArchivedPage）がRestoreボタンクリックでIPC呼び出し
  2. IPCRouterがリクエストをTaskServiceへ転送
  3. TaskServiceがタスク存在チェック、Archived状態チェック
  4. DatabaseManager経由でtasksテーブルのstatusをDraftに変更
  5. 親ステータス同期ロジックを呼び出し（親が存在する場合）
  6. 更新されたTaskエンティティを返却
- Data changes: tasksテーブルのstatusカラムを"archived"から"draft"に更新
- Event publication: N/A
- Responses:
  - Success: 200 + Taskオブジェクト
  - Error: 400（Archived以外）、404（タスク不存在）

### 3.8 HTTP Flow: listTasksPaginated
- operationId: listTasksPaginated
- Maps to REQs: REQ-0024, REQ-0025
- Preconditions: なし
- Steps:
  1. FrontendUI（CompletedPage/ArchivedPage）がページ番号変更時にIPC呼び出し
  2. IPCRouterがリクエストをTaskServiceへ転送
  3. TaskServiceがクエリパラメータ（status, limit, offset）でフィルタ
  4. DatabaseManager経由でtasksテーブルをSELECT（LIMIT/OFFSET付き）
  5. 同時に総件数をCOUNT取得
  6. PaginatedTaskResponse { tasks, total } として返却
- Data changes: N/A（読み取り専用）
- Event publication: N/A
- Responses:
  - Success: 200 + PaginatedTaskResponse（tasks配列 + total件数）
  - Error: 500（DBエラー）
- Design Notes:
  - デフォルトlimit=20、offset=0
  - status未指定時はDraft + Active（既存list_tasksと同じ）
  - 総件数はページネーションUI（"Page 1 of 5 (100 items)"）に使用
  - 日付グループ化はクライアント側で実施（ページ単位）

### 3.9 Internal Flow: QueuePanelドラッグ&ドロップ順序変更
- Trigger: ユーザーがQueuePanelでタスクカードをドラッグ&ドロップ
- Maps to REQs: REQ-0028
- Preconditions: キューに2つ以上のタスクが存在
- Steps:
  1. FrontendUI（QueuePanel）がドラッグ開始を検知（@dnd-kit/core DndContext）
  2. ドラッグ中はプレビュー表示（視覚フィードバック）
  3. ドロップ完了時、新しいタスクID配列を生成
  4. reorderTaskQueue API（POST /queue/reorder）をIPC経由で呼び出し
  5. QueueServiceが新しい順序でpositionカラムを更新
  6. 成功時はUI状態を楽観的更新、失敗時はリロードで元に戻す
- Data changes: task_queueテーブルの全レコードのpositionカラム更新
- Event publication: N/A
- Observability: ドラッグ操作のログ記録
- Design Notes:
  - @dnd-kit/coreライブラリ使用（DndContext, SortableContext, useSortable）
  - 既存のreorderQueue APIを再利用
  - 楽観的UI更新でUX向上

### 3.10 UI Design: ページネーションコンポーネント
- Maps to REQs: REQ-0025
- Component: Pagination.tsx
- Design:
  - UI形式: `< [number input box] >`
    - 前ページボタン（`<`）: currentPage > 1で有効化
    - ページ番号入力フィールド: 直接ジャンプ可能（Enterキー対応）
    - 次ページボタン（`>`）: currentPage < totalPagesで有効化
    - 総ページ数/総件数表示: "Page 1 of 5 (100 items)"形式
  - Props: currentPage, totalPages, totalItems, onPageChange
  - 表示条件: totalPages <= 1で非表示
  - 使用場所: CompletedPage, ArchivedPage
- Behavior:
  - ページ番号入力は1〜totalPagesの範囲でバリデーション
  - 範囲外入力時は自動補正（1 or totalPages）
  - ボタンクリック/Enter押下でonPageChangeコールバック実行

### 3.11 UI Design: DropdownMenuコンポーネント
- Maps to REQs: REQ-0026
- Component: DropdownMenu.tsx
- Design:
  - ライブラリ: Kobalte Dropdown Menu（@kobalte/core 0.13.11）
  - トリガー: 3点リーダー（`⋯`）ボタン
  - メニュー項目:
    1. 復元（Restore）: アイコン + "Restore" テキスト、通常スタイル
    2. 完全削除（Delete Permanently）: アイコン + "Delete Permanently" テキスト、destructive variant（赤色）
  - 既存実装パターン参考: Dialog.tsx（Portal, Content使用）
- Behavior:
  - メニュー外クリックで自動クローズ
  - メニュー項目クリックでonSelect実行後クローズ
  - Archivedページのタスクカード右側に配置（既存2ボタンを置き換え）

### 3.12 UI Design: タイトルspanサイズ調整
- Maps to REQs: REQ-0027
- 対象コンポーネント: TaskPool, QueuePanel, CompletedPage, ArchivedPage
- Design Change:
  - 現状: タイトルspanに`flex-1`クラス → 全幅を占有
  - 変更後: `flex-1`削除 → テキスト内容の幅のみに制限
  - 目的: 無駄な選択領域を防ぐ、UX向上
- 対象箇所:
  - TaskPool親タスク: Line 289-296（spanタグ）
  - TaskPool子タスク: Line 354-361（spanタグ）
  - QueuePanel: Line 105-107（pタグ + spanタグ）
  - CompletedPage: h3タグ
  - ArchivedPage: h3タグ
- 注意点:
  - QueuePanelは`truncate`クラスとの併用あり → `min-w-0`調整が必要
  - 親タスクのonClickハンドラは親div要素で維持（影響なし）

### 3.13 Bug Fix: completed時のupdated_at更新
- Maps to REQs: REQ-0023
- 箇所: `src-tauri/src/service/queue.rs` Line 178-180（remove_from_queue関数）
- 現状:
  ```rust
  diesel::update(tasks::table.find(&task_id))
      .set(tasks::status.eq(&target_status))
      .execute(conn)?;
  ```
- 修正後:
  ```rust
  let now = Utc::now().to_rfc3339();
  diesel::update(tasks::table.find(&task_id))
      .set((
          tasks::status.eq(&target_status),
          tasks::updated_at.eq(&now),
      ))
      .execute(conn)?;
  ```
- 影響: QueuePanelのCompleteボタン押下時、タスクのupdated_atフィールドが正しく更新される
- 参考: service/task.rsのupdate_task関数では正しく実装済み

### 3.14 Event Flow: N/A
- Event-driven architecture is not adopted for TMS-v2

---

## 4. Data Model (Conceptual)
> TMS-v2の主要エンティティと関係。SQLiteベースの関係データモデル。

- Entity/VO mapping notes:
  - Taskエンティティ: id, title, description, status, tags[], parentId, timestamps
  - Tagエンティティ: id, name, color, usageCount, timestamps
  - TaskQueue: taskId, position（順序付きコレクション）
- Key constraints:
  - Task.id: ユニークプライマリキー
  - Tag.id: ユニークプライマリキー
  - Task.parentId: Task.idへのオプショナル外部キー（NULL可）
  - TaskQueue.taskId: Task.idへのユニーク外部キー（一意制約）
- Indexing considerations:
  - Task.status: ステータスフィルタ用
  - Task.parentId: 階層クエリ用
  - TaskQueue.taskId: キュー操作用
  - Tag.name: 検索用

---

## 5. Error Handling & Idempotency
### 5.1 HTTP
- Idempotency policy:
  - Which operations must be idempotent: 全操作（HTTP APIとして）
  - Key: リソースIDベース（PUT/DELETEはパスマラメータ、POSTはサーバー生成ID）
- Retry policy:
  - Which errors are retryable: 500系エラー（DB接続エラー等）
  - Timeouts: IPC通信5秒、DBクエリ3秒

### 5.2 Events (Producer)
- N/A (イベント駆動アーキテクチャ未採用)

---

## 6. Security Design (Feature-specific)
- Authorization points: <where checks occur>
- Data exposure: <PII fields>
- Audit events/logs: <what must be recorded>

---

## 7. Observability (Feature-specific)
- CorrelationId propagation:
  - HTTP: <header name>
  - Events: <header field>
- Logs:
  - Must log: <…>
  - Must not log: <PII>
- Metrics:
  - Success/failure counters: <…>
  - Latency: <…>
- Alerts (hooks): <what matters>

---

## 8. Testing Strategy (Mapped to strict REQs)
> 1REQ=1AC なので、テストも “REQ単位” のカバレッジが基本。

- Unit tests (Rust):
  - REQ-0002: TaskServiceのCRUD操作が正しく動作
  - REQ-0003: 親子関係の作成・削除が循環を防ぐ
  - REQ-0006: キュー操作の重複チェックが機能
  - REQ-0008: 親子ステータス自動同期が正しく動作
  - REQ-0009: 親タスクのキュー登録制限が機能
  - REQ-0011: キーワード検索が正しく動作
  - REQ-0012: フィルタリング機能が正しく動作
  - REQ-0016: Draft以外のタスク編集が拒否される
  - REQ-0017: Draft以外のタスク論理削除が拒否される
  - REQ-0018: Archivedタスクの物理削除が正しく動作
  - REQ-0022: Archivedタスクのrestore機能が正しく動作
  - REQ-0023: completed時のupdated_atが正しく更新される
  - REQ-0024: ページネーションAPIがlimit/offsetで正しく動作し、総件数を返す
- Integration tests (Rust + SQLite):
  - REQ-0004: タスク一覧取得とフィルタリング
  - REQ-0005: 検索機能とタグ管理
  - REQ-0007: キュー画面表示
  - REQ-0010: Draft + Active タスクのデフォルト表示
  - REQ-0019: list_tasks APIのstatusパラメータ対応
  - REQ-0028: reorderTaskQueue APIがキュー順序を正しく更新する
- E2E tests (Tauri):
  - REQ-0001: 開発環境構築後のビルド・実行確認
  - REQ-0013: 完了タスク確認画面の表示
  - REQ-0014: アーカイブタスク確認画面の表示
  - REQ-0015: タスクリスト表示形式とUI改善
  - REQ-0020: フィルターチップがDraft/Activeのみ表示される
  - REQ-0021: TaskQueue空時のUI改善が機能する
  - REQ-0025: ページネーションUIが正しく動作し、ページ遷移が機能する
  - REQ-0026: 3点リーダーメニューが表示され、復元・削除操作が可能
  - REQ-0027: タイトルspanがテキスト幅のみに制限される
  - REQ-0028: QueuePanelでドラッグ&ドロップによる順序変更が機能する
- Contract tests:
  - OpenAPI: 全operationIdのスキーマバリデーション

---

## 9. Open Questions / Decisions Pending
- Q1: <…> / Owner: <…> / Due: <…>

---

## 10. Change Log
- 2025-12-21 Initial component design for TMS-v2 desktop application
- 2025-12-27 Updated ReactUI component to FrontendUI, changed frontend framework to SolidJS with SolidJS Store
- 2025-12-27 親子ステータス自動同期機能の追加、検索・フィルター機能の拡張、UI改善要件の反映 (REQ-0008〜REQ-0015)
- 2025-12-28 Draft状態制限機能、物理削除・restore機能、list_tasks API改良、フィルターUI改善の追加 (REQ-0016〜REQ-0022)
- 2025-12-28 バグ修正（updated_at更新）、ページネーション機能、3点リーダーメニュー、タイトルspan調整、D&D機能の追加 (REQ-0023〜REQ-0028)
