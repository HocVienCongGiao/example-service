variable "service_name" {}

variable "aws_region" {
  type    = string
  default = "us-west-2"
}

variable "aws_account_id" {
   type = string 
 }

variable "api_key" {
  type    = string
  default = "DEFAULT_API_KEY_FOR_NOTIF"
}

variable "tfe_token" {
  type = string
}
