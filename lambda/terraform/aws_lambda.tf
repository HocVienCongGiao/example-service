# https://learn.hashicorp.com/tutorials/terraform/lambda-api-gateway?in=terraform/aws
/*
Each Lambda function must have an associated IAM role which dictates what access it has to other AWS services. 
*/
resource "aws_iam_role" "iam_for_lambda" {
  name = "iam_for_lambda_example-service"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": 
 "sts:AssumeRole"
      ,
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

data "aws_s3_bucket_object" "bean-exampleservice-test2" {
  bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  key    = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
}

data "aws_s3_bucket_object" "bean-exampleservice-test1" {
  bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  key    = "dev-sg-hocvienconggiao/example-service/latest/test1.zip"
}

resource "aws_lambda_function" "bean-exampleservice-test2" {
  s3_bucket     = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  s3_key        = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
  function_name = "exampleservice-test2"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "test2"
  timeout       = 12

  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = base64sha256(data.aws_s3_bucket_object.bean-exampleservice-test2.last_modified)

  runtime = "provided"

  environment {
    variables = {
      API_KEY   = var.api_key
      TFE_TOKEN = var.tfe_token
    }
  }

  # Explicitly declare dependency on EFS mount target.
  # When creating or updating Lambda functions, mount target must be in 'available' lifecycle state.
  depends_on = [
    aws_iam_role_policy_attachment.lambda_logs,
    aws_cloudwatch_log_group.bean-exampleservice-test2
  ]
}

resource "aws_lambda_function" "bean-exampleservice-test1" {
  s3_bucket     = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  s3_key        = "dev-sg-hocvienconggiao/example-service/latest/test1.zip"
  function_name = "exampleservice-test1"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "test1"
  timeout       = 12

  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = base64sha256(data.aws_s3_bucket_object.bean-exampleservice-test1.last_modified)

  runtime = "provided"

  environment {
    variables = {
      API_KEY   = var.api_key
      TFE_TOKEN = var.tfe_token
    }
  }

  # Explicitly declare dependency on EFS mount target.
  # When creating or updating Lambda functions, mount target must be in 'available' lifecycle state.
  depends_on = [
    aws_iam_role_policy_attachment.lambda_logs,
    aws_cloudwatch_log_group.bean-exampleservice-test1
  ]
}

# This is to optionally manage the CloudWatch Log Group for the Lambda Function.
# If skipping this resource configuration, also add "logs:CreateLogGroup" to the IAM policy below.
resource "aws_cloudwatch_log_group" "bean-exampleservice-test2" {
  name              = "/aws/lambda/exampleservice-test2" # Should be the same as function name
  retention_in_days = 14
}
resource "aws_cloudwatch_log_group" "bean-exampleservice-test1" {
  name              = "/aws/lambda/exampleservice-test1" # Should be the same as function name
  retention_in_days = 14
}

# See also the following AWS managed policy: AWSLambdaBasicExecutionRole
resource "aws_iam_policy" "lambda_logging" {
  name        = "lambda_logging_example-service"
  path        = "/"
  description = "IAM policy for logging from a lambda"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*",
      "Effect": "Allow"
    }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "lambda_logs" {
  role       = aws_iam_role.iam_for_lambda.name
  policy_arn = aws_iam_policy.lambda_logging.arn
}
