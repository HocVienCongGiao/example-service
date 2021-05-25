
data "aws_s3_bucket_object" "exampleservice-test2" {
  bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  key    = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
}

module "lambda-function-test2" {
    source "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//platforms/lambda/apps/hvcg/api-gateway/bounded-context/example-service/lambda-function?ref=trigger"
    statement_id = var.statement_id
    action       = var.action
    principal    = var.principal
    source_arn   = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
    s3_bucket     = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
    s3_key        = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
    function_name = "exampleservice-test2"
    handler       = "test2"
    source_code_hash = base64sha256(data.aws_s3_bucket_object.exampleservice-test2.last_modified)
    subnet_ids         = data.aws_subnet_ids.private.ids
    security_group_ids = data.aws_security_groups.sg.ids
}
