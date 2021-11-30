FROM rust:latest as wasm

LABEL org.opencontainers.image.source https://github.com/mCaptcha/mCaptcha

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY browser/ /browser
WORKDIR /browser
RUN wasm-pack build --release

FROM node:16.0.0 as frontend
COPY package.json yarn.lock /src/
COPY --from=wasm /browser /src/browser
WORKDIR /src
RUN yarn install
WORKDIR /
RUN mkdir -p /src/static/cache/bundle
COPY tsconfig.json webpack.config.js jest.config.ts /src/
COPY templates /src/templates
WORKDIR /src
RUN yarn build
COPY scripts /src/scripts
RUN /src/scripts/librejs.sh

FROM rust:latest as rust
WORKDIR /src
RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml .
RUN sed -i '/.*build.rs.*/d' Cargo.toml
RUN sed -i '/.*browser.*/d' Cargo.toml
COPY Cargo.lock .
COPY migrations /src/migrations
COPY sqlx-data.json /src/
COPY src/tests-migrate.rs /src/src/tests-migrate.rs
COPY src/settings.rs /src/src/settings.rs
RUN cargo --version
RUN cargo build --release
COPY . /src
COPY --from=frontend /src/static/cache/bundle /src/static/cache/bundle
RUN cargo build --release 

FROM debian:bullseye
RUN useradd -ms /bin/bash -u 1001 mcaptcha
WORKDIR /home/mcaptcha
COPY --from=rust /src/target/release/mcaptcha /usr/local/bin/
COPY --from=rust /src/config/default.toml /etc/mcaptcha/config.toml
USER mcaptcha
CMD [ "/usr/local/bin/mcaptcha" ]
