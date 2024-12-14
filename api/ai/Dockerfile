FROM rust:1.83-slim-bookworm AS builder

# Install OpenSSL development libraries
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev cmake make \
	&& apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs and perform a build to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
	cargo build --release && \
	rm -rf src/*.rs
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl-dev cmake \
	&& apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/ai /usr/local/bin/ai
WORKDIR /usr/local/bin
EXPOSE 8080
CMD ["ai"]
