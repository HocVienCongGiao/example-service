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

# data "aws_vpcs" "lambda" {
#   filter {
#     name   = "tag:Name"
#     values = ["dev-sg-lambda-apps-hvcg-vpc"]
#   }
# }

data "aws_subnet_ids" "private" {
  vpc_id = data.aws_vpcs.lambda.ids.0
  filter {
    name   = "tag:type"
    values = ["private"]
  }
}
# data "aws_subnet" "test" {
#   for_each = data.aws_subnet_ids.private.ids
#   id       = each.value
# }
