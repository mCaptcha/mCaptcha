FROM node:16.0.0 as frontend
RUN set -ex; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends make
RUN mkdir -p /src/docs/openapi/
COPY package.json yarn.lock /src/
COPY docs/openapi/package.json docs/openapi/yarn.lock /src/docs/openapi/
WORKDIR /src
RUN  yarn install && cd docs/openapi && yarn install
WORKDIR /src
RUN mkdir -p /src/static/cache/bundle
COPY tsconfig.json webpack.config.js jest.config.ts /src/
COPY templates /src/templates/
COPY docs/openapi /src/docs/openapi/
COPY Makefile /src/
COPY scripts /src/scripts
RUN make frontend

FROM rust:latest as planner
RUN cargo install cargo-chef
WORKDIR /src
COPY . /src/
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:latest as cacher
WORKDIR /src/
RUN cargo install cargo-chef
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:latest as rust
WORKDIR /src
COPY . .
COPY --from=cacher /src/target target
#COPY --from=cacher /src/db/db-core/target /src/db/db-core/target 
#COPY --from=cacher /src/db/db-sqlx-postgres/target /src/db/db-sqlx-postgres/target 
#COPY --from=cacher /src/db/db-migrations/target /src/db/db-migrations/target 
#COPY --from=cacher /src/utils/cache-bust/target /src/utils/cache-bust/target 
COPY --from=frontend /src/static/cache/bundle/ /src/static/cache/bundle/
RUN cargo --version
RUN make cache-bust
RUN cargo build --release

FROM debian:bullseye as mCaptcha
LABEL org.opencontainers.image.source https://github.com/mCaptcha/mCaptcha
RUN useradd -ms /bin/bash -u 1001 mcaptcha
WORKDIR /home/mcaptcha
COPY --from=rust /src/target/release/mcaptcha /usr/local/bin/
COPY --from=rust /src/config/default.toml /etc/mcaptcha/config.toml
USER mcaptcha
CMD [ "/usr/local/bin/mcaptcha" ]
