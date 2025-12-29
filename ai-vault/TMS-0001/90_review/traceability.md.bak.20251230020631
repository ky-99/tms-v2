# Traceability Matrix: <Feature Title>

> Confidentiality: <Public / Internal / Confidential>  
> Repo: <repo-name>  
> Ticket: <TICKET-ID>  
> Branch: <branch-name>  
> Owner: <name>  
> Created: 2025-12-29  
> Last Updated: 2025-12-29

---

## 0. Purpose (Non-negotiable)
This document provides an auditable mapping:
- **REQ (requirements.md)** → **Contract (OpenAPI/AsyncAPI)** → **Design components** → **Implementation tasks**
Used for:
- Coverage checks (no missing REQs)
- Drift checks (no unscoped operations/components)
- Review gating

Sources of truth:
- requirements: `10_prd/requirements.md`
- OpenAPI: `30_contract/openapi.yaml`
- AsyncAPI: `30_contract/asyncapi.yaml`
- Architecture: `40_design/architecture.md`
- Design: `40_design/design.md`
- Decisions: `40_design/decisions.md`
- Tasks: `40_design/tasks.md`

---

## 1. Rules (Do not violate)
1. Every row MUST include a **REQ-ID**.
2. Each row MUST include at least one of:
   - **operationId** (HTTP), OR
   - **messageId** (Event)
3. Contract mappings MUST be consistent with:
   - OpenAPI `x-requirements`
   - AsyncAPI `x-requirements`
4. "Component" MUST be a stable name used in design documents.
5. "TASK-ID" MUST exist in tasks.md once design phase starts.
6. If a REQ is intentionally not mapped, mark it explicitly as:
   - `CoverageStatus = N/A` with a reason in Notes (rare, but allowed)

---

## 2. Status Definitions
### 2.1 CoverageStatus
- **Planned**: mapping exists, not implemented yet
- **InProgress**: implementation ongoing
- **Done**: implemented and verified against acceptance criterion
- **Deferred**: intentionally postponed (must have reason)
- **N/A**: not applicable for this feature scope (must have reason)

### 2.2 Verification
- **Unit**: unit tests cover acceptance
- **Integration**: integration tests cover acceptance
- **E2E**: end-to-end tests cover acceptance
- **Manual**: manual verification only (discouraged; must justify)
- **None**: not verified yet

---

## 3. Traceability Matrix (REQ -> Contract -> Design -> Tasks)

> Notes:
> - Use **one row per REQ** as the default.
> - If a single REQ maps to multiple operations/events, either:
>   - list multiple IDs in the same cell (comma-separated), OR
>   - split into multiple rows with the same REQ-ID and different Contract IDs (choose one style and stick to it).
> - Avoid "TBD" in review phase. Use it only in early phases.

| REQ-ID | operationId (HTTP) | messageId (Event) | Components | TASK-IDs | CoverageStatus | Verification | Notes |
|---|---|---|---|---|---|---|---|
| REQ-ID | N/A | N/A | TBD | N/A | Planned | None | - |
| REQ-0001 | N/A | N/A | TBD | TASK-0001 | Done | None | - |
| REQ-0002 | createTask, getTask, updateTask, deleteTask | N/A | TBD | TASK-0002, TASK-0003 | Done | None | - |
| REQ-0003 | N/A | N/A | TBD | TASK-0002, TASK-0004 | Done | None | - |
| REQ-0004 | listTasks | N/A | TBD | TASK-0009, TASK-0010 | Done | None | - |
| REQ-0005 | listTasks, searchTasks, listTags, createTag, updateTag, deleteTag | N/A | TBD | TASK-0005, TASK-0006 | Done | None | - |
| REQ-0006 | getTaskQueue, addTaskToQueue, removeTaskFromQueue, clearTaskQueue, updateQueuePosition, reorderTaskQueue | N/A | TBD | TASK-0002, TASK-0007 | Done | None | - |
| REQ-0007 | getTaskQueue | N/A | TBD | TASK-0009, TASK-0011 | Done | None | - |
| REQ-0008 | N/A | N/A | TBD | TASK-NEW-001, TASK-NEW-004, TASK-NEW-012 | Done | None | - |
| REQ-0009 | addTaskToQueue | N/A | TBD | TASK-NEW-002, TASK-NEW-004 | Done | None | - |
| REQ-0010 | listTasks | N/A | TBD | TASK-NEW-003, TASK-NEW-004 | Done | None | - |
| REQ-0011 | searchTasks | N/A | TBD | TASK-NEW-005 | Done | None | - |
| REQ-0012 | searchTasks | N/A | TBD | TASK-NEW-005 | Done | None | - |
| REQ-0013 | N/A | N/A | TBD | TASK-NEW-008 | Done | None | - |
| REQ-0014 | N/A | N/A | TBD | TASK-NEW-008 | Done | None | - |
| REQ-0015 | N/A | N/A | TaskHoverPopup, TaskPool | TASK-NEW-006, TASK-NEW-007, TASK-NEW-012, TASK-NEW-040 | Done | E2E | Kobalte Popover使用、クリック操作 |
| REQ-0016 | updateTask | N/A | TBD | TASK-NEW-013, TASK-NEW-017, TASK-NEW-018, TASK-NEW-023 | Done | None | - |
| REQ-0017 | deleteTask | N/A | TBD | TASK-NEW-013, TASK-NEW-018 | Done | None | - |
| REQ-0018 | deleteTaskPermanently | N/A | TBD | TASK-NEW-014, TASK-NEW-021 | Done | None | - |
| REQ-0019 | listTasks | N/A | TBD | TASK-NEW-016, TASK-NEW-020 | Done | None | - |
| REQ-0020 | N/A | N/A | TBD | TASK-NEW-019 | Done | None | - |
| REQ-0021 | N/A | N/A | TBD | TASK-NEW-022 | Done | None | - |
| REQ-0022 | restoreTask | N/A | TBD | TASK-NEW-015, TASK-NEW-017, TASK-NEW-021, TASK-NEW-023 | Done | None | - |
| REQ-0023 | N/A | N/A | TBD | TASK-NEW-024, TASK-NEW-035 | Done | None | - |
| REQ-0024 | listTasksPaginated | N/A | TBD | TASK-NEW-025, TASK-NEW-026 | Done | None | - |
| REQ-0025 | listTasksPaginated | N/A | TBD | TASK-NEW-027, TASK-NEW-028, TASK-NEW-029 | Done | None | - |
| REQ-0026 | N/A | N/A | TBD | TASK-NEW-030, TASK-NEW-031 | Done | None | - |
| REQ-0027 | N/A | N/A | TBD | TASK-NEW-032 | Done | None | - |
| REQ-0028 | reorderTaskQueue | N/A | TBD | TASK-NEW-033, TASK-NEW-034, TASK-NEW-035 | Done | None | - |
| REQ-0029 | create_tag, list_tags, update_task | N/A | TagInput, TaskEditDialog, TaskCreateDialog | TASK-NEW-036, TASK-NEW-037, TASK-NEW-040 | Done | E2E | チップ入力方式、オートコンプリート、インライン新規タグ作成 |
| REQ-0030 | search_tasks, list_tags | N/A | TagFilter, TaskPool | TASK-NEW-038, TASK-NEW-040 | Done | E2E | Kobalte Dropdown Menu使用、複数選択OR条件フィルタ |
| REQ-0031 | create_tag | N/A | TagInput (color picker) | TASK-NEW-039, TASK-NEW-040 | Deferred | None | Phase 1実装済み、将来的に柔軟なカラーピッカーに置き換え予定 |
| REQ-0032 | N/A | N/A | CompletedPage, ArchivedPage | TASK-NEW-041 | Done | Manual | ページローディング文字削除、よりクリーンなUI |
| REQ-0033 | N/A | N/A | TaskPool, QueuePanel, CompletedPage, ArchivedPage | TASK-NEW-042 | Done | Manual | タスクタイトル文字数制限、CSS truncateクラス適用 |
| REQ-0034 | N/A | N/A | GlobalStyles (index.css) | TASK-NEW-043 | Done | Manual | グローバルスクロールバー削除、全ブラウザ対応 |
| REQ-0035 | N/A | N/A | TauriConfig, App, GlobalStyles | TASK-NEW-044, TASK-NEW-045 | Done | Manual | タイトルバー削除＋角丸ウィンドウ、透明ウィンドウ＋CSS角丸 |
| REQ-0036 | N/A | N/A | Input, Textarea, Dialog | TASK-NEW-046 | Done | Manual | 入力欄フォーカスリング調整、ring-2→ring-1、透明度30% |
| REQ-0037 | deleteTaskPermanently | N/A | ConfirmDialog, ArchivedPage | TASK-NEW-047, TASK-NEW-048 | Done | E2E | タスク名検証機能付き確認ダイアログ、Tauriプラグイン削除 |
| REQ-0038 | complete_all_queue, clear_task_queue | N/A | QueuePanel, ConfirmDialog, QueueService, queueStore | TASK-NEW-050 | Done | Build | キュー一括削除/完了機能（Complete All/Clear All） |
| REQ-0039 | search_tasks | N/A | TaskPool | TBD | Planned | None | search_tasks API軽量化（未実装） |
| REQ-0040 | N/A | N/A | TagManagementPage | TBD | Planned | None | タグ管理画面（未実装） |
| REQ-0041 | listTasksPaginated | N/A | CompletedPage | TBD | Planned | None | Completedページ子タスク表示改善（未実装） |
| REQ-0042 | N/A | N/A | TaskEditDialog, TaskCreateDialog | TBD | Deferred | None | 作成/編集モーダルUI改善（デザイン要件待ち） |
| REQ-0043 | N/A | N/A | TagFilter | TBD | Deferred | None | タグフィルターUI改善（デザイン要件待ち） |
| REQ-0044 | N/A | N/A | CreateTagDialog | TBD | Deferred | None | タグ作成画面改良（デザイン要件待ち） |
| REQ-0045 | create_tag_group, list_tag_groups | N/A | TagGroupManager | TBD | Deferred | None | タググルーピング機能（デザイン要件待ち） |
| REQ-0046 | N/A | N/A | TaskPool | TASK-NEW-049 | Done | Manual | アーカイブボタン表示変更、アイコン＋title変更 |

---

## 4. Contract Drift Checks (Must be empty at Review)
> ここが埋まっている限り、レビューで「判定OK」は出せない。

### 4.1 HTTP operations without REQ mapping
(None - all operations mapped)

### 4.2 Event messages without REQ mapping
(None - all messages mapped)

### 4.3 Design components without REQ mapping
(Manual check required - components extracted from design.md)

---

## 5. Coverage Summary (Optional but recommended)
- Total REQs: 47
- Mapped to HTTP: 22
- Mapped to Events: 0
- Done: 38
- InProgress: 0
- Planned: 4
- Deferred/N/A: 5

---

## 6. Change Log
- 2025-12-29 Auto-generated by gen_traceability.sh
- 2025-12-29 Manual update: REQ-0015, 0029, 0030, 0031 status updated (タグシステムUI統合完了)
- 2025-12-29 Manual update: REQ-0032〜REQ-0046追加 (UI/UX Phase 3: 9タスク完了、6タスク未実装)
- 2025-12-29 Manual update: REQ-0038実装完了 (キュー一括操作機能、TASK-NEW-050、Done: 37→38, Planned: 5→4, HTTP: 21→22)
