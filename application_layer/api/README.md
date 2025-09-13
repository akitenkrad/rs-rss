# RS-RSS API サーバ

このドキュメントでは、RS-RSSシステムのAPIサーバのREST API仕様について説明します。

## ベースURL

```text
/api/v1
```

## API一覧

| エンドポイント | メソッド | 説明 | 詳細 |
|---|---|---|---|
| `/health/` | GET | 基本ヘルスチェック | [詳細](#11-基本ヘルスチェック) |
| `/health/db` | GET | データベースヘルスチェック | [詳細](#12-データベースヘルスチェック) |
| `/academic-paper/all` | GET | 学術論文一覧取得（ページネーション） | [詳細](#21-学術論文一覧取得-ページネーション) |
| `/academic-paper/paper` | GET | 学術論文詳細取得 | [詳細](#22-学術論文詳細取得) |
| `/academic-paper/add-sse` | GET | 学術論文追加（SSE） | [詳細](#23-学術論文追加-server-sent-events) |
| `/academic-paper/paper-note/select` | GET | 論文ノート取得 | [詳細](#31-論文ノート取得) |
| `/academic-paper/paper-note/create` | POST | 論文ノート作成 | [詳細](#32-論文ノート作成) |
| `/academic-paper/paper-note/update` | PUT | 論文ノート更新 | [詳細](#33-論文ノート更新) |
| `/academic-paper/paper-note/delete` | DELETE | 論文ノート削除 | [詳細](#34-論文ノート削除) |
| `/academic-paper/paper-note/ask-to-agent` | POST | エージェントへの質問 | [詳細](#35-エージェントへの質問) |
| `/web_site/all_web_sites` | GET | Webサイト一覧取得 | [詳細](#41-webサイト一覧取得) |
| `/web_site/all_web_articles` | GET | Web記事一覧取得 | [詳細](#42-web記事一覧取得) |

## 共通レスポンス形式

すべてのAPIレスポンスには以下の共通フィールドが含まれます：

- `status_code`: HTTPステータスコード (number)

## エンドポイント一覧

### 1. ヘルスチェック

#### 1.1 基本ヘルスチェック

- **エンドポイント**: `GET /api/v1/health/`
- **説明**: APIサーバの基本的な稼働状況を確認
- **パラメータ**: なし
- **レスポンス**:

  ```json
  {
    "message": "OK",
    "status_code": 200
  }
  ```

#### 1.2 データベースヘルスチェック

- **エンドポイント**: `GET /api/v1/health/db`
- **説明**: データベースの接続状況を確認
- **パラメータ**: なし
- **レスポンス**:

  ```json
  {
    "message": "OK",
    "status_code": 200
  }
  ```

  エラー時:

  ```json
  {
    "message": "Error",
    "status_code": 500
  }
  ```

### 2. 学術論文 (Academic Paper)

#### 2.1 学術論文一覧取得 (ページネーション)

- **エンドポイント**: `GET /api/v1/academic-paper/all`
- **説明**: 学術論文の一覧をページネーション形式で取得
- **クエリパラメータ**:
  - `limit` (optional): 取得件数の上限 (デフォルト: 20, 最小: 0)
  - `offset` (optional): 取得開始位置 (デフォルト: 0, 最小: 0)

- **レスポンス**:

  ```json
  {
    "total": 100,
    "limit": 20,
    "offset": 0,
    "items": [
      {
        "paper_id": "paper_id_string",
        "ss_id": "semantic_scholar_id",
        "arxiv_id": "arxiv_id",
        "doi": "10.1000/example",
        "title": "論文タイトル",
        "abstract_text": "論文概要",
        "authors": [
          {
            "author_id": "author_id_string",
            "ss_id": "semantic_scholar_author_id",
            "name": "著者名",
            "h_index": 25
          }
        ],
        "tasks": [
          {
            "task_id": "task_id_string",
            "name": "タスク名"
          }
        ],
        "primary_category": "cs.AI",
        "published_date": "2024-01-15",
        "created_at": "2024-01-15",
        "updated_at": "2024-01-15",
        "journal": {
          "journal_id": "journal_id_string",
          "name": "ジャーナル名"
        },
        "text": "論文全文",
        "url": "https://arxiv.org/pdf/example.pdf",
        "citation_count": 10,
        "reference_count": 50,
        "influential_citation_count": 5,
        "bibtex": "@article{...}",
        "summary": "論文要約",
        "background_and_purpose": "背景と目的",
        "methodology": "手法",
        "dataset": "データセット",
        "results": "結果",
        "advantages_limitations_and_future_work": "利点・制限・今後の課題"
      }
    ],
    "status_code": 200
  }
  ```

#### 2.2 学術論文詳細取得

- **エンドポイント**: `GET /api/v1/academic-paper/paper`
- **説明**: 指定したIDの学術論文の詳細情報を取得
- **クエリパラメータ**:
  - `paper_id` (required): 論文ID

- **レスポンス**: 単一の学術論文オブジェクト (上記と同じ形式)

#### 2.3 学術論文追加 (Server-Sent Events)

- **エンドポイント**: `GET /api/v1/academic-paper/add-sse`
- **説明**: 学術論文を追加し、処理状況をServer-Sent Eventsで配信
- **クエリパラメータ**:
  - `title` (required): 論文タイトル (最小1文字)
  - `pdf_url` (required): PDF URL (有効なURL形式)

- **レスポンス**: Server-Sent Events形式で処理状況を配信

### 3. 論文ノート (Paper Note)

#### 3.1 論文ノート取得

- **エンドポイント**: `GET /api/v1/academic-paper/paper-note/select`
- **説明**: 指定した論文IDに関連するノート一覧を取得
- **クエリパラメータ**:
  - `paper_id` (required): 論文ID

- **レスポンス**:

  ```json
  {
    "paper_notes": [
      {
        "paper_note_id": "note_id_string",
        "text": "ノート内容",
        "note_timestamp": "2024-01-15"
      }
    ],
    "status_code": 200
  }
  ```

#### 3.2 論文ノート作成

- **エンドポイント**: `POST /api/v1/academic-paper/paper-note/create`
- **説明**: 新しい論文ノートを作成
- **リクエストボディ**:

  ```json
  {
    "paper_id": "論文ID",
    "text": "ノート内容",
    "note_timestamp": "2024-01-15"
  }
  ```

- **レスポンス**:

  ```json
  {
    "paper_note": {
      "paper_note_id": "note_id_string",
      "text": "ノート内容",
      "note_timestamp": "2024-01-15"
    },
    "status_code": 201
  }
  ```

#### 3.3 論文ノート更新

- **エンドポイント**: `PUT /api/v1/academic-paper/paper-note/update`
- **説明**: 既存の論文ノートを更新
- **リクエストボディ**:

  ```json
  {
    "paper_note_id": "ノートID",
    "paper_id": "論文ID",
    "text": "更新されたノート内容",
    "note_timestamp": "2024-01-15"
  }
  ```

- **レスポンス**: 作成時と同じ形式 (status_code: 201)

#### 3.4 論文ノート削除

- **エンドポイント**: `DELETE /api/v1/academic-paper/paper-note/delete`
- **説明**: 指定した論文ノートを削除
- **リクエストボディ**:

  ```json
  {
    "paper_note_id": "削除するノートID"
  }
  ```

- **レスポンス**:

  ```json
  {
    "status_code": 200
  }
  ```

#### 3.5 エージェントへの質問

- **エンドポイント**: `POST /api/v1/academic-paper/paper-note/ask-to-agent`
- **説明**: 指定した論文ノートに関してエージェントに質問し、回答を取得
- **リクエストボディ**:

  ```json
  {
    "paper_note_id": "論文ノートID",
    "query": "質問内容"
  }
  ```

- **レスポンス**:

  ```json
  {
    "paper_note_id": "note_id_string",
    "text": "エージェントからの回答内容",
    "note_timestamp": "2024-01-15"
  }
  ```

### 4. Webサイト・記事 (Web Site & Article)

#### 4.1 Webサイト一覧取得

- **エンドポイント**: `GET /api/v1/web_site/all_web_sites`
- **説明**: 登録されているWebサイトの一覧をページネーション形式で取得
- **クエリパラメータ**:
  - `limit` (optional): 取得件数の上限 (デフォルト: 20, 最小: 0)
  - `offset` (optional): 取得開始位置 (デフォルト: 0, 最小: 0)

- **レスポンス**:

  ```json
  {
    "total": 50,
    "limit": 20,
    "offset": 0,
    "items": [
      {
        "siteId": "site_id_string",
        "name": "サイト名",
        "url": "https://example.com"
      }
    ],
    "status_code": 200
  }
  ```

#### 4.2 Web記事一覧取得

- **エンドポイント**: `GET /api/v1/web_site/all_web_articles`
- **説明**: 収集されたWeb記事の一覧をページネーション形式で取得
- **クエリパラメータ**:
  - `limit` (optional): 取得件数の上限 (デフォルト: 20, 最小: 0)
  - `offset` (optional): 取得開始位置 (デフォルト: 0, 最小: 0)

- **レスポンス**:

  ```json
  {
    "total": 1000,
    "limit": 20,
    "offset": 0,
    "items": [
      {
        "site_id": "site_id_string",
        "site_name": "サイト名",
        "site_url": "https://example.com",
        "article_id": "article_id_string",
        "title": "記事タイトル",
        "description": "記事説明",
        "url": "https://example.com/article/123",
        "text": "記事本文",
        "html": "記事HTML",
        "timestamp": "2024-01-15",
        "summary": "記事要約",
        "is_new_technology_related": true,
        "is_new_academic_paper_related": false,
        "is_ai_related": true,
        "is_it_related": true,
        "is_new_product_related": false,
        "is_security_related": false,
        "status_id": "status_id_string",
        "status_name": "ステータス名"
      }
    ],
    "status_code": 200
  }
  ```

## エラーレスポンス

APIエラー時は、適切なHTTPステータスコードと共にエラー情報が返されます。

### バリデーションエラー

リクエストパラメータのバリデーションに失敗した場合：

```json
{
  "error": "バリデーションエラーメッセージ",
  "status_code": 400
}
```

### 内部サーバーエラー

サーバー内部でエラーが発生した場合：

```json
{
  "error": "内部サーバーエラー",
  "status_code": 500
}
```

## 認証

現在のAPIでは認証機能は実装されていません。全てのエンドポイントは認証なしでアクセス可能です。

## データ型

- **string**: 文字列
- **number**: 数値 (整数・浮動小数点数)
- **boolean**: 真偽値
- **date**: 日付 (YYYY-MM-DD形式)

## 注意事項

1. 基本的にはRESTfulなAPIデザインに従っており、適切なHTTPメソッドを使用しています
2. 論文ノートのCRUD操作では適切なHTTPメソッド（GET、POST、PUT、DELETE）を使用しています
3. Server-Sent Events形式のエンドポイントは、リアルタイムで処理状況を確認できます
4. ページネーション機能を持つエンドポイントでは、適切なlimitとoffsetを指定してください
5. エージェントへの質問機能（ask-to-agent）では、指定した論文ノートに関する質問に対してAIエージェントが回答を生成します
6. 認証機能は現在実装されていません
