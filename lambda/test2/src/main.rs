use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, Response};
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
        .get("x-username")
        .unwrap_or(&default_header_value);
    // creating an application/json response
    // Ok(json!({
    // "message": "Test 2 is me, how are you?"
    // }))
    let value = json!(
        {
            "message": "Test 2 is me, how are you?"
        }
    );
    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .body(
            serde_json::to_string(&value)
                .expect("unable to serialize serde_json::Value")
                .into(),
        )
        .expect("unable to build http::Response");
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test2_handles() {
        let request = Request::default();
        let expected = json!({
            "message": "Test 2 is me, how are you?"
        })
        .into_response();
        let response = test2(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
