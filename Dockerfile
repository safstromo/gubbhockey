FROM debian:bookworm-slim


RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY ./target/release/gubbhockey /app/gubbhockey
COPY ./target/site /app/site
COPY ./Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"

ENV DATABASE_URL="postgresql://dev:pass@ip:5432/gubbhockey"
ENV OAUTH_CLIENT_ID="id"
ENV OAUTH_CLIENT_SECRET="secret"
ENV OAUTH_AUTH_URL="https://url.com/authorize"
ENV OAUTH_TOKEN_URL="https://url.com/oauth/token"
ENV OAUTH_REDIRECT_URL="https://gubbhockey.com/auth"
ENV OAUTH_LOGOUT_URL="https://url.com/v2/logout?client_id=kEnQwcsluD8F7fmM0DMIiqyFwvaeiJz5&returnTo=https://gubbhockey.com"

EXPOSE 3000

CMD [ "/app/gubbhockey" ]
