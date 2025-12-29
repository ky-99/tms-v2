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
| REQ-0015 | タスクリスト表示形式変更 | SHOULD | Draft | UI | REQ-0004 |
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
| REQ-0028 | キュー順番変更D&D | SHOULD | Draft | UI | REQ-0006 |

**Priority**: MUST / SHOULD / COULD
**Status**: Draft / Approved / Implementing / Done / Deprecated

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

### REQ-0015: タスクリスト表示形式変更
- **Priority**: SHOULD
- **Status**: Draft
- **Area**: UI
- **Actor**: User
- **Preconditions**: タスクプール画面が表示されている
- **Trigger**: 画面表示時
- **Acceptance (the only one)**:
  - **Given**: タスクプール画面
  - **When**: タスク一覧が表示される
  - **Then**: タスクがカード形式ではなくリスト形式（1行）で表示され、階層構造（インデント+展開アイコン）が維持される
- **Negative/Boundary**: 子タスクがない場合は展開アイコンを非表示
- **Depends on**: REQ-0004
- **Notes**: アイコンボタン使用、タスククリックで詳細ポップアップ（読み取り専用）
- **Trace Hooks (optional)**:
  - API: list_tasks
  - Component: TaskPage, TaskDetailModal
  - Task: TASK-NEW-006, TASK-NEW-007

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
- **Status**: Draft
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
