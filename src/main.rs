use axum::{
    routing::get_service,
    Router,
};
use tower_http::services::fs::ServeDir;  // Đảm bảo bạn có import đúng
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Tạo Router để phục vụ tất cả các file từ thư mục "static"
    let app = Router::new()
        .nest_service("/", get_service(ServeDir::new("static")).handle_error(|error| async move {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error))
        }));
        
    // Lấy cổng từ biến môi trường PORT
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port.parse().unwrap()));
    println!("Listening on http://{}", addr);

    // Khởi chạy server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
