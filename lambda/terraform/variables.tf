variable "service_name" {}

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
variable "private_subnet_id" {
  description = "List of private subnet id"

  default = [
    "subnet-02385ede395c1f51a",
    "subnet-0e6dd749246d5c65d", 
    "subnet-00777fd51e0927323",
  ]
  type = list(string)

}

variable "sg_id" {
  description = "List of security group id"

  default = [
    "sg-0f2acf7a4973e5d4c",
  ]
  type = list(string)

}
