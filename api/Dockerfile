# cargo-chef multi-stage build for dependency caching
# see https://crates.io/crates/cargo-chef

FROM lukemathwalker/cargo-chef as planner
WORKDIR /ccolors-api
COPY ccolors-api .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR /ccolors-api
COPY --from=planner /ccolors-api/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /ccolors-api
COPY ccolors-api .
COPY --from=cacher /ccolors-api/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin ccolors-api

FROM rust as runtime
RUN mkdir -p /opt/ccolors/bin
WORKDIR /opt/ccolors/bin
COPY --from=builder /ccolors-api/target/release/ccolors-api .
COPY --from=builder /ccolors-api/.env .
COPY --from=builder /ccolors-api/Rocket.toml .
ENTRYPOINT ["/opt/ccolors/bin/ccolors-api"]