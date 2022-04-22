use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[derive(Serialize, Deserialize)]
struct GenerationRequest {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct GenerationResponse {
    generated_text: String,
}

#[derive(Serialize, Deserialize)]
struct GPTJRequest {
    inputs: String,
}


async fn generate(message: String) -> String {

    let body = GPTJRequest {
        inputs: message
    };

    let client = reqwest::Client::new();
    let res: Vec<GenerationResponse> = client
        .post("https://api-inference.huggingface.co/models/EleutherAI/gpt-j-6B")
        .header("Authorization", "Bearer {API_TOKEN}".to_owned())
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<Vec<GenerationResponse>>()
        .await
        .unwrap();

    res[0].generated_text.to_string()
}

// source: https://github.com/rodneylab/hcaptcha-serverless-rust-worker/blob/main/src/lib.rs
fn preflight_response(headers: &worker::Headers, cors_origin: &str) -> Result<Response> {
    let origin = match headers.get("Origin").unwrap() {
        Some(value) => value,
        None => return Response::empty(),
    };
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    headers.set("Access-Control-Allow-Methods", "POST")?;

    for origin_element in cors_origin.split(',') {
        if origin.eq(origin_element) {
            headers.set("Access-Control-Allow-Origin", &origin)?;
            break;
        }
    }
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(Response::empty()
        .unwrap()
        .with_headers(headers)
        .with_status(204))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .options("/generate", |req, ctx| {
            preflight_response(req.headers(), &ctx.var("CORS_ORIGIN")?.to_string())
        })
        .post_async("/generate", |mut req, _ctx| async move {
            let data: GenerationRequest;
            match req.json().await {
                Ok(res) => data = res,
                Err(_) => return Response::error("Bad request", 400),
            }
            let resp: GenerationResponse = GenerationResponse {
                generated_text: generate(data.message).await,
            };
            let mut headers = worker::Headers::new();
            headers.set("Access-Control-Allow-Origin", "*").unwrap();
            let response = Response::from_json(&json!(resp)).unwrap();
            Ok(response.with_headers(headers))
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
