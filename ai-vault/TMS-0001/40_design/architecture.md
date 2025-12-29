# Architecture: Task Management System v2 (TMS-v2)

> Confidentiality: Internal
> Repo: tms-v2
> Ticket: TMS-0001
> Branch: feature/tms-v2-poc
> Owner: Developer
> Created: 2025-12-21
> Last Updated: 2025-12-21

---

## 0. Purpose
- Define system-level structure, boundaries, deployment/runtime concerns, and non-functional requirements (NFRs).
- This document is NOT a detailed class/module design (that belongs to design.md).

References:
- PRD: `10_prd/PRD.md`
- Requirements: `10_prd/requirements.md`
- Domain: `20_domain/domain.md`
- Glossary: `20_domain/glossary.md`
- OpenAPI: `30_contract/openapi.yaml`
- AsyncAPI: `30_contract/asyncapi.yaml`
- Design: `40_design/design.md`
- Decisions: `40_design/decisions.md`
- Tasks: `40_design/tasks.md`
- Traceability: `90_review/traceability.md`

---

## 1. Scope
### 1.1 In Scope
- 個人用タスク管理デスクトップアプリケーションのシステムアーキテクチャ
- Tauri + Rust + SQLite + SolidJS 技術スタックを使用した実装
- タスクプール管理と日次タスクキューの基本機能
- ローカルデスクトップアプリケーションとして動作するIPCベースの通信

### 1.2 Out of Scope
- 複数ユーザー対応
- クラウド同期機能
- 外部サービス連携（カレンダー、通知サービス等）
- モバイルアプリケーション
- Webブラウザ版

---

## 2. System Context (C4 L1)
> TMS-v2はローカルデスクトップアプリケーションとして動作し、外部システムとの連携はない。

- Users/Clients:
  - 個人ユーザー（デスクトップアプリケーションのエンドユーザー）
- External Systems:
  - なし（ローカルアプリケーションのため）
- Internal Services:
  - なし（単一プロセスアプリケーション）

**Context Diagram (text)**
- ユーザー → (GUI操作) → TMS-v2 Desktop Application
- TMS-v2 Application → (ファイルI/O) → SQLite Database (ローカル)

---

## 3. Bounded Contexts & Ownership
- Context: Task Management
  - Owner: Developer (個人開発)
  - Responsibilities:
    - タスクプールの作成・編集・削除・検索・フィルタリング
    - タスクの親子関係管理と親子ステータス自動同期
    - 日次タスクキューの作成と管理（親タスクのキュー登録制限を含む）
    - タスクのステータス管理（Draft/Active/Completed/Archived）
  - Non-responsibilities:
    - 複数ユーザー管理
    - クラウド同期
    - 外部サービス連携
    - 高度なレポート機能

Integration contracts:
- HTTP: 16 operationIds defined in OpenAPI (tasks CRUD, queue management, tags management, search/filter)
- Events: N/A (イベント駆動アーキテクチャは採用せず)

---

## 4. High-level Architecture (C4 L2: Containers)
> TMS-v2はTauriベースのデスクトップアプリケーションとして、単一プロセス内でフロントエンドとバックエンドが統合される。

- Components/Containers:
  - Frontend Container (SolidJS SPA): UI/UX提供、ユーザー操作の処理
  - Backend Container (Rust Tauri Core): ビジネスロジック、データ永続化、IPC通信
  - Database Container (SQLite): データストレージ
- Data stores:
  - SQLite Database (ローカルファイルベース)
- Message broker:
  - N/A (イベント駆動は採用せず、直接関数呼び出し）

**Key flows**
- User interaction flow: ユーザー → SolidJS UI → IPC → Rust Core → SQLite → IPC → SolidJS UI → ユーザー
- Data persistence flow: SolidJS UI → IPC → Rust Core → SQLite Database

---

## 5. Non-functional Requirements (NFRs)
> デスクトップアプリケーションとして、個人使用を前提とした現実的なNFRを設定。

### 5.1 Performance
- Latency targets: UI操作に対する応答は100ms以内（p95）
- Throughput targets: ローカル操作のため制限なし
- Hot paths: タスク一覧表示、検索操作

### 5.2 Availability & Resilience
- Availability target: ローカルアプリケーションのため99.9%以上（クラッシュ耐性）
- Retry strategy: SQLite操作失敗時は自動リトライ（最大3回）
- Timeout budgets: IPC通信は5秒以内
- Circuit breaking: 不要（ローカル通信）

### 5.3 Consistency & Data Integrity
- Consistency model: Strong consistency（SQLite ACID特性を使用）
- Transaction boundaries: 各操作ごとにトランザクション
- Idempotency requirements: HTTP APIはべき等性を保証

### 5.4 Security & Privacy
- AuthN/AuthZ model: 不要（個人使用、ローカルアプリケーション）
- PII handling: 個人情報は含まれない設計
- Audit requirements: 不要（個人使用）
- Secret management: 不要（ローカルデータのみ）

### 5.5 Observability
- Logs: アプリケーション起動/終了、エラー発生時のログ出力
- Metrics: 基本的な操作カウンター（作成、更新、削除）
- Tracing: 不要（単一プロセス）
- Alerting: エラー発生時のユーザー通知

---

## 6. Deployment & Runtime
- Environments: dev（開発環境）、prod（本番/ユーザーマシン）
- Runtime platform: デスクトップOS（macOS, Windows, Linux）
- Scaling strategy: 不要（単一ユーザー、単一プロセス）
- Failure modes:
  - SQLite corruption: データベース修復機能を提供
  - IPC communication failure: アプリケーション再起動を促す

---

## 7. Data & Messaging Topology
### 7.1 Data ownership / source of truth
- Task entities → SoT: SQLite Database (tasks table)
- Tag entities → SoT: SQLite Database (tags table)
- Task-Tag relationships → SoT: SQLite Database (task_tags table)
- Task Queue → SoT: SQLite Database (task_queue table)

### 7.2 Event channels
- N/A (イベント駆動アーキテクチャは採用せず、直接関数呼び出しを使用)

---

## 8. Key Architectural Decisions (Summary)
> 詳細は decisions.md（ADR）へ。ここは要点だけ。

- AD-001: Tauri + Rust + SolidJS技術スタック採用 / Rationale: クロスプラットフォームデスクトップアプリ開発とパフォーマンス最適化のため / Trade-offs: 学習コスト vs ネイティブパフォーマンス
- AD-002: SQLite単一データベース使用 / Rationale: ローカルデータ保存のシンプルさ / Trade-offs: 同時アクセス制限 vs 容易なバックアップ
- AD-003: IPCベースの内部通信 / Rationale: セキュリティとパフォーマンスのため / Trade-offs: 複雑さ vs 型安全性
- AD-004: イベント駆動アーキテクチャ不採用 / Rationale: シンプルさと開発速度のため / Trade-offs: スケーラビリティ vs 実装容易性
- AD-005: React から SolidJS への技術変更 / Rationale: パフォーマンス最適化とバンドルサイズ削減のため / Trade-offs: エコシステム規模 vs 実行速度

---

## 9. Risks & Open Questions
- Risk: <…> / Mitigation: <…>
- Question: <…> / Owner: <…> / Due: <…>

---

## 10. Change Log
- 2025-12-21 Initial architecture definition for TMS-v2 desktop application
- 2025-12-27 Updated frontend framework from React to SolidJS for performance optimization
- 2025-12-27 親子ステータス自動同期機能の追加、検索・フィルター機能の拡張、UI改善要件の反映 (REQ-0008〜REQ-0015)
