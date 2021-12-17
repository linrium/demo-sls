use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::*;
use js_sys::Promise;
use serde_json::json;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn hello(event: JsValue, _context: JsValue) -> Promise {
    // let http_event: ApiGatewayProxyRequest = serde_wasm_bindgen::from_value(event).unwrap();
    // let num = match http_event.query_string_parameters.get("num") {
    //     Some(v) => v.parse::<i32>().unwrap(),
    //     None => 10,
    // };
    let result = fib(40);
    let data = json!({
        "result": result
    });

    let response = ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(Body::Text(serde_json::to_string_pretty(&data).unwrap())),
        is_base64_encoded: None,
    };
    let js_result = serde_wasm_bindgen::to_value(&response).unwrap();

    Promise::resolve(&js_result)
}