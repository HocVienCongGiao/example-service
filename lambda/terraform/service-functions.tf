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
  
}
