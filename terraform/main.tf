data "aws_caller_identity" "current" {}
data "aws_region" "current" {}

data "aws_route53_zone" "primary" {
  count        = var.route53_zone_id == "" ? 1 : 0
  name         = var.route53_zone_name
  private_zone = false
}

locals {
  user_lambda_arn = "arn:aws:lambda:${data.aws_region.current.id}:${data.aws_caller_identity.current.account_id}:function:${var.user_lambda_name}"
  ai_lambda_arn   = "arn:aws:lambda:${data.aws_region.current.id}:${data.aws_caller_identity.current.account_id}:function:${var.ai_lambda_name}"
  route53_zone_id = var.route53_zone_id != "" ? var.route53_zone_id : data.aws_route53_zone.primary[0].zone_id
}

# -------------------------
# API Gateway HTTP API (v2)
# -------------------------
resource "aws_apigatewayv2_api" "http" {
  name          = "cealum"
  protocol_type = "HTTP"

  cors_configuration {
    allow_origins = ["*"]
    allow_methods = ["POST", "OPTIONS"]
    allow_headers = ["content-type", "authorization"]
  }
}

resource "aws_apigatewayv2_integration" "user" {
  api_id                 = aws_apigatewayv2_api.http.id
  integration_type       = "AWS_PROXY"
  integration_uri        = local.user_lambda_arn
  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_integration" "ai" {
  api_id                 = aws_apigatewayv2_api.http.id
  integration_type       = "AWS_PROXY"
  integration_uri        = local.ai_lambda_arn
  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "register" {
  api_id    = aws_apigatewayv2_api.http.id
  route_key = "POST /register"
  target    = "integrations/${aws_apigatewayv2_integration.user.id}"
}

resource "aws_apigatewayv2_route" "login" {
  api_id    = aws_apigatewayv2_api.http.id
  route_key = "POST /login"
  target    = "integrations/${aws_apigatewayv2_integration.user.id}"
}

resource "aws_apigatewayv2_route" "user_health" {
  api_id    = aws_apigatewayv2_api.http.id
  route_key = "GET /user/health"
  target    = "integrations/${aws_apigatewayv2_integration.user.id}"
}

resource "aws_apigatewayv2_route" "ai_writer" {
  api_id    = aws_apigatewayv2_api.http.id
  route_key = "POST /ai/writer"
  target    = "integrations/${aws_apigatewayv2_integration.ai.id}"
}

resource "aws_apigatewayv2_route" "ai" {
  api_id    = aws_apigatewayv2_api.http.id
  route_key = "POST /ai"
  target    = "integrations/${aws_apigatewayv2_integration.ai.id}"
}

resource "aws_apigatewayv2_stage" "default" {
  api_id      = aws_apigatewayv2_api.http.id
  name        = "$default"
  auto_deploy = true

  default_route_settings {
    throttling_burst_limit = 1
    throttling_rate_limit  = 0.33
  }
}

# Autoriser API Gateway à invoquer les Lambdas
resource "aws_lambda_permission" "apigw_user" {
  statement_id  = "AllowInvokeFromApiGwUser"
  action        = "lambda:InvokeFunction"
  function_name = var.user_lambda_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.http.execution_arn}/*/*"
}

resource "aws_lambda_permission" "apigw_ai" {
  statement_id  = "AllowInvokeFromApiGwAi"
  action        = "lambda:InvokeFunction"
  function_name = var.ai_lambda_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.http.execution_arn}/*/*"
}

# -------------------------
# ACM Certificate (DNS validation)
# IMPORTANT: cert doit être dans la même région que l'API (Regional) :contentReference[oaicite:3]{index=3}
# -------------------------
resource "aws_acm_certificate" "cert" {
  domain_name       = var.api_domain_name
  validation_method = "DNS"
}

resource "aws_route53_record" "acm_validation" {
  for_each = {
    for dvo in aws_acm_certificate.cert.domain_validation_options :
    dvo.domain_name => {
      name  = dvo.resource_record_name
      type  = dvo.resource_record_type
      value = dvo.resource_record_value
    }
  }

  zone_id = local.route53_zone_id
  name    = each.value.name
  type    = each.value.type
  ttl     = 60
  records = [each.value.value]
}

resource "aws_acm_certificate_validation" "cert" {
  certificate_arn         = aws_acm_certificate.cert.arn
  validation_record_fqdns = [for r in aws_route53_record.acm_validation : r.fqdn]
}

# -------------------------
# API Gateway Custom Domain + Mapping
# -------------------------
resource "aws_apigatewayv2_domain_name" "custom" {
  domain_name = var.api_domain_name

  domain_name_configuration {
    certificate_arn = aws_acm_certificate_validation.cert.certificate_arn
    endpoint_type   = "REGIONAL"
    security_policy = "TLS_1_2"
  }
}

resource "aws_route53_record" "api_alias" {
  zone_id = local.route53_zone_id
  name    = var.api_domain_name
  type    = "A"

  alias {
    name                   = aws_apigatewayv2_domain_name.custom.domain_name_configuration[0].target_domain_name
    zone_id                = aws_apigatewayv2_domain_name.custom.domain_name_configuration[0].hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_apigatewayv2_api_mapping" "mapping" {
  api_id      = aws_apigatewayv2_api.http.id
  domain_name = aws_apigatewayv2_domain_name.custom.domain_name
  stage       = aws_apigatewayv2_stage.default.id

  # base_path optionnel : si "api" => /api/...
  api_mapping_key = var.base_path != "" ? var.base_path : null
}
