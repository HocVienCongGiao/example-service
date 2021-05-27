// this file for create API gateway resource, integration and permission
module "create_api-resouce-tes1" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//platforms/lambda/apps/hvcg/api-gateway/bounded-context/example-service/api-gateway?ref=trigger"
  rest_api_id = data.aws_api_gateway_rest_api.query-api.id
  parent_id   = data.aws_api_gateway_rest_api.query-api.root_resource_id
  path_part   = var.path_part_test1
  http_method = var.http_method
  authorization = var.authorization
  integration_http_method = var.integration_http_method
  type                    = var.type
  # uri                     = aws_lambda_function.exampleservice-test1-query-api.invoke_arn
  uri                     = module.lamda-function-test1.this_uri
  source_arn = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
}
