data "aws_s3_bucket_object" "exampleservice-test1-query-api" {
  bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  key    = "dev-sg-hocvienconggiao/example-service/latest/test1.zip"
}

module "lamda-function-test1" {
    source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//platforms/lambda/apps/hvcg/api-gateway/bounded-context/example-service/lambda-function?ref=trigger"
    statement_id = var.statement_id
    action       = var.action
    principal    = var.principal
    source_arn   = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
    source        = ""
    s3_bucket     = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
    s3_key        = "dev-sg-hocvienconggiao/example-service/latest/test1.zip"
    function_name = "exampleservice-test1"
    handler       = "test1"
    subnet_ids    = data.aws_subnet_ids.private.ids
    security_group_ids = data.aws_security_groups.sg.ids
    source_code_hash = base64sha256(data.aws_s3_bucket_object.exampleservice-test1-query-api.last_modified)
}
