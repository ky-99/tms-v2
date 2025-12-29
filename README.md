# TMS-v2 (Task Management System v2)

個人用タスク管理デスクトップアプリケーション

## 技術スタック

- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust + Tauri
- **Database**: SQLite
- **OS Support**: macOS, Windows, Linux

## 開発環境セットアップ

### 前提条件

#### 必須ソフトウェア
- **Node.js**: 18.x LTS以上
- **Rust**: 1.88以上
- **Tauri CLI**: 2.x

#### 推奨IDE
- **Visual Studio Code** + Tauri拡張機能 + rust-analyzer拡張機能

### 環境構築手順

#### 1. Node.jsのインストール

```bash
# Node.js 18.xをインストール
# https://nodejs.org/ からダウンロードしてインストール

# バージョン確認
node --version  # v18.x.x 以上
npm --version   # 9.x.x 以上
```

#### 2. Rustのインストール

```bash
# rustupをインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 環境変数を反映（またはターミナル再起動）
source ~/.cargo/env

# バージョン確認
rustc --version  # 1.88.x 以上
cargo --version  # 1.88.x 以上
```

#### 3. Tauri CLIのインストール

```bash
# Tauri CLIをインストール
npm install -g @tauri-apps/cli

# バージョン確認
tauri --version  # 2.x.x
```

#### 4. macOS固有の設定（Apple Silicon Macの場合）

```bash
# Apple Silicon Macの場合、追加のターゲットをインストール
rustup target add aarch64-apple-darwin
```

#### 5. 依存関係のインストール

```bash
# プロジェクトルートで実行
npm install

# Rust依存関係の確認
cd src-tauri
cargo check
cd ..
```

### 開発ワークフロー

#### 開発サーバーの起動

```bash
# 開発モードで起動（ホットリロード対応）
npm run tauri dev
```

#### ビルド

```bash
# デバッグビルド
npm run tauri build

# リリースビルド
npm run tauri build --release
```

#### プロジェクト構造

```
tms-v2/
├── src/                    # Reactフロントエンド
│   ├── App.tsx            # メインコンポーネント
│   ├── index.tsx          # エントリーポイント
│   └── App.css            # スタイル
├── src-tauri/             # Rustバックエンド
│   ├── src/
│   │   └── main.rs        # Rustエントリーポイント
│   ├── Cargo.toml         # Rust依存関係
│   └── tauri.conf.json    # Tauri設定
├── package.json           # Node.js設定
├── tsconfig.json          # TypeScript設定
├── vite.config.ts         # Vite設定
└── index.html             # HTMLテンプレート
```

### トラブルシューティング

#### Tauri開発サーバーが起動しない場合

```bash
# ポート1420が使用されていないか確認
lsof -i :1420

# Node.jsプロセスを終了
pkill -f node

# 再度試行
npm run tauri dev
```

#### Rustコンパイルエラーが発生する場合

```bash
# Cargoキャッシュのクリア
cd src-tauri
cargo clean
cargo build
```

#### macOSでGUIが表示されない場合

```bash
# XQuartzがインストールされているか確認
brew install xquartz
open -a XQuartz

# DISPLAY環境変数を確認
echo $DISPLAY
```

### 次のステップ

1. **TASK-0001完了確認**: 上記の手順で開発環境が動作することを確認
2. **TASK-0002開始**: SQLiteデータベーススキーマ実装
3. **ビジネスロジック実装**: タスクCRUD機能など

### 関連ドキュメント

- [要件定義](../ai-vault/tms-v2/TMS-0001/10_prd/requirements.md)
- [アーキテクチャ設計](../ai-vault/tms-v2/TMS-0001/40_design/architecture.md)
- [API仕様](../ai-vault/tms-v2/TMS-0001/30_contract/openapi.yaml)
# tms-v2
