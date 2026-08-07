use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn homepage_returns_200() {
    let response = filipnone_eu::app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn homepage_has_body() {
    let response = filipnone_eu::app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(!body.is_empty());
}
