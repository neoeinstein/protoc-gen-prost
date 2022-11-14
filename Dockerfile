FROM rust:1.65.0-alpine3.16@sha256:9aa6c9c3df9cfa2f8c5a648a8f1baf8fac31fc5fc7e916389296df92daccf388 as chef
RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef --version 0.1.46 --locked
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ARG BIN
RUN cargo build --release --bin $BIN

FROM scratch
ARG BIN
COPY --from=builder /app/target/release/$BIN /plugin
ENTRYPOINT ["/plugin"]
