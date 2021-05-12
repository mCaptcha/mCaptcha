FROM node:14.16.0 as frontend

LABEL org.opencontainers.image.source https://github.com/mCaptcha/guard

WORKDIR /src
COPY package.json yarn.lock ./
RUN yarn install
COPY . .
# RUN set -ex; \
#     apt-get update; \
#     DEBIAN_FRONTEND=noninteractive \
#     apt-get install -y --no-install-recommends yarnpkg npm ca-certificates make libssl-dev; \
#     rm -rf /var/lib/apt/lists/*
# RUN pwd
RUN yarn build

FROM rust:latest as rust
COPY --from=frontend /src /src
WORKDIR /src
RUN pwd
RUN ls
RUN cargo build --release 

FROM debian:buster
RUN useradd -ms /bin/bash -u 1001 guard
WORKDIR /home/guard
COPY --from=rust /src/target/release/guard /home/guard/app/
COPY --from=rust /src/config /home/guard/app/
USER guard
WORKDIR /home/guard/app
CMD [ "/home/guard/app/guard" ]
