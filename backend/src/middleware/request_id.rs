use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{HeaderName, HeaderValue},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::time::Instant;
use tracing::Instrument;
use uuid::Uuid;

const REQUEST_ID_HEADER: &str = "x-request-id";

/// Correlation ID for a single request. Read via `req.extensions().get::<RequestId>()`.
#[derive(Debug, Clone)]
pub struct RequestId(pub String);

impl RequestId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct RequestIdMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RequestIdMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddlewareService<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_service(&self, service: S) -> Self::Future {
        std::future::ready(Ok(RequestIdMiddlewareService { service }))
    }
}

pub struct RequestIdMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_id = req
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|v| v.to_str().ok())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        req.extensions_mut().insert(RequestId(request_id.clone()));

        let span = tracing::info_span!(
            "http_request",
            request_id = %request_id,
            method = %req.method(),
            path = %req.path(),
        );

        let started_at = Instant::now();
        let fut = self.service.call(req);

        Box::pin(
            async move {
                tracing::info!("request started");
                let mut res = fut.await?;
                let elapsed_ms = started_at.elapsed().as_millis();
                tracing::info!(
                    status = res.status().as_u16(),
                    elapsed_ms = elapsed_ms as u64,
                    "request completed"
                );

                if let Ok(header_value) = HeaderValue::from_str(&request_id) {
                    res.headers_mut()
                        .insert(HeaderName::from_static("x-request-id"), header_value);
                }

                Ok(res)
            }
            .instrument(span),
        )
    }
}
