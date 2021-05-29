terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
    }
    tfe = {
      version = "~> 0.24.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
  access_key = var.aws_access_key_id
  secret_key = var.aws_secret_access_key
}

module "example-service" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function?ref=trigger"
  service_name = "example-service"

  s3_bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  s3_key = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
  handler = "test2"
  path_part = "example-service"
  http_method = var.http_method
  authorization = var.authorization
  integration_http_method = var.integration_http_method
  type = var.type
  aws_account_id = var.aws_account_id
  aws_region = var.aws_region
}
