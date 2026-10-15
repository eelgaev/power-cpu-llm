use super::*;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use tower::ServiceExt;

fn body() -> Value {
    json!({"messages":[{"role":"user","content":"Hello"}]})
}
fn state(queue: usize) -> (AppState, mpsc::Receiver<Job>) {
    let (jobs, receiver) = mpsc::channel(queue + 1);
    (
        AppState {
            model: Arc::new("test".into()),
            sampling: false,
            jobs,
            slots: Arc::new(Semaphore::new(queue + 1)),
            counter: Arc::new(AtomicU64::new(1)),
        },
        receiver,
    )
}
fn request(value: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/chat/completions")
        .header("content-type", "application/json")
        .body(Body::from(value.to_string()))
        .unwrap()
}

#[tokio::test]
async fn models_and_every_unsupported_field_are_checked() {
    let (state, _receiver) = state(1);
    let router = make_router(state);
    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .uri("/v1/models")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let value: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(value["data"][0]["id"], "test");
    for field in [
        "tools",
        "stop",
        "logprobs",
        "temperature",
        "top_p",
        "top_k",
        "frequency_penalty",
        "presence_penalty",
        "response_format",
        "unknown",
    ] {
        let mut value = body();
        value[field] = json!(null);
        assert_eq!(
            router
                .clone()
                .oneshot(request(value))
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
    for value in [
        json!({"messages":[{"role":"tool","content":"x"}]}),
        json!({"messages":[{"role":"user","content":[{"type":"image_url","image_url":"x"}]}]}),
        json!({"messages":[{"role":"user","content":"x","tool_calls":[]}]}),
        json!({"messages":[],"n":2}),
        json!({"messages":[],"max_tokens":1,"max_completion_tokens":2}),
    ] {
        assert_eq!(
            router
                .clone()
                .oneshot(request(value))
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
}

#[tokio::test]
async fn bounded_queue_and_context_error_do_not_execute() {
    let (state, mut receiver) = state(0);
    let router = make_router(state);
    let pending = tokio::spawn(router.clone().oneshot(request(body())));
    let job = receiver.recv().await.unwrap();
    assert_eq!(
        router
            .clone()
            .oneshot(request(body()))
            .await
            .unwrap()
            .status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    let error = generation_error(generation::Error::ContextLength {
        prompt_tokens: 3841,
        reserved_output_tokens: 256,
        limit: 4096,
    });
    job.ready.send(Err(error)).unwrap();
    let response = pending.await.unwrap().unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let value: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(value["error"]["code"], "context_length_exceeded");
}

#[tokio::test]
async fn success_and_sse_framing_and_disconnect() {
    let (state, mut receiver) = state(1);
    let router = make_router(state);
    let pending = tokio::spawn(router.clone().oneshot(request(body())));
    let mut job = receiver.recv().await.unwrap();
    let (replacement, _) = oneshot::channel();
    std::mem::replace(&mut job.ready, replacement)
        .send(Ok(()))
        .unwrap();
    let expected = json!({"object":"chat.completion","choices":[{"message":{"role":"assistant","content":"Paris"},"finish_reason":"stop"}]});
    job.output.send(expected.clone()).await.unwrap();
    let response = pending.await.unwrap().unwrap();
    let value: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(value, expected);
    drop(job);
    let mut value = body();
    value["stream"] = json!(true);
    let pending = tokio::spawn(router.clone().oneshot(request(value.clone())));
    let mut job = receiver.recv().await.unwrap();
    let (replacement, _) = oneshot::channel();
    std::mem::replace(&mut job.ready, replacement)
        .send(Ok(()))
        .unwrap();
    job.output
        .send(chunk(&job, "test", json!({"content":"Paris"}), Value::Null))
        .await
        .unwrap();
    job.output.send(json!("[DONE]")).await.unwrap();
    let response = pending.await.unwrap().unwrap();
    assert_eq!(response.headers()["content-type"], "text/event-stream");
    let cancelled = job.cancelled.clone();
    drop(job);
    let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
    let text = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(text.contains("chat.completion.chunk"));
    assert!(text.contains("data: [DONE]"));
    assert!(cancelled.load(Ordering::Acquire));
    let pending = tokio::spawn(router.oneshot(request(value)));
    let mut job = receiver.recv().await.unwrap();
    let (replacement, _) = oneshot::channel();
    std::mem::replace(&mut job.ready, replacement)
        .send(Ok(()))
        .unwrap();
    let response = pending.await.unwrap().unwrap();
    drop(response);
    assert!(job.cancelled());
}

#[tokio::test]
async fn tcp_disconnect_marks_connection_cancelled() {
    use axum::serve::Listener;
    use tokio::io::AsyncWriteExt;
    let mut listener = connection::CancelListener::bind("127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let address = listener.0.local_addr().unwrap();
    let mut client = tokio::net::TcpStream::connect(address).await.unwrap();
    let (mut stream, info) = listener.accept().await;
    client.write_all(b"x").await.unwrap();
    let mut byte = [0];
    tokio::io::AsyncReadExt::read_exact(&mut stream, &mut byte)
        .await
        .unwrap();
    drop(client);
    tokio::time::timeout(Duration::from_secs(2), async {
        while !info.cancelled.load(Ordering::Acquire) {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
}

#[test]
fn defaults_and_channel_delta_privacy() {
    let input = ChatInput::parse(body(), "test", false).unwrap();
    assert_eq!(input.config.seed, 7);
    assert_eq!(input.config.max_context_tokens, usize::MAX);
    assert!(!input.include_reasoning);
    assert_eq!(
        delta(
            vec![
                Update::Reasoning("private".into()),
                Update::Content("Paris".into())
            ],
            false,
            true
        ),
        json!({"role":"assistant","content":"Paris"})
    );
}

#[test]
fn sampling_fields_only_for_autoregressive_models() {
    let mut value = body();
    value["temperature"] = json!(0.0);
    value["top_p"] = json!(0.9);
    value["top_k"] = json!(10);
    assert!(ChatInput::parse(value.clone(), "test", false).is_err());
    let input = ChatInput::parse(value, "test", true).unwrap();
    assert_eq!(input.config.sampling.temperature, 0.0);
    assert_eq!(input.config.sampling.top_k, 10);
    let mut bad = body();
    bad["temperature"] = json!(5.0);
    assert!(ChatInput::parse(bad, "test", true).is_err());
}
