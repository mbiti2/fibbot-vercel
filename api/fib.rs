use vercel_runtime::{run, Body, Error, Request, Response};

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let query = req.uri().query().unwrap_or("");
    let params: Vec<_> = url::form_urlencoded::parse(query.as_bytes()).collect();
    let number = params.iter().find(|(k, _)| k == "number")
        .and_then(|(_, v)| v.parse::<u64>().ok())
        .unwrap_or(10);
    let result = fib(number);
    let body = serde_json::json!({
        "number": number,
        "fibonacci": result
    });
    Ok(Response::builder()
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap())
} 