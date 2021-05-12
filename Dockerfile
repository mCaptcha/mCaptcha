FROM node:14.16.0 as frontend

LABEL org.opencontainers.image.source https://github.com/mCaptcha/guard

WORKDIR /src
COPY package.json yarn.lock ./
RUN yarn install
COPY . .
RUN yarn build

FROM rust:latest as rust
COPY --from=frontend /src /src
WORKDIR /src
RUN cargo build --release 

FROM debian:buster
RUN useradd -ms /bin/bash -u 1001 guard
WORKDIR /home/guard
COPY --from=rust /src/target/release/guard /usr/local/bin/
COPY --from=rust /src/config/default.toml /etc/guard/config.toml
USER guard
CMD [ "/usr/local/bin/guard" ]
