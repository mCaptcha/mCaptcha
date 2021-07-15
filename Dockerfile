FROM rust:latest as wasm

LABEL org.opencontainers.image.source https://github.com/mCaptcha/mCaptcha

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY browser/ /browser
WORKDIR /browser
RUN wasm-pack build --release

FROM node:14.16.0 as frontend
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

FROM rust:latest as rust
WORKDIR /src
COPY . .
COPY --from=frontend /src/static/cache/bundle /src/static/cache/bundle
RUN cargo build --release 

FROM debian:buster
RUN useradd -ms /bin/bash -u 1001 mcaptcha
WORKDIR /home/mcaptcha
COPY --from=rust /src/target/release/mcaptcha /usr/local/bin/
COPY --from=rust /src/config/default.toml /etc/mcaptcha/config.toml
USER mcaptcha
CMD [ "/usr/local/bin/mcaptcha" ]
