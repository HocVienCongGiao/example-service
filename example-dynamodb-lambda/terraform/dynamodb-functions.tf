module "MyTable" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/dynamodb-function"
  service_name = var.service_name

  function_name = "my-table"
  table_name    = "MyTable"
  is_in_vpc     = false

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}
