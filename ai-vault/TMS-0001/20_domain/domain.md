# Domain: Task Management System v2 (TMS-v2)

> Confidentiality: Internal
> Repo: tms-v2
> Ticket: TMS-0001
> Branch: feature/tms-v2-poc
> Owner: Developer
> Created: 2024-12-21
> Last Updated: 2024-12-21

---

## 1. Domain Overview
### 1.1 Purpose
TMS-v2は、個人ユーザーがタスクを効率的に管理するためのドメインです。タスクの作成・整理から、日々の実行計画立案までをサポートします。

### 1.2 Core Concept
- **Task**: 実行可能な作業単位
- **TaskPool**: 全タスクの集合体
- **TaskQueue**: 日次実行予定のタスク列

### 1.3 Business Value
ユーザーがタスクの依存関係を把握し、効率的な実行計画を立案できるようにすることで、生産性を向上させる。

---

## 2. Core Entities
### 2.1 Task
タスクは実行可能な最小作業単位であり、以下の属性を持ちます：

- **Identity**: システム内で一意に識別可能なID
- **Title**: タスクの簡潔な名称（必須）
- **Description**: タスクの詳細な説明（任意）
- **Status**: タスクの現在の状態
  - Draft: 下書き状態
  - Active: 実行可能状態
  - Completed: 完了状態
  - Archived: アーカイブ状態
  - **注記**: 親タスクのStatusは子タスクのStatusに基づいて自動的に決定される（BR-013参照）
- **Tags**: タスクに関連付けられたタグの集合（任意）
- **CreatedAt**: 作成日時
- **UpdatedAt**: 最終更新日時

### 2.2 TaskPool
タスクプールは、全てのタスクを管理する集合体です：

- 全てのTaskインスタンスを保持
- タスク間の親子関係を維持
- 検索・フィルタリング機能をサポート

### 2.3 TaskQueue
タスクキューは、日次実行予定のタスクを管理する順序付き集合です：

- TaskPoolから選択されたタスクのリスト
- 実行順序を保持
- 重複タスクの追加を防止
- 当日の作業計画を表現

### 2.4 Tag
タグはタスクの分類・検索を容易にするためのラベルであり、以下の属性を持ちます：

- **Identity**: システム内で一意に識別可能なID
- **Name**: タグの名称（必須、一意）
- **Color**: タグの表示色（任意、UIでの視覚的識別用）
- **CreatedAt**: 作成日時
- **UsageCount**: このタグが付与されているタスク数

---

## 3. Business Rules
### 3.1 Task Management Rules
- **BR-001**: タスクは必ずTitleを持つ必要がある
- **BR-002**: タスクのStatusはDraft → Active → Completed → Archivedの順に遷移する
- **BR-003**: CompletedまたはArchived状態のタスクは編集不可
- **BR-004**: タスクの削除は物理削除ではなく、Archived状態への変更とする

### 3.2 Relationship Rules
- **BR-005**: タスクは親タスクを0または1つ持つことができる（単一親制約）
- **BR-006**: 循環参照は許可されない（親子関係のループ禁止）
- **BR-007**: 子タスクが存在する場合、親タスクは全ての子タスクがArchived OR Completedでない限りArchived状態に遷移できない
- **BR-013**: 親タスクのStatusは子タスクのStatusに基づいて自動的に決定される（親子ステータス自動同期）
  - 全ての子タスクがDraft → 親もDraft
  - 1つでも子タスクがActive → 親もActive
  - 全ての子タスクがCompleted → 親もCompleted
  - 全ての子タスクがArchived OR Completed → 親はArchived可能
- **BR-014**: 親タスクのステータス変更は子タスクのステータス変更時にリアルタイムで実行される
- **BR-015**: 親タスクは子タスクが存在する場合、TaskQueueに追加できない
- **BR-016**: タスクの階層は最大2レベル（親-子）までとする。孫タスクの作成は禁止される（パフォーマンス最適化のため）

### 3.3 Tag Management Rules
- **BR-008**: タグ名は一意である必要がある
- **BR-009**: タグ名は空文字列または空白のみからなるものは許可されない
- **BR-010**: タグ名は50文字以内に制限される
- **BR-011**: 1つのタスクに付与できるタグ数は最大10個まで
- **BR-012**: 使用されていないタグは自動的に削除される

### 3.3 TaskQueue Rules
- **BR-008**: TaskQueueには同じタスクを重複して追加できない
- **BR-009**: TaskQueueに追加できるのはActive状態のタスクのみ
- **BR-010**: TaskQueueは日次でリセットされる（前日のキューは保持）

---

## 4. Workflows / Use Cases
### 4.1 Task Creation Workflow
1. ユーザーが新しいタスクの作成を要求
2. システムがTaskインスタンスを生成
3. ユーザーがTitleとDescriptionを入力
4. システムがTaskをTaskPoolに追加（初期Status: Draft）
5. ユーザーが必要に応じて親タスクを設定

### 4.2 Daily Planning Workflow
1. ユーザーがTaskPoolからタスクを選択
2. システムが選択されたタスクをTaskQueueに追加
3. ユーザーがキューの順序を調整
4. システムが当日の作業計画として保存

### 4.3 Task Execution Workflow
1. ユーザーがTaskQueueからタスクを選択
2. タスクの実行を開始
3. 実行完了時にStatusをCompletedに変更
4. システムがTaskPoolの状態を更新

### 4.4 Parent-Child Status Synchronization Workflow
1. 子タスクのStatusが変更される（Draft/Active/Completed/Archivedのいずれか）
2. システムが親タスクの存在を確認
3. システムが全ての子タスクのStatusを集計
4. システムがBR-013に基づいて親タスクのStatusを計算
5. システムが親タスクのStatusを更新
6. 親タスクにさらに親が存在する場合、再帰的に3〜5を実行

---

## 5. Invariants (Business Constraints)
### 5.1 Data Integrity
- **INV-001**: 全てのTaskは有効なIDを持つ
- **INV-002**: TaskPool内の全タスクは一意のIDを持つ
- **INV-003**: 親子関係は循環しない

### 5.2 State Consistency
- **INV-004**: TaskQueue内のタスクは全てTaskPoolに存在する
- **INV-005**: TaskQueue内のタスクは全てActive状態である
- **INV-006**: 子タスクを持つ親タスクは削除不可

### 5.3 Tag Integrity
- **INV-007**: 全てのTagは有効なNameを持つ
- **INV-008**: TaskPool内の全タスクのタグはTagエンティティとして存在する
- **INV-009**: タグのUsageCountは実際の使用数と一致する

---

## 6. Domain Events
### 6.1 Task Events
- **TaskCreated**: 新しいタスクが作成された
- **TaskUpdated**: タスクが更新された
- **TaskCompleted**: タスクが完了した
- **TaskArchived**: タスクがアーカイブされた
- **ParentStatusSynchronized**: 親タスクのStatusが子タスクの変更により自動更新された

### 6.2 Queue Events
- **TaskAddedToQueue**: タスクがキューに追加された
- **TaskRemovedFromQueue**: タスクがキューから削除された
- **QueueReset**: キューがリセットされた

### 6.3 Tag Events
- **TagCreated**: 新しいタグが作成された
- **TagDeleted**: タグが削除された
- **TagAttached**: タグがタスクに付与された
- **TagDetached**: タグがタスクから削除された

---

## 7. References
- requirements.md: `10_prd/requirements.md`
- glossary.md: `20_domain/glossary.md`
- PRD: `10_prd/PRD.md`

---

## 8. Change Log
- 2024-12-21 ドメイン設計の初期作成 (TMS-v2のPOC開発開始のため)
- 2024-12-21 タグ機能の追加 (タスク分類機能の拡張)
- 2025-12-27 親子ステータス自動同期機能の追加、UI改善要件の反映 (REQ-0008〜REQ-0015)