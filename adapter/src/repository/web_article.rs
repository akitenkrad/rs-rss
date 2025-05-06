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
use shared::error::{AppError, AppResult};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, new)]
pub struct WebSiteRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl WebSiteRepository for WebSiteRepositoryImpl {
    async fn read_we_bsite_by_id(&self, id: &str) -> AppResult<WebSite> {
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))?;
        if rows.is_empty() {
            Err(AppError::RecordNotFound(sqlx::Error::RowNotFound))
        } else {
            let row = rows.first().unwrap();
            Ok(WebSite::from(row.clone()))
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))?;
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))?;
        Ok(())
    }
    async fn create_web_site(&self, web_site: WebSite) -> AppResult<()> {
        sqlx::query!(
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))?;
        Ok(())
    }
    async fn delete_web_site(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM web_site WHERE site_id = $1"#, Uuid::from_str(id).unwrap())
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::error::AppError::DatabaseError(e))?;
        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct WebArticleRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl WebArticleRepository for WebArticleRepositoryImpl {
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))?;
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))?;
        Ok(rows.into_iter().map(WebArticle::from).collect())
    }
    async fn create_web_article(&self, web_article: WebArticle) -> AppResult<()> {
        sqlx::query!(
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
            RETURNING site_id"#,
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
        .map_err(|e| shared::error::AppError::DatabaseError(e))?;
        Ok(())
    }
    async fn delete_web_article(&self, id: &str) -> AppResult<()> {
        sqlx::query!(r#"DELETE FROM web_article WHERE article_id = $1"#, Uuid::from_str(id).unwrap())
            .execute(self.db.inner_ref())
            .await
            .map_err(|e| shared::error::AppError::DatabaseError(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::ConnectionPool;
    use shared::id::WebSiteId;

    #[sqlx::test]
    async fn test_get_website_crud(pool: sqlx::PgPool) {
        let repo = WebSiteRepositoryImpl::new(ConnectionPool::new(pool));
        let website = WebSite::new(WebSiteId::new(), "Test Website".to_string(), "https://testwebsite.com".to_string());

        // Create
        repo.create_web_site(website.clone()).await.unwrap();

        // Read
        let records = repo.read_all_web_sites().await.unwrap();
        assert_eq!(records.len(), 1);

        // Update
        let mut website = records[0].clone();
        website.name = "Updated Website".to_string();
        repo.update_web_site(website.clone()).await.unwrap();
        let updated_records = repo.read_all_web_sites().await.unwrap();
        assert_eq!(updated_records[0].name, "Updated Website");
        assert_eq!(updated_records[0].url, "https://testwebsite.com");

        // Delete
        repo.delete_web_site(&updated_records[0].site_id.to_string()).await.unwrap();
        let records_after_delete = repo.read_all_web_sites().await.unwrap();
        assert_eq!(records_after_delete.len(), 0);
    }
}
