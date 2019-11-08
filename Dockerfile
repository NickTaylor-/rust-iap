FROM rust:1.39 as builder

# We need to cross-compile on non-Alpine for reasons
# see - https://github.com/rust-lang/cargo/issues/5266
#     - https://github.com/rust-lang/rust/issues/40174#issuecomment-538791091
RUN apt-get update
RUN apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

# Build dependencies as a cache layer inside of a dummy app
# We need to do this inside of a dummy app as there is no way to build just dependencies,
# and this significantly reduces build times due to caching.
# see - https://github.com/rust-lang/cargo/issues/2644
#     - https://github.com/rust-lang/cargo/issues/2644#issuecomment-335272535
WORKDIR /app
RUN USER=root cargo init .
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy + build application code
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-iap /app/rust-iap

CMD ["./rust-iap"]