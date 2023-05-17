use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service, ServiceBuilder};

// 服务：将输入字符串转换为问候语
struct GreetingService;

impl Service<String> for GreetingService {
    type Response = String;
    type Error = std::convert::Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, input: String) -> Self::Future {
        Box::pin(async move {
            let response = format!("Hello, {}!", input.to_uppercase());
            Ok(response)
        })
    }
}

// 中间件：计算处理请求所需的时间
struct TimingMiddleware<S> {
    inner: S,
}

impl<S> TimingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        TimingMiddleware { inner }
    }
}

impl<S, Req> Service<Req> for TimingMiddleware<S>
where
    S: Service<Req> + Send,
    Req: Send + Clone + 'static,
    S::Future: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        let start_time = Instant::now();
        let inner = &mut self.inner;
        let req_clone = req.clone();
        Box::pin(async move {
            let result = inner.call(req_clone).await;
            println!("Request took: {:?}", Instant::now() - start_time);
            result
        })
    }
}

pub struct TimingLayer;

impl<S> Layer<S> for TimingLayer {
    type Service = TimingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TimingMiddleware::new(inner)
    }
}

#[tokio::main]
async fn main() {
    // 创建服务并应用中间件
    let mut service = ServiceBuilder::new()
        .layer(TimingLayer)
        .service(GreetingService);

    // 调用服务
    let response = service
        .call("world".to_string())
        .await
        .expect("Service call failed");

    println!("Response: {}", response);
}
