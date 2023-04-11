FROM rust:1.68.2 as builder

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development

WORKDIR /usr/src

# Create blank project
RUN cargo new --bin rust-api

# Cache dependencies
COPY Cargo.toml Cargo.lock /usr/src/rust-api/

WORKDIR /usr/src/rust-api

RUN cargo build --release

COPY src /usr/src/rust-api/src/

# Touched main.rs to force rebuild
RUN touch /usr/src/rust-api/src/main.rs

RUN cargo build --release

FROM debian:buster-slim as runtime

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PROFILE=production
ENV ROCKET_LOG_LEVEL=normal
ENV SUPABASE_API_KEY=supabase_api_key
ENV SUPABASE_API_URL=supabase_api_url

EXPOSE 8000

COPY --from=builder /usr/src/rust-api/target/release/rust-first-api /usr/local/bin

CMD ["/usr/local/bin/rust-first-api"]