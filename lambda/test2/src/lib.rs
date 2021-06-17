use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Serialize)]
struct TokenPayload {
    // Despite the struct field being named `username`, it is going to come
    // from a JSON field called `cognito:username`.
    #[serde(rename(deserialize = "cognito:username"))]
    username: String,
}

/**
{
   "sub":"fd9a7af8-fa76-4b86-af3d-d9634ef52374",
   "aud":"1rav411nccnp73htopbhml8s61",
   "cognito:groups":[
      "OperatorGroup"
   ],
   "event_id":"9645d622-3f4b-42b7-9b4d-71d35da9256d",
   "token_use":"id",
   "auth_time":1623934926,
   "iss":"https:\/\/cognito-idp.ap-southeast-1.amazonaws.com\/ap-southeast-1_9QWSYGzXk",
   "phone_number_verified":true,
   "cognito:username":"dev-operator",
   "phone_number":"+84369140916",
   "exp":1623938526,
   "iat":1623934926
}
*/

pub async fn test2(req: Request, ctx: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    let default_header_value = HeaderValue::from_str("anonymous12").unwrap();
    let auth_header_value = req
        .headers()
        .get("authorization")
        .unwrap_or(&default_header_value);
    let auth_header_str = auth_header_value.to_str().unwrap();
    let username: String;
    if (auth_header_str != "anonymous12") {
        let jwt_token = &auth_header_str.to_string()[7..];
        let token_data: TokenData<TokenPayload> =
            jsonwebtoken::dangerous_insecure_decode(jwt_token).unwrap();
        let token_payload = token_data.claims;
        username = token_payload.username;
    } else {
        username = String::from("anonymous");
    }
    println!("token username {}", username);
    // creating an application/json response
    // Ok(json!({
    // "message": "Test 2 is me, how are you?"
    // }))
    println!("auth_header is {}", auth_header_str);
    println!("req.headers() is {:?}", req.headers());
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
