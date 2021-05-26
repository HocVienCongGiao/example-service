use lambda_http::http::HeaderValue;
use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(test2)).await?;
    Ok(())
}

async fn test2(req: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    let default_header_value = HeaderValue::from_str("anonymous").unwrap();
    let auth_header = req
        .headers()
        .get("X-Username")
        .unwrap_or(&default_header_value);
    // creating an application/json response
    Ok(json!({
    "message": "Test 222 is me, how are you? ".to_owned() + auth_header.to_str().unwrap()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test2_handles() {
        let request = Request::default();
        let expected = json!({
        "message": "Test 333 is me, how are you? anonymous"
        })
        .into_response();
        let response = test2(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
