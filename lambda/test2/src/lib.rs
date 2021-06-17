use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, Response};
use serde_json::json;
type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

pub async fn test2(req: Request, ctx: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    let default_header_value = HeaderValue::from_str("anonymous2").unwrap();
    let auth_header = req
        .headers()
        .get("x-username")
        .unwrap_or(&default_header_value);
    // creating an application/json response
    // Ok(json!({
    // "message": "Test 2 is me, how are you?"
    // }))
    println!("auth_header is {}", auth_header.to_str().unwrap());
    println!(
        "context.identity.cognitoIdentityId is {:?}",
        ctx.identity.unwrap().identity_id
    );
    let value = json!(
        {
            "message": "Test 2 20210616 is me, how are you?"
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
    let test1Response = controller::get_test1().await;
    println!("test1Response {}", test1Response.status);
    Ok(response)
}
