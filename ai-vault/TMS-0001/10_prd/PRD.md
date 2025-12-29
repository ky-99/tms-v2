# PRD: Task Management System v2 (TMS-v2)

> Confidentiality: Internal
> Repo: tms-v2
> Ticket: TMS-0001
> Branch (if any): feature/tms-v2-poc
> Owner: Developer
> Reviewers: -
> Created: 2024-12-21
> Last Updated: 2024-12-21

---

## 1. Summary
- **One-liner**: 個人用のタスクマネジメントアプリケーション。タスクプールからのタスク選択と日次タスクキューの作成により、効率的なタスク管理を実現する。
- **Why now**: 既存のTMSの改善版として、Tauri + Rustの新しい技術スタックでの実装を開始し、クロスプラットフォーム対応とパフォーマンス向上を目指すため。
- **Scope size**: Small（POCとして最小限の機能を先行実装、今後段階的に拡張）

---

## 2. Background / Context
### 2.1 Current State
- 既存のTMS（v1）が稼働しており、基本的なタスク管理機能を提供している
- ユーザーは手動でタスクを管理しており、タスクプールからの選択と日次キューの作成が非効率
- 技術スタックは古く、クロスプラットフォーム対応が不十分

### 2.2 Problem Statement
- タスクの親子関係を視覚的に管理する手段がなく、タスク間の依存関係が不明瞭になる
- 日次タスクの計画立案が手動作業に頼っており、効率が悪い
- 技術スタックの陳腐化により、保守性と拡張性が低下している

### 2.3 Users / Personas
- Primary: 個人ユーザー（開発者自身）
- Secondary: 今後拡張予定の複数ユーザー
- Non-user stakeholders: なし（個人開発プロジェクト）

---

## 3. Goals
### 3.1 Product Goals
- G1: タスクプールから簡単にタスクを選択し、日次タスクキューを作成できるアプリケーションを提供する
- G2: タスクの親子関係を視覚的に管理できるUIを提供する
- G3: Tauri + Rust + SQLite + Reactの技術スタックでクロスプラットフォーム対応を実現する

### 3.2 Success Metrics (Measurable)
- Metric: 環境構築完了率 / Baseline: 0% / Target: 100% / Window: 1日
- Metric: 基本機能（タスクCRUD、キュー管理）実装完了 / Baseline: 未実装 / Target: 完了 / Window: 1週間
- Metric: アプリケーション起動・基本操作のパフォーマンス / Baseline: - / Target: 2秒以内 / Window: 継続

---

## 4. Non-Goals (Explicitly Out of Scope)
- NG1: 複数ユーザー対応（今回は単一ユーザー専用）
- NG2: クラウド同期機能（ローカル専用）
- NG3: 高度なレポート・分析機能（基本的な統計のみ）
- NG4: 外部サービス連携（API連携など）

---

## 5. Scope / Requirements Overview (High-level)
> ここは「要求の要約」まで。詳細な要求とAcceptanceは **requirements.md** に置く。

### 5.1 In Scope (Bullets)
- タスクプールの管理（CRUD操作、親子関係の表示）
- 日次タスクキューの作成と管理
- タスク検索・フィルタリング機能
- ローカル環境構築手順の提供
- Tauri + Rust + SQLite + React技術スタックの採用

### 5.2 User Journey (Optional but recommended)
1. ユーザーがアプリケーションを起動する
2. タスクプール画面でタスクを閲覧・検索・編集する
3. 必要なタスクをキューに追加する
4. タスクキュー画面で当日のタスクを確認・実行する
5. タスク完了時にステータスを更新する

### 5.3 Edge Cases / Exceptions (Overview)
- タスクの循環参照防止
- キューへの重複タスク追加防止
- データベース接続エラー時の処理

---

## 6. Constraints / Assumptions
### 6.1 Constraints (Must comply)
- Business: 個人開発プロジェクトのため、納期・運用制約なし
- Technical: Tauri + Rust + SQLite + Reactの技術スタックを使用
- Security/Compliance: 個人情報は含まれないため、セキュリティ要件は最小限

### 6.2 Assumptions (If false, re-scope)
- A1: ユーザーは開発環境（Node.js, Rust, Tauri CLI）を自身で準備可能
- A2: SQLiteがローカル環境で使用可能
- A3: 単一ユーザーでの使用を前提とし、同時アクセス制御は不要

---

## 7. Dependencies
- Upstream: Node.js, Rust, Tauri CLI, SQLite
- Downstream: なし（ローカルアプリケーション）
- Data: タスクデータ（タイトル、説明、親子関係、ステータスなど）のローカルSQLite保存

---

## 8. Risks / Open Questions
### 8.1 Risks
- R1: Tauri + Rustの学習コスト / Mitigation: POC段階で基本機能のみ実装し、段階的に習得
- R2: 技術スタックの互換性問題 / Mitigation: 公式ドキュメントとコミュニティを活用
- R3: パフォーマンス問題 / Mitigation: 基本的なCRUD操作から開始し、必要に応じて最適化

### 8.2 Open Questions (Decision Pending)
- Q1: UIフレームワークの選択（Reactのみ or 追加ライブラリ） / Owner: Developer / Due: 2024-12-22
- Q2: データベーススキーマの詳細設計 / Owner: Developer / Due: 2024-12-22

---

## 9. Rollout / Release Plan (High-level)
- Release strategy: ローカル開発環境での直接使用（big bang）
- Migration: 既存TMS v1からのデータ移行は別途検討
- Backward compatibility: なし（新規アプリケーション）
- Rollback plan: アプリケーションのアンインストール

---

## 10. Observability / Operations (High-level)
- Logging: エラー発生時のログ出力（個人情報は含まない）
- Metrics: アプリケーション起動時間、タスクCRUD操作の実行時間
- Alerts: データベース接続エラー時のアラート表示
- Support playbook: エラーログを確認し、必要に応じて再起動

---

## 11. References
- requirements.md: `10_prd/requirements.md`
- domain.md: `20_domain/domain.md`
- glossary.md: `20_domain/glossary.md`
- openapi.yaml: `30_contract/openapi.yaml`
- asyncapi.yaml: `30_contract/asyncapi.yaml`
- architecture.md: `40_design/architecture.md`
- design.md: `40_design/design.md`
- decisions.md: `40_design/decisions.md`
- tasks.md: `40_design/tasks.md`
- traceability.md: `90_review/traceability.md`
- context_bundle.md: `90_review/context_bundle.md`

---

## 12. Change Log
- 2024-12-21 初期PRD作成 (TMS-v2のPOC開発開始のため)
