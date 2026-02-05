variable "region" {
  type    = string
  default = "eu-west-3"
}

# Route53 zone (public) where records will be created.
variable "route53_zone_name" {
  type    = string
  default = "cealum.dev"
}

# If set, Terraform will use this zone ID instead of looking up by name.
variable "route53_zone_id" {
  type    = string
  default = ""
}

# Custom domain pour l'API (recommandé: api.cealum.dev)
variable "api_domain_name" {
  type    = string
  default = "api.cealum.dev"
}

# Préfixe de chemin optionnel, ex "api" pour avoir /api/...
# Laisse vide "" si tu veux pas de base path
variable "base_path" {
  type    = string
  default = ""
}

# Noms des Lambdas déjà déployées (par cargo-lambda)
variable "user_lambda_name" {
  type    = string
  default = "user-service"
}

variable "ai_lambda_name" {
  type    = string
  default = "ai-service"
}
