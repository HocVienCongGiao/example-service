module "test2" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/dynamodb-function"
  service_name = var.service_name

  function_name = "my-table"
  event_source_arn = "arn:aws:dynamodb:ap-southeast-1:891616054205:table/dev-sg_MyTable/stream/2021-06-21T05:18:41.574"

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}
