FROM clux/muslrust:nightly AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
RUN apk add --no-cache tree
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/problem /app
CMD ["/app"]
