use axum::body::to_bytes;
use axum::extract::FromRequestParts;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::{json, Value};

use crate::auth::{login, Claims};
use crate::common::{ApiError, Pagination};
use crate::models::Trees;

#[tokio::test]
async fn pagination_preserves_validation_and_defaults() {
    for query in ["", "page=0", "page=abc", "page=1&size=0", "page=1&size=abc"] {
        let (mut parts, _) = Request::builder()
            .uri(format!("/tree/q?{query}"))
            .body(())
            .unwrap()
            .into_parts();
        assert!(Pagination::from_request_parts(&mut parts, &())
            .await
            .is_err());
    }
    for (query, page, size) in [("page=1", 1, 10), ("page=2&size=3&energy=5", 2, 3)] {
        let (mut parts, _) = Request::builder()
            .uri(format!("/tree/q?{query}"))
            .body(())
            .unwrap()
            .into_parts();
        let pagination = Pagination::from_request_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!((pagination.page, pagination.size), (page, Some(size)));
    }
}

#[tokio::test]
async fn claims_reject_missing_and_invalid_credentials() {
    for authorization in [None, Some("Basic abc"), Some("Bearer invalid")] {
        let mut request = Request::builder().uri("/");
        if let Some(value) = authorization {
            request = request.header("authorization", value);
        }
        let (mut parts, _) = request.body(()).unwrap().into_parts();
        assert!(Claims::from_request_parts(&mut parts, &()).await.is_err());
    }
}

#[tokio::test]
async fn login_token_round_trips_through_claims_extractor() {
    let Json(value) = login(Json(serde_json::from_value(json!({"id": 42})).unwrap()))
        .await
        .unwrap();
    let token = value["token"].as_str().unwrap();
    let (mut parts, _) = Request::builder()
        .header("authorization", format!("Bearer {token}"))
        .body(())
        .unwrap()
        .into_parts();
    let claims = Claims::from_request_parts(&mut parts, &()).await.unwrap();
    assert_eq!(claims.id, 42);
    assert_eq!(claims.exp - claims.iat, 90 * 24 * 60 * 60);
}

#[tokio::test]
async fn tree_json_preserves_fields_and_null_datetime_format() {
    let value = json!({
        "id": 7, "user_id": null, "name": "树", "desc": "\"test\"",
        "energy": 10, "created_at": null, "updated_at": null
    });
    let tree: Trees = serde_json::from_value(value.clone()).unwrap();
    let mut expected = value;
    expected["created_at"] = json!("");
    expected["updated_at"] = json!("");
    for (response, expected) in [
        (Json(tree.clone()).into_response(), expected.clone()),
        (Json(vec![tree]).into_response(), json!([expected])),
        (Json(Vec::<Trees>::new()).into_response(), json!([])),
    ] {
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "application/json");
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(serde_json::from_slice::<Value>(&body).unwrap(), expected);
    }
}

#[tokio::test]
async fn api_error_preserves_status_and_body() {
    for (error, status, message) in [
        (ApiError::NotFound, StatusCode::NOT_FOUND, "not found"),
        (
            ApiError::WrongCredentials,
            StatusCode::UNAUTHORIZED,
            "Wrong credentials",
        ),
        (
            ApiError::PageError,
            StatusCode::BAD_REQUEST,
            "page params error",
        ),
    ] {
        let response = error.into_response();
        assert_eq!(response.status(), status);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<Value>(&body).unwrap(),
            json!({"error": message})
        );
    }
}
