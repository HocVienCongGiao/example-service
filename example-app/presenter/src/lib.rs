use domain::test_func;
// use postgres::{Client, Error, NoTls};
use rusoto_lambda::LambdaClient;

#[cfg(test)]
mod tests {
    use rusoto_lambda::{LambdaClient, Lambda, InvocationRequest};
    use bytes::Bytes;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn lambda_client() {
        let lambda_client = LambdaClient::new("ap-southeast-1".parse().unwrap());
        let res = lambda_client.invoke(InvocationRequest {
            client_context: None,
            function_name: "hello".parse().unwrap(),
            invocation_type: None,
            log_type: None,
            payload: Option::from(Bytes::from("{\"message\": \"how are you\"}".as_bytes())),
            qualifier: None,
        }).await;
        let hello_str = String::from_utf8(res.unwrap().payload.unwrap().to_vec()).unwrap();
        println!("Res is {:?}", hello_str);
        let hello: Hello = serde_json::from_str(&*hello_str).unwrap();
        println!(" message is {} ", hello.body)
    }

    #[derive(Deserialize, Serialize)]
    struct Hello {
        body: String
    }

    #[tokio::test]
    async fn invoke_test2() {
        let lambda_client = LambdaClient::new("ap-southeast-1".parse().unwrap());

        let payload = Root {
            resource: "/".to_string(),
            path: "/".to_string(),
            http_method: "GET".to_string(),
            headers: Headers {
                accept: "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9".to_string(),
                accept_encoding: "gzip, deflate, br".to_string(),
                accept_language: "en-US,en;q=0.9".to_string(),
                cookie: "s_fid=7AAB6XMPLAFD9BBF-0643XMPL09956DE2; regStatus=pre-register".to_string(),
                host: "70ixmpl4fl.execute-api.us-east-2.amazonaws.com".to_string(),
                sec_fetch_dest: "document".to_string(),
                sec_fetch_mode: "navigate".to_string(),
                sec_fetch_site: "none".to_string(),
                upgrade_insecure_requests: "1".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36".to_string(),
                x_amzn_trace_id: "Root=1-5e66d96f-7491f09xmpl79d18acf3d050".to_string(),
                x_forwarded_for: "52.255.255.12".to_string(),
                x_forwarded_port: "443".to_string(),
                x_forwarded_proto: "https".to_string()
            },
            multi_value_headers: Default::default(),
            query_string_parameters: Default::default(),
            multi_value_query_string_parameters: Default::default(),
            path_parameters: Default::default(),
            stage_variables: Default::default(),
            request_context: RequestContext {
                resource_id: "id123".to_string(),
                resource_path: "/dummy".to_string(),
                http_method: "GET".to_string(),
                extended_request_id: "ext123".to_string(),
                request_time: "10/Mar/2020:00:03:59 +0000".to_string(),
                path: "/dummy/path".to_string(),
                account_id: "123456789012".to_string(),
                protocol: "HTTP/1.1".to_string(),
                stage: "dev".to_string(),
                domain_prefix: "prefix123".to_string(),
                request_time_epoch: 1583798639428,
                request_id: "id123".to_string(),
                identity: Default::default(),
                domain_name: "".to_string(),
                api_id: "".to_string()
            },
            body: Default::default(),
            is_base64_encoded: false
        };
        let json_payload: Value = json!(payload);
        let json_payload_str = serde_json::to_string(&json_payload).unwrap();
        println!("ok {}", json_payload_str);
        let json_str = (json_payload_str).as_str();
        let json_payload_bytes = json_payload_str.into_bytes();
        let res = lambda_client.invoke(InvocationRequest {
            client_context: None,
            function_name: "dev-sg_example-service_test2".parse().unwrap(),
            invocation_type: None,
            log_type: None,
            payload: Option::from(Bytes::from(json_payload_bytes)),
            qualifier: None,
        }).await;
        let hello_str = String::from_utf8(res.unwrap().payload.unwrap().to_vec()).unwrap();
        println!("Res is {:?}", hello_str);
        // let hello: Hello = serde_json::from_str(&*hello_str).unwrap();
        // println!(" message is {} ", hello.body)
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root {
        pub resource: String,
        pub path: String,
        pub http_method: String,
        pub headers: Headers,
        pub multi_value_headers: MultiValueHeaders,
        pub query_string_parameters: ::serde_json::Value,
        pub multi_value_query_string_parameters: ::serde_json::Value,
        pub path_parameters: ::serde_json::Value,
        pub stage_variables: ::serde_json::Value,
        pub request_context: RequestContext,
        pub body: ::serde_json::Value,
        pub is_base64_encoded: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Headers {
        pub accept: String,
        #[serde(rename = "accept-encoding")]
        pub accept_encoding: String,
        #[serde(rename = "accept-language")]
        pub accept_language: String,
        pub cookie: String,
        #[serde(rename = "Host")]
        pub host: String,
        #[serde(rename = "sec-fetch-dest")]
        pub sec_fetch_dest: String,
        #[serde(rename = "sec-fetch-mode")]
        pub sec_fetch_mode: String,
        #[serde(rename = "sec-fetch-site")]
        pub sec_fetch_site: String,
        #[serde(rename = "upgrade-insecure-requests")]
        pub upgrade_insecure_requests: String,
        #[serde(rename = "User-Agent")]
        pub user_agent: String,
        #[serde(rename = "X-Amzn-Trace-Id")]
        pub x_amzn_trace_id: String,
        #[serde(rename = "X-Forwarded-For")]
        pub x_forwarded_for: String,
        #[serde(rename = "X-Forwarded-Port")]
        pub x_forwarded_port: String,
        #[serde(rename = "X-Forwarded-Proto")]
        pub x_forwarded_proto: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MultiValueHeaders {
        pub accept: Vec<String>,
        #[serde(rename = "accept-encoding")]
        pub accept_encoding: Vec<String>,
        #[serde(rename = "accept-language")]
        pub accept_language: Vec<String>,
        pub cookie: Vec<String>,
        #[serde(rename = "Host")]
        pub host: Vec<String>,
        #[serde(rename = "sec-fetch-dest")]
        pub sec_fetch_dest: Vec<String>,
        #[serde(rename = "sec-fetch-mode")]
        pub sec_fetch_mode: Vec<String>,
        #[serde(rename = "sec-fetch-site")]
        pub sec_fetch_site: Vec<String>,
        #[serde(rename = "upgrade-insecure-requests")]
        pub upgrade_insecure_requests: Vec<String>,
        #[serde(rename = "User-Agent")]
        pub user_agent: Vec<String>,
        #[serde(rename = "X-Amzn-Trace-Id")]
        pub x_amzn_trace_id: Vec<String>,
        #[serde(rename = "X-Forwarded-For")]
        pub x_forwarded_for: Vec<String>,
        #[serde(rename = "X-Forwarded-Port")]
        pub x_forwarded_port: Vec<String>,
        #[serde(rename = "X-Forwarded-Proto")]
        pub x_forwarded_proto: Vec<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestContext {
        pub resource_id: String,
        pub resource_path: String,
        pub http_method: String,
        pub extended_request_id: String,
        pub request_time: String,
        pub path: String,
        pub account_id: String,
        pub protocol: String,
        pub stage: String,
        pub domain_prefix: String,
        pub request_time_epoch: i64,
        pub request_id: String,
        pub identity: Identity,
        pub domain_name: String,
        pub api_id: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Identity {
        pub cognito_identity_pool_id: ::serde_json::Value,
        pub account_id: ::serde_json::Value,
        pub cognito_identity_id: ::serde_json::Value,
        pub caller: ::serde_json::Value,
        pub source_ip: String,
        pub principal_org_id: ::serde_json::Value,
        pub access_key: ::serde_json::Value,
        pub cognito_authentication_type: ::serde_json::Value,
        pub cognito_authentication_provider: ::serde_json::Value,
        pub user_arn: ::serde_json::Value,
        pub user_agent: String,
        pub user: ::serde_json::Value,
    }

}
