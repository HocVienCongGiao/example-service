use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    // let first_name = event["firstName"].as_str().unwrap_or("world");
    println!("Event payload is {:?}", event);
    Ok(json!({ "message": format!("Hello, {:?}!", event) }))
}
