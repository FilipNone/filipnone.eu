use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

use filipnone_eu::state::AppState;

fn test_app() -> axum::Router {
    let state = AppState {
        domain: "localhost".into(),
    };
    filipnone_eu::app(state)
}

#[tokio::test]
async fn homepage_returns_200() {
    let response = test_app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn homepage_has_body() {
    let response = test_app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(!body.is_empty());
}

#[tokio::test]
async fn homepage_renders_html() {
    let response = test_app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Hello, I am Filip"));
}

#[tokio::test]
async fn homepage_uses_domain() {
    let response = test_app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();
    assert!(html.contains("localhost"));
}

#[tokio::test]
async fn panel_returns_200() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/panel")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn panel_renders_html() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/panel")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();
    assert!(html.contains("Admin panel"));
}

#[tokio::test]
async fn static_css_is_served() {
    let response = test_app()
        .oneshot(
            Request::builder()
                .uri("/static/style.css")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
