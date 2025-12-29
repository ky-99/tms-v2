# Glossary: Task Management System v2 (TMS-v2)

> Confidentiality: Internal
> Repo: tms-v2
> Ticket: TMS-0001
> Branch: feature/tms-v2-poc
> Owner: Developer
> Created: 2024-12-21
> Last Updated: 2024-12-21

---

## 1. Core Domain Terms

### A
- **Active Status**: タスクが実行可能な状態。Draft状態から遷移し、実行を開始できる状態。
- **Archived Status**: タスクがアーカイブされた状態。削除されたタスクと同等だが、データは保持される。

### C
- **Child Task**: 親タスクを持つタスク。親タスクのサブタスクとして機能し、依存関係を表現する。
- **Completed Status**: タスクが完了した状態。実行が終了し、これ以上の操作が必要ない状態.

### D
- **Draft Status**: タスクが下書き状態。作成直後または編集中の状態で、まだ実行できない。

### P
- **Parent Task**: 子タスクを持つタスク。複数の子タスクを持つことができ、タスクの階層構造を形成する。

### S
- **Status Synchronization**: 親タスクのステータスが子タスクのステータス変化に基づいて自動的に更新される仕組み。リアルタイムで実行される。

### T
- **Tag**: タスクの分類・検索を容易にするためのラベル。タスクに付与することで、関連タスクをグループ化できる。
- **Task**: 実行可能な最小作業単位。タイトル、説明、ステータス、タグなどの属性を持つ。
- **Task Pool**: 全てのタスクを管理する集合体。タスクの作成、編集、検索を行うための領域。
- **Task Queue**: 日次実行予定のタスクを順序付けて管理するリスト。当日の作業計画を表現する.

---

## 2. Business Rules Terms

- **Circular Reference**: タスクの親子関係がループする状態（例: A→B→A）。システムで禁止されている。
- **Parent-Child Status Synchronization Rule**: 親タスクのステータスを子タスクのステータスから自動計算するルール。全子がDraft→親もDraft、1つでもActive→親もActive、全子がCompleted→親もCompletedとなる。
- **Real-time Propagation**: ステータス変更が即座に親タスクへ伝播する仕組み。子タスク更新時に自動実行される。
- **Single Parent Constraint**: 各タスクが持てる親タスクは最大1つまでという制約。

---

## 3. Technical Terms

- **CRUD Operations**: Create, Read, Update, Deleteの4つの基本操作。
- **Entity**: ドメイン内で一意に識別可能なオブジェクト。
- **Invariant**: 常に維持されなければならないビジネスルールや制約。

---

## 4. User Interface Terms

- **Archived Task Screen**: アーカイブされたタスクを一覧表示する専用画面。削除されたタスクの確認用。
- **Completed Task Screen**: 完了済みタスクを一覧表示する専用画面。過去の作業履歴の確認用。
- **Filter Chip**: フィルタリング条件を表示する小さなボタン状のUI要素。クリックでオン/オフを切り替える。
- **Filter Panel**: 検索バーとフィルターチップを含むUI領域。タスクの絞り込み操作を提供する。
- **List Display**: タスクを一覧形式で表示するUI形式。カード表示よりもコンパクトで、階層構造を維持する。
- **Search Bar**: キーワード検索を行うための入力フィールド。タスクのタイトルや説明文を対象に検索する。
- **Task Detail Popup**: タスクの詳細情報を表示するモーダルダイアログ。読み取り専用で、作成日時、更新日時、親子関係などを表示する。
- **Task Pool Screen**: Draft と Active 状態のタスクを表示し、CRUD操作を行う画面。ツリー構造で親子関係を表示。検索・フィルター機能を含む。
- **Task Queue Screen**: 日次実行予定のタスクを表示する画面。作業の進捗管理を行う。

---

## 5. Process Terms

- **Daily Planning**: タスクプールからタスクを選択し、日次キューを作成するプロセス。
- **Task Execution**: キューからタスクを取り出し、実際に実行するプロセス。

---

## 6. References
- domain.md: `20_domain/domain.md`
- requirements.md: `10_prd/requirements.md`
- PRD: `10_prd/PRD.md`

---

## 7. Change Log
- 2024-12-21 用語集の初期作成 (TMS-v2のPOC開発開始のため)
- 2024-12-21 Tag関連用語の追加 (タスク分類機能の拡張)
- 2025-12-27 親子ステータス自動同期、UI改善関連用語の追加 (REQ-0008〜REQ-0015)