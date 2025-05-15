use crate::agents::developer::DeveloperAgent;

#[tokio::main]
async fn main() {
    let agent = DeveloperAgent::new();
    println!("DeveloperAgent created");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use tower::util::ServiceExt;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_hello_route() {
        // Create router
        let app = create_hello_server("127.0.0.1:8080".parse().unwrap()).await;
        
        // Create a test request
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert status code is 200 OK
        assert_eq!(response.status(), StatusCode::OK);

        // Get the response body as a string
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        // Assert the response body contains our expected message
        assert_eq!(body_str, "Hello, Vibe!");
    }
}
