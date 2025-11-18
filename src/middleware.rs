use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

/// 创建配置好的 TraceLayer 中间件
/// TraceLayer 提供了完整的 HTTP 请求追踪功能，包括：
/// - 请求开始日志
/// - 响应完成日志（包含状态码和耗时）
/// - 错误日志
/// 
/// 注意：由于 TraceLayer 的类型非常复杂，这里使用宏来简化使用
/// 在 routes.rs 中使用 axum_demo::trace_layer!() 来调用
#[macro_export]
macro_rules! trace_layer {
    () => {
        tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                )
            })
            .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                tracing::info!("请求开始: {} {}", request.method(), request.uri());
            })
            .on_response(|_response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                tracing::info!("请求完成 - 状态: {} - 耗时: {:?}", _response.status(), latency);
            })
            .on_failure(|_error: tower_http::classify::ServerErrorsFailureClass, _latency: std::time::Duration, _span: &tracing::Span| {
                tracing::error!("请求失败 - 错误: {:?} - 耗时: {:?}", _error, _latency);
            })
    };
}

/// 认证中间件（示例，实际使用提取器更灵活）
pub async fn auth_middleware(request: Request, next: Next) -> Response {
    // 检查是否需要认证
    let auth_header = request.headers().get("authorization");

    if auth_header.is_none() && !is_public_path(request.uri().path()) {
        return Response::builder()
            .status(axum::http::StatusCode::UNAUTHORIZED)
            .body("需要认证".into())
            .unwrap()
            .into();
    }

    next.run(request).await
}

/// 判断是否为公开路径
fn is_public_path(path: &str) -> bool {
    path.starts_with("/api/auth") || path == "/" || path == "/health"
}

/// 创建中间件栈（示例，当前未使用）
/// 如果需要使用，可以在路由中通过 .layer() 方法应用
/// 注意：ServiceBuilder 的类型会随着添加的 layer 而变化，所以这里不指定具体返回类型
pub fn create_middleware_stack() -> impl tower::Layer<axum::Router> {
    ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]),
        )
}
