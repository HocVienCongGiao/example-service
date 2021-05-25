service_name = "example-service"
path_part_test1    = "example-service"
path_part_test2    = "test2"
http_method  = "ANY"
authorization = "NONE"
integration_http_method = "POST"
type                    = "AWS_PROXY"

statement_id  = "AllowAPIGatewayInvoke"
action        = "lambda:InvokeFunction"
principal     = "apigateway.amazonaws.com"
