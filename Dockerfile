# Use the official Rust image to build the app
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Use a tiny runtime image to keep it fast and light
FROM debian:bookworm-slim
WORKDIR /app
# 'hello' is your binary name. If your project name in Cargo.toml is different, use that name.
COPY --from=builder /app/target/release/hello /app/hello
EXPOSE 4000
CMD ["./hello"]