module "test2" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.example-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.example-service.mutation_api_gateway_resource_id
    
  function_name = "test2"
  depends_on = [
    module.example-service
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}
    
module "test2_id" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.test2.query_api_gateway_resource_id
  mutation_api_parent_id = module.test2.mutation_api_gateway_resource_id
  is_auth_required = false
    
  function_name = "test2_id"
  file_name     = "test2"
  path_part     = "{id}"
  depends_on = [
    module.example-service
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "test3" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.example-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.example-service.mutation_api_gateway_resource_id

  function_name = "test3"
  depends_on = [
    module.example-service
   ]

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}
