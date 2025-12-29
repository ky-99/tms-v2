# Context Bundle: <Feature Title>

> Confidentiality: <Public / Internal / Confidential>  
> Repo: <repo-name>  
> Ticket: <TICKET-ID>  
> Branch: <branch-name>  
> Phase: Implementation
> Updated: 2025-12-29 12:18 (JST)
> Owner: <name>

---

## 0) Operating Rules (Must Follow)
1. **Single Source of Truth**
   - Requirements truth: `10_prd/requirements.md`
   - Domain language truth: `20_domain/glossary.md`
   - Contract truth: `30_contract/openapi.yaml` and `30_contract/asyncapi.yaml`
   - Design truth: `40_design/design.md` and `40_design/architecture.md`
   - Review truth (mapping): `90_review/traceability.md`
2. **REQ discipline**
   - 1 REQ = 1 Acceptance Criterion (strict).
   - No implementation details in requirements.
3. **Traceability discipline**
   - Every contract item MUST map to REQ via `x-requirements`.
   - Every design component MUST declare which REQs it implements.
4. **No guesswork**
   - If information is missing/ambiguous, ask targeted questions before producing final artifacts.
   - Do not invent domain terms; add to glossary first.
5. **Bundle size discipline**
   - This bundle is a **control plane**, not a full dump.
   - Use **excerpts** only. If a section grows too large, replace details with pointers.
   - Focus REQs: default max **10** (extend only with explicit reason).
6. **Output discipline**
   - Produce outputs exactly in the format specified in section 8.
   - If proposing changes, include a **diff-style summary** (Added/Changed/Removed).

---

## 1) Objective (What we are doing now)
- **Current objective**: タスク単位でコードを実装し、テストを含めて DoD を満たす
- **Definition of Done**:
  - DoD-1: 全 TASK が Done または Processing 状態
  - DoD-2: 各 TASK の DoD が満たされている（テスト含む）
  - DoD-3: traceability.md が最新状態に保たれている
---

## 2) Executive Summary (Read-first)
- **One-liner**: 個人用のタスクマネジメントアプリケーションをTauri + Rust + SQLite + Reactで開発する
- **Why**: 既存TMSの改善版として新しい技術スタックでの実装を開始し、クロスプラットフォーム対応とパフォーマンス向上を目指すため
- **Scope**: タスクプール管理と日次タスクキュー作成の基本機能を含む。複数ユーザー対応、クラウド同期、外部サービス連携は除外
- **Top risks**: Tauri + Rustの学習コスト、技術スタックの互換性問題、パフォーマンス問題

---

## 3) Canonical Pointers (Authoritative file paths)
> "最新版"はこのパスに存在する前提（v2など別名ファイルを作らない）。

- PRD: `10_prd/PRD.md`
- Requirements: `10_prd/requirements.md`
- Domain: `20_domain/domain.md`
- Glossary: `20_domain/glossary.md`
- OpenAPI (HTTP): `30_contract/openapi.yaml`
- AsyncAPI (Events): `30_contract/asyncapi.yaml`
- Architecture: `40_design/architecture.md`
- Design: `40_design/design.md`
- Decisions (ADR): `40_design/decisions.md`
- Tasks: `40_design/tasks.md`
- Traceability: `90_review/traceability.md`

---

## 4) Non-negotiables (Constraints)
### 4.1 Product / Business
- 個人開発プロジェクトのため、納期・運用制約なし
- 単一ユーザー専用（今後拡張予定）

### 4.2 Technical
- Tauri + Rust + SQLite + Reactの技術スタックを使用
- ローカルアプリケーションとして実装（クラウド同期なし）
- Node.js 18+, Rust 1.70+ を前提とした開発環境

### 4.3 Security / Compliance
- 個人情報は含まれないため、セキュリティ要件は最小限
- データはローカルSQLiteに保存

---

## 5) Domain Snapshot (Ubiquitous Language)
> 重要語のみ抜粋。用語追加が必要なら先にglossaryへ。

### 5.1 Canonical Terms (Top 10)
- **Task**: 実行可能な最小作業単位。タイトル、説明、ステータス、タグなどの属性を持つ。
- **TaskPool**: 全てのタスクを管理する集合体。タスクの作成、編集、検索を行うための領域。
- **TaskQueue**: 日次実行予定のタスクを順序付けて管理するリスト。当日の作業計画を表現する。
- **Tag**: タスクの分類・検索を容易にするためのラベル。タスクに付与することで、関連タスクをグループ化できる。
- **Parent Task**: 子タスクを持つタスク。複数の子タスクを持つことができ、タスクの階層構造を形成する。
- **Child Task**: 親タスクを持つタスク。親タスクのサブタスクとして機能し、依存関係を表現する。
- **Status**: タスクの現在の状態（Draft/Active/Completed/Archived）。
- **Draft Status**: タスクが下書き状態。作成直後または編集中の状態で、まだ実行できない。
- **Active Status**: タスクが実行可能な状態。Draft状態から遷移し、実行を開始できる状態。
- **Completed Status**: タスクが完了した状態。実行が終了し、これ以上の操作が必要ない状態。

### 5.2 Bounded Context / Ownership (Top-level)
- Context: Task Management / Owner: Developer / Responsibility: 個人用のタスク管理機能の提供（プール管理、キュー管理、タグ分類）

### 5.3 Invariants (If defined)
- **INV-001**: 全てのTaskは有効なIDを持つ
- **INV-002**: TaskPool内の全タスクは一意のIDを持つ
- **INV-003**: 親子関係は循環しない
- **INV-004**: TaskQueue内のタスクは全てTaskPoolに存在する
- **INV-005**: TaskQueue内のタスクは全てActive状態である
- **INV-006**: 子タスクを持つ親タスクは削除不可
- **INV-007**: 全てのTagは有効なNameを持つ
- **INV-008**: TaskPool内の全タスクのタグはTagエンティティとして存在する
- **INV-009**: タグのUsageCountは実際の使用数と一致する

---

## 6) Requirements Snapshot (Strict, excerpt)
> **抜粋でOK**。全文はrequirements.mdが正。
<!-- AUTO:BEGIN requirements_snapshot -->
### 6.1 Focus REQs (This phase)
| REQ | Title | Priority | Status |
|---|---|---|---|
| REQ-0001 | 開発環境構築手順 | MUST | Draft |
| REQ-0002 | タスクCRUD機能 | MUST | Draft |
| REQ-0003 | タスク親子関係管理 | MUST | Draft |
| REQ-0004 | タスクプール画面 | MUST | Draft |
| REQ-0006 | タスクキュー管理 | MUST | Draft |
| REQ-0007 | タスクキュー画面 | MUST | Draft |
| REQ-0008 | 親子タスクステータス自動同期 | MUST | Draft |
| REQ-0009 | 親タスクのキュー登録制限 | MUST | Draft |
| REQ-0010 | タスクプールの表示範囲拡張 | MUST | Draft |
| REQ-0005 | タスク検索・フィルタ機能 | SHOULD | Draft |

### 6.2 Acceptance Details (Only for focus REQs)
- **REQ-0001**
  - Given: クリーンな開発環境
  - When: 構築手順ドキュメントに従って環境構築を実行
  - Then: Tauri + Rust + SQLite + Reactの開発環境が利用可能になり、アプリケーションのビルド・実行が可能
- **REQ-0002**
  - Given: タスク管理画面が表示されている
  - When: ユーザーがタスクの作成・更新・削除を操作
  - Then: SQLiteデータベースに変更が反映され、UIに即時反映される
- **REQ-0003**
  - Given: タスク作成・編集画面
  - When: ユーザーが親タスクを指定して子タスクを作成
  - Then: データベースに親子関係が保存され、UIで関係が視覚的に表示される
- **REQ-0004**
  - Given: アプリケーションが起動している
  - When: ユーザーがタスクプール画面を開く
  - Then: 全タスクとその親子関係がツリー構造で表示され、CRUD操作が可能
- **REQ-0006**
  - Given: タスクプール画面でタスクが選択されている
  - When: ユーザーが「キューに追加」操作を行う
  - Then: 選択されたタスクが日次キューに追加され、重複は許可されない
- **REQ-0007**
  - Given: アプリケーションが起動している
  - When: ユーザーがタスクキュー画面を開く
  - Then: 日次キューのタスクがリスト表示され、完了状態の更新が可能
- **REQ-0008**
  - Given: 親タスクと子タスクが存在する
  - When: 子タスクのステータスが変更される（Draft/Active/Completed/Archivedのいずれか）
  - Then: 親タスクのステータスが以下のルールに従って自動更新される
- **REQ-0009**
  - Given: 親タスク（子タスクを1つ以上持つタスク）が存在する
  - When: ユーザーが親タスクをキューに追加しようとする
  - Then: エラーメッセージが表示され、キューへの追加が拒否される
- **REQ-0010**
  - Given: タスクプール画面
  - When: 画面が表示される
  - Then: Draft および Active ステータスのタスクが両方表示される
- **REQ-0005**
  - Given: タスクプール画面にタスクが表示されている
  - When: ユーザーが検索キーワードを入力またはフィルタを選択
  - Then: 条件に一致するタスクのみが表示される
<!-- AUTO:END requirements_snapshot -->
---

## 7) Contract Snapshot (HTTP + Events) — may be empty
> **空でもOK**。Contractフェーズで埋める。

<!-- AUTO:BEGIN contract_snapshot -->
### 7.1 HTTP Operations (OpenAPI)
| operationId | Path/Method | x-requirements | Notes |
|---|---|---|---|
| addTaskToQueue | POST /queue | REQ-0006, REQ-0009 | - |
| clearTaskQueue | POST /queue/clear | REQ-0006 | - |
| createTag | POST /tags | REQ-0005 | - |
| createTask | POST /tasks | REQ-0002 | - |
| deleteTag | DELETE /tags/{id} | REQ-0005 | - |
| deleteTask | DELETE /tasks/{id} | REQ-0002, REQ-0017 | - |
| deleteTaskPermanently | DELETE /tasks/{id}/permanently | REQ-0018 | - |
| getTask | GET /tasks/{id} | REQ-0002 | - |
| getTaskQueue | GET /queue | REQ-0006, REQ-0007 | - |
| listTags | GET /tags | REQ-0005 | - |
| listTasks | GET /tasks | REQ-0004, REQ-0005, REQ-0010, REQ-0019 | - |
| listTasksPaginated | GET /tasks/paginated | REQ-0024, REQ-0025 | - |
| removeTaskFromQueue | DELETE /queue | REQ-0006 | - |
| reorderTaskQueue | POST /queue/reorder | REQ-0006, REQ-0028 | - |
| restoreTask | POST /tasks/{id}/restore | REQ-0022 | - |
| searchTasks | GET /tasks/search | REQ-0005, REQ-0011, REQ-0012 | - |
| updateQueuePosition | PUT /queue/position | REQ-0006 | - |
| updateTag | PUT /tags/{id} | REQ-0005 | - |
| updateTask | PUT /tasks/{id} | REQ-0002, REQ-0016 | - |

### 7.2 Event Notifications (AsyncAPI)
| messageId | Channel | x-requirements | Notes |
|---|---|---|---|

N/A
<!-- AUTO:END contract_snapshot -->
---

## 8) Tasking: What you must produce now
> ここがAIへの指示書。曖昧な依頼は禁止。

<!-- AUTO:BEGIN tasking_snapshot -->
### 8.1 Current Phase Tasks (from tasks.md)
| TASK-ID | Title | Status | Priority | Maps to |
|---|---|---|---|---|
| TASK-NEW-007 | タスク詳細ポップアップ実装 | UnDone | P2 | REQ-0015 |
| TASK-NEW-009 | カラーパレット適用 | UnDone | P2 | N/A |
| TASK-NEW-033 | D&Dライブラリ統合 | UnDone | P2 | REQ-0028 |
| TASK-NEW-034 | QueuePanel D&D実装 | UnDone | P2 | REQ-0028 |

### 8.2 Task Progress
- Total Tasks: 47
- Done: 43
- Processing: 0
- UnDone: 4
- Progress: 91% (43/47)

### 8.3 Next Actions
1. Start TASK-NEW-007 (タスク詳細ポップアップ実装)
2. Start TASK-NEW-009 (カラーパレット適用)
3. Start TASK-NEW-033 (D&Dライブラリ統合)
<!-- AUTO:END tasking_snapshot -->

---

### 8.4 Manual Instructions (Edit as needed)
> **このセクションは Phase に応じて自動生成**：必要に応じて編集可能

#### 8.4.1 Primary deliverable
- Deliverable type: Code implementation
- Target file path: Source code, tests
- Required sections to modify:
  - (See work steps below)

#### 8.4.2 Work steps (Follow in order)
1. Section 8.1 の Current Phase Tasks から次の TASK を選択
2. 選択した TASK を実装（コード + テスト）
3. tasks.md の Status を更新（UnDone → Processing → Done）
4. gen_all.sh Implementation を実行して進捗を反映
5. 次の TASK に進む

#### 8.4.3 Output format (Strict)
- Provide:
  1. **Proposed edits** (copy-pastable content)
  2. **Diff summary** (Added/Changed/Removed bullets)
  3. **Open questions** (if any; max 10, targeted)

#### 8.4.4 Constraints for this task
- Must keep operationId stable: Yes (no changes)
- Must keep messageId stable: Yes (no changes)
- Must not introduce new terms unless added to glossary first: Yes
- Must update traceability: Yes (auto-updated)
---

## 9) Pre-flight Checks (Gate)
<!-- AUTO:BEGIN preflight_checks -->
### 9.1 Coverage Checks
- [x] All operationIds have x-requirements (19/19)
- [x] All messageIds have x-requirements (0/0)
- [x] Focus REQs defined (10 REQs)

### 9.2 Drift Checks
- [x] No unmapped operations
- [x] No unmapped messages
- [x] No undefined terms in requirements

### 9.3 Quality Metrics
- REQ Coverage: 10 REQs
- API Coverage: 19/19
- Event Coverage: N/A
- Task Completion: 91% (43/47)

### 9.4 Overall Status
**✅ Ready for next phase**
<!-- AUTO:END preflight_checks -->

---

### 9.5 Manual Checks (Review before PR)
> **このセクションは手動確認**：レビュー時にチェック

- [ ] No new terms introduced without glossary update
- [ ] No implementation details leaked into requirements
- [ ] Canonical file paths preserved (no "v2 filename")
- [ ] All tests passing
- [ ] Code review completed

---

## 10) Notes / Context (Optional)
- <補足>（なければ N/A）
