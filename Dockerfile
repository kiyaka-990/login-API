# Change from 1.75 to 1.83 or 'latest'
FROM rust:1.83-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Step 2: Tiny production image
FROM debian:bookworm-slim
WORKDIR /app
# Note: Ensure the binary name 'hello' matches your 'name' in Cargo.toml
COPY --from=builder /app/target/release/hello /app/hello
EXPOSE 4000
CMD ["./hello"]