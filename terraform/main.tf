data "aws_caller_identity" "current" {}
data "aws_region" "current" {}

locals {
  user_lambda_arn = "arn:aws:lambda:${data.aws_region.current.id}:${data.aws_caller_identity.current.account_id}:function:${var.user_lambda_name}"
  ai_lambda_arn   = "arn:aws:lambda:${data.aws_region.current.id}:${data.aws_caller_identity.current.account_id}:function:${var.ai_lambda_name}"
}

# -------------------------
# API Gateway HTTP API (v2)
# -------------------------
resource "aws_apigatewayv2_api" "http" {
  name          = "poc-http"
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

resource "aws_apigatewayv2_route" "ai" {
  api_id    = aws_apigatewayv2_api.http.id
  route_key = "POST /ai"
  target    = "integrations/${aws_apigatewayv2_integration.ai.id}"
}

resource "aws_apigatewayv2_stage" "default" {
  api_id      = aws_apigatewayv2_api.http.id
  name        = "$default"
  auto_deploy = true
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

resource "aws_acm_certificate_validation" "cert" {
  certificate_arn         = aws_acm_certificate.cert.arn
  validation_record_fqdns = [for dvo in aws_acm_certificate.cert.domain_validation_options : dvo.resource_record_name]
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

resource "aws_apigatewayv2_api_mapping" "mapping" {
  api_id      = aws_apigatewayv2_api.http.id
  domain_name = aws_apigatewayv2_domain_name.custom.domain_name
  stage       = aws_apigatewayv2_stage.default.id

  # base_path optionnel : si "api" => /api/...
  api_mapping_key = var.base_path != "" ? var.base_path : null
}
