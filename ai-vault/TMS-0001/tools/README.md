# Feature Template Tools

このディレクトリには、feature-templateの運用を自動化するスクリプト群が含まれています。

## 📋 スクリプト一覧

### 🎯 gen_context.sh（NEW - Phase別コンテキスト自動生成）
**目的**: Phase に応じた context_bundle.md の標準的な内容を自動生成

**更新対象**:
- Section 1: Objective（Phase別の目標と DoD）
- Section 8.4: Manual Instructions（Phase別の作業手順）

**使用方法**:
```bash
# Contractフェーズ用のコンテキストを自動生成
./tools/gen_context.sh --phase Contract

# 機能タイトル指定（オプション）
./tools/gen_context.sh --phase Requirements --feature-title "Password Reset"
```

**Phase別の自動生成内容**:
- **Requirements**: PRD/requirements.md作成の手順
- **Domain**: glossary/domain.md作成の手順
- **Contract**: openapi/asyncapi.yaml作成の手順と Pre-flight Checks
- **Design**: architecture/design/tasks.md作成の手順
- **Implementation**: コード実装とタスク進捗管理の手順
- **Review**: 最終検証とPR準備の手順

---

### 🔄 gen_bundle.sh（主要スクリプト）
**目的**: context_bundle.mdのAUTOブロックを自動更新

**更新対象**:
- Section 6: Requirements Snapshot（requirements.mdから抽出）
- Section 7: Contract Snapshot（openapi.yaml/asyncapi.yamlから抽出）
- Section 8: Tasking Snapshot（tasks.mdから抽出）
- Section 9: Pre-flight Checks（自動検証）

**使用方法**:
```bash
# 基本実行
./tools/gen_bundle.sh

# Phaseを指定
./tools/gen_bundle.sh --phase "Contract"

# 最大REQ数を指定
./tools/gen_bundle.sh --max-req 5

# 特定のREQsを強制的にFocusにする
./tools/gen_bundle.sh --req-ids REQ-0001,REQ-0003,REQ-0007
```

**オプション**:
- `--bundle <path>`: context_bundle.mdのパス（デフォルト: `90_review/context_bundle.md`）
- `--requirements <path>`: requirements.mdのパス（デフォルト: `10_prd/requirements.md`）
- `--openapi <path>`: openapi.yamlのパス（デフォルト: `30_contract/openapi.yaml`）
- `--asyncapi <path>`: asyncapi.yamlのパス（デフォルト: `30_contract/asyncapi.yaml`）
- `--tasks <path>`: tasks.mdのパス（デフォルト: `40_design/tasks.md`）
- `--max-req <n>`: Focus REQsの最大数（デフォルト: 10）
- `--phase <name>`: 現在のPhase名（context_bundle.mdのPhase行を更新）
- `--req-ids <ids>`: カンマ区切りのREQ-IDs（自動選択を上書き）

---

### 🔍 gen_traceability.sh
**目的**: traceability.mdを自動生成

**生成内容**:
- REQ → operationId/messageId → Component → TASK のマッピング
- Contract Drift Checks（マッピング漏れ検出）
- Coverage Summary

**使用方法**:
```bash
# 基本実行
./tools/gen_traceability.sh

# カスタムパス指定
./tools/gen_traceability.sh \
  --traceability 90_review/traceability.md \
  --requirements 10_prd/requirements.md \
  --openapi 30_contract/openapi.yaml \
  --asyncapi 30_contract/asyncapi.yaml \
  --tasks 40_design/tasks.md
```

---

### ✅ validate_phase.sh
**目的**: 各PhaseのDefinition of Done（DoD）を検証

**検証内容**（Phase別）:
- **Requirements**: PRD.md、requirements.md存在、Given/When/Then形式
- **Domain**: glossary.md、domain.md存在
- **Contract**: openapi.yaml、asyncapi.yaml存在、x-requirements設定
- **Design**: architecture.md、design.md、tasks.md存在
- **Implementation**: tasks.mdにDoneまたはProcessingのタスクがある
- **Review**: traceability.md存在、Pre-flight Checks合格

**使用方法**:
```bash
# Contractフェーズを検証
./tools/validate_phase.sh Contract

# Reviewフェーズを検証
./tools/validate_phase.sh Review
```

**終了コード**:
- `0`: DoD満足（次のPhaseへ進んでOK）
- `1`: DoD未満足（修正が必要）

---

### 🚀 gen_all.sh（統合スクリプト）
**目的**: 全自動化スクリプトを一括実行

**実行内容**:
0. Phase別コンテキスト更新（gen_context.sh）← NEW
1. context_bundle.md更新（gen_bundle.sh）
2. Phase DoD検証（validate_phase.sh）
3. traceability.md生成（gen_traceability.sh）※Review/Implementationフェーズのみ
4. サマリー表示と次のステップ提示

**使用方法**:
```bash
# Contractフェーズの全自動更新
./tools/gen_all.sh Contract

# Reviewフェーズの全自動更新（traceability含む）
./tools/gen_all.sh Review

# デフォルト（Contract）
./tools/gen_all.sh
```

---

## 🔄 典型的なワークフロー

### Phase 1: Requirements
```bash
# PRD.md と requirements.md を作成（手動 or AI）
# ...

# 自動更新
./tools/gen_all.sh Requirements

# 検証
./tools/validate_phase.sh Requirements
```

### Phase 2: Domain
```bash
# glossary.md と domain.md を作成（手動 or AI）
# ...

# 自動更新
./tools/gen_all.sh Domain
```

### Phase 3: Contract
```bash
# openapi.yaml と asyncapi.yaml を作成（手動 or AI）
# 全operationId/messageIdにx-requirementsを設定
# ...

# 自動更新（重要！）
./tools/gen_all.sh Contract

# context_bundle.mdのSection 7とSection 9を確認
# - Section 7: Contract Snapshot（operationId/messageId一覧）
# - Section 9: Pre-flight Checks（x-requirements漏れチェック）
```

### Phase 4: Design
```bash
# architecture.md, design.md, tasks.md を作成（手動 or AI）
# ...

# 自動更新
./tools/gen_all.sh Design

# context_bundle.mdのSection 8を確認
# - Section 8: Tasking Snapshot（TASKの進捗）
```

### Phase 5: Implementation
```bash
# コード実装
# tasks.mdのStatusを更新（UnDone → Processing → Done）
# ...

# 自動更新（TASKの進捗反映）
./tools/gen_all.sh Implementation

# traceability.mdが自動生成される
```

### Phase 6: Review
```bash
# 最終チェック
./tools/gen_all.sh Review

# Pre-flight Checks確認
cat 90_review/context_bundle.md | grep -A 20 "Pre-flight Checks"

# 全てOKならPR作成
```

---

## 🛠️ セットアップ

### 前提条件
- Python 3.9+
- PyYAML

### Python仮想環境のセットアップ
```bash
# 仮想環境作成
python3 -m venv .venv

# 有効化
source .venv/bin/activate  # macOS/Linux
# または
.venv\Scripts\activate  # Windows

# PyYAMLインストール
pip install pyyaml
```

### スクリプトに実行権限を付与
```bash
chmod +x tools/*.sh
```

---

## 📝 トラブルシューティング

### PyYAML not found
```bash
source .venv/bin/activate
pip install pyyaml
```

### AUTO block not found
→ context_bundle.mdに対応するAUTOブロックがない可能性があります。
最新のテンプレートを使用しているか確認してください。

### Permission denied
```bash
chmod +x tools/gen_bundle.sh
chmod +x tools/gen_traceability.sh
chmod +x tools/validate_phase.sh
chmod +x tools/gen_all.sh
```

---

## 🎯 ベストプラクティス

1. **各Phaseの最後に必ずgen_all.shを実行**
   - 自動更新漏れを防ぐ
   - Pre-flight Checksで問題を早期発見

2. **契約変更後は必ずgen_bundle.sh実行**
   - openapi.yaml/asyncapi.yaml更新後
   - Contract Snapshotを最新化

3. **TASK完了時にStatusを更新してgen_all.sh実行**
   - Tasking Snapshotに進捗を反映
   - traceability.mdも自動更新

4. **Reviewフェーズ前にvalidate_phase.shで全Phase検証**
   ```bash
   ./tools/validate_phase.sh Requirements
   ./tools/validate_phase.sh Domain
   ./tools/validate_phase.sh Contract
   ./tools/validate_phase.sh Design
   ./tools/validate_phase.sh Implementation
   ```

5. **バックアップファイルは定期的にクリーンアップ**
   ```bash
   # 7日以上前のバックアップを削除
   find . -name "*.bak.*" -mtime +7 -delete
   ```

---

## 🚧 今後の拡張予定

- [ ] CI/CD統合（GitHub Actions）
- [ ] quality_check.sh（コード品質チェック）
- [ ] gen_manifest.sh（manifest.json自動更新）
- [ ] Web UI（ダッシュボード）

---

## 📚 関連ドキュメント

- [context_bundle.md](../90_review/context_bundle.md) - 中央制御ドキュメント
- [traceability.md](../90_review/traceability.md) - トレーサビリティマトリクス
- [requirements.md](../10_prd/requirements.md) - 要件定義
- [openapi.yaml](../30_contract/openapi.yaml) - HTTP API仕様
- [asyncapi.yaml](../30_contract/asyncapi.yaml) - イベント仕様
- [tasks.md](../40_design/tasks.md) - 実装タスク

