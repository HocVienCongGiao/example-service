variable "service_name" {}

variable "organisation" {
  default = "HocVienCongGiao"
}

variable "environment" {
  default = "dev-sg"
}

variable "app_type" {
  default = "apps"
}

variable "aws_region" {
  type    = string
  default = "ap-southeast-1"
}

variable "aws_access_key_id" {}
variable "aws_secret_access_key" {}


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

variable "path_part_test1" {
  type = string
}
variable "path_part_test2" {
  type = string
}

variable "http_method"{
  type = string
}
variable "authorization" {
  type = string
}
variable "integration_http_method" {
  type = string
}
variable "type" {
  type = string
}
