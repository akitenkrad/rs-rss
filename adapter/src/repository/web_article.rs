use crate::database::{
    models::web_article::{WebArticleRecord, WebSiteRecord},
    ConnectionPool,
};
use async_trait::async_trait;
use derive_new::new;
use kernel::{
    models::web_article::{WebArticle, WebSite},
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
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;

        Ok(WebSite::new(WebSiteId::from(res.site_id), web_site.name, web_site.url))
    }
    async fn read_web_site_by_id(&self, id: &str) -> AppResult<WebSite> {
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
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebSite::from(row.clone()))
        }
    }
    async fn read_web_site_by_name(&self, name: &str) -> AppResult<WebSite> {
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
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebSite::from(row.clone()))
        }
    }
    async fn read_or_create_web_site(&self, name: &str, url: &str) -> AppResult<WebSite> {
        match self.read_web_site_by_name(name).await {
            Ok(web_site) => Ok(web_site),
            Err(_) => {
                let web_site = WebSite::new(WebSiteId::new(), name.to_string(), url.to_string());
                self.create_web_site(web_site).await
            }
        }
    }
    async fn read_all_web_sites(&self) -> AppResult<Vec<WebSite>> {
        let rows = sqlx::query_as!(
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
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        Ok(rows.into_iter().map(WebSite::from).collect())
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
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        Ok(())
    }
    async fn delete_web_site(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM web_site WHERE site_id = $1"#, Uuid::from_str(id).unwrap())
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct WebArticleRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl WebArticleRepository for WebArticleRepositoryImpl {
    async fn create_web_article(&self, web_article: WebArticle) -> AppResult<WebArticle> {
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
                is_new_paper_related,
                is_ai_related
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
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
            web_article.is_new_paper_related,
            web_article.is_ai_related
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        Ok(WebArticle::new(
            web_article.site,
            WebArticleId::from(res.article_id),
            web_article.title,
            web_article.description,
            web_article.url,
            web_article.text,
            web_article.html,
            web_article.timestamp,
            web_article.summary,
            web_article.is_new_technology_related,
            web_article.is_new_product_related,
            web_article.is_new_paper_related,
            web_article.is_ai_related,
        ))
    }
    async fn read_todays_articles(&self) -> AppResult<Vec<WebArticle>> {
        let today = chrono::Local::now().date_naive();
        let tomorrow = today + chrono::Duration::days(1);
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                site_id,
                article_id,
                title,
                description,
                url,
                timestamp,
                text,
                html,
                summary,
                is_new_technology_related,
                is_new_product_related,
                is_new_paper_related,
                is_ai_related
            FROM 
                web_article
            WHERE timestamp BETWEEN $1 AND $2"#,
            today,
            tomorrow
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::DatabaseError(e))
        .unwrap();
        Ok(rows.into_iter().map(WebArticle::from).collect())
    }

    async fn read_web_article_by_id(&self, id: &str) -> AppResult<WebArticle> {
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                site_id,
                article_id,
                title,
                description,
                url,
                timestamp,
                text,
                html,
                summary,
                is_new_technology_related,
                is_new_product_related,
                is_new_paper_related,
                is_ai_related
            FROM 
                web_article
            WHERE article_id = $1"#,
            Uuid::from_str(id).unwrap()
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebArticle::from(row.clone()))
        }
    }
    async fn read_web_articles_by_keyword(&self, keyword: &str) -> AppResult<Vec<WebArticle>> {
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                site_id,
                article_id,
                title,
                description,
                url,
                timestamp,
                text,
                html,
                summary,
                is_new_technology_related,
                is_new_product_related,
                is_new_paper_related,
                is_ai_related
            FROM 
                web_article
            WHERE title LIKE $1 OR description LIKE $1 OR summary LIKE $1"#,
            format!("%{}%", keyword)
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        Ok(rows.into_iter().map(WebArticle::from).collect())
    }
    async fn read_all_articles(&self) -> AppResult<Vec<WebArticle>> {
        let rows = sqlx::query_as!(
            WebArticleRecord,
            r#"SELECT
                site_id,
                article_id,
                title,
                description,
                url,
                timestamp,
                text,
                html,
                summary,
                is_new_technology_related,
                is_new_product_related,
                is_new_paper_related,
                is_ai_related
            FROM 
                web_article"#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        Ok(rows.into_iter().map(WebArticle::from).collect())
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
                is_new_paper_related = $10,
                is_ai_related = $11
            WHERE article_id = $12"#,
            web_article.title,
            web_article.description,
            web_article.url,
            web_article.text,
            web_article.html,
            web_article.timestamp,
            web_article.summary,
            web_article.is_new_technology_related,
            web_article.is_new_product_related,
            web_article.is_new_paper_related,
            web_article.is_ai_related,
            Uuid::from(web_article.article_id)
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
        Ok(())
    }
    async fn delete_web_article(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM web_article WHERE article_id = $1"#, Uuid::from_str(id).unwrap())
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::errors::AppError::DatabaseError(e))?;
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
        let web_site = WebSite::new(WebSiteId::new(), "Test Website".to_string(), "https://testwebsite.com".to_string());

        // Create
        let web_site = repo.create_web_site(web_site.clone()).await.unwrap();

        // Read
        let records = repo.read_web_site_by_id(&web_site.site_id.to_string()).await.unwrap();
        assert_eq!(records.name, "Test Website");
        assert_eq!(records.url, "https://testwebsite.com");
        let records = repo.read_web_site_by_name("Test Website").await.unwrap();
        assert_eq!(records.name, "Test Website");
        assert_eq!(records.url, "https://testwebsite.com");
        let records = repo.read_all_web_sites().await.unwrap();
        assert_eq!(records.len(), 1);

        // Update
        let mut web_site = records[0].clone();
        web_site.name = "Updated Website".to_string();
        repo.update_web_site(web_site.clone()).await.unwrap();
        let updated_records = repo.read_all_web_sites().await.unwrap();
        assert_eq!(updated_records[0].name, "Updated Website");
        assert_eq!(updated_records[0].url, "https://testwebsite.com");

        // Delete
        repo.delete_web_site(&updated_records[0].site_id.to_string()).await.unwrap();
        let records_after_delete = repo.read_all_web_sites().await.unwrap();
        assert_eq!(records_after_delete.len(), 0);
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
            assert_eq!(article_1.is_new_paper_related, article_2.is_new_paper_related);
            assert_eq!(article_1.is_ai_related, article_2.is_ai_related);
        }

        // Create a new website
        let web_site_repo = WebSiteRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let web_site = WebSite::new(WebSiteId::new(), "Test Website".to_string(), "https://testwebsite.com".to_string());
        let web_site = web_site_repo.create_web_site(web_site.clone()).await.unwrap();

        // Test web article CRUD operations
        let repo = WebArticleRepositoryImpl::new(ConnectionPool::new(pool.clone()));
        let web_article = WebArticle::new(
            web_site.clone(),
            WebArticleId::new(),
            "Test Article".to_string(),
            "Test Description".to_string(),
            "https://testarticle.com".to_string(),
            "Test Text".to_string(),
            "Test HTML".to_string(),
            chrono::Local::now().date_naive(),
            "Test Summary".to_string(),
            false,
            false,
            false,
            false,
        );

        // Create
        let web_article = repo.create_web_article(web_article.clone()).await.unwrap();

        // Read
        let records = repo.read_web_article_by_id(&web_article.article_id.to_string()).await.unwrap();
        assert_article_eq(&web_article, &records);
        let records = repo.read_web_articles_by_keyword("Test").await.unwrap();
        assert_article_eq(&web_article, &records[0]);
        let records = repo.read_todays_articles().await.unwrap();
        assert_article_eq(&web_article, &records[0]);
        let records = repo.read_all_articles().await.unwrap();
        assert_eq!(records.len(), 1);

        // Update
        let mut web_article = records[0].clone();
        web_article.title = "Updated Article".to_string();
        repo.update_web_article(web_article.clone()).await.unwrap();
        let updated_records = repo.read_all_articles().await.unwrap();
        assert_eq!(updated_records[0].title, "Updated Article");

        // Delete
        repo.delete_web_article(&updated_records[0].article_id.to_string()).await.unwrap();
        let records_after_delete = repo.read_all_articles().await.unwrap();
        assert_eq!(records_after_delete.len(), 0);

        // Clean up the website
        web_site_repo.delete_web_site(&web_site.site_id.to_string()).await.unwrap();
    }
}
