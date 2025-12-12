#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArticleId(pub uuid::Uuid);

impl ArticleId {
    pub(crate) fn new() -> ArticleId {
        ArticleId(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Article {
    pub id: ArticleId,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub category: Option<String>,
    pub active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_struct_fields() {
        let art = Article {
            id: ArticleId::new(),
            sku: "SKU1".into(),
            name: "test-article".into(),
            description: None,
            price: 10.0,
            stock: 0,
            category: None,
            active: true,
        };

        println!("article id: {:?}", art.id);
        assert_eq!(art.stock, 0);
        assert!(art.active);
        assert_eq!(art.sku, "SKU1");
        assert_eq!(art.name, "test-article");
        assert_eq!(art.description, None);
        assert_eq!(art.price, 10.0);
        assert_eq!(art.category, None);
    }
}
