watch:
    pass-cli run --env-file .env.local -- cargo lambda watch

build-user:
    cargo lambda build --release --arm64 --package user_service

build-ai:
    cargo lambda build --release --arm64 --package ai_service

deploy-user:
    #!/usr/bin/env bash
    set -euo pipefail
    umask 077
    tmp_env="$(mktemp)"
    # trap 'rm -f "$tmp_env"' EXIT

    pass-cli inject --in-file .env.deploy --out-file "$tmp_env" --force

    cargo lambda build --release --arm64 --package user_service

    cargo lambda deploy \
      --binary-name user_service user-service \
      --enable-function-url \
      --env-file "$tmp_env" \
      --role arn:aws:iam::977069300030:role/lambda-role

deploy-ai:
    #!/usr/bin/env bash
    set -euo pipefail
    umask 077
    tmp_env="$(mktemp)"
    # trap 'rm -f "$tmp_env"' EXIT

    pass-cli inject --in-file .env.deploy --out-file "$tmp_env" --force

    cargo lambda build --release --arm64 --package ai_service

    cargo lambda deploy \
      --binary-name ai_service ai-service \
      --enable-function-url \
      --env-file "$tmp_env" \
      --role arn:aws:iam::977069300030:role/lambda-role

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


