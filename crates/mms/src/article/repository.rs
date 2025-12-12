use crate::article::model::*;

#[async_trait::async_trait]
pub trait ArticleRepository: Send + Sync {
    async fn create(&self, article: &Article) -> anyhow::Result<()>;
    async fn update(&self, article: &Article) -> anyhow::Result<()>;
    async fn delete(&self, id: ArticleId) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: ArticleId) -> anyhow::Result<Option<Article>>;
    async fn list(&self) -> anyhow::Result<Vec<Article>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_is_implementable() {
        // dummy test only for trait compilability
        fn _assert_trait<T: ArticleRepository>() {}
    }
}