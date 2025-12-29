# Architectural Decision Records (ADR): Task Management System v2 (TMS-v2)

> Confidentiality: Internal
> Repo: tms-v2
> Ticket: TMS-0001
> Branch: feature/tms-v2-poc
> Owner: Developer
> Created: 2025-12-21
> Last Updated: 2025-12-21

---

## 0. Purpose
- Document significant architectural and design decisions with rationale, alternatives considered, and trade-offs.
- Provide historical context for future team members.
- Enable informed decision reversal if context changes.

References:
- PRD: `10_prd/PRD.md`
- Requirements: `10_prd/requirements.md`
- Domain: `20_domain/domain.md`
- Architecture: `40_design/architecture.md`
- Design: `40_design/design.md`

---

## 1. ADR Template (Use for each decision)

### ADR-NNNN: <Decision Title>
- **Status**: <Proposed / Accepted / Deprecated / Superseded>
- **Date**: <YYYY-MM-DD>
- **Deciders**: <name1, name2>
- **Related to**: <REQ-xxxx / operationId / Component>

#### Context
<現状と背景。なぜこの決定が必要か。>

#### Decision
<何を決めたか。明確に1-3行で。>

#### Rationale
<なぜこの決定を選んだか。>

#### Alternatives Considered
1. **Alternative A**: <内容>
   - Pros: <利点>
   - Cons: <欠点>
   - Why not: <却下理由>

2. **Alternative B**: <内容>
   - Pros: <利点>
   - Cons: <欠点>
   - Why not: <却下理由>

#### Consequences
- **Positive**:
  - <良い影響>
- **Negative**:
  - <悪い影響・トレードオフ>
- **Neutral**:
  - <中立的な影響>

#### Implementation Notes (Optional)
- <実装上の注意点>

#### Related Decisions
- Supersedes: <ADR-xxxx>
- Related: <ADR-xxxx>
- Blocked by: <ADR-xxxx>

---

## 2. Active Decisions

### ADR-0001: Tauri + Rust + SolidJS + SQLite技術スタック採用
- **Status**: Accepted
- **Date**: 2025-12-21
- **Deciders**: Developer
- **Related to**: All REQs, System Architecture

#### Context
TMS-v2は個人用のデスクトップアプリケーションとして開発する必要があり、クロスプラットフォーム対応（macOS, Windows, Linux）とネイティブパフォーマンスが求められる。また、既存TMSからの移行プロジェクトであり、技術スタックの刷新が目的。

#### Decision
Tauri 2.x + Rust 1.70+ + SolidJS + SQLiteの技術スタックを採用する。

#### Rationale
- クロスプラットフォームデスクトップアプリケーション開発が可能
- Rustによるネイティブパフォーマンスとメモリ安全性
- SolidJSによる高速なリアクティブUI開発体験
- SQLiteによる軽量で信頼性の高いローカルデータ保存

#### Alternatives Considered
1. **Electron + Node.js + SQLite**
   - Pros: JavaScriptエコシステムの豊富さ、開発者コミュニティの大きさ
   - Cons: メモリ消費量が多い、起動時間が長い
   - Why not: パフォーマンスとリソース消費が個人ユースに過剰

2. **Qt + C++ + SQLite**
   - Pros: 最高のパフォーマンス、ネイティブUI
   - Cons: 学習コストが高い、開発生産性が低い
   - Why not: 開発速度と保守性の観点から不適

3. **Flutter Desktop + Dart + SQLite**
   - Pros: クロスプラットフォーム、単一コードベース
   - Cons: 成熟度が低い、デスクトップサポートが限定的
   - Why not: Tauriの方がデスクトップアプリケーションとして成熟

#### Consequences
- **Positive**:
  - クロスプラットフォーム対応が可能
  - Rustによる型安全性とパフォーマンス
  - SolidJSによる高性能なUI開発
- **Negative**:
  - Rust学習コスト
  - Tauriの比較的若いエコシステム
  - デバッグの複雑さ
- **Neutral**:
  - ビルドサイズが最適化される

#### Implementation Notes
- Tauri 2.xの安定版を使用
- Rust 1.70+でasync/awaitを活用
- SolidJS + TypeScript + Kobalte + Tailwind CSS

---

### ADR-0002: ローカルSQLiteデータベース単一採用
- **Status**: Accepted
- **Date**: 2025-12-21
- **Deciders**: Developer
- **Related to**: REQ-0002, REQ-0003, REQ-0006

#### Context
個人用アプリケーションのため、クラウド同期は不要。データの永続性とACID特性が求められ、複雑なクエリ（階層構造、検索、フィルタリング）に対応する必要がある。

#### Decision
SQLiteを単一データベースとして採用し、全データをローカルに保存する。

#### Rationale
- ACID特性によるデータ整合性保証
- SQLによる複雑なクエリ対応（JOIN, 再帰クエリ等）
- ファイルベースでバックアップが容易
- 追加のインフラ不要

#### Alternatives Considered
1. **JSON/ファイルベース永続化**
   - Pros: シンプル、実装容易
   - Cons: 同時アクセス制御が難しい、複雑クエリが非効率
   - Why not: 階層構造と検索要件に対応できない

2. **IndexedDB**
   - Pros: ブラウザ標準、JavaScriptネイティブ
   - Cons: SQL機能が限定的、バックアップが複雑
   - Why not: SQLiteの方がリレーショナルデータに適する

3. **埋め込みKey-Valueストア (sled, rocksdb)**
   - Pros: 高パフォーマンス
   - Cons: リレーショナルクエリが難しい
   - Why not: タスクの階層関係表現が複雑になる

#### Consequences
- **Positive**:
  - データ整合性の保証
  - 複雑なクエリが可能
  - バックアップ・移行が容易
- **Negative**:
  - SQLiteの学習・運用コスト
  - 同時書き込みの制限
- **Neutral**:
  - ファイルサイズが比較的小さい

---

### ADR-0003: イベント駆動アーキテクチャ不採用
- **Status**: Accepted
- **Date**: 2025-12-21
- **Deciders**: Developer
- **Related to**: System Architecture

#### Context
TMS-v2は個人用アプリケーションであり、マイクロサービス間通信や非同期処理の要件がない。UI操作に対する即時応答が求められる。

#### Decision
イベント駆動アーキテクチャを採用せず、直接関数呼び出しベースの同期処理とする。

#### Rationale
- シンプルなアーキテクチャで開発速度向上
- デバッグの容易さ
- パフォーマンスの最適化（IPC通信のみ）
- 個人ユースに適した複雑さ

#### Alternatives Considered
1. **内部イベントバス採用**
   - Pros: 疎結合、拡張性
   - Cons: 複雑さの増加、デバッグ難易度上昇
   - Why not: 単一プロセスアプリケーションでは過剰

2. **メッセージキュー導入**
   - Pros: 非同期処理、堅牢性
   - Cons: インフラコスト、複雑さ
   - Why not: 同期処理で十分

#### Consequences
- **Positive**:
  - 実装のシンプルさ
  - 開発速度の向上
  - デバッグの容易さ
- **Negative**:
  - スケーラビリティの制限
  - 非同期処理の機会損失
- **Neutral**:
  - IPC通信のみで十分

---

### ADR-0004: RESTful API over IPC採用
- **Status**: Accepted
- **Date**: 2025-12-21
- **Deciders**: Developer
- **Related to**: OpenAPI Contract, REQ-0002, REQ-0006

#### Context
フロントエンド（SolidJS）とバックエンド（Rust）の通信方式を決定する必要がある。OpenAPIで定義されたHTTP APIをIPC経由で実現する。

#### Decision
HTTP REST APIをIPCプロトコルで実装し、フロントエンドから直接HTTPクライアント経由で呼び出し可能とする。

#### Rationale
- OpenAPI定義の再利用
- フロントエンド開発者の馴染みやすさ
- 標準的なエラーハンドリング
- テスト容易性

#### Alternatives Considered
1. **直接関数呼び出し**
   - Pros: シンプル、型安全性
   - Cons: OpenAPI定義の活用不可、テストしにくい
   - Why not: 契約の再利用性が低い

2. **GraphQL over IPC**
   - Pros: 柔軟なクエリ、スキーマ駆動
   - Cons: 追加の複雑さ、学習コスト
   - Why not: REST APIで十分

#### Consequences
- **Positive**:
  - OpenAPI定義の活用
  - 標準的なHTTPセマンティクス
  - テストツールの利用
- **Negative**:
  - IPCレイヤーの追加複雑さ
  - HTTPオーバーヘッド
- **Neutral**:
  - 開発生産性の向上

---

### ADR-0005: React から SolidJS への技術変更
- **Status**: Accepted
- **Date**: 2025-12-27
- **Deciders**: Developer
- **Related to**: ADR-0001, Frontend Architecture, REQ-0004, REQ-0007

#### Context
当初ADR-0001でReactをフロントエンドフレームワークとして採用したが、実装段階でデスクトップアプリケーションのパフォーマンス要件を再検討した結果、よりパフォーマンスの高いSolidJSへの変更を決定。

#### Decision
フロントエンドフレームワークをReactからSolidJSに変更する。

#### Rationale
- **パフォーマンス**: SolidJSは仮想DOMを使用せず、直接DOMを更新するため、Reactと比較して高速
- **リアクティビティ**: よりシンプルかつ効率的なリアクティビティモデル
- **バンドルサイズ**: React + Redux/Zustandより小さいバンドルサイズ
- **デスクトップアプリ最適化**: ネイティブアプリケーションのパフォーマンス最適化に適している
- **学習曲線**: Reactライクな構文で学習コストが低い

#### Alternatives Considered
1. **Reactを継続使用**
   - Pros: エコシステムが大きい、コミュニティが活発
   - Cons: 仮想DOMのオーバーヘッド、状態管理ライブラリが必要
   - Why not: パフォーマンス要件を満たすためにはより軽量なフレームワークが適切

2. **Svelte**
   - Pros: コンパイル時最適化、小さいバンドルサイズ
   - Cons: SolidJSより成熟度が低い、TypeScript統合
   - Why not: SolidJSの方がリアクティビティモデルが洗練されている

#### Consequences
- **Positive**:
  - 高速なUI更新とレンダリング
  - 組み込みStore機能により外部状態管理ライブラリ不要
  - 小さいバンドルサイズによる高速な起動時間
  - Reactライクな構文で移行コストが低い
- **Negative**:
  - Reactと比較してエコシステムが小さい
  - コミュニティリソースが限定的
- **Neutral**:
  - Kobalte（Headless UI）によるアクセシブルなコンポーネント
  - Tailwind CSSとの統合

#### Implementation Notes
- SolidJS + TypeScript
- 状態管理: SolidJS Store（createStore）
- UIコンポーネント: Kobalte（Headless UI）
- スタイリング: Tailwind CSS v3
- ディレクトリ構成: src/api/, src/types/, src/stores/, src/components/, src/pages/

#### Related Decisions
- Supersedes: ADR-0001（部分的に更新）
- Related: ADR-0004（IPC通信設計）

---

## 3. Deprecated / Superseded Decisions
> 削除せずにここに移動し、理由を明記する。

### ADR-0000: <Title> (DEPRECATED)
- **Deprecated on**: <YYYY-MM-DD>
- **Reason**: <なぜ廃止されたか>
- **Superseded by**: <ADR-xxxx or N/A>
- **Original decision**: <簡潔に>

---

## 4. Decision Summary Table (Quick reference)
| ADR-ID | Title | Status | Date | Impact | Related REQs |
|---|---|---|---|---|---|
| ADR-0001 | Tauri + Rust + SolidJS + SQLite技術スタック採用 | Accepted | 2025-12-21 | High | All |
| ADR-0002 | ローカルSQLiteデータベース単一採用 | Accepted | 2025-12-21 | High | REQ-0002, REQ-0003, REQ-0006 |
| ADR-0003 | イベント駆動アーキテクチャ不採用 | Accepted | 2025-12-21 | Medium | System Architecture |
| ADR-0004 | RESTful API over IPC採用 | Accepted | 2025-12-21 | Medium | REQ-0002, REQ-0006 |
| ADR-0005 | React から SolidJS への技術変更 | Accepted | 2025-12-27 | High | REQ-0004, REQ-0007 |

---

## 5. Decision Categories (Optional grouping)
### Technology Choices
- ADR-xxxx: ...

### Data & Persistence
- ADR-xxxx: ...

### Security & Privacy
- ADR-xxxx: ...

### Performance & Scalability
- ADR-xxxx: ...

### Integration & APIs
- ADR-xxxx: ...

---

## 6. Change Log
- 2025-12-21 Initial ADRs for TMS-v2 architecture decisions
- 2025-12-27 Added ADR-0005 for React to SolidJS technology change, updated ADR-0001

