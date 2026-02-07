output "execute_api_url" {
  value = aws_apigatewayv2_api.http.api_endpoint
}

output "custom_domain_base" {
  value = "https://${var.api_domain_name}"
}

output "custom_domain_example_paths" {
  value = var.base_path != "" ? "https://${var.api_domain_name}/${var.base_path}/login  |  /register  |  /ai" : "https://${var.api_domain_name}/login  |  /register  |  /writer/ai"
}

output "acm_dns_validation_records" {
  value = {
    for dvo in aws_acm_certificate.cert.domain_validation_options :
    dvo.domain_name => {
      name  = dvo.resource_record_name
      type  = dvo.resource_record_type
      value = dvo.resource_record_value
    }
  }
}

output "custom_domain_dns_target" {
  value = {
    record_type    = "A (ALIAS)"
    name           = var.api_domain_name
    target         = aws_apigatewayv2_domain_name.custom.domain_name_configuration[0].target_domain_name
    hosted_zone_id = aws_apigatewayv2_domain_name.custom.domain_name_configuration[0].hosted_zone_id
  }
}
