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
  service_name = var.service_name
  aws_account_id = var.aws_account_id
  aws_region = var.aws_region
}
