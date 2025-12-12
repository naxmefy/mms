use axum::{routing::post, Json, Router};
use serde::Deserialize;
use crate::article::model::ArticleId;

#[derive(Deserialize)]
pub struct CreateArticleRequest {
    pub sku: String,
    pub name: String,
    pub price: f64,
}

pub fn router() -> Router {
    Router::new().route("/create", post(create_article))
}

async fn create_article(Json(_req): Json<CreateArticleRequest>) -> Json<String> {
    let id = ArticleId::new();
    Json(format!("Article {:?} created", id))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}, Router};
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_article_endpoint() {
        let app = Router::new().route("/create", axum::routing::post(super::create_article));
        let req = Request::builder()
            .method("POST")
            .uri("/create")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"sku":"123","name":"Test","price":9.99}"#))
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}