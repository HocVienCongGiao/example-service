use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{handler, lambda_runtime, Body, Context, IntoResponse, Request, Response};
use serde_json::json;
use test2::test2;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

// use pg_embed::postgres::PgEmbed;
use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn integration_works() {
    initialise();
    println!("is it working?");
    let request = Request::default();
    let expected = json!(
        {"name":"No Name for status 200","photoUrls":[],"status":"200"}
    )
    .into_response();
    let response = test2::test2(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    assert_eq!(response.body(), expected.body())
}
