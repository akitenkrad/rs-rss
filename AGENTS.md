# RS-RSS プロジェクト - LLM Agent 向けガイド

## プロジェクト概要

RS-RSS（rsrss）は、学術論文とWeb記事を収集・管理するRSSフィードベースのアプリケーションです。RustバックエンドとReactフロントエンドで構成され、AI機能を組み込んだモダンなアーキテクチャを採用しています。

## アーキテクチャ概要

### システム構成

- **Backend**: Rust (Axum フレームワーク)
- **Frontend**: React + JavaScript
- **Database**: PostgreSQL
- **コンテナ**: Docker Compose
- **デプロイ**: AWS (BuildSpec付き)

### レイヤー構造

```text
backend/
├── application_layer/     # アプリケーション層
│   ├── api/              # REST API サーバー
│   ├── academic_paper_crawler/  # 学術論文クローラー
│   ├── web_article_crawler/     # Web記事クローラー
│   └── commands/         # コマンドライン処理
├── middle_layer/         # ミドル層
│   └── kernel/          # ビジネスロジック・ドメインモデル
├── common_layer/         # 共通層
│   ├── registry/        # DI コンテナ
│   └── shared/          # 共有ユーティリティ
├── data_layer/          # データ層
└── adapter/             # アダプター層
    ├── database/        # データベース接続
    ├── repository/      # データアクセス
    └── migrations/      # DBマイグレーション

frontend/
├── src/
│   ├── components/      # Reactコンポーネント
│   │   ├── api/        # APIクライアント・モデル
│   │   │   └── models/ # データモデルクラス
│   │   ├── academic_paper_*/  # 学術論文関連コンポーネント
│   │   └── web_article_*/     # Web記事関連コンポーネント
│   └── sample_data/     # サンプルデータ
```

## 主要機能

### 1. 学術論文管理

- **PDF解析**: ArXiv、Semantic Scholarからの自動メタデータ取得
- **AI要約**: OpenAI APIを使用した日本語要約・分析
- **論文ノート**: 各論文に対するメモ・コメント機能
- **エージェント対話**: 論文内容に関するAI質問応答

### 2. Web記事収集

- **RSS/Atomフィード**: 技術ブログ・ニュースサイトからの自動収集
- **カテゴリ分類**: AI、IT、セキュリティ等の自動分類
- **対応サイト**: LINE、DeNA、CyberAgent、GitHub、AWS等の技術ブログ

### 3. REST API

- **エンドポイント**: `/api/v1/` 配下
- **認証**: 現在未実装（開発中）
- **レスポンス形式**: JSON + 型付きモデルクラス

## 重要なファイル・ディレクトリ

### Backend重要ファイル

```text
backend/application_layer/api/src/
├── handler/              # HTTPハンドラー
├── models/              # APIレスポンスモデル
└── route/               # ルーティング定義

backend/kernel/src/models/     # ドメインモデル
├── academic_paper.rs    # 学術論文モデル
├── paper_note.rs       # 論文ノートモデル
└── web_article.rs      # Web記事モデル
```

### Frontend重要ファイル

```text
frontend/src/components/api/
├── Api.js              # APIクライアント
└── models/             # データモデルクラス
    ├── AcademicPaper.js
    ├── PaperNote.js
    ├── WebArticle.js
    └── WebSite.js
```

## API エンドポイント一覧

### ヘルスチェック

- `GET /health/` - 基本ヘルスチェック
- `GET /health/db` - データベースヘルスチェック

### 学術論文

- `GET /api/v1/academic-paper/all` - 論文一覧（ページネーション）
- `GET /api/v1/academic-paper/paper` - 論文詳細取得
- `GET /api/v1/academic-paper/add-sse` - 論文追加（Server-Sent Events）

### 論文ノート

- `GET /api/v1/academic-paper/paper-note/select` - ノート取得
- `POST /api/v1/academic-paper/paper-note/create` - ノート作成
- `PUT /api/v1/academic-paper/paper-note/update` - ノート更新
- `DELETE /api/v1/academic-paper/paper-note/delete` - ノート削除
- `POST /api/v1/academic-paper/paper-note/ask-to-agent` - AIエージェント質問

### Web記事・サイト

- `GET /api/v1/web_site/all_web_sites` - Webサイト一覧
- `GET /api/v1/web_site/all_web_articles` - Web記事一覧

## 開発・デバッグ情報

### 環境変数（.env）

```env
DATABASE_URL=postgresql://...     # PostgreSQL接続文字列
OPENAI_API_KEY=sk-...            # OpenAI API キー
OPENAI_MODEL_ID=gpt-4            # 使用するOpenAIモデル
REACT_APP_API_BASE_URL=http://localhost:8080  # API Base URL
```

### 起動方法

```bash
# Docker Compose使用
docker-compose up

# 個別起動
cd backend && cargo run --bin server start-dashboard
cd frontend && npm start
```

### データベース

- **マイグレーション**: `backend/adapter/migrations/`
- **テーブル**: academic_papers, paper_notes, web_articles, web_sites等

## LLM Agent作業時の注意点

### 1. コード変更時

- **Backend**: Rustの型安全性を維持し、適切なエラーハンドリングを実装
- **Frontend**: APIレスポンスは必ず対応するモデルクラスでインスタンス化
- **API**: バックエンドとフロントエンドのモデル構造を同期させる

### 2. 新機能追加時

- **レイヤー構造**: 適切な層に機能を配置（ビジネスロジックはkernel層）
- **依存性注入**: registry層を通じてサービス間の依存関係を管理
- **テスト**: 各層で適切なユニット・統合テストを追加

### 3. AI機能関連

- **OpenAI API**: token制限とコスト最適化を考慮
- **プロンプト**: 日本語での学術論文分析に特化した設計
- **エラー処理**: API呼び出し失敗時の適切なフォールバック

### 4. データ処理

- **PDF解析**: メモリ効率とファイルサイズ制限を考慮
- **RSS解析**: 様々なフィード形式（RSS2.0、Atom）に対応
- **文字化け**: 日本語処理での文字エンコーディング問題に注意

## 設計図・ドキュメント

- **ER図**: `design/images/er.png`
- **クラス図**: `design/images/class_diagram_*.png`
- **API仕様**: `application_layer/api/README.md`

このプロジェクトは継続的に進化しており、新機能の追加や既存機能の改善が頻繁に行われています。変更時は常に型安全性とアーキテクチャの整合性を保つよう注意してください。
