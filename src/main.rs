use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    IntoResponse, Request, RequestExt, Response,
    Body
};
use serde_json::json;

#[inline]
pub fn fib(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative", n);
    }

    match n {
        0 => panic!("0 is error"),
        1 | 2 => 1,
        3 => 2,
        _ => fib(n - 1) + fib(n - 2)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let result = fib(40);
    let data = json!({
        "result": result
    });
    Ok(Response::builder()
    .status(200)
    .body(Body::Text(data.to_string()))
    .expect("failed to render response"))
}