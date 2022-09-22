FROM lukemathwalker/cargo-chef:latest-rust-1.63.0-slim-buster AS chef
WORKDIR app

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    gcc \
    git \
    libssl-dev \
    pkg-config \
    cmake \
    && rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
#
#RUN apt-get update && apt-get install -y --no-install-recommends \
#    inetutils-ping
# Build application
COPY . .
RUN cargo build -r

FROM builder AS runner-case-one
ENTRYPOINT cargo run -r --bin case_one

FROM builder AS runner-case-two
ENTRYPOINT cargo run -r --bin case_two

FROM builder AS runner-router
ENTRYPOINT cargo run -r --bin router
