variable "region" {
  type    = string
  default = "eu-west-3"
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
