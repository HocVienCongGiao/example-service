data "aws_api_gateway_rest_api" "query-api" {
  name = "QueryApi"
}

# data "terraform_remote_state" "vpc" {
#   backend = "remote"
#   config = {
#     organization = "HocVienCongGiao"
#     workspaces = {
#       name = "dev-sg-lambda-apps-hvcg-vpc"
#     }
#   }
# }

data "aws_vpcs" "lambda" {
  tags = {
    Name = "dev-sg-lambda-apps-hvcg-vpc"
  }
}
data "aws_subnet_ids" "private" {
  vpc_id = tolist(data.aws_vpcs.lambda.ids)[0]
  filter {
    name   = "tag:type"
    values = ["private"]
  }
}
data "aws_security_groups" "sg" {
  filter {
    name   = "vpc-id"
    values = tolist([data.aws_subnet_ids.private.vpc_id])
  }
}

data "terraform_remote_state" "api-gateway" {
  backend = "remote"

  config = {
    organization = var.organisation
    workspaces = {
      name = "${var.environment}-lambda-apps-hvcg-api-gateway"
    }
  }
}
