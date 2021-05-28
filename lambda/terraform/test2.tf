
module "test2" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function?ref=trigger"
  service_name = "example-service"
  statement_id = var.statement_id
  action = var.action
  principal = var.principal
  source_arn = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
  s3_bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  s3_key = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
  function_name = "test2"
  handler = "test2"
  path_part = "test2"
  http_method = var.http_method
  authorization = var.authorization
  integration_http_method = var.integration_http_method
  type = var.type
  # uri                     = aws_lambda_function.exampleservice-test1-query-api.invoke_arn
  aws_account_id = var.aws_account_id
  aws_region = var.aws_region
  parent_id = module.example-service.api_gateway_resource_id
}

//module "lambda-function-test2" {
//    source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//platforms/lambda/apps/hvcg/api-gateway/bounded-context/example-service/lambda-function?ref=trigger"
//    statement_id = var.statement_id
//    action       = var.action
//    principal    = var.principal
//    source_arn   = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
//    s3_bucket     = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
//    s3_key        = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
//    function_name = "exampleservice-test2"
//    handler       = "test2"
//    source_code_hash = base64sha256(data.aws_s3_bucket_object.exampleservice-test2.last_modified)
//    subnet_ids         = data.aws_subnet_ids.private.ids
//    security_group_ids = data.aws_security_groups.sg.ids
//}
//
//module "create_api-resouce-test2" {
//    source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//platforms/lambda/apps/hvcg/api-gateway/bounded-context/example-service/api-gateway?ref=trigger"
//    rest_api_id = data.aws_api_gateway_rest_api.query-api.id
//    parent_id   = data.aws_api_gateway_rest_api.query-api.root_resource_id
//    path_part   = var.path_part_test2
//    http_method = var.http_method
//    authorization = var.authorization
//    integration_http_method = var.integration_http_method
//    type                    = var.type
//    # uri                     = aws_lambda_function.exampleservice-test1-query-api.invoke_arn
//    uri                     = module.lambda-function-test2.invoke_arn
//    source_arn = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
//}
