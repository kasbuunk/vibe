use axum::{routing::get, Router, response::Html};
use tokio::net::TcpListener;
use std::net::SocketAddr;

pub async fn create_hello_server(_addr: SocketAddr) -> Router {
    Router::new().route("/", get(hello))
}

async fn hello() -> Html<&'static str> {
    Html("Hello, Vibe!")
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    println!("Listening on {}", addr);
    
    let app = create_hello_server(addr).await;
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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
