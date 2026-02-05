lambda_role := "arn:aws:iam::977069300030:role/lambda-role"

watch:
    pass-cli run --env-file .env.local -- cargo lambda watch

build package:
    cargo lambda build --release --arm64 --package {{package}}

build-user:
    just build user

migrate:
    sqlx migrate run --source crates/db_migrations/migrations

build-ai:
    just build ai

deploy package binary function:
    #!/usr/bin/env bash
    set -euo pipefail
    umask 077
    tmp_env="$(mktemp)"
    # trap 'rm -f "$tmp_env"' EXIT

    pass-cli inject --in-file .env.deploy --out-file "$tmp_env" --force

    just build {{package}}

    cargo lambda deploy \
      --binary-name {{binary}} {{function}} \
      --enable-function-url \
      --env-file "$tmp_env" \
      --role {{lambda_role}}

deploy-user:
    just deploy user user user-service

deploy-ai:
    just deploy ai ai ai-service

deploy-all: deploy-user deploy-ai

autorize:
  aws lambda update-function-url-config \
  --function-name user-service \
  --auth-type NONE \
  --region eu-west-3

public:
  aws lambda add-permission \
  --function-name user-service \
  --statement-id function-url-public \
  --action lambda:InvokeFunctionUrl \
  --principal "*" \
  --function-url-auth-type NONE \
  --region eu-west-3
