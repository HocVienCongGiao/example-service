module "test2" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  parent_id = module.example-service.api_gateway_resource_id
  function_name = "test2"

  environment = var.environment
}
