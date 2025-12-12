use super::{model::Article, repository::ArticleRepository};
use crate::article::model::ArticleId;
use anyhow::Result;
use uuid::Uuid;

pub struct ArticleService<R: ArticleRepository> {
    repo: R,
}

impl<R: ArticleRepository> ArticleService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> Result<Vec<Article>> {
        self.repo.list().await
    }

    pub async fn create_article(&self, sku: String, name: String, price: f64) -> Result<Article> {
        if price < 0.0 {
            anyhow::bail!("Price cannot be negative");
        }

        let article = Article {
            id: ArticleId(Uuid::new_v4()),
            sku,
            name,
            description: None,
            price,
            stock: 0,
            category: None,
            active: true,
        };

        self.repo.create(&article).await?;
        Ok(article)
    }

    pub async fn update_article(&self, article: &Article) -> Result<Article> {
        if article.price < 0.0 {
            anyhow::bail!("Price cannot be negative");
        }
        self.repo.update(article).await?;
        Ok(article.clone())
    }

    pub async fn delete_article(&self, id: ArticleId) -> Result<()> {
        self.repo.delete(id).await
    }

    pub async fn get_article(&self, id: ArticleId) -> Result<Option<Article>> {
        self.repo.get_by_id(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    struct InMemoryRepo {
        items: Arc<Mutex<Vec<Article>>>,
    }

    impl InMemoryRepo {
        fn new() -> Self {
            Self {
                items: Arc::new(Mutex::new(vec![])),
            }
        }
    }

    #[async_trait]
    impl ArticleRepository for InMemoryRepo {
        async fn create(&self, article: &Article) -> Result<()> {
            self.items.lock().unwrap().push(article.clone());
            Ok(())
        }
        async fn update(&self, article: &Article) -> Result<()> {
            let mut items = self.items.lock().unwrap();
            if let Some(pos) = items.iter().position(|x| x.id == article.id) {
                items[pos] = article.clone();
                Ok(())
            } else {
                anyhow::bail!("Article not found")
            }
        }

        async fn delete(&self, id: ArticleId) -> Result<()> {
            let mut items = self.items.lock().unwrap();
            let len_before = items.len();
            items.retain(|x| x.id != id);
            if items.len() == len_before {
                anyhow::bail!("Article not found");
            }
            Ok(())
        }

        async fn get_by_id(&self, id: ArticleId) -> Result<Option<Article>> {
            let items = self.items.lock().unwrap();
            Ok(items.iter().find(|x| x.id == id).cloned())
        }

        async fn list(&self) -> Result<Vec<Article>> {
            Ok(self.items.lock().unwrap().clone())
        }
    }

    #[tokio::test]
    async fn list_articles() {
        let repo = InMemoryRepo::new();
        let service = ArticleService::new(repo);
        service
            .create_article("SKU123".into(), "Test".into(), 9.99)
            .await
            .unwrap();
        service
            .create_article("SKU456".into(), "Test_2".into(), 19.99)
            .await
            .unwrap();

        let items = service.list().await.unwrap();
        assert_eq!(items.len(), 2);
    }

    #[tokio::test]
    async fn get_by_id_article() {
        let repo = InMemoryRepo::new();
        let service = ArticleService::new(repo);

        let article = service
            .create_article("SKU123".into(), "Test".into(), 9.99)
            .await
            .unwrap();
        let fetched = service.get_article(article.id).await.unwrap().unwrap();

        assert_eq!(fetched.sku, "SKU123");
    }

    #[tokio::test]
    async fn create_article_success() {
        let repo = InMemoryRepo::new();
        let service = ArticleService::new(repo);
        let article = service
            .create_article("SKU123".into(), "Test".into(), 9.99)
            .await
            .unwrap();
        assert_eq!(article.sku, "SKU123");
    }

    #[tokio::test]
    async fn create_article_negative_price() {
        let repo = InMemoryRepo::new();
        let service = ArticleService::new(repo);
        let res = service
            .create_article("SKU1".into(), "Fail".into(), -1.0)
            .await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn update_article() {
        let repo = InMemoryRepo::new();
        let service = ArticleService::new(repo);

        let mut article = service
            .create_article("SKU123".into(), "Test".into(), 9.99)
            .await
            .unwrap();

        article.name = "Updated".into();
        service.update_article(&article).await.unwrap();

        let fetched = service.get_article(article.id).await.unwrap().unwrap();
        assert_eq!(fetched.name, "Updated");
    }

    #[tokio::test]
    async fn delete_article() {
        let repo = InMemoryRepo::new();
        let service = ArticleService::new(repo);

        let article = service
            .create_article("SKU123".into(), "Test".into(), 9.99)
            .await
            .unwrap();
        let article_id = article.id;

        service.delete_article(article_id.clone()).await.unwrap();

        let fetched = service.get_article(article_id).await.unwrap();
        assert!(fetched.is_none());
    }
}
