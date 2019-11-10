FROM rust:1.39 as builder

# We need to cross-compile on non-Alpine for reasons
# see: https://github.com/rust-lang/cargo/issues/5266
# https://github.com/rust-lang/rust/issues/40174#issuecomment-538791091
RUN apt-get update
RUN apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Create a dummy application so that we can install dependencies
RUN USER=root cargo init --bin
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --target x86_64-unknown-linux-musl

RUN rm -rf src
COPY src/*.rs src/
RUN rm /app/target/x86_64-unknown-linux-musl/release/deps/rust_iap*
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-iap /app/rust-iap

EXPOSE 3000
CMD ["./rust-iap"]