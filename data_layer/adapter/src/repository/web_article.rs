use crate::database::{
    models::web_article::{PaginatedWebSiteRecord, WebArticleRecord, WebSiteRecord},
    ConnectionPool,
};
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    models::{
        list::PaginatedList,
        web_article::{WebArticle, WebArticleListOptions, WebSite, WebSiteListOptions},
    },
    repository::web_article::{WebArticleRepository, WebSiteRepository},
};
use shared::{
    errors::{AppError, AppResult},
    id::{WebArticleId, WebSiteId},
};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, new)]
pub struct WebSiteRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl WebSiteRepository for WebSiteRepositoryImpl {
    async fn create_web_site(&self, web_site: WebSite) -> AppResult<WebSite> {
        let res = sqlx::query!(
            r#"INSERT INTO web_site (
                name,
                url
            ) VALUES ($1, $2)
            RETURNING site_id"#,
            web_site.name,
            web_site.url
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        Ok(WebSite::new(WebSiteId::from(res.site_id), web_site.name, web_site.url))
    }
    async fn select_web_site_by_id(&self, id: &str) -> AppResult<WebSite> {
        let rows = sqlx::query_as!(
            WebSiteRecord,
            r#"SELECT
                site_id,
                name,
                url
            FROM 
                web_site
            WHERE site_id = $1"#,
            Uuid::from_str(id).unwrap()
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebSite::from(row.clone()))
        }
    }
    async fn select_web_site_by_name(&self, name: &str) -> AppResult<WebSite> {
        let rows = sqlx::query_as!(
            WebSiteRecord,
            r#"SELECT
                site_id,
                name,
                url
            FROM 
                web_site
            WHERE name = $1"#,
            name
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebSite::from(row.clone()))
        }
    }
    async fn select_or_create_web_site(&self, name: &str, url: &str) -> AppResult<WebSite> {
        match self.select_web_site_by_name(name).await {
            Ok(web_site) => Ok(web_site),
            Err(_) => {
                let web_site = WebSite::new(WebSiteId::new(), name.to_string(), url.to_string());
                self.create_web_site(web_site).await
            }
        }
    }
    async fn select_all_web_sites_paginated(&self, options: WebSiteListOptions) -> AppResult<PaginatedList<WebSite>> {
        let WebSiteListOptions { limit, offset } = options;
        let rows = sqlx::query_as!(
            PaginatedWebSiteRecord,
            r#"
            SELECT
                COUNT(*) OVER() AS "total!",
                ws.site_id as site_id,
                ws.name as name,
                ws.url as url
            FROM 
                web_site as ws
            ORDER BY ws.created_at DESC
            LIMIT $1
            OFFSET $2"#,
            limit,
            offset
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        let total = rows.first().map_or(0, |row| row.total);
        let site_ids = rows.into_iter().map(|row| row.site_id).collect::<Vec<WebSiteId>>();

        let items = sqlx::query_as!(
            WebSiteRecord,
            r#"SELECT
                site_id,
                name,
                url
            FROM 
                web_site
            WHERE site_id = ANY($1::uuid[])"#,
            &site_ids as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        let items = items.into_iter().map(WebSite::from).collect::<Vec<WebSite>>();

        Ok(PaginatedList::new(total, limit, offset, items))
    }
    async fn update_web_site(&self, web_site: WebSite) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE web_site SET name = $1, url = $2 WHERE site_id = $3"#,
            web_site.name,
            web_site.url,
            Uuid::from(web_site.site_id)
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        Ok(())
    }
    async fn delete_web_site(&self, id: &str) -> AppResult<()> {
        sqlx::query!(
            r#"DELETE FROM web_site WHERE site_id = $1"#,
            Uuid::from_str(id).unwrap()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct WebArticleRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl WebArticleRepository for WebArticleRepositoryImpl {
    async fn create_web_article(&self, web_article: &mut WebArticle) -> AppResult<WebArticle> {
        // Check if the article already exists
        let existing_article = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                ws.site_id AS site_id,
                ws.name AS site_name,
                ws.url AS site_url,
                wa.article_id,
                wa.title,
                wa.description,
                wa.url,
                wa.timestamp,
                wa.text,
                wa.html,
                wa.summary,
                wa.is_new_technology_related,
                wa.is_new_product_related,
                wa.is_new_academic_paper_related,
                wa.is_ai_related,
                wa.is_security_related,
                wa.is_it_related
            FROM web_article AS wa
            JOIN web_site AS ws ON wa.site_id = ws.site_id
            WHERE wa.url = $1"#,
            web_article.url
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        if !existing_article.is_empty() {
            return Ok(existing_article[0].clone().into());
        }

        // Check if the site exists
        let site = sqlx::query_as!(
            WebSiteRecord,
            r#"SELECT site_id, name, url FROM web_site WHERE name = $1"#,
            web_article.site.name
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        if site.is_empty() {
            let res = sqlx::query!(
                r#"INSERT INTO web_site (site_id, name, url) VALUES ($1, $2, $3) RETURNING site_id"#,
                Uuid::from(web_article.site.site_id),
                web_article.site.name,
                web_article.site.url
            )
            .fetch_one(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::SqlxError(e))?;

            web_article.site = WebSite::new(
                WebSiteId::from(res.site_id),
                web_article.site.name.clone(),
                web_article.site.url.clone(),
            );
        } else {
            web_article.site = WebSite::new(
                WebSiteId::from(site[0].site_id),
                web_article.site.name.clone(),
                web_article.site.url.clone(),
            );
        }

        // Insert the article into the database
        let res = sqlx::query!(
            r#"INSERT INTO web_article (
                site_id,
                article_id,
                title,
                description,
                url,
                text,
                html,
                timestamp,
                summary,
                is_new_technology_related,
                is_new_product_related,
                is_new_academic_paper_related,
                is_ai_related,
                is_security_related,
                is_it_related,
                status_id
            ) SELECT 
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, status_id
            FROM status WHERE name = 'todo'
            RETURNING article_id"#,
            Uuid::from(web_article.site.site_id),
            Uuid::from(web_article.article_id),
            web_article.title,
            web_article.description,
            web_article.url,
            web_article.text,
            web_article.html,
            web_article.timestamp,
            web_article.summary,
            web_article.is_new_technology_related,
            web_article.is_new_product_related,
            web_article.is_new_academic_paper_related,
            web_article.is_ai_related,
            web_article.is_security_related,
            web_article.is_it_related,
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        Ok(WebArticle::new(
            web_article.site.clone(),
            WebArticleId::from(res.article_id),
            web_article.title.clone(),
            web_article.description.clone(),
            web_article.url.clone(),
            web_article.text.clone(),
            web_article.html.clone(),
            web_article.timestamp,
            web_article.summary.clone(),
            web_article.is_new_technology_related,
            web_article.is_new_product_related,
            web_article.is_new_academic_paper_related,
            web_article.is_ai_related,
            web_article.is_security_related,
            web_article.is_it_related,
        ))
    }
    async fn select_todays_web_articles(&self) -> AppResult<Vec<WebArticle>> {
        let today = chrono::Local::now().date_naive();
        let tomorrow = today + chrono::Duration::days(1);
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                ws.site_id AS site_id,
                ws.name AS site_name,
                ws.url AS site_url,
                wa.article_id,
                wa.title,
                wa.description,
                wa.url,
                wa.timestamp,
                wa.text,
                wa.html,
                wa.summary,
                wa.is_new_technology_related,
                wa.is_new_product_related,
                wa.is_new_academic_paper_related,
                wa.is_ai_related,
                wa.is_security_related,
                wa.is_it_related
            FROM 
                web_article as wa
            JOIN web_site as ws ON wa.site_id = ws.site_id
            WHERE wa.timestamp BETWEEN $1 AND $2"#,
            today,
            tomorrow
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))
        .unwrap();

        let sites = sqlx::query_as!(
            WebSiteRecord,
            r#"SELECT
                site_id,
                name,
                url
            FROM 
                web_site"#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))
        .unwrap();

        let web_articles: Vec<WebArticle> = rows
            .into_iter()
            .map(|row| {
                let site = sites
                    .iter()
                    .find(|s| s.site_id == row.site_id)
                    .map(|s| WebSite::from(s.clone()))
                    .unwrap_or_else(|| WebSite::new(WebSiteId::new(), "Unknown".to_string(), "Unknown".to_string()));
                let mut article = WebArticle::from(row);
                article.site = site;
                article
            })
            .collect();

        Ok(web_articles)
    }

    async fn select_web_article_by_id(&self, id: &str) -> AppResult<WebArticle> {
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                ws.site_id AS site_id,
                ws.name AS site_name,
                ws.url AS site_url,
                wa.article_id,
                wa.title,
                wa.description,
                wa.url,
                wa.timestamp,
                wa.text,
                wa.html,
                wa.summary,
                wa.is_new_technology_related,
                wa.is_new_product_related,
                wa.is_new_academic_paper_related,
                wa.is_ai_related,
                wa.is_security_related,
                wa.is_it_related
            FROM 
                web_article AS wa
            JOIN web_site AS ws ON wa.site_id = ws.site_id
            WHERE wa.article_id = $1"#,
            Uuid::from_str(id).unwrap()
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebArticle::from(row.clone()))
        }
    }
    async fn select_web_articles_by_keyword(&self, keyword: &str) -> AppResult<Vec<WebArticle>> {
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                ws.site_id AS site_id,
                ws.name AS site_name,
                ws.url AS site_url,
                wa.article_id,
                wa.title,
                wa.description,
                wa.url,
                wa.timestamp,
                wa.text,
                wa.html,
                wa.summary,
                wa.is_new_technology_related,
                wa.is_new_product_related,
                wa.is_new_academic_paper_related,
                wa.is_ai_related,
                wa.is_security_related,
                wa.is_it_related
            FROM 
                web_article AS wa
            JOIN web_site AS ws ON wa.site_id = ws.site_id
            WHERE wa.title LIKE $1 OR wa.description LIKE $1 OR wa.summary LIKE $1"#,
            format!("%{}%", keyword)
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        Ok(rows.into_iter().map(WebArticle::from).collect())
    }
    async fn select_web_article_by_url(&self, url: &str) -> AppResult<WebArticle> {
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                ws.site_id AS site_id,
                ws.name AS site_name,
                ws.url AS site_url,
                wa.article_id,
                wa.title,
                wa.description,
                wa.url,
                wa.timestamp,
                wa.text,
                wa.html,
                wa.summary,
                wa.is_new_technology_related,
                wa.is_new_product_related,
                wa.is_new_academic_paper_related,
                wa.is_ai_related,
                wa.is_security_related,
                wa.is_it_related
            FROM 
                web_article AS wa
            JOIN web_site AS ws ON wa.site_id = ws.site_id
            WHERE wa.url = $1"#,
            url
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebArticle::from(row.clone()))
        }
    }
    async fn select_or_create_web_article(&self, web_article: WebArticle) -> AppResult<WebArticle> {
        match self.select_web_article_by_url(&web_article.url).await {
            Ok(web_article) => Ok(web_article),
            Err(_) => {
                let web_article = WebArticle::new(
                    web_article.site,
                    web_article.article_id,
                    web_article.title,
                    web_article.description,
                    web_article.url,
                    web_article.text,
                    web_article.html,
                    web_article.timestamp,
                    web_article.summary,
                    web_article.is_new_technology_related,
                    web_article.is_new_product_related,
                    web_article.is_new_academic_paper_related,
                    web_article.is_ai_related,
                    web_article.is_security_related,
                    web_article.is_it_related,
                );
                self.create_web_article(&mut web_article.clone()).await
            }
        }
    }
    async fn select_all_web_articles(&self) -> AppResult<Vec<WebArticle>> {
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                ws.site_id AS site_id,
                ws.name AS site_name,
                ws.url AS site_url,
                wa.article_id,
                wa.title,
                wa.description,
                wa.url,
                wa.timestamp,
                wa.text,
                wa.html,
                wa.summary,
                wa.is_new_technology_related,
                wa.is_new_product_related,
                wa.is_new_academic_paper_related,
                wa.is_ai_related,
                wa.is_security_related,
                wa.is_it_related
            FROM 
                web_article AS wa
            JOIN web_site AS ws ON wa.site_id = ws.site_id
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        Ok(rows.into_iter().map(WebArticle::from).collect())
    }
    async fn select_paginated_web_articles(
        &self,
        options: WebArticleListOptions,
    ) -> AppResult<PaginatedList<WebArticle>> {
        let WebArticleListOptions { limit, offset } = options;
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"
            SELECT
                ws.site_id AS site_id,
                ws.name AS site_name,
                ws.url AS site_url,
                wa.article_id,
                wa.title,
                wa.description,
                wa.url,
                wa.timestamp,
                wa.text,
                wa.html,
                wa.summary,
                wa.is_new_technology_related,
                wa.is_new_product_related,
                wa.is_new_academic_paper_related,
                wa.is_ai_related,
                wa.is_security_related,
                wa.is_it_related
            FROM 
                web_article AS wa
            JOIN web_site AS ws ON wa.site_id = ws.site_id
            LIMIT $1
            OFFSET $2"#,
            limit,
            offset
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;

        let total = rows.len() as i64;
        let items = rows.into_iter().map(WebArticle::from).collect::<Vec<WebArticle>>();
        Ok(PaginatedList::<WebArticle>::new(total, limit, offset, items))
    }
    async fn update_web_article(&self, web_article: WebArticle) -> AppResult<()> {
        sqlx::query!(
            r#"UPDATE web_article SET
                title = $1,
                description = $2,
                url = $3,
                text = $4,
                html = $5,
                timestamp = $6,
                summary = $7,
                is_new_technology_related = $8,
                is_new_product_related = $9,
                is_new_academic_paper_related = $10,
                is_ai_related = $11,
                is_security_related = $12,
                is_it_related = $13
            WHERE article_id = $14"#,
            web_article.title,
            web_article.description,
            web_article.url,
            web_article.text,
            web_article.html,
            web_article.timestamp,
            web_article.summary,
            web_article.is_new_technology_related,
            web_article.is_new_product_related,
            web_article.is_new_academic_paper_related,
            web_article.is_ai_related,
            web_article.is_security_related,
            web_article.is_it_related,
            Uuid::from(web_article.article_id)
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        Ok(())
    }
    async fn delete_web_article(&self, id: &str) -> AppResult<()> {
        sqlx::query!(
            r#"DELETE FROM web_article WHERE article_id = $1"#,
            Uuid::from_str(id).unwrap()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::SqlxError(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::ConnectionPool;
    use shared::id::{WebArticleId, WebSiteId};

    #[sqlx::test]
    async fn test_website_crud(pool: sqlx::PgPool) {
        let repo = WebSiteRepositoryImpl::new(ConnectionPool::new(pool));
        let web_site = WebSite::new(
            WebSiteId::new(),
            "Test Website".to_string(),
            "https://testwebsite.com".to_string(),
        );

        // Create
        let web_site = repo.create_web_site(web_site.clone()).await.unwrap();

        // Read
        let options = WebSiteListOptions { limit: 10, offset: 0 };
        let records = repo.select_web_site_by_id(&web_site.site_id.to_string()).await.unwrap();
        assert_eq!(records.name, "Test Website");
        assert_eq!(records.url, "https://testwebsite.com");
        let records = repo.select_web_site_by_name("Test Website").await.unwrap();
        assert_eq!(records.name, "Test Website");
        assert_eq!(records.url, "https://testwebsite.com");
        let records = repo.select_all_web_sites_paginated(options.clone()).await.unwrap();
        assert_eq!(records.items.len(), 1);

        // Update
        let mut web_site = records.items[0].clone();
        web_site.name = "Updated Website".to_string();
        repo.update_web_site(web_site.clone()).await.unwrap();
        let updated_records = repo.select_all_web_sites_paginated(options.clone()).await.unwrap();
        assert_eq!(updated_records.items[0].name, "Updated Website");
        assert_eq!(updated_records.items[0].url, "https://testwebsite.com");

        // Delete
        repo.delete_web_site(&updated_records.items[0].site_id.to_string())
            .await
            .unwrap();
        let records_after_delete = repo.select_all_web_sites_paginated(options.clone()).await.unwrap();
        assert_eq!(records_after_delete.items.len(), 0);
    }

    #[sqlx::test]
    async fn test_web_article_crud(pool: sqlx::PgPool) {
        fn assert_article_eq(article_1: &WebArticle, article_2: &WebArticle) {
            assert_eq!(article_1.title, article_2.title);
            assert_eq!(article_1.description, article_2.description);
            assert_eq!(article_1.url, article_2.url);
            assert_eq!(article_1.text, article_2.text);
            assert_eq!(article_1.html, article_2.html);
            assert_eq!(article_1.timestamp, article_2.timestamp);
            assert_eq!(article_1.summary, article_2.summary);
            assert_eq!(article_1.is_new_technology_related, article_2.is_new_technology_related);
            assert_eq!(article_1.is_new_product_related, article_2.is_new_product_related);
            assert_eq!(
                article_1.is_new_academic_paper_related,
                article_2.is_new_academic_paper_related
            );
            assert_eq!(article_1.is_ai_related, article_2.is_ai_related);
        }

        let web_site_repo = WebSiteRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let repo = WebArticleRepositoryImpl::new(ConnectionPool::new(pool.clone()));

        let web_site = WebSite::new(
            WebSiteId::new(),
            "Test Website".to_string(),
            "https://testwebsite.com".to_string(),
        );

        // Test web article CRUD operations
        let web_article = WebArticle::new(
            web_site.clone(),
            WebArticleId::new(),
            "Test Article".to_string(),
            "Test Description".to_string(),
            "https://testarticle.com".to_string(),
            "Test Text".to_string(),
            "<HTML><TEST>test</TEST></HTML>".to_string(),
            chrono::Local::now().date_naive(),
            "Test Summary".to_string(),
            false,
            false,
            false,
            false,
            false,
            false,
        );

        // Create
        let web_article = repo.create_web_article(&mut web_article.clone()).await.unwrap();

        // Read
        let records = repo
            .select_web_article_by_id(&web_article.article_id.to_string())
            .await
            .unwrap();
        assert_article_eq(&web_article, &records);
        let records = repo.select_web_articles_by_keyword("Test").await.unwrap();
        assert_article_eq(&web_article, &records[0]);
        let records = repo.select_todays_web_articles().await.unwrap();
        assert_article_eq(&web_article, &records[0]);
        let records = repo.select_all_web_articles().await.unwrap();
        assert_eq!(records.len(), 1);

        // Update
        let mut web_article = records[0].clone();
        web_article.title = "Updated Article".to_string();
        repo.update_web_article(web_article.clone()).await.unwrap();
        let updated_records = repo.select_all_web_articles().await.unwrap();
        assert_eq!(updated_records[0].title, "Updated Article");

        // Delete
        repo.delete_web_article(&updated_records[0].article_id.to_string())
            .await
            .unwrap();
        let records_after_delete = repo.select_all_web_articles().await.unwrap();
        assert_eq!(records_after_delete.len(), 0);

        // Clean up the website
        web_site_repo
            .delete_web_site(&web_site.site_id.to_string())
            .await
            .unwrap();
    }
}
