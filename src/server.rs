mod connection;
mod request;

use crate::engine::{
    chat::{ChatTemplate, PreparedChat},
    completion::{CompletionAssembler, CompletionFinish},
    execution::MatrixBackend,
    generation::{self, Event, GenerationSession},
    model::ModelDefinition,
    output::{InitialContext, Markers, OutputParser, Update},
    text::{ReferenceLimits, ReferenceText},
    tokenizer::Tokenizer,
    weights::ModelFile,
};
use axum::{
    Json, Router,
    extract::{ConnectInfo, DefaultBodyLimit, State, rejection::JsonRejection},
    http::StatusCode,
    response::{
        IntoResponse, Response, Sse,
        sse::{Event as SseEvent, KeepAlive},
    },
    routing::{get, post},
};
use futures_core::Stream;
use serde_json::{Value, json};
use std::{
    convert::Infallible,
    path::PathBuf,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::{Context, Poll},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore, mpsc, oneshot};

pub use connection::Connection;
pub use request::ChatInput;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub model_path: PathBuf,
    pub model_id: String,
    pub workers: usize,
    pub backend: MatrixBackend,
    /// Chat template source that overrides the GGUF's own; `None` uses the GGUF's.
    pub template: Option<String>,
    pub queue: usize,
    /// Serving context limit (prompt plus reserved output canvases); `None`
    /// means the model's own context. Runtime cache limits are sized for it.
    pub max_context: Option<usize>,
    /// Prompt and committed-canvas tokens per encoder pass (results do not
    /// depend on it; larger chunks amortize weight streaming).
    pub encoder_chunk: usize,
    /// Optional backend for prompt prefill only (see
    /// `ReferenceText::set_prefill_backend`).
    pub prefill_backend: Option<MatrixBackend>,
    /// Split prompt chunks between two CPU packages (see
    /// `ReferenceText::new_split_sockets`); decoding stays on the first.
    pub split_sockets: bool,
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
    pub code: &'static str,
    pub param: Option<String>,
}
impl ApiError {
    pub fn invalid(message: impl Into<String>, param: &str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
            code: "invalid_request",
            param: Some(param.into()),
        }
    }
    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
            code: "generation_failed",
            param: None,
        }
    }
    pub fn value(&self) -> Value {
        json!({"error":{"message":self.message,"type":if self.status.is_client_error(){"invalid_request_error"}else{"server_error"},"param":self.param,"code":self.code}})
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.value())).into_response()
    }
}

fn generation_error(error: generation::Error) -> ApiError {
    match error {
        generation::Error::ContextLength {
            prompt_tokens,
            reserved_output_tokens,
            limit,
        } => ApiError {
            status: StatusCode::BAD_REQUEST,
            message: format!(
                "prompt_tokens={prompt_tokens}, reserved_output_tokens={reserved_output_tokens}, limit={limit}"
            ),
            code: "context_length_exceeded",
            param: Some("max_completion_tokens".into()),
        },
        error => ApiError::internal(error.to_string()),
    }
}

struct Job {
    input: ChatInput,
    id: String,
    created: u64,
    cancelled: Arc<AtomicBool>,
    connection: Option<Arc<AtomicBool>>,
    ready: oneshot::Sender<Result<(), ApiError>>,
    output: mpsc::Sender<Value>,
    _permit: OwnedSemaphorePermit,
}
impl Job {
    fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
            || self
                .connection
                .as_ref()
                .is_some_and(|flag| flag.load(Ordering::Acquire))
            || self.output.is_closed()
    }
}

#[derive(Clone)]
struct AppState {
    model: Arc<String>,
    /// Autoregressive models accept OpenAI sampling fields.
    sampling: bool,
    jobs: mpsc::Sender<Job>,
    slots: Arc<Semaphore>,
    counter: Arc<AtomicU64>,
}

pub struct Server {
    router: Router,
    context: usize,
}
impl Server {
    pub fn load(config: ServerConfig) -> Result<Self, String> {
        if config.queue > 64 {
            return Err("queue must be at most 64".into());
        }
        let (jobs, mut receiver) = mpsc::channel::<Job>(config.queue + 1);
        let (startup, initialized) = std::sync::mpsc::sync_channel(1);
        let model_id = config.model_id.clone();
        std::thread::Builder::new().name("power-cpu-llm-generation".into()).spawn(move || {
            let mut run = || -> Result<(), String> {
                let file = ModelFile::open(&config.model_path).map_err(|error| error.to_string())?;
                let model = ModelDefinition::bind(file.inventory()).map_err(|error| error.to_string())?;
                let tokenizer = Tokenizer::from_vocabulary(&model.vocabulary).map_err(|error| error.to_string())?;
                let template = match &config.template {
                    None => ChatTemplate::from_gguf(&model.vocabulary, &tokenizer),
                    Some(source) => ChatTemplate::new(source, &tokenizer),
                }.map_err(|error| error.to_string())?;
                let context = config.max_context.unwrap_or(model.spec.context).min(model.spec.context);
                if context == 0 { return Err("max context must be positive".into()); }
                let limits = ReferenceLimits::for_context(&model.spec, context);
                let runtime = if config.split_sockets {
                    ReferenceText::new_split_sockets(&file, &model, limits, config.workers, config.backend)
                } else {
                    ReferenceText::new_with_backend(&file, &model, limits, config.workers, config.backend)
                };
                let mut runtime = runtime.map_err(|error| error.to_string())?;
                runtime.set_prefill_backend(config.prefill_backend).map_err(|error| error.to_string())?;
                eprintln!("Server model {} ({:?}); backend {}; worker CPUs {:?}; context limit {} (model context {}).", config.model_id, model.spec.kind, runtime.matrix_backend(), runtime.worker_cpus(), context, model.spec.context);
                let _ = startup.send(Ok((context, model.spec.kind == crate::engine::model::ModelKind::Autoregressive)));
                while let Some(job) = receiver.blocking_recv() {
                    execute(&runtime, &tokenizer, &template, &config.model_id, context, config.encoder_chunk, job);
                }
                Ok(())
            };
            if let Err(error) = run() { let _ = startup.send(Err(error)); }
        }).map_err(|error| error.to_string())?;
        let (context, sampling) = initialized.recv().map_err(|error| error.to_string())??;
        Ok(Self {
            context,
            router: make_router(AppState {
                model: Arc::new(model_id),
                sampling,
                jobs,
                slots: Arc::new(Semaphore::new(config.queue + 1)),
                counter: Arc::new(AtomicU64::new(1)),
            }),
        })
    }
    pub fn router(&self) -> Router {
        self.router.clone()
    }
    pub fn serve(self, bind: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()?;
        runtime.block_on(async move {
            let listener = connection::CancelListener::bind(bind).await?;
            eprintln!(
                "Listening on {bind}; one generation at a time; context limit {}.",
                self.context
            );
            axum::serve(
                listener,
                self.router
                    .into_make_service_with_connect_info::<Connection>(),
            )
            .with_graceful_shutdown(async {
                let _ = tokio::signal::ctrl_c().await;
            })
            .await
        })?;
        Ok(())
    }
}

fn make_router(state: AppState) -> Router {
    Router::new()
        .route("/v1/models", get(models))
        .route("/v1/chat/completions", post(chat))
        .fallback(|| async {
            ApiError {
                status: StatusCode::NOT_FOUND,
                message: "unknown endpoint".into(),
                code: "not_found",
                param: None,
            }
        })
        .layer(DefaultBodyLimit::max(1024 * 1024))
        .with_state(state)
}
async fn models(State(state): State<AppState>) -> Json<Value> {
    Json(
        json!({"object":"list","data":[{"id":*state.model,"object":"model","owned_by":"power-cpu-llm"}]}),
    )
}
struct CancelGuard(Arc<AtomicBool>);
impl Drop for CancelGuard {
    fn drop(&mut self) {
        self.0.store(true, Ordering::Release);
    }
}

async fn chat(
    State(state): State<AppState>,
    connection: Result<ConnectInfo<Connection>, axum::extract::rejection::ExtensionRejection>,
    body: Result<Json<Value>, JsonRejection>,
) -> Result<Response, ApiError> {
    let Json(body) = body.map_err(|error| ApiError::invalid(error.body_text(), "body"))?;
    let input = ChatInput::parse(body, &state.model, state.sampling)?;
    let stream = input.stream;
    let permit = state
        .slots
        .clone()
        .try_acquire_owned()
        .map_err(|_| ApiError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            message: "generation queue is full".into(),
            code: "queue_full",
            param: None,
        })?;
    let cancelled = Arc::new(AtomicBool::new(false));
    let guard = CancelGuard(cancelled.clone());
    let (ready_tx, ready_rx) = oneshot::channel();
    let (output, mut receiver) = mpsc::channel(8);
    let job = Job {
        input,
        id: format!("chatcmpl-{}", state.counter.fetch_add(1, Ordering::Relaxed)),
        created: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        cancelled,
        connection: connection.ok().map(|info| info.0.cancelled),
        ready: ready_tx,
        output,
        _permit: permit,
    };
    state
        .jobs
        .try_send(job)
        .map_err(|_| ApiError::internal("generation worker unavailable"))?;
    ready_rx
        .await
        .map_err(|_| ApiError::internal("generation worker stopped"))??;
    if stream {
        Ok(Sse::new(EventStream {
            receiver,
            _guard: guard,
        })
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(10)))
        .into_response())
    } else {
        let value = receiver
            .recv()
            .await
            .ok_or_else(|| ApiError::internal("generation worker stopped"))?;
        drop(guard);
        if value.get("error").is_some() {
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(value)).into_response())
        } else {
            Ok(Json(value).into_response())
        }
    }
}

struct EventStream {
    receiver: mpsc::Receiver<Value>,
    _guard: CancelGuard,
}
impl Stream for EventStream {
    type Item = Result<SseEvent, Infallible>;
    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(context).map(|value| {
            value.map(|value| {
                Ok(
                    SseEvent::default().data(if value == Value::String("[DONE]".into()) {
                        "[DONE]".into()
                    } else {
                        value.to_string()
                    }),
                )
            })
        })
    }
}

fn chunk(job: &Job, model: &str, delta: Value, finish: Value) -> Value {
    json!({"id":job.id,"created":job.created,"model":model,"object":"chat.completion.chunk","choices":[{"index":0,"delta":delta,"finish_reason":finish}]})
}
fn delta(updates: Vec<Update>, include_reasoning: bool, first: bool) -> Value {
    let mut content = String::new();
    let mut reasoning = String::new();
    let mut has_reasoning = false;
    for update in updates {
        match update {
            Update::Content(text) => content.push_str(&text),
            Update::Reasoning(text) => {
                has_reasoning = true;
                reasoning.push_str(&text);
            }
            Update::ReasoningStarted => has_reasoning = true,
            Update::ReasoningEnded => {}
        }
    }
    let mut value = json!({});
    if first {
        value["role"] = json!("assistant");
    }
    if !content.is_empty() {
        value["content"] = json!(content);
    }
    if include_reasoning && has_reasoning {
        value["reasoning_content"] = json!(reasoning);
    }
    value
}
fn execute(
    runtime: &ReferenceText<'_>,
    tokenizer: &Tokenizer,
    template: &ChatTemplate,
    model: &str,
    context: usize,
    encoder_chunk: usize,
    job: Job,
) {
    let prepare = || -> Result<PreparedChat, ApiError> {
        template
            .prepare(tokenizer, &job.input.chat)
            .map_err(|error| ApiError::invalid(error.to_string(), "messages"))
    };
    let prepared = match prepare() {
        Ok(prepared) => prepared,
        Err(error) => {
            let _ = job.ready.send(Err(error));
            return;
        }
    };
    let config = generation::GenerationConfig {
        max_context_tokens: context,
        encoder_chunk_tokens: encoder_chunk,
        ..job.input.config.clone()
    };
    let mut session = match GenerationSession::new(runtime, &prepared.ids, config) {
        Ok(session) => session,
        Err(error) => {
            let _ = job.ready.send(Err(generation_error(error)));
            return;
        }
    };
    let mut assembler = match CompletionAssembler::new(tokenizer, &prepared) {
        Ok(assembler) => assembler,
        Err(error) => {
            let _ = job.ready.send(Err(ApiError::internal(error.to_string())));
            return;
        }
    };
    let mut parser = match Markers::new(tokenizer)
        .and_then(|markers| InitialContext::from_prompt_ids(tokenizer, &markers, &prepared.ids))
        .and_then(|context| OutputParser::new(tokenizer, context))
    {
        Ok(parser) => parser,
        Err(error) => {
            let _ = job.ready.send(Err(ApiError::internal(error.to_string())));
            return;
        }
    };
    let mut job = job;
    let (replacement, _) = oneshot::channel();
    if std::mem::replace(&mut job.ready, replacement)
        .send(Ok(()))
        .is_err()
    {
        session.cancel();
        return;
    }
    let mut first = true;
    let mut failure = None;
    while !session.is_finished() {
        if job.cancelled() {
            session.cancel();
            break;
        }
        let event = match session.advance() {
            Ok(event) => event,
            Err(error) => {
                failure = Some(generation_error(error));
                break;
            }
        };
        assembler.observe(&event);
        if let Event::Commit { tokens, .. } = &event {
            match parser.push(tokens) {
                Ok(updates) if job.input.stream && !job.cancelled() => {
                    if job
                        .output
                        .blocking_send(chunk(
                            &job,
                            model,
                            delta(updates, job.input.include_reasoning, first),
                            Value::Null,
                        ))
                        .is_err()
                    {
                        session.cancel();
                        break;
                    }
                    first = false;
                }
                Ok(_) => {}
                Err(error) => {
                    failure = Some(ApiError::internal(error.to_string()));
                    session.cancel();
                    break;
                }
            }
        }
    }
    let parsed = parser.finish();
    let completion = assembler.finish(
        session.result().expect("finished session"),
        job.input.include_reasoning,
    );
    if job.cancelled() {
        return;
    }
    let mut value = completion.to_json();
    if let Some(error) = failure {
        value = error.value();
    } else if let CompletionFinish::Error(error) = &completion.finish {
        value = ApiError::internal(error.to_string()).value();
    }
    if job.input.stream {
        if value.get("error").is_some() {
            let _ = job.output.blocking_send(value);
        } else {
            if !parsed.trailing.is_empty() {
                let _ = job.output.blocking_send(chunk(
                    &job,
                    model,
                    delta(parsed.trailing, job.input.include_reasoning, first),
                    Value::Null,
                ));
            }
            let mut final_chunk = chunk(
                &job,
                model,
                json!({}),
                value["choices"][0]["finish_reason"].clone(),
            );
            final_chunk["usage"] = value["usage"].clone();
            let _ = job.output.blocking_send(final_chunk);
        }
        let _ = job.output.blocking_send(json!("[DONE]"));
    } else {
        if value.get("error").is_none() {
            value["id"] = json!(job.id);
            value["created"] = json!(job.created);
            value["model"] = json!(model);
        }
        let _ = job.output.blocking_send(value);
    }
}

#[cfg(test)]
mod tests;
