FROM node:10.24 as frontend

COPY . /build-frontend

FROM debian:buster

LABEL org.opencontainers.image.source https://github.com/mCaptcha/guard

RUN set -ex; \
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends yarnpkg npm ca-certificates make libssl-dev; \
    rm -rf /var/lib/apt/lists/*


WORKDIR /build-frontend
RUN pwd
RUN cd /build-frontend && npm install
RUN cd /build-frontend && npm build


FROM rust:latest as build
WORKDIR /
COPY --from=0 /build-frontend /src
RUN cargo install --path /src
RUN useradd -ms /bin/bash -u 1001 guard
WORKDIR /home/guard
USER guard
EXPOSE 7000
ENTRYPOINT [ "/usr/local/cargo/bin/guard" ]
