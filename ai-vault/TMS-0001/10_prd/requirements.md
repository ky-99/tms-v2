# Requirements: Task Management System v2 (TMS-v2)

> Confidentiality: Internal
> Repo: tms-v2
> Ticket: TMS-0001
> Branch: feature/tms-v2-poc
> Owner: Developer
> Created: 2024-12-21
> Last Updated: 2024-12-21

---

## 0. Rules (Do not violate)
1. **1 REQ = 1 Acceptance Criterion**（1つのREQは、1つの検証可能な受入条件だけを持つ）
2. REQ本文は **実装手段を書かない**（Redis, DBテーブル名, ライブラリ名などは design.md 側）
3. 受入条件は **テスト可能な形式**（Given/When/Then または 入出力/状態/制約）
4. 例外やバリエーションは **別REQとして分割**する（成功系REQと失敗系REQを混ぜない）
5. 依存がある場合は `Depends on:` で明示する（これが後工程の整合性の要）
6. 変更で不要になったREQは削除せず `Status: Deprecated` にする

---

## 1. Glossary Link
- Terms: `20_domain/glossary.md`
- Domain: `20_domain/domain.md`

---

## 2. Requirement Index
> ここは一覧。詳細は各REQセクションに書く。

| REQ-ID | Title | Priority | Status | Area | Depends on |
|---|---|---|---|---|---|
| REQ-0001 | 開発環境構築手順 | MUST | Draft | Development | - |
| REQ-0002 | タスクCRUD機能 | MUST | Draft | Task Management | REQ-0001 |
| REQ-0003 | タスク親子関係管理 | MUST | Draft | Task Management | REQ-0002 |
| REQ-0004 | タスクプール画面 | MUST | Draft | UI | REQ-0003 |
| REQ-0005 | タスク検索・フィルタ機能 | SHOULD | Draft | UI | REQ-0004 |
| REQ-0006 | タスクキュー管理 | MUST | Draft | Task Management | REQ-0002 |
| REQ-0007 | タスクキュー画面 | MUST | Draft | UI | REQ-0006 |
| REQ-0008 | 親子タスクステータス自動同期 | MUST | Draft | Task Management | REQ-0003 |
| REQ-0009 | 親タスクのキュー登録制限 | MUST | Draft | Task Management | REQ-0008 |
| REQ-0010 | タスクプールの表示範囲拡張 | MUST | Draft | UI | REQ-0004 |
| REQ-0011 | タスク検索バー | SHOULD | Draft | UI | REQ-0010 |
| REQ-0012 | タスクフィルター機能 | SHOULD | Draft | UI | REQ-0011 |
| REQ-0013 | 完了タスク確認画面 | SHOULD | Draft | UI | REQ-0002 |
| REQ-0014 | アーカイブタスク確認画面 | SHOULD | Draft | UI | REQ-0002 |
| REQ-0015 | タスクリスト表示形式変更 | SHOULD | Done | UI | REQ-0004 |
| REQ-0016 | Draft状態タスクのみ編集可能 | MUST | Done | Task Management | REQ-0002 |
| REQ-0017 | Draft状態タスクのみ論理削除可能 | MUST | Done | Task Management | REQ-0002 |
| REQ-0018 | Archivedタスクの物理削除 | MUST | Done | Task Management | REQ-0017 |
| REQ-0019 | list_tasks APIのstatusパラメータ対応 | MUST | Done | Task Management | REQ-0002 |
| REQ-0020 | フィルターUIからCompletedチップ削除 | SHOULD | Done | UI | REQ-0012 |
| REQ-0021 | TaskQueue空時UI改善 | SHOULD | Done | UI | REQ-0007 |
| REQ-0022 | Archiveページrestore機能 | MUST | Done | Task Management | REQ-0014 |
| REQ-0023 | バグ修正: completed時のupdated_at更新 | MUST | Done | Task Management | REQ-0006 |
| REQ-0024 | list_tasks APIページネーション | MUST | Done | Task Management | REQ-0002 |
| REQ-0025 | Completed/Archivedページページネーション UI | SHOULD | Done | UI | REQ-0024 |
| REQ-0026 | Archivedページ3点リーダーメニュー | SHOULD | Done | UI | REQ-0014 |
| REQ-0027 | タイトルspanサイズ調整 | SHOULD | Done | UI | REQ-0004 |
| REQ-0028 | キュー順番変更D&D | SHOULD | Done | UI | REQ-0006 |
| REQ-0029 | タグシステムUI統合 | SHOULD | Done | UI | REQ-0005 |
| REQ-0030 | タグフィルター展開式UI | SHOULD | Done | UI | REQ-0029 |
| REQ-0031 | タグカラー管理 | COULD | Hold | UI | REQ-0029 |

**Priority**: MUST / SHOULD / COULD
**Status**: Draft / Approved / Implementing / Done / Hold / Deprecated

---

## 3. Requirements (Strict)

### REQ-0001: 開発環境構築手順
- **Priority**: MUST
- **Status**: Draft
- **Area**: Development
- **Actor**: Developer
- **Preconditions**: None
- **Trigger**: 開発開始時
- **Acceptance (the only one)**:
  - **Given**: クリーンな開発環境
  - **When**: 構築手順ドキュメントに従って環境構築を実行
  - **Then**: Tauri + Rust + SQLite + Reactの開発環境が利用可能になり、アプリケーションのビルド・実行が可能
- **Negative/Boundary**: 依存関係のインストール失敗時は個別対応
- **Depends on**: None
- **Notes**: Node.js 18+, Rust 1.70+ を前提とする
- **Trace Hooks (optional)**:
  - API: TBD
  - Component: TBD
  - Task: ENV-001

---

### REQ-0002: タスクCRUD機能
- **Priority**: MUST
- **Status**: Draft
- **Area**: Task Management
- **Actor**: User
- **Preconditions**: アプリケーションが起動している
- **Trigger**: ユーザーの操作
- **Acceptance (the only one)**:
  - **Given**: タスク管理画面が表示されている
  - **When**: ユーザーがタスクの作成・更新・削除を操作
  - **Then**: SQLiteデータベースに変更が反映され、UIに即時反映される
- **Negative/Boundary**: バリデーションエラー時は適切なエラーメッセージを表示
- **Depends on**: REQ-0001
- **Notes**: タスクにはタイトル、説明、ステータスを含む
- **Trace Hooks (optional)**:
  - API: TBD
  - Component: TBD
  - Task: CRUD-001

---

### REQ-0003: タスク親子関係管理
- **Priority**: MUST
- **Status**: Draft
- **Area**: Task Management
- **Actor**: User
- **Preconditions**: タスクが存在する
- **Trigger**: タスク作成・編集時
- **Acceptance (the only one)**:
  - **Given**: タスク作成・編集画面
  - **When**: ユーザーが親タスクを指定して子タスクを作成
  - **Then**: データベースに親子関係が保存され、UIで関係が視覚的に表示される
- **Negative/Boundary**:
  - 循環参照は許可しない
  - 階層は最大2レベル（親-子）まで。孫タスクの作成は禁止（パフォーマンス最適化のため）
- **Depends on**: REQ-0002
- **Notes**: タスクは親を持つか、親であるかのいずれか
- **Trace Hooks (optional)**:
  - API: TBD
  - Component: TBD
  - Task: REL-001

---

### REQ-0004: タスクプール画面
- **Priority**: MUST
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: アプリケーション起動
- **Trigger**: プール画面への遷移
- **Acceptance (the only one)**:
  - **Given**: アプリケーションが起動している
  - **When**: ユーザーがタスクプール画面を開く
  - **Then**: 全タスクとその親子関係がツリー構造で表示され、CRUD操作が可能
- **Negative/Boundary**: 大量のタスク時はパフォーマンスを考慮
- **Depends on**: REQ-0003
- **Notes**: Reactコンポーネントで実装
- **Trace Hooks (optional)**:
  - API: TBD
  - Component: TaskPool
  - Task: UI-001

---

### REQ-0005: タスク検索・フィルタ機能
- **Priority**: SHOULD
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクプール画面表示中
- **Trigger**: 検索入力またはフィルタ選択
- **Acceptance (the only one)**:
  - **Given**: タスクプール画面にタスクが表示されている
  - **When**: ユーザーが検索キーワードを入力またはフィルタを選択
  - **Then**: 条件に一致するタスクのみが表示される
- **Negative/Boundary**: 検索結果が0件の場合は適切なメッセージ表示
- **Depends on**: REQ-0004
- **Notes**: タイトル、説明、ステータスでの検索・フィルタ
- **Trace Hooks (optional)**:
  - API: TBD
  - Component: TaskSearch
  - Task: UI-002

---

### REQ-0006: タスクキュー管理
- **Priority**: MUST
- **Status**: Draft
- **Area**: Task Management
- **Actor**: User
- **Preconditions**: タスクプールにタスクが存在する
- **Trigger**: キュー追加操作
- **Acceptance (the only one)**:
  - **Given**: タスクプール画面でタスクが選択されている
  - **When**: ユーザーが「キューに追加」操作を行う
  - **Then**: 選択されたタスクが日次キューに追加され、重複は許可されない
- **Negative/Boundary**: キューからの削除も可能
- **Depends on**: REQ-0002
- **Notes**: 日次タスクの管理機能
- **Trace Hooks (optional)**:
  - API: TBD
  - Component: TBD
  - Task: QUEUE-001

---

### REQ-0007: タスクキュー画面
- **Priority**: MUST
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: キューにタスクが追加されている
- **Trigger**: キュー画面への遷移
- **Acceptance (the only one)**:
  - **Given**: アプリケーションが起動している
  - **When**: ユーザーがタスクキュー画面を開く
  - **Then**: 日次キューのタスクがリスト表示され、完了状態の更新が可能
- **Negative/Boundary**: キューが空の場合は適切なメッセージ表示
- **Depends on**: REQ-0006
- **Notes**: 今日の作業予定を表示
- **Trace Hooks (optional)**:
  - API: TBD
  - Component: TaskQueue
  - Task: UI-003

---

### REQ-0008: 親子タスクステータス自動同期
- **Priority**: MUST
- **Status**: Draft
- **Area**: Task Management
- **Actor**: System (自動処理)
- **Preconditions**: 親子関係を持つタスクが存在する
- **Trigger**: 子タスクのステータス変更時
- **Acceptance (the only one)**:
  - **Given**: 親タスクと子タスクが存在する
  - **When**: 子タスクのステータスが変更される（Draft/Active/Completed/Archivedのいずれか）
  - **Then**: 親タスクのステータスが以下のルールに従って自動更新される
    - 全子タスクがDraft → 親もDraft
    - 1つでも子タスクがActive → 親もActive
    - 全子タスクがCompleted → 親もCompleted
    - 全子タスクがArchived OR Completed → 親のアーカイブが可能
- **Negative/Boundary**: 循環参照が検出された場合はエラー
- **Depends on**: REQ-0003
- **Notes**: リアルタイム同期、再帰的に親→祖父...と更新
- **Trace Hooks (optional)**:
  - API: update_task, delete_task, add_to_queue, remove_from_queue
  - Component: TaskService
  - Task: TASK-NEW-001

---

### REQ-0009: 親タスクのキュー登録制限
- **Priority**: MUST
- **Status**: Draft
- **Area**: Task Management
- **Actor**: User
- **Preconditions**: タスクプール画面が表示されている
- **Trigger**: キューへの追加操作
- **Acceptance (the only one)**:
  - **Given**: 親タスク（子タスクを1つ以上持つタスク）が存在する
  - **When**: ユーザーが親タスクをキューに追加しようとする
  - **Then**: エラーメッセージが表示され、キューへの追加が拒否される
- **Negative/Boundary**: 子タスクを持たないタスクは通常通りキューに追加可能
- **Depends on**: REQ-0008
- **Notes**: 直接の子タスクの存在のみチェック（孫タスクは考慮不要）
- **Trace Hooks (optional)**:
  - API: add_to_queue
  - Component: QueueService
  - Task: TASK-NEW-002

---

### REQ-0010: タスクプールの表示範囲拡張
- **Priority**: MUST
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: アプリケーションが起動している
- **Trigger**: タスクプール画面表示時
- **Acceptance (the only one)**:
  - **Given**: タスクプール画面
  - **When**: 画面が表示される
  - **Then**: Draft および Active ステータスのタスクが両方表示される
- **Negative/Boundary**: Completed/Archivedタスクは表示されない
- **Depends on**: REQ-0004
- **Notes**: Activeタスクはキューにも表示される（重複表示OK）
- **Trace Hooks (optional)**:
  - API: list_tasks
  - Component: TaskPage
  - Task: TASK-NEW-003

---

### REQ-0011: タスク検索バー
- **Priority**: SHOULD
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクプール画面が表示されている
- **Trigger**: 検索バーへの入力
- **Acceptance (the only one)**:
  - **Given**: タスクプール画面の上部に検索バーが配置されている
  - **When**: ユーザーがキーワードを入力する
  - **Then**: タイトルまたは説明文に入力キーワードを含むタスクのみが表示される
- **Negative/Boundary**: キーワード未入力時は全タスク表示
- **Depends on**: REQ-0010
- **Notes**: バックエンドのsearch_tasks APIを使用
- **Trace Hooks (optional)**:
  - API: search_tasks
  - Component: SearchBar
  - Task: TASK-NEW-005

---

### REQ-0012: タスクフィルター機能
- **Priority**: SHOULD
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクプール画面が表示されている
- **Trigger**: フィルターチップのクリック
- **Acceptance (the only one)**:
  - **Given**: 検索バーの横にステータスフィルターとタグフィルターのチップが表示されている
  - **When**: ユーザーがフィルターチップをクリックしてトグルする
  - **Then**: 選択されたステータスおよびタグに一致するタスクのみが表示される
- **Negative/Boundary**: フィルター未選択時は全タスク表示
- **Depends on**: REQ-0011
- **Notes**: チップ形式UI、複数選択可能
- **Trace Hooks (optional)**:
  - API: search_tasks
  - Component: FilterChip
  - Task: TASK-NEW-005

---

### REQ-0013: 完了タスク確認画面
- **Priority**: SHOULD
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: アプリケーションが起動している
- **Trigger**: 完了タスク画面への遷移
- **Acceptance (the only one)**:
  - **Given**: ナビゲーションから「完了済み」を選択
  - **When**: 完了タスク画面が表示される
  - **Then**: Completedステータスのタスクのみがリスト形式で表示される
- **Negative/Boundary**: 完了タスクが0件の場合は適切なメッセージ表示
- **Depends on**: REQ-0002
- **Notes**: 専用ページとして実装、ルーティング `/completed`
- **Trace Hooks (optional)**:
  - API: search_tasks (status=completed)
  - Component: CompletedPage
  - Task: TASK-NEW-008

---

### REQ-0014: アーカイブタスク確認画面
- **Priority**: SHOULD
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: アプリケーションが起動している
- **Trigger**: アーカイブタスク画面への遷移
- **Acceptance (the only one)**:
  - **Given**: ナビゲーションから「アーカイブ」を選択
  - **When**: アーカイブタスク画面が表示される
  - **Then**: Archivedステータスのタスクのみがリスト形式で表示される
- **Negative/Boundary**: アーカイブタスクが0件の場合は適切なメッセージ表示
- **Depends on**: REQ-0002
- **Notes**: 専用ページとして実装、ルーティング `/archived`
- **Trace Hooks (optional)**:
  - API: search_tasks (status=archived)
  - Component: ArchivedPage
  - Task: TASK-NEW-008

---

### REQ-0015: タスク詳細ホバーポップアップ
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクプール画面が表示されている
- **Trigger**: タスクタイトルクリック
- **Acceptance (the only one)**:
  - **Given**: タスクプール画面にタスクカードが表示されている
  - **When**: ユーザーがタスクタイトルをクリック
  - **Then**: 詳細ポップアップがタイトル上部または下部に表示され、description、tags（色付き）が確認できる
- **Negative/Boundary**: 外部クリックまたは再クリックでポップアップは非表示。ポップアップは読み取り専用（編集不可）。タイトルクリック時は親タスクの展開/折りたたみトグルは発生しない
- **Depends on**: REQ-0004
- **Notes**: Kobalte Popoverを使用（placement="top"）。タグ表示はREQ-0029実装済み。クリック操作に変更（ホバー遅延削除）、stopPropagation()で親タスクトグルと分離
- **Trace Hooks (optional)**:
  - API: N/A（既存データ使用）
  - Component: TaskHoverPopup
  - Task: TASK-NEW-007

---

### REQ-0016: Draft状態タスクのみ編集可能
- **Priority**: MUST
- **Status**: Done
- **Area**: Task Management
- **Actor**: User, System
- **Preconditions**: タスクが存在する
- **Trigger**: タスク編集操作
- **Acceptance (the only one)**:
  - **Given**: タスクがActive、Completed、またはArchived状態
  - **When**: ユーザーまたはシステムが編集を試みる
  - **Then**: バックエンドがTaskNotDraftエラーを返し、フロントエンドでは編集ボタンが非表示になる
- **Negative/Boundary**: Draft状態のタスクは編集可能。親のステータスに関わらず、子タスク自身のステータスで判定
- **Depends on**: REQ-0002
- **Notes**: バックエンド・フロントエンド両方で制御を実施
- **Trace Hooks (optional)**:
  - API: update_task
  - Component: TaskService, TaskPool
  - Task: TASK-NEW-013, TASK-NEW-018

---

### REQ-0017: Draft状態タスクのみ論理削除可能
- **Priority**: MUST
- **Status**: Done
- **Area**: Task Management
- **Actor**: User, System
- **Preconditions**: タスクが存在する
- **Trigger**: タスク削除操作
- **Acceptance (the only one)**:
  - **Given**: タスクがActive、Completed状態
  - **When**: ユーザーまたはシステムが削除（Archived化）を試みる
  - **Then**: バックエンドがTaskNotDraftエラーを返し、フロントエンドでは削除ボタンが非表示になる
- **Negative/Boundary**: Draft状態のタスクは論理削除（Archived化）可能。Archivedタスクは物理削除のみ可能（REQ-0018参照）
- **Depends on**: REQ-0002
- **Notes**: 論理削除はステータスをArchivedに変更する操作
- **Trace Hooks (optional)**:
  - API: delete_task
  - Component: TaskService, TaskPool
  - Task: TASK-NEW-013, TASK-NEW-018

---

### REQ-0018: Archivedタスクの物理削除
- **Priority**: MUST
- **Status**: Done
- **Area**: Task Management
- **Actor**: User
- **Preconditions**: タスクがArchived状態
- **Trigger**: アーカイブページでの削除操作
- **Acceptance (the only one)**:
  - **Given**: Archivedページでタスクが表示されている
  - **When**: ユーザーが削除ボタンをクリック
  - **Then**: タスクがデータベースから完全に削除され、復元不可能になる
- **Negative/Boundary**: Archived以外のステータスのタスクは物理削除不可。子タスクを持つタスクは削除不可
- **Depends on**: REQ-0017
- **Notes**: 新規API delete_task_permanently を実装。関連レコード（task_tags, task_queue）もCASCADE削除
- **Trace Hooks (optional)**:
  - API: delete_task_permanently
  - Component: TaskService, ArchivedPage
  - Task: TASK-NEW-014, TASK-NEW-021

---

### REQ-0019: list_tasks APIのstatusパラメータ対応
- **Priority**: MUST
- **Status**: Done
- **Area**: Task Management
- **Actor**: System
- **Preconditions**: なし
- **Trigger**: list_tasks API呼び出し
- **Acceptance (the only one)**:
  - **Given**: list_tasks APIにstatusパラメータが追加されている
  - **When**: status = Some(vec![TaskStatus::Completed])で呼び出す
  - **Then**: Completedタスクのみが返却される
- **Negative/Boundary**: status = Noneの場合はDraft + Activeを返す（後方互換性維持）
- **Depends on**: REQ-0002
- **Notes**: Optional配列パラメータ。CompletedPage、ArchivedPageで使用
- **Trace Hooks (optional)**:
  - API: list_tasks
  - Component: TaskService, CompletedPage, ArchivedPage
  - Task: TASK-NEW-016, TASK-NEW-020

---

### REQ-0020: フィルターUIからCompletedチップ削除
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクプール画面が表示されている
- **Trigger**: フィルターチップ表示時
- **Acceptance (the only one)**:
  - **Given**: TaskPool画面
  - **When**: フィルターチップを表示
  - **Then**: Draft、Activeの2つのみ表示される（Completedチップは非表示）
- **Negative/Boundary**: フィルター判定は親タスクのステータスで行う。親がフィルタ条件に合致すれば子タスク（全ステータス）も表示
- **Depends on**: REQ-0012
- **Notes**: CompletedタスクはCompletedPageで確認可能なため、フィルターから削除
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: TaskPool
  - Task: TASK-NEW-019

---

### REQ-0021: TaskQueue空時UI改善
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: TaskQueueが空
- **Trigger**: QueuePanel表示時
- **Acceptance (the only one)**:
  - **Given**: TaskQueueが空
  - **When**: QueuePanelを表示
  - **Then**: キュー領域全体を占める点線枠と「Queue is empty」メッセージが表示される
- **Negative/Boundary**: キューにタスクがある場合は通常表示
- **Depends on**: REQ-0007
- **Notes**: 固定高さh-64からflex-1に変更。メッセージも「Queue is empty」に変更
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: QueuePanel
  - Task: TASK-NEW-022

---

### REQ-0022: Archiveページrestore機能
- **Priority**: MUST
- **Status**: Done
- **Area**: Task Management
- **Actor**: User
- **Preconditions**: タスクがArchived状態
- **Trigger**: Archivedページでのrestore操作
- **Acceptance (the only one)**:
  - **Given**: Archivedページでタスクが表示されている
  - **When**: Restoreボタンをクリック
  - **Then**: タスクがDraft状態に戻り、TaskPoolで再表示される
- **Negative/Boundary**: Archived以外のステータスのタスクはrestore不可
- **Depends on**: REQ-0014
- **Notes**: 新規API restore_task を実装。親ステータス同期ロジックも呼び出す
- **Trace Hooks (optional)**:
  - API: restore_task
  - Component: TaskService, ArchivedPage
  - Task: TASK-NEW-015, TASK-NEW-021

---

### REQ-0023: バグ修正: completed時のupdated_at更新
- **Priority**: MUST
- **Status**: Done
- **Area**: Task Management
- **Actor**: System
- **Preconditions**: タスクがキューに登録されている
- **Trigger**: QueuePanelでのComplete操作
- **Acceptance (the only one)**:
  - **Given**: キューにタスクが登録されている
  - **When**: ユーザーがCompleteボタンをクリックし、タスクがcompletedステータスに更新される
  - **Then**: データベースのupdated_atフィールドが現在時刻で更新される
- **Negative/Boundary**: statusのみ更新してupdated_atを更新しない動作はバグとして修正対象
- **Depends on**: REQ-0006
- **Notes**: `service/queue.rs`の`remove_from_queue`関数（178-180行）でupdated_atが未更新の問題を修正
- **Trace Hooks (optional)**:
  - API: remove_from_queue (internal service)
  - Component: QueuePanel
  - Task: TASK-NEW-024

---

### REQ-0024: list_tasks APIページネーション
- **Priority**: MUST
- **Status**: Done
- **Area**: Task Management
- **Actor**: System
- **Preconditions**: タスクがデータベースに存在する
- **Trigger**: list_tasks_paginated API呼び出し
- **Acceptance (the only one)**:
  - **Given**: list_tasks_paginatedエンドポイントが実装されている
  - **When**: limit=20, offset=0で呼び出す
  - **Then**: 最大20件のタスクリストと総件数が返却される
- **Negative/Boundary**: limit未指定時はデフォルト20件。offsetが総件数を超える場合は空配列を返す
- **Depends on**: REQ-0002
- **Notes**: 既存list_tasksは後方互換のため維持。新規関数としてlist_tasks_paginatedを追加。レスポンス型はPaginatedTaskResponse { tasks, total }
- **Trace Hooks (optional)**:
  - API: list_tasks_paginated
  - Component: TaskService
  - Task: TASK-NEW-025

---

### REQ-0025: Completed/Archivedページページネーション UI
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: CompletedまたはArchivedページが表示されている
- **Trigger**: ページネーション操作
- **Acceptance (the only one)**:
  - **Given**: 20件を超えるタスクが存在する
  - **When**: ページネーションUIで「次ページ」ボタンをクリックまたはページ番号を入力
  - **Then**: 指定されたページのタスク（最大20件）が表示される
- **Negative/Boundary**: 総ページ数が1以下の場合、ページネーションUIは非表示
- **Depends on**: REQ-0024
- **Notes**: UI形式は `< [number box] >`（前ページボタン、ページ番号入力フィールド、次ページボタン）。総ページ数/総件数も表示（例: "Page 1 of 5 (100 items)"）
- **Trace Hooks (optional)**:
  - API: list_tasks_paginated
  - Component: Pagination, CompletedPage, ArchivedPage
  - Task: TASK-NEW-027, TASK-NEW-028, TASK-NEW-029

---

### REQ-0026: Archivedページ3点リーダーメニュー
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: Archivedページでタスクが表示されている
- **Trigger**: 3点リーダーボタンクリック
- **Acceptance (the only one)**:
  - **Given**: Archivedページでタスクカードが表示されている
  - **When**: 3点リーダー（`⋯`）ボタンをクリック
  - **Then**: ドロップダウンメニューが表示され、「復元」と「完全削除」の2つの操作が選択可能
- **Negative/Boundary**: メニュー外クリックでメニューが閉じる
- **Depends on**: REQ-0014
- **Notes**: Kobalte Dropdown Menuを使用。復元は通常スタイル、完全削除はdestructive variant（赤色表示）
- **Trace Hooks (optional)**:
  - API: N/A (既存API使用)
  - Component: DropdownMenu, ArchivedPage
  - Task: TASK-NEW-030, TASK-NEW-031

---

### REQ-0027: タイトルspanサイズ調整
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクカードが表示されている
- **Trigger**: タスクカード表示時
- **Acceptance (the only one)**:
  - **Given**: TaskPool、QueuePanel、CompletedPage、ArchivedPageいずれかでタスクカードが表示されている
  - **When**: タスクタイトルが表示される
  - **Then**: タイトルspanの幅がテキスト内容の幅に一致し、無駄な選択領域が発生しない
- **Negative/Boundary**: 親タスクのonClickハンドラは親div要素で維持（影響なし）
- **Depends on**: REQ-0004
- **Notes**: タイトルspanから`flex-1`クラスを削除。TaskPool親タスク（行289-296）、子タスク（行354-361）、QueuePanel（行105-107）、CompletedPage、ArchivedPageの全タスクカードが対象
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: TaskPool, QueuePanel, CompletedPage, ArchivedPage
  - Task: TASK-NEW-032

---

### REQ-0028: キュー順番変更D&D
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: キューに複数のタスクが登録されている
- **Trigger**: タスクカードのドラッグ操作
- **Acceptance (the only one)**:
  - **Given**: QueuePanelに2つ以上のタスクが表示されている
  - **When**: ユーザーがタスクカードをドラッグして別の位置にドロップ
  - **Then**: キュー内のタスク順序が変更され、データベースのposition値が更新される
- **Negative/Boundary**: ドラッグ中はプレビュー表示。API呼び出し失敗時は元の順序に戻す
- **Depends on**: REQ-0006
- **Notes**: @dnd-kit/coreライブラリを使用。既存のreorderQueue APIを活用。楽観的UI更新 + エラー時リロード
- **Trace Hooks (optional)**:
  - API: reorder_queue
  - Component: QueuePanel
  - Task: TASK-NEW-033, TASK-NEW-034

---

### REQ-0029: タグシステムUI統合
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスク作成/編集Dialogが表示されている
- **Trigger**: タグ入力欄での操作
- **Acceptance (the only one)**:
  - **Given**: タスク作成/編集Dialog
  - **When**: ユーザーがタグ入力欄で既存タグを選択、または新規タグを作成（名前+色を指定）
  - **Then**: タグがタスクに紐付けられ、保存後にタグフィルターで検索可能になり、ホバーポップアップでタグが表示される
- **Negative/Boundary**: 新規タグ作成時は名前が必須、色はオプショナル（デフォルト色を使用）
- **Depends on**: REQ-0005
- **Notes**: チップ入力方式、オートコンプリート機能、インライン新規タグ作成（プリセット8色から選択）
- **Trace Hooks (optional)**:
  - API: create_tag (新規タグ作成時), list_tags (既存タグ取得), update_task (タグ紐付け)
  - Component: TagInput, TaskEditDialog
  - Task: TASK-NEW-036, TASK-NEW-037

---

### REQ-0030: タグフィルター展開式UI
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: TaskPool画面が表示されている
- **Trigger**: 「+ Tags」ボタンクリック
- **Acceptance (the only one)**:
  - **Given**: TaskPool画面の検索バー横に「+ Tags」ボタンが表示されている
  - **When**: ユーザーが「+ Tags」ボタンをクリックし、ドロップダウンから1つ以上のタグを選択
  - **Then**: 選択されたタグに一致するタスク（OR条件）のみが表示され、ボタンには選択中のタグ数が表示される（例: `+ Tags (2)`）
- **Negative/Boundary**: タグ未選択時は全タスク表示。ドロップダウン外クリックでメニューを閉じる
- **Depends on**: REQ-0029
- **Notes**: Kobalte Dropdown Menu使用、チェックボックスで複数選択可能、search_tasks APIを呼び出し
- **Trace Hooks (optional)**:
  - API: search_tasks (tags パラメータ), list_tags
  - Component: TagFilter, TaskPool
  - Task: TASK-NEW-038

---

### REQ-0031: タグカラー管理
- **Priority**: COULD
- **Status**: Hold
- **Area**: UI
- **Actor**: User
- **Preconditions**: 新規タグ作成時
- **Trigger**: タグ作成Dialogでのカラー選択
- **Acceptance (the only one)**:
  - **Given**: 新規タグ作成Dialog（インライン表示）
  - **When**: ユーザーがプリセット8色のいずれかを選択
  - **Then**: タグにカラーが設定され、タグチップ表示時に背景色として反映される
- **Negative/Boundary**: 色未選択時はデフォルト色（gray）を使用
- **Depends on**: REQ-0029
- **Notes**: Phase 1ではプリセット8色（Red, Orange, Yellow, Green, Blue, Indigo, Purple, Pink）を実装済み。将来的により柔軟なカラーピッカーに置き換え予定のため保留
- **Trace Hooks (optional)**:
  - API: create_tag (color パラメータ)
  - Component: ColorPicker, TagInput
  - Task: TASK-NEW-039

---

### REQ-0032: ページローディング文字削除
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: CompletedPage、ArchivedPageを表示
- **Trigger**: ページ読み込み時
- **Acceptance (the only one)**:
  - **Given**: CompletedPageまたはArchivedPageを開いた
  - **When**: データ読み込み中
  - **Then**: "Loading..."テキストが表示されず、読み込み完了後に直接タスクリストが表示される
- **Negative/Boundary**: エラー時の表示は別途対応
- **Depends on**: REQ-0024
- **Notes**: ローディング状態のUI表示を削除し、よりクリーンなUXを実現
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: CompletedPage, ArchivedPage
  - Task: TASK-NEW-041

---

### REQ-0033: タスクタイトル文字数制限
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクカードが表示されている
- **Trigger**: タスクカード表示時
- **Acceptance (the only one)**:
  - **Given**: TaskPool、QueuePanel、CompletedPage、ArchivedPageいずれかでタスクカードが表示されている
  - **When**: タスクタイトルが長い
  - **Then**: タイトルが一定幅で切り捨てられ、`...`で省略表示される（CSS text-overflow: ellipsis）
- **Negative/Boundary**: ホバー時にTaskHoverPopupで全文表示可能
- **Depends on**: REQ-0015
- **Notes**: CSS `truncate`クラスと`max-width`を使用してタイトル表示を制限
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: TaskPool, QueuePanel, CompletedPage, ArchivedPage
  - Task: TASK-NEW-042

---

### REQ-0034: グローバルスクロールバー削除
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: アプリケーション起動
- **Trigger**: 常時適用
- **Acceptance (the only one)**:
  - **Given**: アプリケーションが起動している
  - **When**: スクロール可能なエリアが存在する
  - **Then**: スクロールバーが視覚的に表示されず、スクロール機能のみ有効
- **Negative/Boundary**: マウスホイール、タッチパッド、キーボードでのスクロールは正常動作
- **Depends on**: N/A
- **Notes**: CSS `scrollbar-width: none`および`::-webkit-scrollbar { display: none }`をグローバル適用
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: N/A (グローバルCSS)
  - Task: TASK-NEW-043

---

### REQ-0035: ウィンドウ装飾削除と角丸
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI/Platform
- **Actor**: User
- **Preconditions**: アプリケーション起動
- **Trigger**: ウィンドウ表示時
- **Acceptance (the only one)**:
  - **Given**: アプリケーションが起動している
  - **When**: ウィンドウが表示される
  - **Then**: タイトルバー（閉じる/最小化/最大化ボタン）が非表示になり、ウィンドウの角が2px丸くなる
- **Negative/Boundary**: macOSネイティブのウィンドウ操作（トラフィックライト、ドラッグ移動等）は維持
- **Depends on**: N/A
- **Notes**: tauri.conf.jsonで`decorations: false`、`transparent: true`、`macOSPrivateApi: true`を設定。CSSでhtml/body/#rootに`border-radius: 2px`を適用
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: N/A (グローバルCSS、Tauri設定)
  - Task: TASK-NEW-044, TASK-NEW-045

---

### REQ-0036: 入力欄フォーカスリング調整
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: 入力欄が存在する
- **Trigger**: 入力欄フォーカス時
- **Acceptance (the only one)**:
  - **Given**: Input、Textarea、Selectいずれかの入力欄がある
  - **When**: 入力欄にフォーカスする
  - **Then**: フォーカスリングの幅が1px、透明度30%で表示され、控えめなハイライトになる
- **Negative/Boundary**: エラー状態時は赤色のリングを維持
- **Depends on**: N/A
- **Notes**: `focus:ring-2` → `focus:ring-1`、`focus:ring-ring` → `focus:ring-ring/30`に変更
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: Input, Badge, Dialog, TagFilter, Pagination
  - Task: TASK-NEW-046

---

### REQ-0037: カスタム確認ダイアログ（タスク名検証付き）
- **Priority**: MUST
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: 破壊的操作（永久削除等）を実行
- **Trigger**: 永久削除ボタンクリック
- **Acceptance (the only one)**:
  - **Given**: Archivedページでタスクの永久削除を実行
  - **When**: 「Delete permanently」をクリック
  - **Then**: カスタム確認ダイアログが表示され、タスク名を正確に入力しないと削除ボタンが無効化される
- **Negative/Boundary**:
  - タスク名が一致しない場合、エラーメッセージ表示（固定高さで表示領域を確保）
  - キャンセルボタンでダイアログを閉じる
  - 入力欄の枠線色は通常時グレー、エラー時のみ赤
- **Depends on**: REQ-0018
- **Notes**: Kobalte Dialogベースの汎用ConfirmDialogコンポーネントを実装。`requireVerification`オプションでテキスト入力確認を有効化。Tauri plugin-dialogへの依存を削除
- **Trace Hooks (optional)**:
  - API: N/A (既存API使用)
  - Component: ConfirmDialog, ArchivedPage
  - Task: TASK-NEW-047, TASK-NEW-048

---

### REQ-0038: キュー一括操作（未実装）
- **Priority**: SHOULD
- **Status**: Planned
- **Area**: UI
- **Actor**: User
- **Preconditions**: QueuePanelに複数タスクが存在
- **Trigger**: 一括操作ボタンクリック
- **Acceptance (the only one)**:
  - **Given**: QueuePanelに2つ以上のタスクがある
  - **When**: 「すべて完了」または「すべて削除」ボタンをクリック
  - **Then**: キュー内の全タスクが一括で完了またはキューから削除される
- **Negative/Boundary**: 確認ダイアログで操作をキャンセル可能
- **Depends on**: REQ-0006
- **Notes**: 新規API `batch_complete_queue`, `batch_remove_from_queue`を実装予定
- **Trace Hooks (optional)**:
  - API: batch_complete_queue (新規), batch_remove_from_queue (新規)
  - Component: QueuePanel
  - Task: TBD

---

### REQ-0039: search_tasks API軽量化（未実装）
- **Priority**: SHOULD
- **Status**: Planned
- **Area**: Backend
- **Actor**: System
- **Preconditions**: タグフィルター使用時
- **Trigger**: TagFilterでタグ選択
- **Acceptance (the only one)**:
  - **Given**: TagFilterでタグが選択されている
  - **When**: search_tasksまたは新規search_task_ids APIを呼び出す
  - **Then**: タスクIDのみを返す軽量レスポンスが返される
- **Negative/Boundary**: 既存のsearch_tasks APIは後方互換性のため維持
- **Depends on**: REQ-0030
- **Notes**: 新規API `search_task_ids`を実装し、IDのみ返すことでパフォーマンス向上
- **Trace Hooks (optional)**:
  - API: search_task_ids (新規)
  - Component: TagFilter
  - Task: TBD

---

### REQ-0040: タグ管理画面（未実装）
- **Priority**: SHOULD
- **Status**: Planned
- **Area**: UI
- **Actor**: User
- **Preconditions**: タグが存在する
- **Trigger**: タグ管理ページ（/tags）を開く
- **Acceptance (the only one)**:
  - **Given**: タグ管理ページ（/tags）を開いた
  - **When**: タグのリストが表示される
  - **Then**: タグの作成、編集、削除が可能（使用中タグ削除時は警告表示）
- **Negative/Boundary**: 使用中のタグ削除時は確認ダイアログを表示
- **Depends on**: REQ-0029
- **Notes**: 新規ルート`/tags`を追加。既存API（list_tags, create_tag, update_tag, delete_tag）を使用
- **Trace Hooks (optional)**:
  - API: N/A (既存API使用)
  - Component: TagManagementPage (新規)
  - Task: TBD

---

### REQ-0041: Completedページ子タスク表示改善（未実装）
- **Priority**: COULD
- **Status**: Planned
- **Area**: UI
- **Actor**: User
- **Preconditions**: Completedページで子タスクが表示されている
- **Trigger**: Completedページ表示時
- **Acceptance (the only one)**:
  - **Given**: Completedページで子タスクが表示されている
  - **When**: 子タスクのタイトルが表示される
  - **Then**: タイトルが「@親タスク名/子タスク名」形式で表示される
- **Negative/Boundary**: 親タスク情報が取得できない場合は子タスク名のみ表示
- **Depends on**: REQ-0024
- **Notes**: `list_tasks_paginated` APIに`parent_title: Option<String>`を追加するオプションAを推奨
- **Trace Hooks (optional)**:
  - API: list_tasks_paginated (拡張)
  - Component: CompletedPage
  - Task: TBD

---

### REQ-0042: 作成/編集モーダルUI改善（未実装、デザイン要件待ち）
- **Priority**: COULD
- **Status**: Deferred
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスク作成/編集ダイアログを開く
- **Trigger**: 新規タスク作成または編集ボタンクリック
- **Acceptance (the only one)**:
  - **Given**: タスク作成/編集ダイアログが開いている
  - **When**: ダイアログが表示される
  - **Then**: 改善されたUIレイアウトで入力欄が表示される
- **Negative/Boundary**: デザイン要件提供後に実装
- **Depends on**: REQ-0001
- **Notes**: デザイン要件待ち
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: Dialog, TaskPage
  - Task: TBD

---

### REQ-0043: タグフィルターUI改善（未実装、デザイン要件待ち）
- **Priority**: COULD
- **Status**: Deferred
- **Area**: UI
- **Actor**: User
- **Preconditions**: タグフィルターを使用
- **Trigger**: タグフィルターボタンクリック
- **Acceptance (the only one)**:
  - **Given**: TagFilterが表示されている
  - **When**: タグフィルターボタンをクリック
  - **Then**: 改善されたUIでタグ選択が可能
- **Negative/Boundary**: デザイン要件提供後に実装
- **Depends on**: REQ-0030
- **Notes**: デザイン要件待ち
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: TagFilter
  - Task: TBD

---

### REQ-0044: タグ作成画面改良（未実装、デザイン要件待ち）
- **Priority**: COULD
- **Status**: Deferred
- **Area**: UI
- **Actor**: User
- **Preconditions**: タグ作成を実行
- **Trigger**: タグ作成UI表示
- **Acceptance (the only one)**:
  - **Given**: タグ作成UIが表示されている
  - **When**: タグ情報を入力
  - **Then**: 改善されたUIでタグ作成が可能
- **Negative/Boundary**: デザイン要件提供後に実装
- **Depends on**: REQ-0029
- **Notes**: デザイン要件待ち
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: TagInput
  - Task: TBD

---

### REQ-0045: タググルーピング機能（未実装、デザイン要件待ち）
- **Priority**: WONT
- **Status**: Deferred
- **Area**: UI/Backend
- **Actor**: User
- **Preconditions**: タグが複数存在する
- **Trigger**: タググループ管理画面を開く
- **Acceptance (the only one)**:
  - **Given**: タググループ管理画面が開いている
  - **When**: タグをグループに分類
  - **Then**: グループごとにタグが整理表示される
- **Negative/Boundary**: デザイン要件提供後に実装
- **Depends on**: REQ-0029
- **Notes**: 新規テーブル`tag_groups`、新規API `create_tag_group`, `list_tag_groups`が必要。優先度P3で保留
- **Trace Hooks (optional)**:
  - API: create_tag_group (新規), list_tag_groups (新規)
  - Component: TagGroupManagementPage (新規)
  - Task: TBD

---

### REQ-0046: アーカイブボタン表示変更
- **Priority**: SHOULD
- **Status**: Done
- **Area**: UI
- **Actor**: User
- **Preconditions**: Draft状態のタスクカードが表示されている
- **Trigger**: タスクカードホバー時
- **Acceptance (the only one)**:
  - **Given**: Draft状態のタスクカードが表示されている
  - **When**: タスクカードにホバー
  - **Then**: 削除ボタン（ゴミ箱アイコン）の代わりにアーカイブボタン（アーカイブボックスアイコン）が表示される
- **Negative/Boundary**: ボタンの色は他のボタンと同じ白（foreground）
- **Depends on**: REQ-0016
- **Notes**: アイコンを`Trash2Icon`から`ArchiveIcon`に変更、色を`text-destructive`から削除（デフォルト色を使用）
- **Trace Hooks (optional)**:
  - API: N/A
  - Component: TaskPool
  - Task: TASK-NEW-049

---

## 4. Requirement Split / Merge Log
> 粒度調整の履歴を残す（後工程での参照ズレを防ぐ）

- <YYYY-MM-DD> Split: REQ-0012 -> REQ-0045, REQ-0046 (reason)
- <YYYY-MM-DD> Merge: REQ-0020 -> REQ-0018 (Deprecated) (reason)

---

## 5. Change Log
- 2024-12-21 初期要件定義作成 (TMS-v2のPOC開始のため)
- 2025-12-27 追加要件定義 (REQ-0008〜REQ-0015) - 親子ステータス同期、検索・フィルター、UI改善
- 2025-12-28 追加要件定義 (REQ-0016〜REQ-0022) - Draft状態制限、物理削除、restore機能、list_tasks API改良、UI改善
- 2025-12-28 追加要件定義 (REQ-0023〜REQ-0028) - バグ修正、ページネーション、3点リーダーメニュー、タイトルspan調整、D&D機能
- 2025-12-29 追加要件定義 (REQ-0029〜REQ-0031) - タグシステムUI統合、タグフィルター展開式、タグカラー管理（Phase 1）、REQ-0015修正（ホバーポップアップ化）
- 2025-12-29 ステータス更新 - REQ-0015, 0029, 0030: Done, REQ-0031: Hold（将来的に柔軟なカラーピッカーに置き換え予定）
- 2025-12-29 追加要件定義 (REQ-0032〜REQ-0046) - UI/UX改善第3弾（ローディング削除、文字数制限、スクロールバー削除、タイトルバー削除、角丸、フォーカスリング調整、ConfirmDialog実装、アーカイブボタン変更）、API軽量化、タグ管理画面、タググルーピング（未実装含む）
- 2025-12-29 ステータス更新 - REQ-0032, 0033, 0034, 0035, 0036, 0037, 0046: Done, REQ-0038〜0041: Planned, REQ-0042〜0045: Deferred（デザイン要件待ち）
